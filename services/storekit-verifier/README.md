# FocalPoint StoreKit Verifier — Cloudflare Worker

Server-side verification of Apple StoreKit 2 JWS transactions. Validates signature chains, expiry, and transaction details before unlocking entitlements.

## Endpoints

### POST /verify

Verifies a signed StoreKit 2 transaction.

**Request:**
```json
{
  "signedTransaction": "eyJhbGc...",
  "bundleId": "com.koosha.focalpoint"
}
```

**Response (Success 200):**
```json
{
  "valid": true,
  "productId": "com.focalpoint.plus.monthly",
  "originalTransactionId": "txn-1234567890",
  "expiresDate": "1719792000000",
  "environment": "Sandbox",
  "status": "Active"
}
```

**Response (Failure 401):**
```json
{
  "valid": false,
  "error": "JWS signature verification failed"
}
```

### GET /health

Health check endpoint.

**Response (200):**
```json
{
  "status": "ok"
}
```

## Verification Flow

1. **Decode JWS**: Extract header, payload, and signature from the JWT.
2. **Extract Certificate Chain**: Read x5c from JWT header.
3. **Validate Signature**: Verify signature against the leaf certificate (x5c[0]).
4. **Chain Validation**: Validate the certificate chain up to Apple's Root CA G3.
5. **Check Expiry**: Verify `expiresDate` is in the future.
6. **Return Entitlements**: On success, return product ID, original transaction ID, expiry, and status.

## Deployment

### Prerequisites

- Wrangler CLI: `bun x wrangler --version`
- Cloudflare account with Workers enabled

### Deploy to Production

```bash
# Install dependencies
bun install

# Deploy to Cloudflare
bun x wrangler deploy --env production

# View logs
bun x wrangler tail --env production
```

### Deploy to Staging

```bash
bun x wrangler deploy --env staging
```

### Local Development

```bash
bun x wrangler dev
# Worker will be available at http://localhost:8787
```

## Environment Variables

Configure in `wrangler.toml`:

| Variable | Description | Default |
|----------|-------------|---------|
| `ENVIRONMENT` | Sandbox or production | `sandbox` |
| `APPLE_ROOT_CA_PEM` | Apple Root CA G3 (optional override) | Hardcoded in `apple_ca.ts` |

## Testing

Run unit tests locally:

```bash
bun test src/index.test.ts
```

Tests cover:
1. ✅ Valid JWT decoding
2. ✅ Tampered signature detection
3. ✅ Expired transaction flagging
4. ✅ x5c certificate chain extraction
5. ✅ Malformed JWT rejection

## Example cURL

```bash
# Verify a transaction
curl -X POST https://api.focalpoint.app/storekit/verify \
  -H "Content-Type: application/json" \
  -d '{
    "signedTransaction": "eyJhbGciOiJFUzI1NiIsIng1YyI6WyJNSUlDVldDQ0FjR2dBd0lCQWdJVVYiXSwia2lkIjoia2V5LWlkLTEifQ.eyJpc3MiOiJodHRwczovL2FwcGxlaWQuYXBwbGUuY29tIiwi...",
    "bundleId": "com.koosha.focalpoint"
  }'

# Check health
curl https://api.focalpoint.app/storekit/health
```

## Fail-Open TTL Strategy (iOS Client)

If the worker is unreachable:
1. Grant entitlement locally with 24h TTL cache.
2. Store cache in Keychain alongside original receipt.
3. On next app launch, attempt worker verification.
4. If still unreachable, re-check cached TTL (extend by 24h if valid).
5. Only revoke entitlement if cache expires (avoid offline lock-outs).

## Security Considerations

- ✅ JWS signature validation required before entitlements unlock.
- ✅ x5c certificate chain validation (prevents key rotation attacks).
- ✅ Expiry checks prevent expired subscriptions from being honored.
- ❌ Full certificate chain validation not yet implemented (TODO for production).

## Notes

- Hono + Bun provides a lightweight, fast worker runtime.
- Certificate verification uses Web Crypto API (native support in Cloudflare Workers).
- For production, ensure Apple Root CA G3 is pinned in `apple_ca.ts`.
