W3-11: Storage backend unification
===
11:    [01;31m[Kpub trait[m[K EventStore: Send + Sync {
24:    [01;31m[Kpub trait[m[K RuleStore: Send + Sync {
33:    [01;31m[Kpub trait[m[K WalletStore: Send + Sync {
52:    [01;31m[Kpub trait[m[K PenaltyStore: Send + Sync {
===
33:[01;31m[Kpub struct[m[K SyncRecord {
45:[01;31m[Kpub struct[m[K SyncOutcome {
53:[01;31m[Kpub enum[m[K SyncStatus {
67:[01;31m[Kpub struct[m[K ConflictRecord {
78:[01;31m[Kpub enum[m[K ConflictResolution {
91:[01;31m[Kpub trait[m[K SyncStore: Send + Sync {
122:[01;31m[Kpub struct[m[K MemorySyncStore {
