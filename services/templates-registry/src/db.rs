//! SQLite database for template packs and ratings.

use crate::models::{PackSummary, Rating};
use anyhow::Result;
use chrono::Utc;
use focus_templates::TemplatePack;
use rusqlite::{params, Connection, OptionalExtension};
use walkdir::WalkDir;

pub struct TemplatesDb {
    path: String,
}

impl TemplatesDb {
    pub fn new(path: &str) -> Result<Self> {
        Ok(TemplatesDb {
            path: path.to_string(),
        })
    }

    fn conn(&self) -> Result<Connection> {
        Ok(Connection::open(&self.path)?)
    }

    /// Initialize database schema.
    pub fn init_schema(&self) -> Result<()> {
        let conn = self.conn()?;

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS packs (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                author TEXT NOT NULL,
                description TEXT NOT NULL,
                sha256 TEXT NOT NULL UNIQUE,
                signature TEXT,
                signed_by TEXT,
                readme TEXT,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            );

            CREATE TABLE IF NOT EXISTS ratings (
                id TEXT PRIMARY KEY,
                pack_id TEXT NOT NULL,
                rating INTEGER NOT NULL CHECK(rating >= 1 AND rating <= 5),
                comment TEXT,
                submitted_at DATETIME NOT NULL,
                ip_hash TEXT NOT NULL,
                FOREIGN KEY (pack_id) REFERENCES packs(id)
            );

            CREATE INDEX IF NOT EXISTS idx_ratings_pack_id ON ratings(pack_id);
            CREATE INDEX IF NOT EXISTS idx_ratings_ip_hash ON ratings(ip_hash);
            "#,
        )?;

        Ok(())
    }

    /// Load template packs from a catalog directory (examples/templates/).
    pub fn load_catalog_from_path(&self, catalog_path: &str) -> Result<usize> {
        let mut count = 0;
        for entry in WalkDir::new(catalog_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|x| x == "toml"))
        {
            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                if let Ok(pack) = TemplatePack::from_toml_str(&content) {
                    let sha256 = focus_templates::signing::digest_pack(&pack)?;
                    let readme = std::fs::read_to_string(
                        entry.path().with_extension("md")
                    ).ok();

                    self.upsert_pack(&pack, &sha256, None, None, readme)?;
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    /// Upsert a template pack (insert or update).
    pub fn upsert_pack(
        &self,
        pack: &TemplatePack,
        sha256: &str,
        signature: Option<&str>,
        signed_by: Option<&str>,
        readme: Option<String>,
    ) -> Result<()> {
        let conn = self.conn()?;
        let now = Utc::now();

        conn.execute(
            "INSERT OR REPLACE INTO packs
             (id, name, version, author, description, sha256, signature, signed_by, readme, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                &pack.id,
                &pack.name,
                &pack.version,
                &pack.author,
                &pack.description,
                sha256,
                signature,
                signed_by,
                readme,
                now,
                now,
            ],
        )?;

        Ok(())
    }

    /// Search packs by query (name/author substring match).
    pub fn search_packs(&self, q: &str) -> Result<Vec<PackSummary>> {
        let conn = self.conn()?;
        let query_lower = format!("%{}%", q.to_lowercase());

        let mut stmt = conn.prepare(
            "SELECT id, name, version, author, description, sha256, signed_by FROM packs
             WHERE LOWER(name) LIKE ? OR LOWER(author) LIKE ?
             ORDER BY name ASC",
        )?;

        let packs = stmt.query_map(params![&query_lower, &query_lower], |row| {
            let pack_id: String = row.get(0)?;
            let id = pack_id.clone();
            let name = row.get(1)?;
            let version = row.get(2)?;
            let author = row.get(3)?;
            let description = row.get(4)?;
            let sha256 = row.get(5)?;
            let signed_by = row.get(6)?;

            // Fetch ratings for this pack
            let (avg_rating, rating_count) = self
                .get_pack_ratings(&pack_id)
                .unwrap_or((None, 0));

            Ok(PackSummary {
                id,
                name,
                version,
                author,
                description,
                sha256,
                signed_by,
                avg_rating,
                rating_count,
            })
        })?;

        Ok(packs.collect::<std::result::Result<Vec<_>, _>>()?)
    }

    /// Get a single pack by ID.
    pub fn get_pack(&self, id: &str) -> Result<Option<crate::models::PackManifest>> {
        let conn = self.conn()?;

        let result = conn
            .query_row(
                "SELECT name, version, author, description, sha256, signature, signed_by, readme
                 FROM packs WHERE id = ?",
                params![id],
                |row| {
                    Ok(crate::models::PackManifest {
                        id: id.to_string(),
                        name: row.get(0)?,
                        version: row.get(1)?,
                        author: row.get(2)?,
                        description: row.get(3)?,
                        sha256: row.get(4)?,
                        signature: row.get(5)?,
                        signed_by: row.get(6)?,
                        readme: row.get(7)?,
                        avg_rating: None,
                        rating_count: 0,
                    })
                },
            )
            .optional()?;

        if let Some(mut manifest) = result {
            let (avg_rating, rating_count) = self.get_pack_ratings(id)?;
            manifest.avg_rating = avg_rating;
            manifest.rating_count = rating_count;
            Ok(Some(manifest))
        } else {
            Ok(None)
        }
    }

    /// Submit a rating for a pack.
    pub fn add_rating(
        &self,
        pack_id: &str,
        rating: u8,
        comment: Option<String>,
        ip_hash: &str,
    ) -> Result<()> {
        let conn = self.conn()?;
        let rating_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        conn.execute(
            "INSERT INTO ratings (id, pack_id, rating, comment, submitted_at, ip_hash)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![rating_id, pack_id, rating, comment, now, ip_hash,],
        )?;

        Ok(())
    }

    /// Get average rating and count for a pack.
    fn get_pack_ratings(&self, pack_id: &str) -> Result<(Option<f32>, usize)> {
        let conn = self.conn()?;

        let (avg, count): (Option<f32>, usize) = conn.query_row(
            "SELECT AVG(CAST(rating AS FLOAT)), COUNT(*) FROM ratings WHERE pack_id = ?",
            params![pack_id],
            |row| {
                let avg: Option<f32> = row.get(0)?;
                let count: i64 = row.get(1)?;
                Ok((avg, count as usize))
            },
        )?;

        Ok((avg, count))
    }

    /// Get all ratings for a pack (for auditing/export).
    pub fn get_ratings(&self, pack_id: &str) -> Result<Vec<Rating>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, pack_id, rating, comment, submitted_at, ip_hash FROM ratings WHERE pack_id = ?",
        )?;

        let ratings = stmt.query_map(params![pack_id], |row| {
            Ok(Rating {
                id: row.get(0)?,
                pack_id: row.get(1)?,
                rating: row.get(2)?,
                comment: row.get(3)?,
                submitted_at: row.get(4)?,
                ip_hash: row.get(5)?,
            })
        })?;

        Ok(ratings.collect::<std::result::Result<Vec<_>, _>>()?)
    }

    /// Check if an IP has reached rating limit in the last hour.
    pub fn check_rating_limit(&self, ip_hash: &str) -> Result<bool> {
        let conn = self.conn()?;
        let one_hour_ago = Utc::now() - chrono::Duration::hours(1);

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM ratings WHERE ip_hash = ? AND submitted_at > ?",
            params![ip_hash, one_hour_ago],
            |row| row.get(0),
        )?;

        Ok(count >= 10)
    }
}
