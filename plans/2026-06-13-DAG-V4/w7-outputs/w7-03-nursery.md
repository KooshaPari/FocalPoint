W7-03: clippy::nursery pass
[1m[92m    Checking[0m phenotype-error-core v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-error-core)
[1m[92m    Checking[0m focus-observability v0.0.1 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability)
[1m[92m    Checking[0m focus-coaching v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-coaching)
[1m[92m    Checking[0m focus-time v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-time)
[1m[92m    Checking[0m focus-crypto v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-crypto)
[1m[92m   Compiling[0m focus-ffi v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi)
[1m[92m   Compiling[0m melosviz-desktop v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/melosviz/desktop/src-tauri)
[1m[92m    Checking[0m focus-release-bot v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-release-bot)
[1m[92m    Checking[0m phenotype-crypto v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto)
[1m[92m    Checking[0m focus-plugin-sdk v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-time/src/lib.rs:22:5
   [1m[94m|[0m
[1m[94m22[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(initial: DateTim[1m[94m...[0m
[1m[94m23[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m24[0m [1m[94m|[0m [1m[33m|[0m             fixed: std::sync::M[1m[94m...[0m
[1m[94m25[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m26[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m22[0m [1m[94m| [0m    pub[92m const[0m fn new(initial: DateTime<Utc>) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `focus-time` (lib) generated 1 warning (run `cargo clippy --fix --lib -p focus-time -- -W clippy::nursery` to apply 1 suggestion)
[1m[92m    Checking[0m focus-icon-gen v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-icon-gen)
[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
  [1m[94m--> [0mcrates/focus-crypto/src/keychain.rs:28:1
   [1m[94m|[0m
[1m[94m28[0m [1m[94m|[0m [1m[33m/[0m /// Thread-safe in-memory [`Sec[1m[94m...[0m
[1m[94m29[0m [1m[94m|[0m [1m[33m|[0m /// exit. Intended for tests an[1m[94m...[0m
[1m[94m30[0m [1m[94m|[0m [1m[33m|[0m /// platforms when the caller h[1m[94m...[0m
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
   [1m[94m= [0m[1mnote[0m: `-W clippy::too-long-first-doc-paragraph` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::too_long_first_doc_paragraph)]`

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
  [1m[94m--> [0mcrates/focus-crypto/src/keychain.rs:70:1
   [1m[94m|[0m
[1m[94m70[0m [1m[94m|[0m [1m[33m/[0m /// No-op [`SecureSecretStore`][1m[94m...[0m
[1m[94m71[0m [1m[94m|[0m [1m[33m|[0m /// Used on build targets witho[1m[94m...[0m
[1m[94m72[0m [1m[94m|[0m [1m[33m|[0m /// Callers that want to tolera[1m[94m...[0m
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
[1m[96mhelp[0m: add an empty line
   [1m[94m|[0m
[1m[94m70[0m [1m[94m|[0m /// No-op [`SecureSecretStore`] that returns a clear error from every method.
[1m[94m71[0m [92m+ ///[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-crypto/src/keychain.rs:77:5
   [1m[94m|[0m
[1m[94m77[0m [1m[94m|[0m [1m[33m/[0m     pub fn new() -> Self {
[1m[94m78[0m [1m[94m|[0m [1m[33m|[0m         Self
[1m[94m79[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m77[0m [1m[94m| [0m    pub[92m const[0m fn new() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
   [1m[94m--> [0mcrates/focus-crypto/src/keychain.rs:109:5
    [1m[94m|[0m
[1m[94m109[0m [1m[94m|[0m [1m[33m/[0m     /// Apple Security framewo[1m[94m...[0m
[1m[94m110[0m [1m[94m|[0m [1m[33m|[0m     /// as generic passwords u[1m[94m...[0m
[1m[94m111[0m [1m[94m|[0m [1m[33m|[0m     /// restart and (on macOS)[1m[94m...[0m
[1m[94m112[0m [1m[94m|[0m [1m[33m|[0m     /// ACLs.
[1m[94m113[0m [1m[94m|[0m [1m[33m|[0m     ///
[1m[94m114[0m [1m[94m|[0m [1m[33m|[0m     /// Traces to: FR-DATA-002.
    [1m[94m|[0m [1m[33m|____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph

[1m[33mwarning[0m: `focus-crypto` (lib) generated 4 warnings (run `cargo clippy --fix --lib -p focus-crypto -- -W clippy::nursery` to apply 2 suggestions)
[1m[92m    Checking[0m agent-orchestrator v0.1.0 (/Users/kooshapari/CodeProjects/Phenotype/repos/tooling/agent-orchestrator)
[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
  [1m[94m--> [0mcrates/phenotype-error-core/src/lib.rs:10:55
   [1m[94m|[0m
[1m[94m10[0m [1m[94m|[0m [1m[94m...[0mrialize, PartialEq)]
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq
   [1m[94m= [0m[1mnote[0m: `-W clippy::derive-partial-eq-without-eq` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::derive_partial_eq_without_eq)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/phenotype-error-core/src/lib.rs:180:5
    [1m[94m|[0m
[1m[94m180[0m [1m[94m|[0m [1m[33m/[0m     pub fn is_client_error(&se[1m[94m...[0m
[1m[94m181[0m [1m[94m|[0m [1m[33m|[0m         matches!(
[1m[94m182[0m [1m[94m|[0m [1m[33m|[0m             self,
[1m[94m183[0m [1m[94m|[0m [1m[33m|[0m             Self::NotFound { .. }
[1m[94m...[0m   [1m[33m|[0m
[1m[94m190[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m180[0m [1m[94m| [0m    pub[92m const[0m fn is_client_error(&self) -> bool {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/phenotype-error-core/src/lib.rs:193:5
    [1m[94m|[0m
[1m[94m193[0m [1m[94m|[0m [1m[33m/[0m     pub fn is_server_error(&se[1m[94m...[0m
[1m[94m194[0m [1m[94m|[0m [1m[33m|[0m         matches!(
[1m[94m195[0m [1m[94m|[0m [1m[33m|[0m             self,
[1m[94m196[0m [1m[94m|[0m [1m[33m|[0m             Self::Storage { .. }
[1m[94m...[0m   [1m[33m|[0m
[1m[94m201[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m193[0m [1m[94m| [0m    pub[92m const[0m fn is_server_error(&self) -> bool {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/phenotype-error-core/src/lib.rs:204:5
    [1m[94m|[0m
[1m[94m204[0m [1m[94m|[0m [1m[33m/[0m     pub fn is_retryable(&self)[1m[94m...[0m
[1m[94m205[0m [1m[94m|[0m [1m[33m|[0m         matches!(
[1m[94m206[0m [1m[94m|[0m [1m[33m|[0m             self,
[1m[94m207[0m [1m[94m|[0m [1m[33m|[0m             Self::Timeout { ..[1m[94m...[0m
[1m[94m208[0m [1m[94m|[0m [1m[33m|[0m         )
[1m[94m209[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m204[0m [1m[94m| [0m    pub[92m const[0m fn is_retryable(&self) -> bool {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/phenotype-error-core/src/lib.rs:229:57
    [1m[94m|[0m
[1m[94m229[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/phenotype-error-core/src/lib.rs:242:5
    [1m[94m|[0m
[1m[94m242[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_request_id(mut[1m[94m...[0m
[1m[94m243[0m [1m[94m|[0m [1m[33m|[0m         self.request_id = Some[1m[94m...[0m
[1m[94m244[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m245[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m242[0m [1m[94m| [0m    pub[92m const[0m fn with_request_id(mut self, id: Uuid) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-error-core/src/lib.rs:273:50
    [1m[94m|[0m
[1m[94m273[0m [1m[94m|[0m [1m[94m...[0mxt) -> Result<T> {
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
    [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-error-core/src/lib.rs:278:56
    [1m[94m|[0m
[1m[94m278[0m [1m[94m|[0m [1m[94m...[0mg>) -> Result<T> {
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/focus-observability/src/metrics.rs:82:17
   [1m[94m|[0m
[1m[94m81[0m [1m[94m|[0m       pub fn global() -> Arc<Self> {
   [1m[94m|[0m [1m[94m __________________________________-[0m
[1m[94m82[0m [1m[94m|[0m [1m[94m|[0m         let mut instance = METRICS_IN[1m[94m...[0m
   [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^^^^[0m
[1m[94m83[0m [1m[94m|[0m [1m[94m|[0m         if let Some(metrics) = instan[1m[94m...[0m
[1m[94m84[0m [1m[94m|[0m [1m[94m|[0m             Arc::clone(metrics)
[1m[94m...[0m  [1m[94m|[0m
[1m[94m98[0m [1m[94m|[0m [1m[94m|[0m     }
   [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `instance` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
   [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
   [1m[94m|[0m
[1m[94m89[0m [92m~ [0m                    *instance = Some(Arc::clone(&arc));
[1m[94m90[0m [92m+                     drop(instance);[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
  [1m[94m--> [0mcrates/focus-observability/src/metrics.rs:83:9
   [1m[94m|[0m
[1m[94m83[0m [1m[94m|[0m [1m[33m/[0m [1m[94m...[0m   if let Some(metrics) = in[1m[94m...[0m
[1m[94m84[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       Arc::clone(metrics)
[1m[94m85[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   } else {
[1m[94m86[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       match Self::new() {
[1m[94m...[0m  [1m[33m|[0m
[1m[94m97[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   }
   [1m[94m|[0m [1m[33m|_______^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
   [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`
[1m[96mhelp[0m: try
   [1m[94m|[0m
[1m[94m83[0m [92m~ [0m        [92minstance.as_ref().map_or_else(|| match Self::new() {[0m
[1m[94m84[0m [92m+                 Ok(metrics) => {[0m
[1m[94m85[0m [92m+                     let arc = Arc::new(metrics);[0m
[1m[94m86[0m [92m+                     *instance = Some(Arc::clone(&arc));[0m
[1m[94m87[0m [92m+                     arc[0m
[1m[94m88[0m [92m+                 }[0m
[1m[94m89[0m [92m+                 Err(e) => {[0m
[1m[94m90[0m [92m+                     error!("failed to create global metrics registry: {}", e);[0m
[1m[94m91[0m [92m+                     panic!("metrics registry initialization failed: {}", e);[0m
[1m[94m92[0m [92m+                 }[0m
[1m[94m93[0m [92m+             }, |metrics| Arc::clone(metrics))[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-observability/src/metrics.rs:138:13
    [1m[94m|[0m
[1m[94m136[0m [1m[94m|[0m       pub fn gather_text_format(&self) -> anyhow::Result<String> {
    [1m[94m|[0m [1m[94m ________________________________________________________________-[0m
[1m[94m137[0m [1m[94m|[0m [1m[94m|[0m         use prometheus::Encoder;
[1m[94m138[0m [1m[94m|[0m [1m[94m|[0m         let r = self.registry.read();
    [1m[94m|[0m [1m[94m|[0m             [1m[33m^[0m
[1m[94m139[0m [1m[94m|[0m [1m[94m|[0m         let metrics = r.gather();
[1m[94m...[0m   [1m[94m|[0m
[1m[94m142[0m [1m[94m|[0m [1m[94m|[0m         Ok(String::from_utf8(buffer)?)
[1m[94m143[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `r` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m138[0m [92m~ [0m        
[1m[94m139[0m [92m+         let metrics = self.registry.read().gather();[0m
[1m[94m140[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-observability/src/privacy_filter.rs:38:5
   [1m[94m|[0m
[1m[94m38[0m [1m[94m|[0m [1m[33m/[0m     pub fn new() -> Self {
[1m[94m39[0m [1m[94m|[0m [1m[33m|[0m         Self
[1m[94m40[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m38[0m [1m[94m| [0m    pub[92m const[0m fn new() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-crypto/src/hashing.rs:41:9
   [1m[94m|[0m
[1m[94m41[0m [1m[94m|[0m         Hash(v)
   [1m[94m|[0m         [1m[33m^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-icon-gen/src/lib.rs:33:26
   [1m[94m|[0m
[1m[94m33[0m [1m[94m|[0m [1m[94m...[0mlf, other: Rgb, t: f32) -> Rgb {
   [1m[94m|[0m               [1m[33m^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-crypto/src/hashing.rs:57:5
   [1m[94m|[0m
[1m[94m57[0m [1m[94m|[0m [1m[33m/[0m     pub fn sha256() -> Self {
[1m[94m58[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m59[0m [1m[94m|[0m [1m[33m|[0m             algorithm: HashAlgo[1m[94m...[0m
[1m[94m60[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m61[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m57[0m [1m[94m| [0m    pub[92m const[0m fn sha256() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-crypto/src/hashing.rs:62:5
   [1m[94m|[0m
[1m[94m62[0m [1m[94m|[0m [1m[33m/[0m     pub fn blake3() -> Self {
[1m[94m63[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m64[0m [1m[94m|[0m [1m[33m|[0m             algorithm: HashAlgo[1m[94m...[0m
[1m[94m65[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m66[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m62[0m [1m[94m| [0m    pub[92m const[0m fn blake3() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-icon-gen/src/lib.rs:33:42
   [1m[94m|[0m
[1m[94m33[0m [1m[94m|[0m [1m[94m...[0mt: f32) -> Rgb {
   [1m[94m|[0m               [1m[33m^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-icon-gen/src/lib.rs:35:9
   [1m[94m|[0m
[1m[94m35[0m [1m[94m|[0m         Rgb {
   [1m[94m|[0m         [1m[33m^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: multiply and add expressions can be calculated more efficiently and accurately[0m
  [1m[94m--> [0mcrates/focus-icon-gen/src/lib.rs:36:16
   [1m[94m|[0m
[1m[94m36[0m [1m[94m|[0m [1m[94m...[0mr: (self.r as f32 * (1.0 - t) + other.r as f32 * t) as[1m[94m...[0m
   [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: consider using: `(self.r as f32).mul_add(1.0 - t, other.r as f32 * t)`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#suboptimal_flops
   [1m[94m= [0m[1mnote[0m: `-W clippy::suboptimal-flops` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::suboptimal_flops)]`

[1m[33mwarning[0m[1m: multiply and add expressions can be calculated more efficiently and accurately[0m
  [1m[94m--> [0mcrates/focus-icon-gen/src/lib.rs:37:16
   [1m[94m|[0m
[1m[94m37[0m [1m[94m|[0m [1m[94m...[0mg: (self.g as f32 * (1.0 - t) + other.g as f32 * t) as[1m[94m...[0m
   [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: consider using: `(self.g as f32).mul_add(1.0 - t, other.g as f32 * t)`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#suboptimal_flops

[1m[33mwarning[0m[1m: multiply and add expressions can be calculated more efficiently and accurately[0m
  [1m[94m--> [0mcrates/focus-icon-gen/src/lib.rs:38:16
   [1m[94m|[0m
[1m[94m38[0m [1m[94m|[0m [1m[94m...[0mb: (self.b as f32 * (1.0 - t) + other.b as f32 * t) as[1m[94m...[0m
   [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: consider using: `(self.b as f32).mul_add(1.0 - t, other.b as f32 * t)`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#suboptimal_flops

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-crypto/src/hashing.rs:67:5
   [1m[94m|[0m
[1m[94m67[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_algorithm(algor[1m[94m...[0m
[1m[94m68[0m [1m[94m|[0m [1m[33m|[0m         Self { algorithm }
[1m[94m69[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m67[0m [1m[94m| [0m    pub[92m const[0m fn with_algorithm(algorithm: HashAlgorithm) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-observability/src/spans.rs:23:5
   [1m[94m|[0m
[1m[94m23[0m [1m[94m|[0m [1m[33m/[0m     pub fn as_str(&self) -> &'s[1m[94m...[0m
[1m[94m24[0m [1m[94m|[0m [1m[33m|[0m         match self {
[1m[94m25[0m [1m[94m|[0m [1m[33m|[0m             SpanKind::Connector[1m[94m...[0m
[1m[94m26[0m [1m[94m|[0m [1m[33m|[0m             SpanKind::RuleEval [1m[94m...[0m
[1m[94m...[0m  [1m[33m|[0m
[1m[94m30[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m23[0m [1m[94m| [0m    pub[92m const[0m fn as_str(&self) -> &'static str {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-observability/src/spans.rs:25:13
   [1m[94m|[0m
[1m[94m25[0m [1m[94m|[0m             SpanKind::ConnectorSy[1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-observability/src/spans.rs:26:13
   [1m[94m|[0m
[1m[94m26[0m [1m[94m|[0m             SpanKind::RuleEval =>[1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-observability/src/spans.rs:27:13
   [1m[94m|[0m
[1m[94m27[0m [1m[94m|[0m             SpanKind::AuditAppend[1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-observability/src/spans.rs:28:13
   [1m[94m|[0m
[1m[94m28[0m [1m[94m|[0m             SpanKind::WalletMutat[1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-crypto/src/key_derivation.rs:25:5
   [1m[94m|[0m
[1m[94m25[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(iterations: u32,[1m[94m...[0m
[1m[94m26[0m [1m[94m|[0m [1m[33m|[0m         Self { iterations, salt }
[1m[94m27[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m25[0m [1m[94m| [0m    pub[92m const[0m fn new(iterations: u32, salt: Vec<u8>) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-crypto/src/key_derivation.rs:51:5
   [1m[94m|[0m
[1m[94m51[0m [1m[94m|[0m [1m[33m/[0m     pub fn new() -> Self {
[1m[94m52[0m [1m[94m|[0m [1m[33m|[0m         Self
[1m[94m53[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m51[0m [1m[94m| [0m    pub[92m const[0m fn new() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-observability/src/spans.rs:46:5
   [1m[94m|[0m
[1m[94m46[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(connector_id: St[1m[94m...[0m
[1m[94m47[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m48[0m [1m[94m|[0m [1m[33m|[0m             connector_id,
[1m[94m49[0m [1m[94m|[0m [1m[33m|[0m             state: None,
[1m[94m...[0m  [1m[33m|[0m
[1m[94m53[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m46[0m [1m[94m| [0m    pub[92m const[0m fn new(connector_id: String) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-observability/src/spans.rs:60:5
   [1m[94m|[0m
[1m[94m60[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_duration(mut se[1m[94m...[0m
[1m[94m61[0m [1m[94m|[0m [1m[33m|[0m         self.duration_ms = Some[1m[94m...[0m
[1m[94m62[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m63[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m60[0m [1m[94m| [0m    pub[92m const[0m fn with_duration(mut self, duration_ms: u64) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `phenotype-error-core` (lib) generated 8 warnings (run `cargo clippy --fix --lib -p phenotype-error-core -- -W clippy::nursery` to apply 8 suggestions)
[1m[92m    Checking[0m focus-errors v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-errors)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-observability/src/spans.rs:86:5
   [1m[94m|[0m
[1m[94m86[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(rule_id: String)[1m[94m...[0m
[1m[94m87[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m88[0m [1m[94m|[0m [1m[33m|[0m             rule_id,
[1m[94m89[0m [1m[94m|[0m [1m[33m|[0m             rule_type: None,
[1m[94m...[0m  [1m[33m|[0m
[1m[94m94[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m86[0m [1m[94m| [0m    pub[92m const[0m fn new(rule_id: String) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-observability/src/spans.rs:101:5
    [1m[94m|[0m
[1m[94m101[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_matched(mut se[1m[94m...[0m
[1m[94m102[0m [1m[94m|[0m [1m[33m|[0m         self.matched = Some(ma[1m[94m...[0m
[1m[94m103[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m104[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m101[0m [1m[94m| [0m    pub[92m const[0m fn with_matched(mut self, matched: bool) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-observability/src/spans.rs:106:5
    [1m[94m|[0m
[1m[94m106[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_duration(mut s[1m[94m...[0m
[1m[94m107[0m [1m[94m|[0m [1m[33m|[0m         self.duration_ms = Som[1m[94m...[0m
[1m[94m108[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m109[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m106[0m [1m[94m| [0m    pub[92m const[0m fn with_duration(mut self, duration_ms: u64) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-observability/src/spans.rs:130:5
    [1m[94m|[0m
[1m[94m130[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(audit_type: Str[1m[94m...[0m
[1m[94m131[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m132[0m [1m[94m|[0m [1m[33m|[0m             audit_type,
[1m[94m133[0m [1m[94m|[0m [1m[33m|[0m             entry_count: None,
[1m[94m...[0m   [1m[33m|[0m
[1m[94m137[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m130[0m [1m[94m| [0m    pub[92m const[0m fn new(audit_type: String) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-observability/src/spans.rs:139:5
    [1m[94m|[0m
[1m[94m139[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_entry_count(mu[1m[94m...[0m
[1m[94m140[0m [1m[94m|[0m [1m[33m|[0m         self.entry_count = Som[1m[94m...[0m
[1m[94m141[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m142[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m139[0m [1m[94m| [0m    pub[92m const[0m fn with_entry_count(mut self, count: usize) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-observability/src/spans.rs:144:5
    [1m[94m|[0m
[1m[94m144[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_duration(mut s[1m[94m...[0m
[1m[94m145[0m [1m[94m|[0m [1m[33m|[0m         self.duration_ms = Som[1m[94m...[0m
[1m[94m146[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m147[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m144[0m [1m[94m| [0m    pub[92m const[0m fn with_duration(mut self, duration_ms: u64) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `focus-icon-gen` (lib) generated 6 warnings (run `cargo clippy --fix --lib -p focus-icon-gen -- -W clippy::nursery` to apply 6 suggestions)
[1m[92m    Checking[0m phenotype-contracts v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-contracts)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-observability/src/spans.rs:167:5
    [1m[94m|[0m
[1m[94m167[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(wallet_id: Stri[1m[94m...[0m
[1m[94m168[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m169[0m [1m[94m|[0m [1m[33m|[0m             wallet_id,
[1m[94m170[0m [1m[94m|[0m [1m[33m|[0m             delta,
[1m[94m...[0m   [1m[33m|[0m
[1m[94m174[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m167[0m [1m[94m| [0m    pub[92m const[0m fn new(wallet_id: String, delta: i64) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `phenotype-crypto` (lib) generated 6 warnings (run `cargo clippy --fix --lib -p phenotype-crypto -- -W clippy::nursery` to apply 6 suggestions)
[1m[92m    Checking[0m focus-asset-fetcher v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher)
[1m[92m    Checking[0m focus-ci-watcher v0.0.1 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ci-watcher)
[1m[33mwarning[0m: `focus-observability` (lib) generated 18 warnings (run `cargo clippy --fix --lib -p focus-observability -- -W clippy::nursery` to apply 15 suggestions)
[1m[92m    Checking[0m focus-audit v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit)
[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mtooling/agent-orchestrator/src/lib.rs:105:9
    [1m[94m|[0m
[1m[94m105[0m [1m[94m|[0m         TrackerState {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
    [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[92m    Checking[0m focus-domain v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-domain)
[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-coaching/src/lib.rs:251:17
    [1m[94m|[0m
[1m[94m247[0m [1m[94m|[0m       ) -> anyhow::Result<Option<String>> {
    [1m[94m|[0m [1m[94m _________________________________________-[0m
[1m[94m248[0m [1m[94m|[0m [1m[94m|[0m         if self.responses.is_empty() {
[1m[94m249[0m [1m[94m|[0m [1m[94m|[0m             return Ok(None);
[1m[94m250[0m [1m[94m|[0m [1m[94m|[0m         }
[1m[94m251[0m [1m[94m|[0m [1m[94m|[0m         let mut idx = self.cursor.lock().exp[1m[94m...[0m
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^[0m
[1m[94m...[0m   [1m[94m|[0m
[1m[94m254[0m [1m[94m|[0m [1m[94m|[0m         Ok(Some(out))
[1m[94m255[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `idx` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m253[0m [92m~ [0m        *idx += 1;
[1m[94m254[0m [92m+         drop(idx);[0m
    [1m[94m|[0m

[1m[33mwarning[0m: `agent-orchestrator` (lib) generated 1 warning (run `cargo clippy --fix --lib -p agent-orchestrator -- -W clippy::nursery` to apply 1 suggestion)
[1m[92m    Checking[0m focus-result v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-result)
[1m[92m    Checking[0m phenotype-test-utils v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-test-utils)
[1m[33mwarning[0m: `focus-coaching` (lib) generated 1 warning
[1m[92m    Checking[0m focus-mascot v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mascot)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-asset-fetcher/src/lib.rs:36:5
   [1m[94m|[0m
[1m[94m36[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(cache_dir: PathB[1m[94m...[0m
[1m[94m37[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m38[0m [1m[94m|[0m [1m[33m|[0m             cache_dir,
[1m[94m39[0m [1m[94m|[0m [1m[33m|[0m             output_sfx_dir,
[1m[94m...[0m  [1m[33m|[0m
[1m[94m45[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m36[0m [1m[94m| [0m    pub[92m const[0m fn new(cache_dir: PathBuf, output_sfx_dir: PathBuf, output_simlish_dir: PathBuf) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-asset-fetcher/src/lib.rs:47:5
   [1m[94m|[0m
[1m[94m47[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_dry_run(mut sel[1m[94m...[0m
[1m[94m48[0m [1m[94m|[0m [1m[33m|[0m         self.dry_run = dry_run;
[1m[94m49[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m50[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m47[0m [1m[94m| [0m    pub[92m const[0m fn with_dry_run(mut self, dry_run: bool) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: exponent for bases 2 and e can be computed more accurately[0m
   [1m[94m--> [0mcrates/focus-asset-fetcher/src/lib.rs:183:49
    [1m[94m|[0m
[1m[94m183[0m [1m[94m|[0m [1m[94m...[0m", 2_f32.powf(semitones / 12.0)));
    [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: consider using: `(semitones / 12.0).exp2()`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#suboptimal_flops
    [1m[94m= [0m[1mnote[0m: `-W clippy::suboptimal-flops` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::suboptimal_flops)]`

[1m[33mwarning[0m[1m: all if blocks contain the same code at the start[0m
   [1m[94m--> [0mcrates/focus-asset-fetcher/src/lib.rs:187:5
    [1m[94m|[0m
[1m[94m187[0m [1m[94m|[0m [1m[33m/[0m     if let Some(gain_db) = asset.gain_db {
[1m[94m188[0m [1m[94m|[0m [1m[33m|[0m         cmd.push("-af".to_string());
    [1m[94m|[0m [1m[33m|____________________________________^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#branches_sharing_code
    [1m[94m= [0m[1mnote[0m: `-W clippy::branches-sharing-code` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::branches_sharing_code)]`
[1m[96mhelp[0m: consider moving these statements before the if
    [1m[94m|[0m
[1m[94m187[0m [92m~ [0m    [92mcmd.push("-af".to_string());[0m
[1m[94m188[0m [92m+     if let Some(gain_db) = asset.gain_db {[0m
    [1m[94m|[0m

[1m[92m    Checking[0m focus-hash v0.1.0 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-hash)
[1m[33mwarning[0m: `focus-asset-fetcher` (lib) generated 4 warnings (run `cargo clippy --fix --lib -p focus-asset-fetcher -- -W clippy::nursery` to apply 3 suggestions)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-audit/src/lib.rs:93:5
   [1m[94m|[0m
[1m[94m93[0m [1m[94m|[0m [1m[33m/[0m     pub fn new() -> Self {
[1m[94m94[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m95[0m [1m[94m|[0m [1m[33m|[0m             records: Vec::new(),
[1m[94m96[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m97[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m93[0m [1m[94m| [0m    pub[92m const[0m fn new() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
   [1m[94m--> [0mcrates/focus-audit/src/lib.rs:229:1
    [1m[94m|[0m
[1m[94m229[0m [1m[94m|[0m [1m[33m/[0m /// Lighter-weight injectable [1m[94m...[0m
[1m[94m230[0m [1m[94m|[0m [1m[33m|[0m /// must serialize the given p[1m[94m...[0m
[1m[94m231[0m [1m[94m|[0m [1m[33m|[0m /// their underlying store. Al[1m[94m...[0m
[1m[94m232[0m [1m[94m|[0m [1m[33m|[0m /// thread (hence `Send + Sync`).
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
    [1m[94m= [0m[1mnote[0m: `-W clippy::too-long-first-doc-paragraph` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::too_long_first_doc_paragraph)]`

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-audit/src/lib.rs:304:17
    [1m[94m|[0m
[1m[94m303[0m [1m[94m|[0m       fn append(&self, record: AuditRecord) -> anyhow::Result<()> {
    [1m[94m|[0m [1m[94m _________________________________________________________________-[0m
[1m[94m304[0m [1m[94m|[0m [1m[94m|[0m         let mut chain = self
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m305[0m [1m[94m|[0m [1m[94m|[0m             .chain
[1m[94m306[0m [1m[94m|[0m [1m[94m|[0m             .lock()
[1m[94m...[0m   [1m[94m|[0m
[1m[94m317[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m318[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `chain` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m316[0m [92m~ [0m        chain.records.push(record);
[1m[94m317[0m [92m+         drop(chain);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-audit/src/lib.rs:411:17
    [1m[94m|[0m
[1m[94m410[0m [1m[94m|[0m       ) -> anyhow::Result<()> {
    [1m[94m|[0m [1m[94m _____________________________-[0m
[1m[94m411[0m [1m[94m|[0m [1m[94m|[0m         let mut g = self
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^[0m
[1m[94m412[0m [1m[94m|[0m [1m[94m|[0m             .records
[1m[94m413[0m [1m[94m|[0m [1m[94m|[0m             .lock()
[1m[94m...[0m   [1m[94m|[0m
[1m[94m421[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m422[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `g` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m411[0m [92m~ [0m        
[1m[94m412[0m [92m+         self[0m
[1m[94m413[0m [92m+             .records[0m
[1m[94m414[0m [92m+             .lock()[0m
[1m[94m415[0m [92m+             .map_err(|e| anyhow::anyhow!("capturing audit sink poisoned: {e}"))?.push(([0m
[1m[94m416[0m [92m+             record_type.to_string(),[0m
[1m[94m417[0m [92m+             subject_ref.to_string(),[0m
[1m[94m418[0m [92m+             payload,[0m
[1m[94m419[0m [92m+             now,[0m
[1m[94m420[0m [92m+         ));[0m
[1m[94m421[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-domain/src/lib.rs:107:5
    [1m[94m|[0m
[1m[94m107[0m [1m[94m|[0m [1m[33m/[0m     pub fn is_hard(&self) -> b[1m[94m...[0m
[1m[94m108[0m [1m[94m|[0m [1m[33m|[0m         matches!(self, Rigidit[1m[94m...[0m
[1m[94m109[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m107[0m [1m[94m| [0m    pub[92m const[0m fn is_hard(&self) -> bool {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-domain/src/lib.rs:108:24
    [1m[94m|[0m
[1m[94m108[0m [1m[94m|[0m [1m[94m...[0ms!(self, Rigidity::Hard)
    [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
    [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-domain/src/lib.rs:112:5
    [1m[94m|[0m
[1m[94m112[0m [1m[94m|[0m [1m[33m/[0m     pub fn is_soft(&self) -> b[1m[94m...[0m
[1m[94m113[0m [1m[94m|[0m [1m[33m|[0m         matches!(self, Rigidit[1m[94m...[0m
[1m[94m114[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m112[0m [1m[94m| [0m    pub[92m const[0m fn is_soft(&self) -> bool {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-domain/src/lib.rs:113:24
    [1m[94m|[0m
[1m[94m113[0m [1m[94m|[0m [1m[94m...[0ms!(self, Rigidity::Soft)
    [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-domain/src/lib.rs:117:5
    [1m[94m|[0m
[1m[94m117[0m [1m[94m|[0m [1m[33m/[0m     pub fn semi_cost(&self) ->[1m[94m...[0m
[1m[94m118[0m [1m[94m|[0m [1m[33m|[0m         match self {
[1m[94m119[0m [1m[94m|[0m [1m[33m|[0m             Rigidity::Semi(c) [1m[94m...[0m
[1m[94m120[0m [1m[94m|[0m [1m[33m|[0m             _ => None,
[1m[94m121[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m122[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m117[0m [1m[94m| [0m    pub[92m const[0m fn semi_cost(&self) -> Option<&RigidityCost> {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-domain/src/lib.rs:119:13
    [1m[94m|[0m
[1m[94m119[0m [1m[94m|[0m             Rigidity::Semi(c) =>[1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m: `focus-domain` (lib) generated 6 warnings (run `cargo clippy --fix --lib -p focus-domain -- -W clippy::nursery` to apply 6 suggestions)
[1m[92m    Checking[0m focus-events v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-events)
[1m[33mwarning[0m[1m: future cannot be sent between threads safely[0m
   [1m[94m--> [0mcrates/phenotype-test-utils/src/lib.rs:195:64
    [1m[94m|[0m
[1m[94m195[0m [1m[94m|[0m [1m[94m...[0m&E) -> Result<()> {
    [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mfuture returned by `publish` is not `Send`[0m
    [1m[94m|[0m
[1m[92mnote[0m: captured value is not `Send` because `&` references cannot be sent unless their referent is `Sync`
   [1m[94m--> [0mcrates/phenotype-test-utils/src/lib.rs:195:50
    [1m[94m|[0m
[1m[94m195[0m [1m[94m|[0m [1m[94m...[0m>(&self, event: &E) -> Result[1m[94m...[0m
    [1m[94m|[0m             [1m[92m^^^^^[0m [1m[92mhas type `&E` which is not `Send`, because `E` is not `Sync`[0m
    [1m[94m= [0m[1mnote[0m: `E` doesn't implement `std::marker::Sync`
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#future_not_send
    [1m[94m= [0m[1mnote[0m: `-W clippy::future-not-send` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::future_not_send)]`

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/phenotype-test-utils/src/lib.rs:198:17
    [1m[94m|[0m
[1m[94m195[0m [1m[94m|[0m       async fn publish<E: serde::Serialize>(&self, event: &E) -> Result<()> {
    [1m[94m|[0m [1m[94m ___________________________________________________________________________-[0m
[1m[94m196[0m [1m[94m|[0m [1m[94m|[0m         let value = serde_json::to_value(event)
[1m[94m197[0m [1m[94m|[0m [1m[94m|[0m             .map_err(|e| PhenotypeError::serialization(e.to_string()))?;
[1m[94m198[0m [1m[94m|[0m [1m[94m|[0m         let mut events = self.events.lock().unwrap();
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^^[0m
[1m[94m199[0m [1m[94m|[0m [1m[94m|[0m         events.push(value);
[1m[94m200[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m201[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `events` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m198[0m [92m~ [0m        
[1m[94m199[0m [92m+         self.events.lock().unwrap().push(value);[0m
[1m[94m200[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m[1m: future cannot be sent between threads safely[0m
   [1m[94m--> [0mcrates/phenotype-test-utils/src/lib.rs:203:73
    [1m[94m|[0m
[1m[94m203[0m [1m[94m|[0m [1m[94m...[0mE]) -> Result<()> {
    [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mfuture returned by `publish_batch` is not `Send`[0m
    [1m[94m|[0m
[1m[92mnote[0m: captured value is not `Send` because `&` references cannot be sent unless their referent is `Sync`
   [1m[94m--> [0mcrates/phenotype-test-utils/src/lib.rs:203:56
    [1m[94m|[0m
[1m[94m203[0m [1m[94m|[0m [1m[94m...[0m>(&self, events: &[E]) -> Res[1m[94m...[0m
    [1m[94m|[0m             [1m[92m^^^^^^[0m [1m[92mhas type `&[E]` which is not `Send`, because `[E]` is not `Sync`[0m
    [1m[94m= [0m[1mnote[0m: `E` doesn't implement `std::marker::Sync`
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#future_not_send

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/phenotype-test-utils/src/lib.rs:204:17
    [1m[94m|[0m
[1m[94m203[0m [1m[94m|[0m       async fn publish_batch<E: serde::Serialize>(&self, events: &[E]) -> Result<()> {
    [1m[94m|[0m [1m[94m ____________________________________________________________________________________-[0m
[1m[94m204[0m [1m[94m|[0m [1m[94m|[0m         let mut store = self.events.lock().unwrap();
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m205[0m [1m[94m|[0m [1m[94m|[0m         for event in events {
[1m[94m206[0m [1m[94m|[0m [1m[94m|[0m             let value = serde_json::to_value(event)
[1m[94m...[0m   [1m[94m|[0m
[1m[94m210[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m211[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `store` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m204[0m [92m~ [0m        
[1m[94m205[0m [92m+             self.events.lock().unwrap().push(value);[0m
[1m[94m206[0m [1m[94m|[0m         for event in events {
[1m[94m207[0m [1m[94m|[0m             let value = serde_json::to_value(event)
[1m[94m208[0m [1m[94m|[0m                 .map_err(|e| PhenotypeError::serialization(e.to_string()))?;
[1m[94m209[0m [92m~ [0m            
    [1m[94m|[0m

[1m[33mwarning[0m: `focus-audit` (lib) generated 4 warnings (run `cargo clippy --fix --lib -p focus-audit -- -W clippy::nursery` to apply 1 suggestion)
[1m[92m    Checking[0m focus-rewards v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rewards)
[1m[33mwarning[0m: `phenotype-test-utils` (lib) generated 4 warnings
[1m[92m    Checking[0m focus-planning v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning)
[1m[92m    Checking[0m focus-penalties v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-hash/src/lib.rs:31:5
   [1m[94m|[0m
[1m[94m31[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(bytes: Vec<u8>) [1m[94m...[0m
[1m[94m32[0m [1m[94m|[0m [1m[33m|[0m         Self(bytes)
[1m[94m33[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m31[0m [1m[94m| [0m    pub[92m const[0m fn new(bytes: Vec<u8>) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `focus-hash` (lib) generated 1 warning (run `cargo clippy --fix --lib -p focus-hash -- -W clippy::nursery` to apply 1 suggestion)
[1m[92m    Checking[0m focus-calendar v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-calendar)
[1m[33mwarning[0m[1m: redundant clone[0m
  [1m[94m--> [0mcrates/focus-asset-fetcher/src/main.rs:79:18
   [1m[94m|[0m
[1m[94m79[0m [1m[94m|[0m         cache_dir.clone(),
   [1m[94m|[0m                  [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
   [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
  [1m[94m--> [0mcrates/focus-asset-fetcher/src/main.rs:79:9
   [1m[94m|[0m
[1m[94m79[0m [1m[94m|[0m         cache_dir.clone(),
   [1m[94m|[0m         [1m[92m^^^^^^^^^[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone
   [1m[94m= [0m[1mnote[0m: `-W clippy::redundant-clone` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::redundant_clone)]`

[1m[33mwarning[0m[1m: redundant clone[0m
  [1m[94m--> [0mcrates/focus-asset-fetcher/src/main.rs:80:23
   [1m[94m|[0m
[1m[94m80[0m [1m[94m|[0m         output_sfx_dir.clone(),
   [1m[94m|[0m                       [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
   [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
  [1m[94m--> [0mcrates/focus-asset-fetcher/src/main.rs:80:9
   [1m[94m|[0m
[1m[94m80[0m [1m[94m|[0m         output_sfx_dir.clone(),
   [1m[94m|[0m         [1m[92m^^^^^^^^^^^^^^[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone

[1m[33mwarning[0m[1m: redundant clone[0m
  [1m[94m--> [0mcrates/focus-asset-fetcher/src/main.rs:81:27
   [1m[94m|[0m
[1m[94m81[0m [1m[94m|[0m         output_simlish_dir.clone(),
   [1m[94m|[0m                           [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
   [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
  [1m[94m--> [0mcrates/focus-asset-fetcher/src/main.rs:81:9
   [1m[94m|[0m
[1m[94m81[0m [1m[94m|[0m         output_simlish_dir.clone(),
   [1m[94m|[0m         [1m[92m^^^^^^^^^^^^^^^^^^[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone

[1m[33mwarning[0m: `focus-asset-fetcher` (bin "focalpoint-fetch-assets") generated 3 warnings (run `cargo clippy --fix --bin "focalpoint-fetch-assets" -p focus-asset-fetcher -- -W clippy::nursery` to apply 3 suggestions)
[1m[92m    Checking[0m phenotype-workflow v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-workflow)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-mascot/src/lib.rs:99:5
    [1m[94m|[0m
[1m[94m 99[0m [1m[94m|[0m [1m[33m/[0m     pub fn default_bubble_for([1m[94m...[0m
[1m[94m100[0m [1m[94m|[0m [1m[33m|[0m         match pose {
[1m[94m101[0m [1m[94m|[0m [1m[33m|[0m             Pose::Confident =>[1m[94m...[0m
[1m[94m102[0m [1m[94m|[0m [1m[33m|[0m             Pose::Encouraging [1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m109[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m 99[0m [1m[94m| [0m    pub[92m const[0m fn default_bubble_for(pose: Pose) -> &'static str {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/focus-plugin-sdk/src/capabilities/http.rs:74:17
   [1m[94m|[0m
[1m[94m73[0m [1m[94m|[0m       fn check_rate_limit(&self, plugin_id: &str) -> Result<(), PluginError> {
   [1m[94m|[0m [1m[94m ____________________________________________________________________________-[0m
[1m[94m74[0m [1m[94m|[0m [1m[94m|[0m         let mut state = self.rate_limit.lock().unwrap();
   [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m75[0m [1m[94m|[0m [1m[94m|[0m         let now = Utc::now();
[1m[94m76[0m [1m[94m|[0m [1m[94m|[0m         let cutoff = now - Duration::minutes(1);
[1m[94m...[0m  [1m[94m|[0m
[1m[94m92[0m [1m[94m|[0m [1m[94m|[0m     }
   [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `state` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
   [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: merge the temporary construction with its single usage
   [1m[94m|[0m
[1m[94m74[0m [92m~ [0m        
[1m[94m75[0m [92m+         let records = self.rate_limit.lock().unwrap().or_default();[0m
[1m[94m76[0m [1m[94m|[0m         let now = Utc::now();
[1m[94m77[0m [1m[94m|[0m         let cutoff = now - Duration::minutes(1);
[1m[94m78[0m [1m[94m|[0m
[1m[94m79[0m [92m~ [0m        
   [1m[94m|[0m

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
  [1m[94m--> [0mcrates/focus-events/src/dedup.rs:20:24
   [1m[94m|[0m
[1m[94m20[0m [1m[94m|[0m #[derive(Debug, Error, PartialEq)]
   [1m[94m|[0m                        [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq
   [1m[94m= [0m[1mnote[0m: `-W clippy::derive-partial-eq-without-eq` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::derive_partial_eq_without_eq)]`

[1m[33mwarning[0m: `focus-mascot` (lib) generated 1 warning (run `cargo clippy --fix --lib -p focus-mascot -- -W clippy::nursery` to apply 1 suggestion)
[1m[92m    Checking[0m release-cut v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/tooling/release-cut)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-events/src/lib.rs:57:5
   [1m[94m|[0m
[1m[94m57[0m [1m[94m|[0m [1m[33m/[0m     pub fn as_str(&self) -> &'s[1m[94m...[0m
[1m[94m58[0m [1m[94m|[0m [1m[33m|[0m         match self {
[1m[94m59[0m [1m[94m|[0m [1m[33m|[0m             WellKnownEventType:[1m[94m...[0m
[1m[94m60[0m [1m[94m|[0m [1m[33m|[0m             WellKnownEventType:[1m[94m...[0m
[1m[94m...[0m  [1m[33m|[0m
[1m[94m71[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m57[0m [1m[94m| [0m    pub[92m const[0m fn as_str(&self) -> &'static str {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-events/src/lib.rs:59:13
   [1m[94m|[0m
[1m[94m59[0m [1m[94m|[0m [1m[94m...[0m     WellKnownEventType::Assig[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-events/src/lib.rs:60:13
   [1m[94m|[0m
[1m[94m60[0m [1m[94m|[0m [1m[94m...[0m     WellKnownEventType::Assig[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-events/src/lib.rs:61:13
   [1m[94m|[0m
[1m[94m61[0m [1m[94m|[0m [1m[94m...[0m     WellKnownEventType::Cours[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-events/src/lib.rs:62:13
   [1m[94m|[0m
[1m[94m62[0m [1m[94m|[0m [1m[94m...[0m     WellKnownEventType::Event[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-events/src/lib.rs:63:13
   [1m[94m|[0m
[1m[94m63[0m [1m[94m|[0m [1m[94m...[0m     WellKnownEventType::Event[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-events/src/lib.rs:64:13
   [1m[94m|[0m
[1m[94m64[0m [1m[94m|[0m [1m[94m...[0m     WellKnownEventType::TaskC[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-events/src/lib.rs:65:13
   [1m[94m|[0m
[1m[94m65[0m [1m[94m|[0m [1m[94m...[0m     WellKnownEventType::TaskA[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-events/src/lib.rs:66:13
   [1m[94m|[0m
[1m[94m66[0m [1m[94m|[0m [1m[94m...[0m     WellKnownEventType::Sleep[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-events/src/lib.rs:67:13
   [1m[94m|[0m
[1m[94m67[0m [1m[94m|[0m [1m[94m...[0m     WellKnownEventType::Exerc[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-penalties/src/lib.rs:18:1
   [1m[94m|[0m
[1m[94m18[0m [1m[94m|[0m [1m[33m/[0m fn default_rigidity_hard() -> R[1m[94m...[0m
[1m[94m19[0m [1m[94m|[0m [1m[33m|[0m     Rigidity::Hard
[1m[94m20[0m [1m[94m|[0m [1m[33m|[0m }
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m18[0m [1m[94m| [0m[92mconst [0mfn default_rigidity_hard() -> Rigidity {
   [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-events/src/lib.rs:68:13
   [1m[94m|[0m
[1m[94m68[0m [1m[94m|[0m [1m[94m...[0m     WellKnownEventType::AppSe[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-events/src/lib.rs:69:13
   [1m[94m|[0m
[1m[94m69[0m [1m[94m|[0m [1m[94m...[0m     WellKnownEventType::AppSe[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-rewards/src/lib.rs:67:5
   [1m[94m|[0m
[1m[94m67[0m [1m[94m|[0m [1m[33m/[0m     pub fn balance(&self) -> i64 {
[1m[94m68[0m [1m[94m|[0m [1m[33m|[0m         self.earned_credits - s[1m[94m...[0m
[1m[94m69[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m67[0m [1m[94m| [0m    pub[92m const[0m fn balance(&self) -> i64 {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
   [1m[94m--> [0mcrates/focus-events/src/lib.rs:109:9
    [1m[94m|[0m
[1m[94m109[0m [1m[94m|[0m [1m[33m/[0m         if let Some(wk) = Well[1m[94m...[0m
[1m[94m110[0m [1m[94m|[0m [1m[33m|[0m             EventType::WellKno[1m[94m...[0m
[1m[94m111[0m [1m[94m|[0m [1m[33m|[0m         } else {
[1m[94m112[0m [1m[94m|[0m [1m[33m|[0m             EventType::Custom([1m[94m...[0m
[1m[94m113[0m [1m[94m|[0m [1m[33m|[0m         }
    [1m[94m|[0m [1m[33m|_________^[0m [1m[33mhelp: try: `WellKnownEventType::from_canonical(type_str).map_or_else(|| EventType::Custom(format!("{connector_id}:{type_str}")), |wk| EventType::WellKnown(wk))`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
    [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-events/src/lib.rs:110:13
    [1m[94m|[0m
[1m[94m110[0m [1m[94m|[0m [1m[94m...[0m       EventType::WellKnown(wk)
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-events/src/lib.rs:112:13
    [1m[94m|[0m
[1m[94m112[0m [1m[94m|[0m [1m[94m...[0m       EventType::Custom(form[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-events/src/lib.rs:120:13
    [1m[94m|[0m
[1m[94m120[0m [1m[94m|[0m [1m[94m...[0m       EventType::WellKnown(w[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-events/src/lib.rs:121:13
    [1m[94m|[0m
[1m[94m121[0m [1m[94m|[0m [1m[94m...[0m       EventType::Custom(s) =[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-plugin-sdk/src/runtime.rs:99:5
    [1m[94m|[0m
[1m[94m 99[0m [1m[94m|[0m [1m[33m/[0m     pub fn is_running(&self) -[1m[94m...[0m
[1m[94m100[0m [1m[94m|[0m [1m[33m|[0m         false // Phase-1: no c[1m[94m...[0m
[1m[94m101[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m 99[0m [1m[94m| [0m    pub[92m const[0m fn is_running(&self) -> bool {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-calendar/src/lib.rs:56:5
   [1m[94m|[0m
[1m[94m56[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(start: DateTime<[1m[94m...[0m
[1m[94m57[0m [1m[94m|[0m [1m[33m|[0m         Self { start, end }
[1m[94m58[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m56[0m [1m[94m| [0m    pub[92m const[0m fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-penalties/src/lib.rs:111:5
    [1m[94m|[0m
[1m[94m111[0m [1m[94m|[0m [1m[33m/[0m     pub fn quote_bypass(&self,[1m[94m...[0m
[1m[94m112[0m [1m[94m|[0m [1m[33m|[0m         if cost < 0 {
[1m[94m113[0m [1m[94m|[0m [1m[33m|[0m             return Err(Penalty[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m127[0m [1m[94m|[0m [1m[33m|[0m         })
[1m[94m128[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m111[0m [1m[94m| [0m    pub[92m const[0m fn quote_bypass(&self, cost: i64) -> Result<BypassQuote> {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-calendar/src/lib.rs:102:13
    [1m[94m|[0m
[1m[94m101[0m [1m[94m|[0m       async fn list_events(&self, range: DateRange) -> anyhow::Result<Vec<CalendarEvent>> {
    [1m[94m|[0m [1m[94m _________________________________________________________________________________________-[0m
[1m[94m102[0m [1m[94m|[0m [1m[94m|[0m         let guard = self.inner.read().await;
    [1m[94m|[0m [1m[94m|[0m             [1m[33m^^^^^[0m
[1m[94m103[0m [1m[94m|[0m [1m[94m|[0m         let mut out: Vec<CalendarEvent> = guard
[1m[94m104[0m [1m[94m|[0m [1m[94m|[0m             .iter()
[1m[94m...[0m   [1m[94m|[0m
[1m[94m109[0m [1m[94m|[0m [1m[94m|[0m         Ok(out)
[1m[94m110[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m102[0m [92m~ [0m        
[1m[94m103[0m [92m+         let mut out = self.inner.read().await.collect();[0m
[1m[94m104[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m[1m: use Option::map_or instead of an if let/else[0m
   [1m[94m--> [0mcrates/focus-penalties/src/lib.rs:133:9
    [1m[94m|[0m
[1m[94m133[0m [1m[94m|[0m [1m[33m/[0m         match self.strict_mode[1m[94m...[0m
[1m[94m134[0m [1m[94m|[0m [1m[33m|[0m             Some(exp) => exp >[1m[94m...[0m
[1m[94m135[0m [1m[94m|[0m [1m[33m|[0m             None => false,
[1m[94m136[0m [1m[94m|[0m [1m[33m|[0m         }
    [1m[94m|[0m [1m[33m|_________^[0m [1m[33mhelp: try: `self.strict_mode_until.map_or(false, |exp| exp > now)`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
    [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-calendar/src/lib.rs:126:17
    [1m[94m|[0m
[1m[94m125[0m [1m[94m|[0m       async fn delete_event(&self, id: &str) -> anyhow::Result<()> {
    [1m[94m|[0m [1m[94m __________________________________________________________________-[0m
[1m[94m126[0m [1m[94m|[0m [1m[94m|[0m         let mut guard = self.inner.write().await;
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m127[0m [1m[94m|[0m [1m[94m|[0m         guard.retain(|e| e.id != id);
[1m[94m128[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m129[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m126[0m [92m~ [0m        
[1m[94m127[0m [92m+         self.inner.write().await.retain(|e| e.id != id);[0m
[1m[94m128[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m: `focus-events` (lib) generated 18 warnings (run `cargo clippy --fix --lib -p focus-events -- -W clippy::nursery` to apply 17 suggestions)
[1m[92m    Checking[0m focus-connectors v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors)
[1m[33mwarning[0m: `focus-rewards` (lib) generated 1 warning (run `cargo clippy --fix --lib -p focus-rewards -- -W clippy::nursery` to apply 1 suggestion)
[1m[92m    Checking[0m focus-rules v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-planning/src/lib.rs:38:5
   [1m[94m|[0m
[1m[94m38[0m [1m[94m|[0m [1m[33m/[0m     pub fn fixed(d: Duration) -[1m[94m...[0m
[1m[94m39[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m40[0m [1m[94m|[0m [1m[33m|[0m             fixed: Some(d),
[1m[94m41[0m [1m[94m|[0m [1m[33m|[0m             estimate: None,
[1m[94m42[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m43[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m38[0m [1m[94m| [0m    pub[92m const[0m fn fixed(d: Duration) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-planning/src/lib.rs:45:5
   [1m[94m|[0m
[1m[94m45[0m [1m[94m|[0m [1m[33m/[0m     pub fn estimated(p50: Durat[1m[94m...[0m
[1m[94m46[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m47[0m [1m[94m|[0m [1m[33m|[0m             fixed: None,
[1m[94m48[0m [1m[94m|[0m [1m[33m|[0m             estimate: Some(Esti[1m[94m...[0m
[1m[94m49[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m50[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m45[0m [1m[94m| [0m    pub[92m const[0m fn estimated(p50: Duration, p90: Duration) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-planning/src/lib.rs:54:5
   [1m[94m|[0m
[1m[94m54[0m [1m[94m|[0m [1m[33m/[0m     pub fn planning_duration(&s[1m[94m...[0m
[1m[94m55[0m [1m[94m|[0m [1m[33m|[0m         if let Some(f) = self.f[1m[94m...[0m
[1m[94m56[0m [1m[94m|[0m [1m[33m|[0m             return f;
[1m[94m...[0m  [1m[33m|[0m
[1m[94m61[0m [1m[94m|[0m [1m[33m|[0m         Duration::zero()
[1m[94m62[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m54[0m [1m[94m| [0m    pub[92m const[0m fn planning_duration(&self) -> Duration {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-planning/src/lib.rs:64:5
   [1m[94m|[0m
[1m[94m64[0m [1m[94m|[0m [1m[33m/[0m     pub fn is_fixed(&self) -> b[1m[94m...[0m
[1m[94m65[0m [1m[94m|[0m [1m[33m|[0m         self.fixed.is_some()
[1m[94m66[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m64[0m [1m[94m| [0m    pub[92m const[0m fn is_fixed(&self) -> bool {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-planning/src/lib.rs:93:39
   [1m[94m|[0m
[1m[94m93[0m [1m[94m|[0m [1m[94m...[0m u32) -> Priority {
   [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-planning/src/lib.rs:98:9
   [1m[94m|[0m
[1m[94m98[0m [1m[94m|[0m         Priority {
   [1m[94m|[0m         [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-planning/src/lib.rs:124:5
    [1m[94m|[0m
[1m[94m124[0m [1m[94m|[0m [1m[33m/[0m     pub fn none() -> Self {
[1m[94m125[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m126[0m [1m[94m|[0m [1m[33m|[0m             when: None,
[1m[94m127[0m [1m[94m|[0m [1m[33m|[0m             rigidity: Rigidity[1m[94m...[0m
[1m[94m128[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m129[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m124[0m [1m[94m| [0m    pub[92m const[0m fn none() -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-planning/src/lib.rs:131:5
    [1m[94m|[0m
[1m[94m131[0m [1m[94m|[0m [1m[33m/[0m     pub fn hard(when: DateTime[1m[94m...[0m
[1m[94m132[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m133[0m [1m[94m|[0m [1m[33m|[0m             when: Some(when),
[1m[94m134[0m [1m[94m|[0m [1m[33m|[0m             rigidity: Rigidity[1m[94m...[0m
[1m[94m135[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m136[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m131[0m [1m[94m| [0m    pub[92m const[0m fn hard(when: DateTime<Utc>) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-planning/src/lib.rs:138:5
    [1m[94m|[0m
[1m[94m138[0m [1m[94m|[0m [1m[33m/[0m     pub fn soft(when: DateTime[1m[94m...[0m
[1m[94m139[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m140[0m [1m[94m|[0m [1m[33m|[0m             when: Some(when),
[1m[94m141[0m [1m[94m|[0m [1m[33m|[0m             rigidity: Rigidity[1m[94m...[0m
[1m[94m142[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m143[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m138[0m [1m[94m| [0m    pub[92m const[0m fn soft(when: DateTime<Utc>) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-planning/src/lib.rs:174:5
    [1m[94m|[0m
[1m[94m174[0m [1m[94m|[0m [1m[33m/[0m     pub fn atomic() -> Self {
[1m[94m175[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m176[0m [1m[94m|[0m [1m[33m|[0m             allow_split: false,
[1m[94m177[0m [1m[94m|[0m [1m[33m|[0m             min_chunk: Duratio[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m181[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m174[0m [1m[94m| [0m    pub[92m const[0m fn atomic() -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `focus-calendar` (lib) generated 3 warnings (run `cargo clippy --fix --lib -p focus-calendar -- -W clippy::nursery` to apply 1 suggestion)
[1m[92m    Checking[0m focus-always-on v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on)
[1m[33mwarning[0m: `focus-penalties` (lib) generated 3 warnings (run `cargo clippy --fix --lib -p focus-penalties -- -W clippy::nursery` to apply 2 suggestions)
[1m[92m    Checking[0m focus-events-core v0.1.0 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-events-core)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-planning/src/lib.rs:250:5
    [1m[94m|[0m
[1m[94m250[0m [1m[94m|[0m [1m[33m/[0m     pub fn can_transition_to(&[1m[94m...[0m
[1m[94m251[0m [1m[94m|[0m [1m[33m|[0m         use TaskStatus::*;
[1m[94m252[0m [1m[94m|[0m [1m[33m|[0m         match (self, next) {
[1m[94m253[0m [1m[94m|[0m [1m[33m|[0m             (Pending, Schedule[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m263[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m250[0m [1m[94m| [0m    pub[92m const[0m fn can_transition_to(&self, next: &TaskStatus) -> bool {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-planning/src/lib.rs:250:44
    [1m[94m|[0m
[1m[94m250[0m [1m[94m|[0m [1m[94m...[0mnext: &TaskStatus) -> bool {
    [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
   [1m[94m--> [0mcrates/focus-planning/src/lib.rs:304:1
    [1m[94m|[0m
[1m[94m304[0m [1m[94m|[0m [1m[33m/[0m /// Sync persistent-task port.[1m[94m...[0m
[1m[94m305[0m [1m[94m|[0m [1m[33m|[0m /// this through `Arc<dyn Task[1m[94m...[0m
[1m[94m306[0m [1m[94m|[0m [1m[33m|[0m /// `AuditStore` pattern: SQLi[1m[94m...[0m
[1m[94m307[0m [1m[94m|[0m [1m[33m|[0m /// are safe to invoke from as[1m[94m...[0m
[1m[94m308[0m [1m[94m|[0m [1m[33m|[0m ///
[1m[94m309[0m [1m[94m|[0m [1m[33m|[0m /// Traces to: FR-DATA-001, FR[1m[94m...[0m
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
    [1m[94m= [0m[1mnote[0m: `-W clippy::too-long-first-doc-paragraph` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::too_long_first_doc_paragraph)]`

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-planning/src/lib.rs:361:17
    [1m[94m|[0m
[1m[94m360[0m [1m[94m|[0m       fn upsert(&self, user_id: uuid::Uuid, task: &Task) -> anyhow::Result<()> {
    [1m[94m|[0m [1m[94m ______________________________________________________________________________-[0m
[1m[94m361[0m [1m[94m|[0m [1m[94m|[0m         let mut g = self
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^[0m
[1m[94m362[0m [1m[94m|[0m [1m[94m|[0m             .inner
[1m[94m363[0m [1m[94m|[0m [1m[94m|[0m             .lock()
[1m[94m...[0m   [1m[94m|[0m
[1m[94m370[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m371[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `g` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m368[0m [92m~ [0m            g.push((user_id, task.clone()));
[1m[94m369[0m [92m+             drop(g);[0m
    [1m[94m|[0m

[1m[33mwarning[0m: `focus-plugin-sdk` (lib) generated 2 warnings (run `cargo clippy --fix --lib -p focus-plugin-sdk -- -W clippy::nursery` to apply 1 suggestion)
[1m[92m    Checking[0m phenotype-event-sourcing v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-event-sourcing)
[1m[33mwarning[0m: `focus-planning` (lib) generated 14 warnings (run `cargo clippy --fix --lib -p focus-planning -- -W clippy::nursery` to apply 12 suggestions)
[1m[92m    Checking[0m focus-scheduler v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler)
[1m[33mwarning[0m[1m: redundant clone[0m
   [1m[94m--> [0mtooling/release-cut/src/planner.rs:116:45
    [1m[94m|[0m
[1m[94m116[0m [1m[94m|[0m [1m[94m...[0md_version.clone(),
    [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
    [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
   [1m[94m--> [0mtooling/release-cut/src/planner.rs:116:34
    [1m[94m|[0m
[1m[94m116[0m [1m[94m|[0m [1m[94m...[0mversion: old_version.clone(),
    [1m[94m|[0m             [1m[92m^^^^^^^^^^^[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone
    [1m[94m= [0m[1mnote[0m: `-W clippy::redundant-clone` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::redundant_clone)]`

[1m[33mwarning[0m: `release-cut` (bin "release-cut") generated 1 warning (run `cargo clippy --fix --bin "release-cut" -p release-cut -- -W clippy::nursery` to apply 1 suggestion)
[1m[92m    Checking[0m phenotype-policy-engine v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-policy-engine)
[1m[92m    Checking[0m settly v0.1.0 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config)
[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
  [1m[94m--> [0mcrates/focus-icon-gen/src/bin.rs:62:19
   [1m[94m|[0m
[1m[94m62[0m [1m[94m|[0m       let out_dir = if let Some(d[1m[94m...[0m
   [1m[94m|[0m [1m[33m ___________________^[0m
[1m[94m63[0m [1m[94m|[0m [1m[33m|[0m         dir
[1m[94m64[0m [1m[94m|[0m [1m[33m|[0m     } else {
[1m[94m65[0m [1m[94m|[0m [1m[33m|[0m         PathBuf::from(
[1m[94m...[0m  [1m[33m|[0m
[1m[94m68[0m [1m[94m|[0m [1m[33m|[0m     };
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
   [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`
[1m[96mhelp[0m: try
   [1m[94m|[0m
[1m[94m62[0m [92m~ [0m    let out_dir = [92margs.output_dir.map_or_else(|| PathBuf::from([0m
[1m[94m63[0m [92m+             "../../apps/ios/FocalPoint/Sources/FocalPointApp/Resources/Assets.xcassets/AppIcon.appiconset/",[0m
[1m[94m64[0m [92m~         ), |dir| dir)[0m;
   [1m[94m|[0m

[1m[33mwarning[0m: `focus-icon-gen` (bin "focalpoint-icon-gen") generated 1 warning
[1m[92m    Checking[0m phenotype-derive v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-derive)
[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-always-on/src/lib.rs:33:13
   [1m[94m|[0m
[1m[94m33[0m [1m[94m|[0m             NudgeKind::StartFocus[1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-always-on/src/lib.rs:34:13
   [1m[94m|[0m
[1m[94m34[0m [1m[94m|[0m             NudgeKind::TakeBreak [1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-always-on/src/lib.rs:35:13
   [1m[94m|[0m
[1m[94m35[0m [1m[94m|[0m             NudgeKind::ReviewDead[1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-always-on/src/lib.rs:36:13
   [1m[94m|[0m
[1m[94m36[0m [1m[94m|[0m             NudgeKind::StreakAtRi[1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-always-on/src/lib.rs:37:13
   [1m[94m|[0m
[1m[94m37[0m [1m[94m|[0m             NudgeKind::WindDown =[1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-always-on/src/lib.rs:136:17
    [1m[94m|[0m
[1m[94m110[0m [1m[94m|[0m     #[async_instrumented]
    [1m[94m|[0m     [1m[94m---------------------[0m [1m[94mtemporary `activity` is currently being dropped at the end of its contained scope[0m
[1m[94m...[0m
[1m[94m136[0m [1m[94m|[0m         let mut activity = self.[1m[94m...[0m
    [1m[94m|[0m                 [1m[33m^^^^^^^^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m136[0m [92m~ [0m        
[1m[94m137[0m [92m+                 self.activity.lock().await.insert((day, hour), avg);[0m
[1m[94m138[0m [1m[94m|[0m         for ((day, hour), (successes, total)) in bucket_counts {
[1m[94m139[0m [1m[94m|[0m             if total > 0 {
[1m[94m140[0m [1m[94m|[0m                 let avg = successes as f32 / total as f32;
[1m[94m141[0m [92m~ [0m                
    [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-events-core/src/lib.rs:138:17
    [1m[94m|[0m
[1m[94m137[0m [1m[94m|[0m       pub async fn subscribe(&self, topic: &str) -> FocusResult<EventSubscription> {
    [1m[94m|[0m [1m[94m __________________________________________________________________________________-[0m
[1m[94m138[0m [1m[94m|[0m [1m[94m|[0m         let mut topics = self.topics.write().await;
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^^[0m
[1m[94m139[0m [1m[94m|[0m [1m[94m|[0m
[1m[94m140[0m [1m[94m|[0m [1m[94m|[0m         let sender = topics
[1m[94m...[0m   [1m[94m|[0m
[1m[94m152[0m [1m[94m|[0m [1m[94m|[0m         Ok(subscription)
[1m[94m153[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `topics` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m138[0m [92m~ [0m        
[1m[94m139[0m [92m+         let sender = self.topics.write().await.or_insert_with(|| broadcast::channel(self.config.max_subscribers).0);[0m
[1m[94m140[0m [1m[94m|[0m
[1m[94m141[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-events-core/src/lib.rs:209:5
    [1m[94m|[0m
[1m[94m209[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(subscription: E[1m[94m...[0m
[1m[94m210[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m211[0m [1m[94m|[0m [1m[33m|[0m             inner: subscription,
[1m[94m212[0m [1m[94m|[0m [1m[33m|[0m             filter,
[1m[94m213[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m214[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m209[0m [1m[94m| [0m    pub[92m const[0m fn new(subscription: EventSubscription, filter: F) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-always-on/src/lib.rs:257:1
    [1m[94m|[0m
[1m[94m257[0m [1m[94m|[0m [1m[33m/[0m fn weekday_name(day: u32) -> &[1m[94m...[0m
[1m[94m258[0m [1m[94m|[0m [1m[33m|[0m     match day {
[1m[94m259[0m [1m[94m|[0m [1m[33m|[0m         0 => "Monday",
[1m[94m260[0m [1m[94m|[0m [1m[33m|[0m         1 => "Tuesday",
[1m[94m...[0m   [1m[33m|[0m
[1m[94m268[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m257[0m [1m[94m| [0m[92mconst [0mfn weekday_name(day: u32) -> &'static str {
    [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m: `focus-events-core` (lib) generated 2 warnings (run `cargo clippy --fix --lib -p focus-events-core -- -W clippy::nursery` to apply 1 suggestion)
[1m[92m    Checking[0m focus-entitlements v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-entitlements)
[1m[33mwarning[0m: `focus-always-on` (lib) generated 7 warnings (run `cargo clippy --fix --lib -p focus-always-on -- -W clippy::nursery` to apply 6 suggestions)
[1m[92m    Checking[0m focus-sync-store v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync-store)
[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
  [1m[94m--> [0mcrates/phenotype-event-sourcing/src/event.rs:17:48
   [1m[94m|[0m
[1m[94m17[0m [1m[94m|[0m [1m[94m...[0mrialize, PartialEq, Default)]
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq
   [1m[94m= [0m[1mnote[0m: `-W clippy::derive-partial-eq-without-eq` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::derive_partial_eq_without_eq)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-event-sourcing/src/event.rs:45:5
   [1m[94m|[0m
[1m[94m45[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_causation_id(mu[1m[94m...[0m
[1m[94m46[0m [1m[94m|[0m [1m[33m|[0m         self.metadata.causation[1m[94m...[0m
[1m[94m47[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m48[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m45[0m [1m[94m| [0m    pub[92m const[0m fn with_causation_id(mut self, id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-event-sourcing/src/event.rs:50:5
   [1m[94m|[0m
[1m[94m50[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_correlation_id([1m[94m...[0m
[1m[94m51[0m [1m[94m|[0m [1m[33m|[0m         self.metadata.correlati[1m[94m...[0m
[1m[94m52[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m53[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m50[0m [1m[94m| [0m    pub[92m const[0m fn with_correlation_id(mut self, id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/phenotype-event-sourcing/src/memory.rs:46:17
   [1m[94m|[0m
[1m[94m45[0m [1m[94m|[0m       async fn append<T: Serialize>(&self, aggregate_id: &str, event: Envelope<T>) -> Result<i64> {
   [1m[94m|[0m [1m[94m _________________________________________________________________________________________________-[0m
[1m[94m46[0m [1m[94m|[0m [1m[94m|[0m         let mut store = self.events.lock().unwrap();
   [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m47[0m [1m[94m|[0m [1m[94m|[0m         let entry = store.entry(aggregate_id.to_string()).or_default();
[1m[94m48[0m [1m[94m|[0m [1m[94m|[0m         let value = serde_json::to_value(&event.payload)
[1m[94m...[0m  [1m[94m|[0m
[1m[94m59[0m [1m[94m|[0m [1m[94m|[0m         Ok(entry.len() as i64)
[1m[94m60[0m [1m[94m|[0m [1m[94m|[0m     }
   [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `store` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
   [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: merge the temporary construction with its single usage
   [1m[94m|[0m
[1m[94m46[0m [92m~ [0m        
[1m[94m47[0m [92m+         let entry = self.events.lock().unwrap().or_default();[0m
[1m[94m48[0m [92m~ [0m        
   [1m[94m|[0m

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
 [1m[94m--> [0mcrates/phenotype-event-sourcing/src/snapshot.rs:4:48
  [1m[94m|[0m
[1m[94m4[0m [1m[94m|[0m [1m[94m...[0mrialize, PartialEq)]
  [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
  [1m[94m|[0m
  [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-event-sourcing/src/snapshot.rs:20:5
   [1m[94m|[0m
[1m[94m20[0m [1m[94m|[0m [1m[33m/[0m     pub fn should_snapshot(&sel[1m[94m...[0m
[1m[94m21[0m [1m[94m|[0m [1m[33m|[0m         event_count > 0 && even[1m[94m...[0m
[1m[94m22[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m20[0m [1m[94m| [0m    pub[92m const[0m fn should_snapshot(&self, event_count: i64) -> bool {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-event-sourcing/src/upcaster.rs:14:5
   [1m[94m|[0m
[1m[94m14[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(major: u64, mino[1m[94m...[0m
[1m[94m15[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m16[0m [1m[94m|[0m [1m[33m|[0m             major,
[1m[94m17[0m [1m[94m|[0m [1m[33m|[0m             minor,
[1m[94m...[0m  [1m[33m|[0m
[1m[94m20[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m14[0m [1m[94m| [0m    pub[92m const[0m fn new(major: u64, minor: u64, patch: u64) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `phenotype-event-sourcing` (lib) generated 7 warnings (run `cargo clippy --fix --lib -p phenotype-event-sourcing -- -W clippy::nursery` to apply 6 suggestions)
[1m[92m    Checking[0m focus-telemetry v0.0.1 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-telemetry)
[1m[92m    Checking[0m bench-guard v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/tooling/bench-guard)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-policy-engine/src/policy.rs:15:5
   [1m[94m|[0m
[1m[94m15[0m [1m[94m|[0m [1m[33m/[0m     pub fn is_enabled(&self) ->[1m[94m...[0m
[1m[94m16[0m [1m[94m|[0m [1m[33m|[0m         self.enabled
[1m[94m17[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m15[0m [1m[94m| [0m    pub[92m const[0m fn is_enabled(&self) -> bool {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-scheduler/src/lib.rs:143:5
    [1m[94m|[0m
[1m[94m143[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(working_hours_d[1m[94m...[0m
[1m[94m144[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m145[0m [1m[94m|[0m [1m[33m|[0m             default_working_ho[1m[94m...[0m
[1m[94m146[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m147[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m143[0m [1m[94m| [0m    pub[92m const[0m fn new(working_hours_default: WorkingHoursSpec) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
 [1m[94m--> [0mcrates/phenotype-policy-engine/src/result.rs:4:48
  [1m[94m|[0m
[1m[94m4[0m [1m[94m|[0m [1m[94m...[0mrialize, PartialEq)]
  [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
  [1m[94m|[0m
  [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq
  [1m[94m= [0m[1mnote[0m: `-W clippy::derive-partial-eq-without-eq` implied by `-W clippy::nursery`
  [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::derive_partial_eq_without_eq)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-policy-engine/src/result.rs:12:5
   [1m[94m|[0m
[1m[94m12[0m [1m[94m|[0m [1m[33m/[0m     pub fn is_allowed(&self) ->[1m[94m...[0m
[1m[94m13[0m [1m[94m|[0m [1m[33m|[0m         matches!(self, PolicyRe[1m[94m...[0m
[1m[94m14[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m12[0m [1m[94m| [0m    pub[92m const[0m fn is_allowed(&self) -> bool {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-policy-engine/src/result.rs:13:24
   [1m[94m|[0m
[1m[94m13[0m [1m[94m|[0m [1m[94m...[0m(self, PolicyResult::Allow)
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-policy-engine/src/result.rs:16:5
   [1m[94m|[0m
[1m[94m16[0m [1m[94m|[0m [1m[33m/[0m     pub fn is_denied(&self) -> [1m[94m...[0m
[1m[94m17[0m [1m[94m|[0m [1m[33m|[0m         matches!(self, PolicyRe[1m[94m...[0m
[1m[94m18[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m16[0m [1m[94m| [0m    pub[92m const[0m fn is_denied(&self) -> bool {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-policy-engine/src/result.rs:17:24
   [1m[94m|[0m
[1m[94m17[0m [1m[94m|[0m [1m[94m...[0m(self, PolicyResult::Deny { .. })
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: function call inside of `unwrap_or`[0m
   [1m[94m--> [0mcrates/focus-scheduler/src/lib.rs:177:50
    [1m[94m|[0m
[1m[94m177[0m [1m[94m|[0m [1m[94m...[0mk).unwrap_or(self.default_working_hours.clone());
    [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: try: `unwrap_or_else(|| self.default_working_hours.clone())`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#or_fun_call
    [1m[94m= [0m[1mnote[0m: `-W clippy::or-fun-call` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::or_fun_call)]`

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
 [1m[94m--> [0mcrates/phenotype-policy-engine/src/rule.rs:4:48
  [1m[94m|[0m
[1m[94m4[0m [1m[94m|[0m [1m[94m...[0mrialize, PartialEq)]
  [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
  [1m[94m|[0m
  [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
  [1m[94m--> [0mcrates/phenotype-policy-engine/src/rule.rs:14:48
   [1m[94m|[0m
[1m[94m14[0m [1m[94m|[0m [1m[94m...[0mrialize, PartialEq)]
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: multiply and add expressions can be calculated more efficiently and accurately[0m
   [1m[94m--> [0mcrates/focus-scheduler/src/lib.rs:407:25
    [1m[94m|[0m
[1m[94m407[0m [1m[94m|[0m [1m[94m...[0m m = 1.0 + 2.0 * ratio;
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: consider using: `2.0f64.mul_add(ratio, 1.0)`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#suboptimal_flops
    [1m[94m= [0m[1mnote[0m: `-W clippy::suboptimal-flops` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::suboptimal_flops)]`

[1m[33mwarning[0m: `phenotype-policy-engine` (lib) generated 8 warnings (run `cargo clippy --fix --lib -p phenotype-policy-engine -- -W clippy::nursery` to apply 8 suggestions)
[1m[92m    Checking[0m pheno-tracing v0.1.0 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/pheno-tracing)
[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
 [1m[94m--> [0mcrates/focus-rules/src/builder.rs:1:1
  [1m[94m|[0m
[1m[94m1[0m [1m[94m|[0m [1m[33m/[0m //! Rule authoring primitives: a[1m[94m...[0m
[1m[94m2[0m [1m[94m|[0m [1m[33m|[0m //! `describe_dsl()` catalog emi[1m[94m...[0m
[1m[94m3[0m [1m[94m|[0m [1m[33m|[0m //! consumers (the in-app Rule A[1m[94m...[0m
[1m[94m4[0m [1m[94m|[0m [1m[33m|[0m //! browser-hosted connector/rul[1m[94m...[0m
[1m[94m5[0m [1m[94m|[0m [1m[33m|[0m //!
[1m[94m6[0m [1m[94m|[0m [1m[33m|[0m //! The catalog mirrors every Co[1m[94m...[0m
  [1m[94m|[0m [1m[33m|_^[0m
  [1m[94m|[0m
  [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
  [1m[94m= [0m[1mnote[0m: `-W clippy::too-long-first-doc-paragraph` implied by `-W clippy::nursery`
  [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::too_long_first_doc_paragraph)]`

[1m[33mwarning[0m: `focus-scheduler` (lib) generated 3 warnings (run `cargo clippy --fix --lib -p focus-scheduler -- -W clippy::nursery` to apply 2 suggestions)
[1m[92m    Checking[0m focus-rituals v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals)
[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
  [1m[94m--> [0mcrates/focus-connectors/src/derived.rs:34:1
   [1m[94m|[0m
[1m[94m34[0m [1m[94m|[0m [1m[33m/[0m /// Wraps a set of base connect[1m[94m...[0m
[1m[94m35[0m [1m[94m|[0m [1m[33m|[0m /// base (with the shared incom[1m[94m...[0m
[1m[94m36[0m [1m[94m|[0m [1m[33m|[0m /// cursors must compose extern[1m[94m...[0m
[1m[94m37[0m [1m[94m|[0m [1m[33m|[0m /// pipes them through the tran[1m[94m...[0m
[1m[94m38[0m [1m[94m|[0m [1m[33m|[0m ///
[1m[94m39[0m [1m[94m|[0m [1m[33m|[0m /// `next_cursor` is the lexico[1m[94m...[0m
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
   [1m[94m= [0m[1mnote[0m: `-W clippy::too-long-first-doc-paragraph` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::too_long_first_doc_paragraph)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-connectors/src/mcp_bridge.rs:49:5
   [1m[94m|[0m
[1m[94m49[0m [1m[94m|[0m [1m[33m/[0m     pub fn event_map(&self) -> [1m[94m...[0m
[1m[94m50[0m [1m[94m|[0m [1m[33m|[0m         &self.event_map
[1m[94m51[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m49[0m [1m[94m| [0m    pub[92m const[0m fn event_map(&self) -> &HashMap<String, String> {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
   [1m[94m--> [0mcrates/focus-rules/src/builder.rs:355:1
    [1m[94m|[0m
[1m[94m355[0m [1m[94m|[0m [1m[33m/[0m /// Fluent builder for [`Rule`[1m[94m...[0m
[1m[94m356[0m [1m[94m|[0m [1m[33m|[0m /// browser-hosted builders th[1m[94m...[0m
[1m[94m357[0m [1m[94m|[0m [1m[33m|[0m /// wizard constructs a [`supe[1m[94m...[0m
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-rules/src/builder.rs:381:5
    [1m[94m|[0m
[1m[94m381[0m [1m[94m|[0m [1m[33m/[0m     pub fn id(mut self, id: Uu[1m[94m...[0m
[1m[94m382[0m [1m[94m|[0m [1m[33m|[0m         self.rule.id = id;
[1m[94m383[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m384[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m381[0m [1m[94m| [0m    pub[92m const[0m fn id(mut self, id: Uuid) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-rules/src/builder.rs:399:5
    [1m[94m|[0m
[1m[94m399[0m [1m[94m|[0m [1m[33m/[0m     pub fn priority(mut self, [1m[94m...[0m
[1m[94m400[0m [1m[94m|[0m [1m[33m|[0m         self.rule.priority = p;
[1m[94m401[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m402[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m399[0m [1m[94m| [0m    pub[92m const[0m fn priority(mut self, p: i32) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-rules/src/builder.rs:404:5
    [1m[94m|[0m
[1m[94m404[0m [1m[94m|[0m [1m[33m/[0m     pub fn cooldown(mut self, [1m[94m...[0m
[1m[94m405[0m [1m[94m|[0m [1m[33m|[0m         self.rule.cooldown = S[1m[94m...[0m
[1m[94m406[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m407[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m404[0m [1m[94m| [0m    pub[92m const[0m fn cooldown(mut self, d: Duration) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-rules/src/builder.rs:409:5
    [1m[94m|[0m
[1m[94m409[0m [1m[94m|[0m [1m[33m/[0m     pub fn duration(mut self, [1m[94m...[0m
[1m[94m410[0m [1m[94m|[0m [1m[33m|[0m         self.rule.duration = S[1m[94m...[0m
[1m[94m411[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m412[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m409[0m [1m[94m| [0m    pub[92m const[0m fn duration(mut self, d: Duration) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-rules/src/builder.rs:419:5
    [1m[94m|[0m
[1m[94m419[0m [1m[94m|[0m [1m[33m/[0m     pub fn enabled(mut self, e[1m[94m...[0m
[1m[94m420[0m [1m[94m|[0m [1m[33m|[0m         self.rule.enabled = en[1m[94m...[0m
[1m[94m421[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m422[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m419[0m [1m[94m| [0m    pub[92m const[0m fn enabled(mut self, enabled: bool) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-rules/src/lib.rs:20:1
   [1m[94m|[0m
[1m[94m20[0m [1m[94m|[0m [1m[33m/[0m fn default_rigidity_hard() -> R[1m[94m...[0m
[1m[94m21[0m [1m[94m|[0m [1m[33m|[0m     Rigidity::Hard
[1m[94m22[0m [1m[94m|[0m [1m[33m|[0m }
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m20[0m [1m[94m| [0m[92mconst [0mfn default_rigidity_hard() -> Rigidity {
   [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-connectors/src/signature_verifiers.rs:133:17
    [1m[94m|[0m
[1m[94m131[0m [1m[94m|[0m     #[async_instrumented]
    [1m[94m|[0m     [1m[94m---------------------[0m [1m[94mtemporary `cache` is currently being dropped at the end of its contained scope[0m
[1m[94m132[0m [1m[94m|[0m     async fn fetch_or_cache_jwks[1m[94m...[0m
[1m[94m133[0m [1m[94m|[0m         let mut cache = self.jwk[1m[94m...[0m
    [1m[94m|[0m                 [1m[33m^^^^^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m159[0m [92m~ [0m        cache.cached_at = Some(chrono::Utc::now());
[1m[94m160[0m [92m+         drop(cache);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
  [1m[94m--> [0mcrates/phenotype-config/src/adapters/formats.rs:81:13
   [1m[94m|[0m
[1m[94m81[0m [1m[94m|[0m [1m[33m/[0m [1m[94m...[0m   if let Some(i) = n.as_i64[1m[94m...[0m
[1m[94m82[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       serde_json::Value::Nu[1m[94m...[0m
[1m[94m83[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   } else if let Some(f) = n[1m[94m...[0m
[1m[94m84[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       serde_json::Number::f[1m[94m...[0m
[1m[94m...[0m  [1m[33m|[0m
[1m[94m88[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       serde_json::Value::Null
[1m[94m89[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   }
   [1m[94m|[0m [1m[33m|_______^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
   [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`
[1m[96mhelp[0m: try
   [1m[94m|[0m
[1m[94m81[0m [92m~ [0m            [92mn.as_i64().map_or_else(|| if let Some(f) = n.as_f64() {[0m
[1m[94m82[0m [92m+                 serde_json::Number::from_f64(f)[0m
[1m[94m83[0m [92m+                     .map(serde_json::Value::Number)[0m
[1m[94m84[0m [92m+                     .unwrap_or(serde_json::Value::Null)[0m
[1m[94m85[0m [92m+             } else {[0m
[1m[94m86[0m [92m+                 serde_json::Value::Null[0m
[1m[94m87[0m [92m+             }, |i| serde_json::Value::Number(i.into()))[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:30:13
   [1m[94m|[0m
[1m[94m30[0m [1m[94m|[0m             Tier::Free => write!([1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:31:13
   [1m[94m|[0m
[1m[94m31[0m [1m[94m|[0m             Tier::Plus => write!([1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:32:13
   [1m[94m|[0m
[1m[94m32[0m [1m[94m|[0m             Tier::Pro => write!(f[1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:33:13
   [1m[94m|[0m
[1m[94m33[0m [1m[94m|[0m             Tier::Family => write[1m[94m...[0m
   [1m[94m|[0m             [1m[33m^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-config/src/adapters/sources.rs:61:5
   [1m[94m|[0m
[1m[94m61[0m [1m[94m|[0m [1m[33m/[0m     pub fn new() -> Self {
[1m[94m62[0m [1m[94m|[0m [1m[33m|[0m         Self { prefix: None }
[1m[94m63[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m61[0m [1m[94m| [0m    pub[92m const[0m fn new() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/phenotype-config/src/adapters/sources.rs:110:5
    [1m[94m|[0m
[1m[94m110[0m [1m[94m|[0m [1m[33m/[0m     pub fn new() -> Self {
[1m[94m111[0m [1m[94m|[0m [1m[33m|[0m         Self { args: Vec::new() }
[1m[94m112[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m110[0m [1m[94m| [0m    pub[92m const[0m fn new() -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-config/src/application/builder.rs:63:5
   [1m[94m|[0m
[1m[94m63[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_cli_args(self) [1m[94m...[0m
[1m[94m64[0m [1m[94m|[0m [1m[33m|[0m         // In real implementati[1m[94m...[0m
[1m[94m65[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m66[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m63[0m [1m[94m| [0m    pub[92m const[0m fn with_cli_args(self) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:50:5
   [1m[94m|[0m
[1m[94m50[0m [1m[94m|[0m [1m[33m/[0m     pub fn free() -> Self {
[1m[94m51[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m52[0m [1m[94m|[0m [1m[33m|[0m             tier: Tier::Free,
[1m[94m53[0m [1m[94m|[0m [1m[33m|[0m             expires_at: None,
[1m[94m...[0m  [1m[33m|[0m
[1m[94m56[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m50[0m [1m[94m| [0m    pub[92m const[0m fn free() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:59:5
   [1m[94m|[0m
[1m[94m59[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_tier(tier: Tier[1m[94m...[0m
[1m[94m60[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m61[0m [1m[94m|[0m [1m[33m|[0m             tier,
[1m[94m62[0m [1m[94m|[0m [1m[33m|[0m             expires_at: Some(ex[1m[94m...[0m
[1m[94m...[0m  [1m[33m|[0m
[1m[94m65[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m59[0m [1m[94m| [0m    pub[92m const[0m fn with_tier(tier: Tier, expires_at: DateTime<Utc>, receipt_signature: String) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:26:36
   [1m[94m|[0m
[1m[94m26[0m [1m[94m|[0m [1m[94m...[0mOption<ConfigPath> {
   [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:27:46
   [1m[94m|[0m
[1m[94m27[0m [1m[94m|[0m [1m[94m...[0mp, _)| ConfigPath(p.to_string()))
   [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: use Option::map_or instead of an if let/else[0m
  [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:69:9
   [1m[94m|[0m
[1m[94m69[0m [1m[94m|[0m [1m[33m/[0m         match self.expires_at {
[1m[94m70[0m [1m[94m|[0m [1m[33m|[0m             None => true, // Fr[1m[94m...[0m
[1m[94m71[0m [1m[94m|[0m [1m[33m|[0m             Some(exp) => now < [1m[94m...[0m
[1m[94m72[0m [1m[94m|[0m [1m[33m|[0m         }
   [1m[94m|[0m [1m[33m|_________^[0m [1m[33mhelp: try: `self.expires_at.map_or(true, |exp| now < exp)`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
   [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:60:15
   [1m[94m|[0m
[1m[94m60[0m [1m[94m|[0m     Array(Vec<ConfigValue>),
   [1m[94m|[0m               [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:61:28
   [1m[94m|[0m
[1m[94m61[0m [1m[94m|[0m [1m[94m...[0mtring, ConfigValue>),
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:152:1
    [1m[94m|[0m
[1m[94m152[0m [1m[94m|[0m [1m[33m/[0m pub fn connector_refresh_caden[1m[94m...[0m
[1m[94m153[0m [1m[94m|[0m [1m[33m|[0m     match entitlement.tier {
[1m[94m154[0m [1m[94m|[0m [1m[33m|[0m         Tier::Free => 240,
[1m[94m155[0m [1m[94m|[0m [1m[33m|[0m         Tier::Plus | Tier::Pro[1m[94m...[0m
[1m[94m156[0m [1m[94m|[0m [1m[33m|[0m     }
[1m[94m157[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m152[0m [1m[94m| [0mpub[92m const[0m fn connector_refresh_cadence_minutes(entitlement: &Entitlement) -> u32 {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:162:1
    [1m[94m|[0m
[1m[94m162[0m [1m[94m|[0m [1m[33m/[0m pub fn max_active_connectors(e[1m[94m...[0m
[1m[94m163[0m [1m[94m|[0m [1m[33m|[0m     match entitlement.tier {
[1m[94m164[0m [1m[94m|[0m [1m[33m|[0m         Tier::Free => 1,
[1m[94m165[0m [1m[94m|[0m [1m[33m|[0m         Tier::Plus | Tier::Pro[1m[94m...[0m
[1m[94m166[0m [1m[94m|[0m [1m[33m|[0m     }
[1m[94m167[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m162[0m [1m[94m| [0mpub[92m const[0m fn max_active_connectors(entitlement: &Entitlement) -> u32 {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:66:53
   [1m[94m|[0m
[1m[94m66[0m [1m[94m|[0m [1m[94m...[0mption<&ConfigValue> {
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:70:17
   [1m[94m|[0m
[1m[94m70[0m [1m[94m|[0m [1m[94m...[0m       ConfigValue::Object(map[1m[94m...[0m
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:80:53
   [1m[94m|[0m
[1m[94m80[0m [1m[94m|[0m [1m[94m...[0mvalue: ConfigValue) {
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:90:17
   [1m[94m|[0m
[1m[94m90[0m [1m[94m|[0m [1m[94m...[0m       ConfigValue::Object(map[1m[94m...[0m
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:92:64
   [1m[94m|[0m
[1m[94m92[0m [1m[94m|[0m [1m[94m...[0mith(|| ConfigValue::Object(Has[1m[94m...[0m
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:99:16
   [1m[94m|[0m
[1m[94m99[0m [1m[94m|[0m [1m[94m...[0mif let ConfigValue::Object(map[1m[94m...[0m
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:109:13
    [1m[94m|[0m
[1m[94m109[0m [1m[94m|[0m [1m[94m...[0m       ConfigValue::String(s)[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:110:13
    [1m[94m|[0m
[1m[94m110[0m [1m[94m|[0m [1m[94m...[0m       ConfigValue::Bool(b) =[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:111:13
    [1m[94m|[0m
[1m[94m111[0m [1m[94m|[0m [1m[94m...[0m       ConfigValue::Number(n)[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:117:5
    [1m[94m|[0m
[1m[94m117[0m [1m[94m|[0m [1m[33m/[0m     pub fn is_null(&self) -> b[1m[94m...[0m
[1m[94m118[0m [1m[94m|[0m [1m[33m|[0m         matches!(self, ConfigV[1m[94m...[0m
[1m[94m119[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m117[0m [1m[94m| [0m    pub[92m const[0m fn is_null(&self) -> bool {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:118:24
    [1m[94m|[0m
[1m[94m118[0m [1m[94m|[0m [1m[94m...[0m(self, ConfigValue::Null)
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:128:43
    [1m[94m|[0m
[1m[94m128[0m [1m[94m|[0m [1m[94m...[0mrap_or(ConfigValue::Null)
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:134:40
    [1m[94m|[0m
[1m[94m134[0m [1m[94m|[0m [1m[94m...[0mull => ConfigValue::Null,
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:135:43
    [1m[94m|[0m
[1m[94m135[0m [1m[94m|[0m [1m[94m...[0m(b) => ConfigValue::Bool(*b),
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:136:45
    [1m[94m|[0m
[1m[94m136[0m [1m[94m|[0m [1m[94m...[0m(n) => ConfigValue::Number(n.[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:137:45
    [1m[94m|[0m
[1m[94m137[0m [1m[94m|[0m [1m[94m...[0m(s) => ConfigValue::String(s.[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:139:17
    [1m[94m|[0m
[1m[94m139[0m [1m[94m|[0m [1m[94m...[0m       ConfigValue::Array(arr[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:141:47
    [1m[94m|[0m
[1m[94m141[0m [1m[94m|[0m [1m[94m...[0map) => ConfigValue::Object(
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:150:9
    [1m[94m|[0m
[1m[94m150[0m [1m[94m|[0m         ConfigValue::Bool(b)
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:156:9
    [1m[94m|[0m
[1m[94m156[0m [1m[94m|[0m         ConfigValue::String(s)
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:162:9
    [1m[94m|[0m
[1m[94m162[0m [1m[94m|[0m         ConfigValue::String(s.to[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:168:9
    [1m[94m|[0m
[1m[94m168[0m [1m[94m|[0m         ConfigValue::Number(n as[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:174:9
    [1m[94m|[0m
[1m[94m174[0m [1m[94m|[0m         ConfigValue::Number(n as[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:180:9
    [1m[94m|[0m
[1m[94m180[0m [1m[94m|[0m         ConfigValue::Number(n)
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:235:1
    [1m[94m|[0m
[1m[94m235[0m [1m[94m|[0m [1m[33m/[0m pub fn voice_provider(entitlem[1m[94m...[0m
[1m[94m236[0m [1m[94m|[0m [1m[33m|[0m     match entitlement.tier {
[1m[94m237[0m [1m[94m|[0m [1m[33m|[0m         Tier::Free => VoicePro[1m[94m...[0m
[1m[94m238[0m [1m[94m|[0m [1m[33m|[0m         Tier::Plus => VoicePro[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m241[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m235[0m [1m[94m| [0mpub[92m const[0m fn voice_provider(entitlement: &Entitlement) -> VoiceProvider {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:246:1
    [1m[94m|[0m
[1m[94m246[0m [1m[94m|[0m [1m[33m/[0m pub fn can_use_live_activity(e[1m[94m...[0m
[1m[94m247[0m [1m[94m|[0m [1m[33m|[0m     matches!(entitlement.tier,[1m[94m...[0m
[1m[94m248[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m246[0m [1m[94m| [0mpub[92m const[0m fn can_use_live_activity(entitlement: &Entitlement) -> bool {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:253:1
    [1m[94m|[0m
[1m[94m253[0m [1m[94m|[0m [1m[33m/[0m pub fn can_use_homekit_widget([1m[94m...[0m
[1m[94m254[0m [1m[94m|[0m [1m[33m|[0m     matches!(entitlement.tier,[1m[94m...[0m
[1m[94m255[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m253[0m [1m[94m| [0mpub[92m const[0m fn can_use_homekit_widget(entitlement: &Entitlement) -> bool {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:260:1
    [1m[94m|[0m
[1m[94m260[0m [1m[94m|[0m [1m[33m/[0m pub fn audit_retention_days(en[1m[94m...[0m
[1m[94m261[0m [1m[94m|[0m [1m[33m|[0m     match entitlement.tier {
[1m[94m262[0m [1m[94m|[0m [1m[33m|[0m         Tier::Free => 7,
[1m[94m263[0m [1m[94m|[0m [1m[33m|[0m         Tier::Plus => 90,
[1m[94m...[0m   [1m[33m|[0m
[1m[94m267[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m260[0m [1m[94m| [0mpub[92m const[0m fn audit_retention_days(entitlement: &Entitlement) -> u32 {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:272:1
    [1m[94m|[0m
[1m[94m272[0m [1m[94m|[0m [1m[33m/[0m pub fn can_use_cloudkit_sync(e[1m[94m...[0m
[1m[94m273[0m [1m[94m|[0m [1m[33m|[0m     matches!(entitlement.tier,[1m[94m...[0m
[1m[94m274[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m272[0m [1m[94m| [0mpub[92m const[0m fn can_use_cloudkit_sync(entitlement: &Entitlement) -> bool {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:279:1
    [1m[94m|[0m
[1m[94m279[0m [1m[94m|[0m [1m[33m/[0m pub fn nudge_limit_per_day(ent[1m[94m...[0m
[1m[94m280[0m [1m[94m|[0m [1m[33m|[0m     match entitlement.tier {
[1m[94m281[0m [1m[94m|[0m [1m[33m|[0m         Tier::Free => 0,
[1m[94m282[0m [1m[94m|[0m [1m[33m|[0m         Tier::Plus => 3,
[1m[94m...[0m   [1m[33m|[0m
[1m[94m285[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m279[0m [1m[94m| [0mpub[92m const[0m fn nudge_limit_per_day(entitlement: &Entitlement) -> u32 {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:290:1
    [1m[94m|[0m
[1m[94m290[0m [1m[94m|[0m [1m[33m/[0m pub fn has_proactive_nudges(en[1m[94m...[0m
[1m[94m291[0m [1m[94m|[0m [1m[33m|[0m     matches!(entitlement.tier,[1m[94m...[0m
[1m[94m292[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m290[0m [1m[94m| [0mpub[92m const[0m fn has_proactive_nudges(entitlement: &Entitlement) -> bool {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:297:1
    [1m[94m|[0m
[1m[94m297[0m [1m[94m|[0m [1m[33m/[0m pub fn can_customize_coachy(en[1m[94m...[0m
[1m[94m298[0m [1m[94m|[0m [1m[33m|[0m     matches!(entitlement.tier,[1m[94m...[0m
[1m[94m299[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m297[0m [1m[94m| [0mpub[92m const[0m fn can_customize_coachy(entitlement: &Entitlement) -> bool {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:304:1
    [1m[94m|[0m
[1m[94m304[0m [1m[94m|[0m [1m[33m/[0m pub fn has_template_marketplac[1m[94m...[0m
[1m[94m305[0m [1m[94m|[0m [1m[33m|[0m     matches!(entitlement.tier,[1m[94m...[0m
[1m[94m306[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m304[0m [1m[94m| [0mpub[92m const[0m fn has_template_marketplace(entitlement: &Entitlement) -> bool {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:318:1
    [1m[94m|[0m
[1m[94m318[0m [1m[94m|[0m [1m[33m/[0m pub fn analytics_tier(entitlem[1m[94m...[0m
[1m[94m319[0m [1m[94m|[0m [1m[33m|[0m     match entitlement.tier {
[1m[94m320[0m [1m[94m|[0m [1m[33m|[0m         Tier::Free => Analytic[1m[94m...[0m
[1m[94m321[0m [1m[94m|[0m [1m[33m|[0m         Tier::Plus => Analytic[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m324[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m318[0m [1m[94m| [0mpub[92m const[0m fn analytics_tier(entitlement: &Entitlement) -> AnalyticsTier {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-entitlements/src/lib.rs:343:1
    [1m[94m|[0m
[1m[94m343[0m [1m[94m|[0m [1m[33m/[0m pub fn support_priority(entitl[1m[94m...[0m
[1m[94m344[0m [1m[94m|[0m [1m[33m|[0m     match entitlement.tier {
[1m[94m345[0m [1m[94m|[0m [1m[33m|[0m         Tier::Free => SupportP[1m[94m...[0m
[1m[94m346[0m [1m[94m|[0m [1m[33m|[0m         Tier::Plus => SupportP[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m349[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m343[0m [1m[94m| [0mpub[92m const[0m fn support_priority(entitlement: &Entitlement) -> SupportPriority {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:200:5
    [1m[94m|[0m
[1m[94m200[0m [1m[94m|[0m [1m[33m/[0m     pub fn from_values(values:[1m[94m...[0m
[1m[94m201[0m [1m[94m|[0m [1m[33m|[0m         Self { values, source:[1m[94m...[0m
[1m[94m202[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m200[0m [1m[94m| [0m    pub[92m const[0m fn from_values(values: HashMap<String, ConfigValue>) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/config.rs:231:37
    [1m[94m|[0m
[1m[94m231[0m [1m[94m|[0m [1m[94m...[0m other: &Config) {
    [1m[94m|[0m             [1m[33m^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-config/src/domain/layers.rs:97:5
   [1m[94m|[0m
[1m[94m97[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_strategy(strate[1m[94m...[0m
[1m[94m98[0m [1m[94m|[0m [1m[33m|[0m         Self { layers: Vec::new[1m[94m...[0m
[1m[94m99[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m97[0m [1m[94m| [0m    pub[92m const[0m fn with_strategy(strategy: MergeStrategy) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-rules/src/lib.rs:211:5
    [1m[94m|[0m
[1m[94m211[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_cooldowns(cool[1m[94m...[0m
[1m[94m212[0m [1m[94m|[0m [1m[33m|[0m         Self { cooldowns }
[1m[94m213[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m211[0m [1m[94m| [0m    pub[92m const[0m fn with_cooldowns(cooldowns: HashMap<Uuid, DateTime<Utc>>) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-rules/src/lib.rs:216:5
    [1m[94m|[0m
[1m[94m216[0m [1m[94m|[0m [1m[33m/[0m     pub fn cooldowns(&self) ->[1m[94m...[0m
[1m[94m217[0m [1m[94m|[0m [1m[33m|[0m         &self.cooldowns
[1m[94m218[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m216[0m [1m[94m| [0m    pub[92m const[0m fn cooldowns(&self) -> &HashMap<Uuid, DateTime<Utc>> {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/phenotype-config/src/domain/validation.rs:21:5
   [1m[94m|[0m
[1m[94m21[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(keys: Vec<String[1m[94m...[0m
[1m[94m22[0m [1m[94m|[0m [1m[33m|[0m         Self { keys }
[1m[94m23[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m21[0m [1m[94m| [0m    pub[92m const[0m fn new(keys: Vec<String>) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this looks like a formatting argument but it is not part of a formatting macro[0m
   [1m[94m--> [0mcrates/focus-rules/src/lib.rs:486:23
    [1m[94m|[0m
[1m[94m486[0m [1m[94m|[0m [1m[94m...[0m   .replace("{event_type}", &[1m[94m...[0m
    [1m[94m|[0m                 [1m[33m^^^^^^^^^^^^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#literal_string_with_formatting_args
    [1m[94m= [0m[1mnote[0m: `-W clippy::literal-string-with-formatting-args` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::literal_string_with_formatting_args)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/validation.rs:102:5
    [1m[94m|[0m
[1m[94m102[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_min(mut self, [1m[94m...[0m
[1m[94m103[0m [1m[94m|[0m [1m[33m|[0m         self.min = Some(min);
[1m[94m104[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m105[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m102[0m [1m[94m| [0m    pub[92m const[0m fn with_min(mut self, min: f64) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/phenotype-config/src/domain/validation.rs:107:5
    [1m[94m|[0m
[1m[94m107[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_max(mut self, [1m[94m...[0m
[1m[94m108[0m [1m[94m|[0m [1m[33m|[0m         self.max = Some(max);
[1m[94m109[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m110[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m107[0m [1m[94m| [0m    pub[92m const[0m fn with_max(mut self, max: f64) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
   [1m[94m--> [0mcrates/focus-rules/src/lib.rs:516:1
    [1m[94m|[0m
[1m[94m516[0m [1m[94m|[0m [1m[33m/[0m /// Rewrite a rule's explanati[1m[94m...[0m
[1m[94m517[0m [1m[94m|[0m [1m[33m|[0m /// payload. Falls back to the[1m[94m...[0m
[1m[94m518[0m [1m[94m|[0m [1m[33m|[0m /// any failure (kill switch, [1m[94m...[0m
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/infrastructure/error.rs:21:13
   [1m[94m|[0m
[1m[94m21[0m [1m[94m|[0m [1m[94m...[0m       ConfigKitError::Config([1m[94m...[0m
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/infrastructure/error.rs:22:13
   [1m[94m|[0m
[1m[94m22[0m [1m[94m|[0m [1m[94m...[0m       ConfigKitError::Init(ms[1m[94m...[0m
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/infrastructure/error.rs:23:13
   [1m[94m|[0m
[1m[94m23[0m [1m[94m|[0m [1m[94m...[0m       ConfigKitError::Runtime[1m[94m...[0m
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/phenotype-config/src/infrastructure/error.rs:24:13
   [1m[94m|[0m
[1m[94m24[0m [1m[94m|[0m [1m[94m...[0m       ConfigKitError::Shutdow[1m[94m...[0m
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
   [1m[94m--> [0mcrates/focus-rules/src/lib.rs:560:5
    [1m[94m|[0m
[1m[94m560[0m [1m[94m|[0m [1m[33m/[0m     if let Some(prefix) = expe[1m[94m...[0m
[1m[94m561[0m [1m[94m|[0m [1m[33m|[0m         name.starts_with(prefix)
[1m[94m562[0m [1m[94m|[0m [1m[33m|[0m     } else {
[1m[94m563[0m [1m[94m|[0m [1m[33m|[0m         name == expected
[1m[94m564[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m [1m[33mhelp: try: `expected.strip_suffix('*').map_or_else(|| name == expected, |prefix| name.starts_with(prefix))`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
    [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-sync-store/src/lib.rs:168:17
    [1m[94m|[0m
[1m[94m167[0m [1m[94m|[0m       async fn push(&self, records: Vec<SyncRecord>) -> anyhow::Result<u32> {
    [1m[94m|[0m [1m[94m ___________________________________________________________________________-[0m
[1m[94m168[0m [1m[94m|[0m [1m[94m|[0m         let mut inner = self.inner.lock().await;
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m169[0m [1m[94m|[0m [1m[94m|[0m         let count = records.len() as u32;
[1m[94m170[0m [1m[94m|[0m [1m[94m|[0m         inner.push_count += count;
[1m[94m171[0m [1m[94m|[0m [1m[94m|[0m         inner.records.extend(records);
[1m[94m172[0m [1m[94m|[0m [1m[94m|[0m         Ok(count)
[1m[94m173[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `inner` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m171[0m [92m~ [0m        inner.records.extend(records);
[1m[94m172[0m [92m+         drop(inner);[0m
    [1m[94m|[0m

[1m[33mwarning[0m: `focus-entitlements` (lib) generated 20 warnings (run `cargo clippy --fix --lib -p focus-entitlements -- -W clippy::nursery` to apply 19 suggestions)
[1m[92m    Checking[0m focus-ui v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ui)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-connectors/src/lib.rs:13:1
   [1m[94m|[0m
[1m[94m13[0m [1m[94m|[0m [1m[33m/[0m fn default_verification_tier() [1m[94m...[0m
[1m[94m14[0m [1m[94m|[0m [1m[33m|[0m     VerificationTier::Verified
[1m[94m15[0m [1m[94m|[0m [1m[33m|[0m }
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m13[0m [1m[94m| [0m[92mconst [0mfn default_verification_tier() -> VerificationTier {
   [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m: `focus-sync-store` (lib) generated 1 warning
[1m[33mwarning[0m: `settly` (lib) generated 42 warnings (run `cargo clippy --fix --lib -p settly -- -W clippy::nursery` to apply 41 suggestions)
[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-connectors/src/lib.rs:182:13
    [1m[94m|[0m
[1m[94m181[0m [1m[94m|[0m       pub fn catalog(&self) -> Vec<ConnectorListing> {
    [1m[94m|[0m [1m[94m ____________________________________________________-[0m
[1m[94m182[0m [1m[94m|[0m [1m[94m|[0m         let g = self.listings.read().expect("connector [1m[94m...[0m
    [1m[94m|[0m [1m[94m|[0m             [1m[33m^[0m
[1m[94m183[0m [1m[94m|[0m [1m[94m|[0m         let mut v: Vec<ConnectorListing> = g.clone();
[1m[94m184[0m [1m[94m|[0m [1m[94m|[0m         v.sort_by(|a, b| {
[1m[94m...[0m   [1m[94m|[0m
[1m[94m190[0m [1m[94m|[0m [1m[94m|[0m         v
[1m[94m191[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `g` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m182[0m [92m~ [0m        
[1m[94m183[0m [92m+         let mut v = self.listings.read().expect("connector registry poisoned").clone();[0m
[1m[94m184[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-connectors/src/lib.rs:222:1
    [1m[94m|[0m
[1m[94m222[0m [1m[94m|[0m [1m[33m/[0m fn tier_rank(t: &VerificationT[1m[94m...[0m
[1m[94m223[0m [1m[94m|[0m [1m[33m|[0m     match t {
[1m[94m224[0m [1m[94m|[0m [1m[33m|[0m         VerificationTier::Offi[1m[94m...[0m
[1m[94m225[0m [1m[94m|[0m [1m[33m|[0m         VerificationTier::Veri[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m229[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m222[0m [1m[94m| [0m[92mconst [0mfn tier_rank(t: &VerificationTier) -> u8 {
    [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
   [1m[94m--> [0mcrates/focus-connectors/src/lib.rs:236:1
    [1m[94m|[0m
[1m[94m236[0m [1m[94m|[0m [1m[33m/[0m /// A raw webhook delivery as [1m[94m...[0m
[1m[94m237[0m [1m[94m|[0m [1m[33m|[0m /// [`WebhookHandler`] impleme[1m[94m...[0m
[1m[94m238[0m [1m[94m|[0m [1m[33m|[0m /// (`headers` typically carri[1m[94m...[0m
[1m[94m239[0m [1m[94m|[0m [1m[33m|[0m /// `body` → `Vec<NormalizedEv[1m[94m...[0m
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph

[1m[33mwarning[0m: `focus-rules` (lib) generated 13 warnings (run `cargo clippy --fix --lib -p focus-rules -- -W clippy::nursery` to apply 8 suggestions)
[1m[92m    Checking[0m focus-templates v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates)
[1m[92m    Checking[0m focus-policy v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy)
[1m[92m    Checking[0m focuspoint-e2e v0.0.1 (/Users/kooshapari/CodeProjects/Phenotype/repos/tests/e2e)
[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/pheno-tracing/src/adapters.rs:27:17
   [1m[94m|[0m
[1m[94m26[0m [1m[94m|[0m       async fn submit(&self, op: TraceOperation) -> TraceResult {
   [1m[94m|[0m [1m[94m _______________________________________________________________-[0m
[1m[94m27[0m [1m[94m|[0m [1m[94m|[0m         let mut spans = self.spans.lock().unwrap();
   [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m28[0m [1m[94m|[0m [1m[94m|[0m         spans.push(op.clone());
[1m[94m29[0m [1m[94m|[0m [1m[94m|[0m         TraceResult {
[1m[94m...[0m  [1m[94m|[0m
[1m[94m34[0m [1m[94m|[0m [1m[94m|[0m     }
   [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `spans` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
   [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: merge the temporary construction with its single usage
   [1m[94m|[0m
[1m[94m27[0m [92m~ [0m        
[1m[94m28[0m [92m+         self.spans.lock().unwrap().push(op.clone());[0m
[1m[94m29[0m [92m~ [0m        
   [1m[94m|[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-telemetry/src/audit.rs:47:20
   [1m[94m|[0m
[1m[94m47[0m [1m[94m|[0m                 Ok(AuditRecord {
   [1m[94m|[0m                    [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: future cannot be sent between threads safely[0m
   [1m[94m--> [0mcrates/focus-telemetry/src/lib.rs:173:56
    [1m[94m|[0m
[1m[94m173[0m [1m[94m|[0m [1m[94m...[0mol) -> Result<()> {
    [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mfuture returned by `flush_batch` is not `Send`[0m
    [1m[94m|[0m
[1m[92mnote[0m: future is not `Send` as this value is used across an await
   [1m[94m--> [0mcrates/focus-telemetry/src/lib.rs:220:14
    [1m[94m|[0m
[1m[94m189[0m [1m[94m|[0m         let mut stmt = conn.prep[1m[94m...[0m
    [1m[94m|[0m             [1m[94m--------[0m [1m[94mhas type `rusqlite::Statement<'_>` which is not `Send`[0m
[1m[94m...[0m
[1m[94m220[0m [1m[94m|[0m             .await?;
    [1m[94m|[0m              [1m[92m^^^^^[0m [1m[92mawait occurs here, with `mut stmt` maybe used later[0m
    [1m[94m= [0m[1mnote[0m: `*mut rusqlite::libsqlite3_sys::sqlite3_stmt` doesn't implement `std::marker::Send`
[1m[92mnote[0m: future is not `Send` as this value is used across an await
   [1m[94m--> [0mcrates/focus-telemetry/src/lib.rs:220:14
    [1m[94m|[0m
[1m[94m189[0m [1m[94m|[0m         let mut stmt = conn.prep[1m[94m...[0m
    [1m[94m|[0m             [1m[94m--------[0m [1m[94mhas type `rusqlite::Statement<'_>` which is not `Send`[0m
[1m[94m...[0m
[1m[94m220[0m [1m[94m|[0m             .await?;
    [1m[94m|[0m              [1m[92m^^^^^[0m [1m[92mawait occurs here, with `mut stmt` maybe used later[0m
    [1m[94m= [0m[1mnote[0m: `std::cell::RefCell<rusqlite::inner_connection::InnerConnection>` doesn't implement `std::marker::Sync`
[1m[92mnote[0m: future is not `Send` as this value is used across an await
   [1m[94m--> [0mcrates/focus-telemetry/src/lib.rs:220:14
    [1m[94m|[0m
[1m[94m189[0m [1m[94m|[0m         let mut stmt = conn.prep[1m[94m...[0m
    [1m[94m|[0m             [1m[94m--------[0m [1m[94mhas type `rusqlite::Statement<'_>` which is not `Send`[0m
[1m[94m...[0m
[1m[94m220[0m [1m[94m|[0m             .await?;
    [1m[94m|[0m              [1m[92m^^^^^[0m [1m[92mawait occurs here, with `mut stmt` maybe used later[0m
    [1m[94m= [0m[1mnote[0m: `std::cell::RefCell<hashlink::lru_cache::LruCache<std::sync::Arc<str>, rusqlite::raw_statement::RawStatement>>` doesn't implement `std::marker::Sync`
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#future_not_send
    [1m[94m= [0m[1mnote[0m: `-W clippy::future-not-send` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::future_not_send)]`

[1m[33mwarning[0m: `focus-connectors` (lib) generated 7 warnings (run `cargo clippy --fix --lib -p focus-connectors -- -W clippy::nursery` to apply 3 suggestions)
[1m[92m    Checking[0m focus-sync v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync)
[1m[92m    Checking[0m connector-github v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github)
[1m[92m    Checking[0m connector-gcal v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal)
[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
   [1m[94m--> [0mcrates/focus-telemetry/src/lib.rs:272:5
    [1m[94m|[0m
[1m[94m272[0m [1m[94m|[0m [1m[33m/[0m     if let Ok(parsed) = url.pa[1m[94m...[0m
[1m[94m273[0m [1m[94m|[0m [1m[33m|[0m         parsed
[1m[94m274[0m [1m[94m|[0m [1m[33m|[0m             .host_str()
[1m[94m275[0m [1m[94m|[0m [1m[33m|[0m             .map(|h| h.to_stri[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m278[0m [1m[94m|[0m [1m[33m|[0m         "unknown".to_string()
[1m[94m279[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
    [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`
[1m[96mhelp[0m: try
    [1m[94m|[0m
[1m[94m272[0m [92m~ [0m    [92murl.parse::<url::Url>().map_or_else(|_| "unknown".to_string(), |parsed| parsed[0m
[1m[94m273[0m [92m+             .host_str()[0m
[1m[94m274[0m [92m+             .map(|h| h.to_string())[0m
[1m[94m275[0m [92m+             .unwrap_or_else(|| "unknown".to_string()))[0m
    [1m[94m|[0m

[1m[33mwarning[0m: `pheno-tracing` (lib) generated 1 warning
[1m[92m    Checking[0m connector-canvas v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas)
[1m[33mwarning[0m: `focus-telemetry` (lib) generated 3 warnings (run `cargo clippy --fix --lib -p focus-telemetry -- -W clippy::nursery` to apply 1 suggestion)
[1m[92m    Checking[0m connector-linear v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-linear)
[1m[92m    Checking[0m connector-notion v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-notion)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-policy/src/lib.rs:71:5
   [1m[94m|[0m
[1m[94m71[0m [1m[94m|[0m [1m[33m/[0m     pub fn new() -> Self {
[1m[94m72[0m [1m[94m|[0m [1m[33m|[0m         Self
[1m[94m73[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m71[0m [1m[94m| [0m    pub[92m const[0m fn new() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
   [1m[94m--> [0mcrates/focus-policy/src/lib.rs:265:1
    [1m[94m|[0m
[1m[94m265[0m [1m[94m|[0m [1m[33m/[0m /// Sink the platform enforcem[1m[94m...[0m
[1m[94m266[0m [1m[94m|[0m [1m[33m|[0m /// core. Every method must be[1m[94m...[0m
[1m[94m267[0m [1m[94m|[0m [1m[33m|[0m /// the sink is expected to be[1m[94m...[0m
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
    [1m[94m= [0m[1mnote[0m: `-W clippy::too-long-first-doc-paragraph` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::too_long_first_doc_paragraph)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-rituals/src/lib.rs:147:5
    [1m[94m|[0m
[1m[94m147[0m [1m[94m|[0m [1m[33m/[0m     pub fn skipped(task_id: Uu[1m[94m...[0m
[1m[94m148[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m149[0m [1m[94m|[0m [1m[33m|[0m             task_id,
[1m[94m150[0m [1m[94m|[0m [1m[33m|[0m             actual_minutes: 0,
[1m[94m...[0m   [1m[33m|[0m
[1m[94m154[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m147[0m [1m[94m| [0m    pub[92m const[0m fn skipped(task_id: Uuid) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-rituals/src/lib.rs:155:5
    [1m[94m|[0m
[1m[94m155[0m [1m[94m|[0m [1m[33m/[0m     pub fn completed(task_id: [1m[94m...[0m
[1m[94m156[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m157[0m [1m[94m|[0m [1m[33m|[0m             task_id,
[1m[94m158[0m [1m[94m|[0m [1m[33m|[0m             actual_minutes,
[1m[94m...[0m   [1m[33m|[0m
[1m[94m162[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m155[0m [1m[94m| [0m    pub[92m const[0m fn completed(task_id: Uuid, actual_minutes: u32, at: DateTime<Utc>) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-rituals/src/lib.rs:163:5
    [1m[94m|[0m
[1m[94m163[0m [1m[94m|[0m [1m[33m/[0m     pub fn cancelled(task_id: [1m[94m...[0m
[1m[94m164[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m165[0m [1m[94m|[0m [1m[33m|[0m             task_id,
[1m[94m166[0m [1m[94m|[0m [1m[33m|[0m             actual_minutes: 0,
[1m[94m...[0m   [1m[33m|[0m
[1m[94m170[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m163[0m [1m[94m| [0m    pub[92m const[0m fn cancelled(task_id: Uuid) -> Self {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-rituals/src/lib.rs:425:1
    [1m[94m|[0m
[1m[94m425[0m [1m[94m|[0m [1m[33m/[0m fn classify(actual: &TaskActua[1m[94m...[0m
[1m[94m426[0m [1m[94m|[0m [1m[33m|[0m     if actual.cancelled {
[1m[94m427[0m [1m[94m|[0m [1m[33m|[0m         return Classification:[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m444[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m425[0m [1m[94m| [0m[92mconst [0mfn classify(actual: &TaskActual, planned_minutes: u32) -> Classification {
    [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
   [1m[94m--> [0mcrates/focus-rituals/src/lib.rs:487:26
    [1m[94m|[0m
[1m[94m487[0m [1m[94m|[0m       let deadline_label = match[1m[94m...[0m
    [1m[94m|[0m [1m[33m __________________________^[0m
[1m[94m488[0m [1m[94m|[0m [1m[33m|[0m         None => "no-deadline".[1m[94m...[0m
[1m[94m489[0m [1m[94m|[0m [1m[33m|[0m         Some(when) => when.to_[1m[94m...[0m
[1m[94m490[0m [1m[94m|[0m [1m[33m|[0m     };
    [1m[94m|[0m [1m[33m|_____^[0m [1m[33mhelp: try: `task.deadline.when.map_or_else(|| "no-deadline".to_string(), |when| when.to_rfc3339())`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
    [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
   [1m[94m--> [0mcrates/focus-rituals/src/lib.rs:555:5
    [1m[94m|[0m
[1m[94m555[0m [1m[94m|[0m [1m[33m/[0m     if let Some(first) = prior[1m[94m...[0m
[1m[94m556[0m [1m[94m|[0m [1m[33m|[0m         let trunc = truncate(&[1m[94m...[0m
[1m[94m557[0m [1m[94m|[0m [1m[33m|[0m         format!("Morning. Star[1m[94m...[0m
[1m[94m558[0m [1m[94m|[0m [1m[33m|[0m     } else {
[1m[94m559[0m [1m[94m|[0m [1m[33m|[0m         STATIC_OPENING_FALLBAC[1m[94m...[0m
[1m[94m560[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
[1m[96mhelp[0m: try
    [1m[94m|[0m
[1m[94m555[0m [92m~ [0m    [92mpriorities.first().map_or_else(|| STATIC_OPENING_FALLBACK.to_string(), |first| {[0m
[1m[94m556[0m [92m+         let trunc = truncate(&first.title, 50);[0m
[1m[94m557[0m [92m+         format!("Morning. Start with: {trunc}.")[0m
[1m[94m558[0m [92m+     })[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:132:1
    [1m[94m|[0m
[1m[94m132[0m [1m[94m|[0m [1m[33m/[0m fn default_enabled() -> bool {
[1m[94m133[0m [1m[94m|[0m [1m[33m|[0m     true
[1m[94m134[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m132[0m [1m[94m| [0m[92mconst [0mfn default_enabled() -> bool {
    [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:136:48
    [1m[94m|[0m
[1m[94m136[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq
    [1m[94m= [0m[1mnote[0m: `-W clippy::derive-partial-eq-without-eq` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::derive_partial_eq_without_eq)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:147:39
    [1m[94m|[0m
[1m[94m147[0m [1m[94m|[0m [1m[94m...[0mnt(e) => Trigger::Event(e),
    [1m[94m|[0m             [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
    [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:148:42
    [1m[94m|[0m
[1m[94m148[0m [1m[94m|[0m [1m[94m...[0mle(s) => Trigger::Schedule(s),
    [1m[94m|[0m             [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:149:45
    [1m[94m|[0m
[1m[94m149[0m [1m[94m|[0m [1m[94m...[0mge(k) => Trigger::StateChange[1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:154:48
    [1m[94m|[0m
[1m[94m154[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:163:9
    [1m[94m|[0m
[1m[94m163[0m [1m[94m|[0m         Condition {
    [1m[94m|[0m         [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:170:1
    [1m[94m|[0m
[1m[94m170[0m [1m[94m|[0m [1m[33m/[0m /// Action variants available [1m[94m...[0m
[1m[94m171[0m [1m[94m|[0m [1m[33m|[0m /// but only exposes variants [1m[94m...[0m
[1m[94m172[0m [1m[94m|[0m [1m[33m|[0m /// deliberately omitted; it m[1m[94m...[0m
[1m[94m173[0m [1m[94m|[0m [1m[33m|[0m /// wholesale from a template.
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
    [1m[94m= [0m[1mnote[0m: `-W clippy::too-long-first-doc-paragraph` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::too_long_first_doc_paragraph)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-linear/src/api.rs:19:5
   [1m[94m|[0m
[1m[94m19[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(http: Client) ->[1m[94m...[0m
[1m[94m20[0m [1m[94m|[0m [1m[33m|[0m         Self { http }
[1m[94m21[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m19[0m [1m[94m| [0m    pub[92m const[0m fn new(http: Client) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:174:48
    [1m[94m|[0m
[1m[94m174[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:213:1
    [1m[94m|[0m
[1m[94m213[0m [1m[94m|[0m [1m[33m/[0m fn default_rigidity() -> Rigid[1m[94m...[0m
[1m[94m214[0m [1m[94m|[0m [1m[33m|[0m     RigidityDraft::Hard
[1m[94m215[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m213[0m [1m[94m| [0m[92mconst [0mfn default_rigidity() -> RigidityDraft {
    [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:220:36
    [1m[94m|[0m
[1m[94m220[0m [1m[94m|[0m [1m[94m...[0m:Hard => Rigidity::Hard,
    [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:221:36
    [1m[94m|[0m
[1m[94m221[0m [1m[94m|[0m [1m[94m...[0m:Semi => Rigidity::Semi(Rigid[1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:222:36
    [1m[94m|[0m
[1m[94m222[0m [1m[94m|[0m [1m[94m...[0m:Soft => Rigidity::Soft,
    [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:230:52
    [1m[94m|[0m
[1m[94m230[0m [1m[94m|[0m [1m[94m...[0munt } => Action::GrantCredit [1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:231:53
    [1m[94m|[0m
[1m[94m231[0m [1m[94m|[0m [1m[94m...[0munt } => Action::DeductCredit[1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:236:18
    [1m[94m|[0m
[1m[94m236[0m [1m[94m|[0m             } => Action::Block {
    [1m[94m|[0m                  [1m[33m^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:241:49
    [1m[94m|[0m
[1m[94m241[0m [1m[94m|[0m [1m[94m...[0mile } => Action::Unblock { pr[1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:242:54
    [1m[94m|[0m
[1m[94m242[0m [1m[94m|[0m [1m[94m...[0mame } => Action::StreakIncrem[1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:243:50
    [1m[94m|[0m
[1m[94m243[0m [1m[94m|[0m [1m[94m...[0mame } => Action::StreakReset([1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-templates/src/lib.rs:244:48
    [1m[94m|[0m
[1m[94m244[0m [1m[94m|[0m [1m[94m...[0mage } => Action::Notify(messa[1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m: `focus-policy` (lib) generated 2 warnings (run `cargo clippy --fix --lib -p focus-policy -- -W clippy::nursery` to apply 1 suggestion)
[1m[92m    Checking[0m connector-fitbit v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-fitbit)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-linear/src/events.rs:14:5
   [1m[94m|[0m
[1m[94m14[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(account_id: Uuid[1m[94m...[0m
[1m[94m15[0m [1m[94m|[0m [1m[33m|[0m         Self { account_id }
[1m[94m16[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m14[0m [1m[94m| [0m    pub[92m const[0m fn new(account_id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/connector-linear/src/models.rs:17:50
   [1m[94m|[0m
[1m[94m17[0m [1m[94m|[0m [1m[94m...[0m-> Vec<LinearIssue> {
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
  [1m[94m--> [0mcrates/connector-linear/src/models.rs:18:9
   [1m[94m|[0m
[1m[94m18[0m [1m[94m|[0m [1m[33m/[0m [1m[94m...[0m   if let Some(issues) = json
[1m[94m19[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       .get("data")
[1m[94m20[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       .and_then(|d| d.get("[1m[94m...[0m
[1m[94m21[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       .and_then(|i| i.get("[1m[94m...[0m
[1m[94m...[0m  [1m[33m|[0m
[1m[94m43[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       vec![]
[1m[94m44[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   }
   [1m[94m|[0m [1m[33m|_______^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
   [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`
[1m[96mhelp[0m: try
   [1m[94m|[0m
[1m[94m18[0m [92m~ [0m        [92mjson[0m
[1m[94m19[0m [92m+             .get("data")[0m
[1m[94m20[0m [92m+             .and_then(|d| d.get("issues"))[0m
[1m[94m21[0m [92m+             .and_then(|i| i.get("nodes"))[0m
[1m[94m22[0m [92m+             .and_then(|n| n.as_array()).map_or_else(|| vec![], |issues| issues[0m
[1m[94m23[0m [92m+                 .iter()[0m
[1m[94m24[0m [92m+                 .filter_map(|issue| {[0m
[1m[94m25[0m [92m+                     Some(LinearIssue {[0m
[1m[94m26[0m [92m+                         id: issue.get("id")?.as_str()?.into(),[0m
[1m[94m27[0m [92m+                         identifier: issue.get("identifier")?.as_str()?.into(),[0m
[1m[94m28[0m [92m+                         title: issue.get("title")?.as_str()?.into(),[0m
[1m[94m29[0m [92m+                         state: issue[0m
[1m[94m30[0m [92m+                             .get("state")[0m
[1m[94m31[0m [92m+                             .and_then(|s| s.get("name"))[0m
[1m[94m32[0m [92m+                             .and_then(|n| n.as_str())[0m
[1m[94m33[0m [92m+                             .unwrap_or("Unknown")[0m
[1m[94m34[0m [92m+                             .into(),[0m
[1m[94m35[0m [92m+                         created_at: issue.get("createdAt")?.as_str()?.into(),[0m
[1m[94m36[0m [92m+                         updated_at: issue.get("updatedAt")?.as_str()?.into(),[0m
[1m[94m37[0m [92m+                     })[0m
[1m[94m38[0m [92m+                 })[0m
[1m[94m39[0m [92m+                 .collect())[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/connector-linear/src/models.rs:27:26
   [1m[94m|[0m
[1m[94m27[0m [1m[94m|[0m [1m[94m...[0m  Some(LinearIssue {
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-linear/src/lib.rs:55:5
   [1m[94m|[0m
[1m[94m55[0m [1m[94m|[0m [1m[33m/[0m     pub fn account_id(mut self,[1m[94m...[0m
[1m[94m56[0m [1m[94m|[0m [1m[33m|[0m         self.account_id = id;
[1m[94m57[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m58[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m55[0m [1m[94m| [0m    pub[92m const[0m fn account_id(mut self, id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `focus-rituals` (lib) generated 6 warnings (run `cargo clippy --fix --lib -p focus-rituals -- -W clippy::nursery` to apply 4 suggestions)
[1m[92m    Checking[0m connector-strava v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-strava)
[1m[33mwarning[0m: `focus-templates` (lib) generated 20 warnings (run `cargo clippy --fix --lib -p focus-templates -- -W clippy::nursery` to apply 19 suggestions)
[1m[92m    Checking[0m connector-testkit v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-testkit)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mmelosviz/desktop/src-tauri/src/commands.rs:28:5
   [1m[94m|[0m
[1m[94m28[0m [1m[94m|[0m [1m[33m/[0m     pub fn success(data: T) -> [1m[94m...[0m
[1m[94m29[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m30[0m [1m[94m|[0m [1m[33m|[0m             ok: true,
[1m[94m31[0m [1m[94m|[0m [1m[33m|[0m             data: Some(data),
[1m[94m...[0m  [1m[33m|[0m
[1m[94m34[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m28[0m [1m[94m| [0m    pub[92m const[0m fn success(data: T) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-sync/src/cursor_store.rs:36:5
   [1m[94m|[0m
[1m[94m36[0m [1m[94m|[0m [1m[33m/[0m     pub fn new() -> Self {
[1m[94m37[0m [1m[94m|[0m [1m[33m|[0m         Self
[1m[94m38[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m36[0m [1m[94m| [0m    pub[92m const[0m fn new() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/connector-linear/src/lib.rs:118:13
    [1m[94m|[0m
[1m[94m117[0m [1m[94m|[0m       async fn sync(&self, _cursor: Option<String>) -> Result<SyncOutcome> {
    [1m[94m|[0m [1m[94m __________________________________________________________________________-[0m
[1m[94m118[0m [1m[94m|[0m [1m[94m|[0m         let client = self.client.lock().await;
    [1m[94m|[0m [1m[94m|[0m             [1m[33m^^^^^^[0m
[1m[94m119[0m [1m[94m|[0m [1m[94m|[0m         let mapper = LinearEventMapper::new(self.account_id);
[1m[94m120[0m [1m[94m|[0m [1m[94m|[0m         let mut events = Vec::new();
[1m[94m...[0m   [1m[94m|[0m
[1m[94m137[0m [1m[94m|[0m [1m[94m|[0m         })
[1m[94m138[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `client` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m129[0m [92m~ [0m        }
[1m[94m130[0m [92m+         drop(client);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
   [1m[94m--> [0mcrates/focus-sync/src/cursor_store.rs:102:1
    [1m[94m|[0m
[1m[94m102[0m [1m[94m|[0m [1m[33m/[0m /// Canonical entity-type name[1m[94m...[0m
[1m[94m103[0m [1m[94m|[0m [1m[33m|[0m /// persists a connector's las[1m[94m...[0m
[1m[94m104[0m [1m[94m|[0m [1m[33m|[0m /// sites) so the SQLite migra[1m[94m...[0m
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
    [1m[94m= [0m[1mnote[0m: `-W clippy::too-long-first-doc-paragraph` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::too_long_first_doc_paragraph)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-notion/src/api.rs:19:5
   [1m[94m|[0m
[1m[94m19[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(http: Client) ->[1m[94m...[0m
[1m[94m20[0m [1m[94m|[0m [1m[33m|[0m         Self { http }
[1m[94m21[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m19[0m [1m[94m| [0m    pub[92m const[0m fn new(http: Client) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-sync/src/event_sink.rs:25:5
   [1m[94m|[0m
[1m[94m25[0m [1m[94m|[0m [1m[33m/[0m     pub fn new() -> Self {
[1m[94m26[0m [1m[94m|[0m [1m[33m|[0m         Self
[1m[94m27[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m25[0m [1m[94m| [0m    pub[92m const[0m fn new() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-sync/src/retry.rs:28:1
   [1m[94m|[0m
[1m[94m28[0m [1m[94m|[0m [1m[33m/[0m fn pseudo_random_u64(seed: u64)[1m[94m...[0m
[1m[94m29[0m [1m[94m|[0m [1m[33m|[0m     // SplitMix64
[1m[94m30[0m [1m[94m|[0m [1m[33m|[0m     let mut z = seed.wrapping_a[1m[94m...[0m
[1m[94m31[0m [1m[94m|[0m [1m[33m|[0m     z = (z ^ (z >> 30)).wrappin[1m[94m...[0m
[1m[94m32[0m [1m[94m|[0m [1m[33m|[0m     z = (z ^ (z >> 27)).wrappin[1m[94m...[0m
[1m[94m33[0m [1m[94m|[0m [1m[33m|[0m     z ^ (z >> 31)
[1m[94m34[0m [1m[94m|[0m [1m[33m|[0m }
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m28[0m [1m[94m| [0m[92mconst [0mfn pseudo_random_u64(seed: u64) -> u64 {
   [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m: `connector-linear` (lib) generated 7 warnings (run `cargo clippy --fix --lib -p connector-linear -- -W clippy::nursery` to apply 5 suggestions)
[1m[92m    Checking[0m connector-readwise v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-readwise)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-notion/src/events.rs:14:5
   [1m[94m|[0m
[1m[94m14[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(account_id: Uuid[1m[94m...[0m
[1m[94m15[0m [1m[94m|[0m [1m[33m|[0m         Self { account_id }
[1m[94m16[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m14[0m [1m[94m| [0m    pub[92m const[0m fn new(account_id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: redundant clone[0m
  [1m[94m--> [0mcrates/connector-notion/src/events.rs:47:33
   [1m[94m|[0m
[1m[94m47[0m [1m[94m|[0m [1m[94m...[0m id: p.id.clone(),
   [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
   [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
  [1m[94m--> [0mcrates/connector-notion/src/events.rs:47:29
   [1m[94m|[0m
[1m[94m47[0m [1m[94m|[0m [1m[94m...[0m   id: p.id.clone(),
   [1m[94m|[0m           [1m[92m^^^^[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone
   [1m[94m= [0m[1mnote[0m: `-W clippy::redundant-clone` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::redundant_clone)]`

[1m[33mwarning[0m[1m: redundant clone[0m
  [1m[94m--> [0mcrates/connector-notion/src/events.rs:82:33
   [1m[94m|[0m
[1m[94m82[0m [1m[94m|[0m [1m[94m...[0m id: t.id.clone(),
   [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
   [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
  [1m[94m--> [0mcrates/connector-notion/src/events.rs:82:29
   [1m[94m|[0m
[1m[94m82[0m [1m[94m|[0m [1m[94m...[0m   id: t.id.clone(),
   [1m[94m|[0m           [1m[92m^^^^[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/connector-notion/src/models.rs:17:50
   [1m[94m|[0m
[1m[94m17[0m [1m[94m|[0m [1m[94m...[0m-> Vec<NotionPage> {
   [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
  [1m[94m--> [0mcrates/connector-notion/src/models.rs:18:9
   [1m[94m|[0m
[1m[94m18[0m [1m[94m|[0m [1m[33m/[0m [1m[94m...[0m   if let Some(results) = js[1m[94m...[0m
[1m[94m19[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       results
[1m[94m20[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           .iter()
[1m[94m21[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           .filter_map(|page| {
[1m[94m...[0m  [1m[33m|[0m
[1m[94m47[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       vec![]
[1m[94m48[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   }
   [1m[94m|[0m [1m[33m|_______^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
   [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`
[1m[96mhelp[0m: try
   [1m[94m|[0m
[1m[94m18[0m [92m~ [0m        [92mjson.get("results").and_then(|r| r.as_array()).map_or_else(|| vec![], |results| results[0m
[1m[94m19[0m [92m+                 .iter()[0m
[1m[94m20[0m [92m+                 .filter_map(|page| {[0m
[1m[94m21[0m [92m+                     let title = page[0m
[1m[94m22[0m [92m+                         .get("properties")[0m
[1m[94m23[0m [92m+                         .and_then(|p| p.get("title"))[0m
[1m[94m24[0m [92m+                         .and_then(|t| t.get("title"))[0m
[1m[94m25[0m [92m+                         .and_then(|arr| arr.as_array())[0m
[1m[94m26[0m [92m+                         .and_then(|arr| arr.first())[0m
[1m[94m27[0m [92m+                         .and_then(|t| t.get("plain_text"))[0m
[1m[94m28[0m [92m+                         .and_then(|t| t.as_str())[0m
[1m[94m29[0m [92m+                         .unwrap_or("Untitled");[0m
[1m[94m30[0m [92m+ [0m
[1m[94m31[0m [92m+                     Some(NotionPage {[0m
[1m[94m32[0m [92m+                         id: page.get("id")?.as_str()?.into(),[0m
[1m[94m33[0m [92m+                         title: title.into(),[0m
[1m[94m34[0m [92m+                         icon: page[0m
[1m[94m35[0m [92m+                             .get("icon")[0m
[1m[94m36[0m [92m+                             .and_then(|i| i.get("emoji"))[0m
[1m[94m37[0m [92m+                             .and_then(|e| e.as_str())[0m
[1m[94m38[0m [92m+                             .map(|s| s.into()),[0m
[1m[94m39[0m [92m+                         created_time: page.get("created_time")?.as_str()?.into(),[0m
[1m[94m40[0m [92m+                         last_edited_time: page.get("last_edited_time")?.as_str()?.into(),[0m
[1m[94m41[0m [92m+                         url: page.get("url")?.as_str()?.into(),[0m
[1m[94m42[0m [92m+                     })[0m
[1m[94m43[0m [92m+                 })[0m
[1m[94m44[0m [92m+                 .collect())[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/connector-notion/src/models.rs:32:26
   [1m[94m|[0m
[1m[94m32[0m [1m[94m|[0m [1m[94m...[0m  Some(NotionPage {
   [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/connector-notion/src/models.rs:62:50
   [1m[94m|[0m
[1m[94m62[0m [1m[94m|[0m [1m[94m...[0m-> Vec<NotionTask> {
   [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
   [1m[94m--> [0mcrates/connector-notion/src/models.rs:63:9
    [1m[94m|[0m
[1m[94m 63[0m [1m[94m|[0m [1m[33m/[0m [1m[94m...[0m   if let Some(results) = j[1m[94m...[0m
[1m[94m 64[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       results
[1m[94m 65[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           .iter()
[1m[94m 66[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           .filter_map(|tas[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m100[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       vec![]
[1m[94m101[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   }
    [1m[94m|[0m [1m[33m|_______^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
[1m[96mhelp[0m: try
    [1m[94m|[0m
[1m[94m 63[0m [92m~ [0m        [92mjson.get("results").and_then(|r| r.as_array()).map_or_else(|| vec![], |results| results[0m
[1m[94m 64[0m [92m+                 .iter()[0m
[1m[94m 65[0m [92m+                 .filter_map(|task| {[0m
[1m[94m 66[0m [92m+                     let title = task[0m
[1m[94m 67[0m [92m+                         .get("properties")[0m
[1m[94m 68[0m [92m+                         .and_then(|p| p.get("title"))[0m
[1m[94m 69[0m [92m+                         .and_then(|t| t.get("title"))[0m
[1m[94m 70[0m [92m+                         .and_then(|arr| arr.as_array())[0m
[1m[94m 71[0m [92m+                         .and_then(|arr| arr.first())[0m
[1m[94m 72[0m [92m+                         .and_then(|t| t.get("plain_text"))[0m
[1m[94m 73[0m [92m+                         .and_then(|t| t.as_str())[0m
[1m[94m 74[0m [92m+                         .unwrap_or("Untitled");[0m
[1m[94m 75[0m [92m+ [0m
[1m[94m 76[0m [92m+                     let completed = task[0m
[1m[94m 77[0m [92m+                         .get("properties")[0m
[1m[94m 78[0m [92m+                         .and_then(|p| p.get("Completed"))[0m
[1m[94m 79[0m [92m+                         .and_then(|c| c.get("checkbox"))[0m
[1m[94m 80[0m [92m+                         .and_then(|c| c.as_bool())[0m
[1m[94m 81[0m [92m+                         .unwrap_or(false);[0m
[1m[94m 82[0m [92m+ [0m
[1m[94m 83[0m [92m+                     Some(NotionTask {[0m
[1m[94m 84[0m [92m+                         id: task.get("id")?.as_str()?.into(),[0m
[1m[94m 85[0m [92m+                         title: title.into(),[0m
[1m[94m 86[0m [92m+                         completed,[0m
[1m[94m 87[0m [92m+                         due_date: task[0m
[1m[94m 88[0m [92m+                             .get("properties")[0m
[1m[94m 89[0m [92m+                             .and_then(|p| p.get("Due"))[0m
[1m[94m 90[0m [92m+                             .and_then(|d| d.get("date"))[0m
[1m[94m 91[0m [92m+                             .and_then(|d| d.get("start"))[0m
[1m[94m 92[0m [92m+                             .and_then(|s| s.as_str())[0m
[1m[94m 93[0m [92m+                             .map(|s| s.into()),[0m
[1m[94m 94[0m [92m+                         last_edited_time: task.get("last_edited_time")?.as_str()?.into(),[0m
[1m[94m 95[0m [92m+                     })[0m
[1m[94m 96[0m [92m+                 })[0m
[1m[94m 97[0m [92m+                 .collect())[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/connector-notion/src/models.rs:84:26
   [1m[94m|[0m
[1m[94m84[0m [1m[94m|[0m [1m[94m...[0m  Some(NotionTask {
   [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-notion/src/lib.rs:46:5
   [1m[94m|[0m
[1m[94m46[0m [1m[94m|[0m [1m[33m/[0m     pub fn account_id(mut self,[1m[94m...[0m
[1m[94m47[0m [1m[94m|[0m [1m[33m|[0m         self.account_id = id;
[1m[94m48[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m49[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m46[0m [1m[94m| [0m    pub[92m const[0m fn account_id(mut self, id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-testkit/src/lib.rs:15:5
   [1m[94m|[0m
[1m[94m15[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(connector: C) ->[1m[94m...[0m
[1m[94m16[0m [1m[94m|[0m [1m[33m|[0m         Self { connector }
[1m[94m17[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m15[0m [1m[94m| [0m    pub[92m const[0m fn new(connector: C) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
  [1m[94m--> [0mcrates/connector-github/src/api.rs:28:1
   [1m[94m|[0m
[1m[94m28[0m [1m[94m|[0m [1m[33m/[0m /// Defensive pagination cap — [1m[94m...[0m
[1m[94m29[0m [1m[94m|[0m [1m[33m|[0m /// ever points back at us. Git[1m[94m...[0m
[1m[94m30[0m [1m[94m|[0m [1m[33m|[0m /// last 1k events, well beyond[1m[94m...[0m
[1m[94m31[0m [1m[94m|[0m [1m[33m|[0m /// polling window.
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
   [1m[94m= [0m[1mnote[0m: `-W clippy::too-long-first-doc-paragraph` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::too_long_first_doc_paragraph)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-github/src/auth.rs:88:5
   [1m[94m|[0m
[1m[94m88[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_token(token: Gi[1m[94m...[0m
[1m[94m89[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m90[0m [1m[94m|[0m [1m[33m|[0m             inner: std::sync::M[1m[94m...[0m
[1m[94m91[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m92[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m88[0m [1m[94m| [0m    pub[92m const[0m fn with_token(token: GitHubToken) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `melosviz-desktop` (lib) generated 1 warning (run `cargo clippy --fix --lib -p melosviz-desktop -- -W clippy::nursery` to apply 1 suggestion)
[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/connector-notion/src/lib.rs:109:13
    [1m[94m|[0m
[1m[94m108[0m [1m[94m|[0m       async fn sync(&self, _cursor: Option<String>) -> Result<SyncOutcome> {
    [1m[94m|[0m [1m[94m __________________________________________________________________________-[0m
[1m[94m109[0m [1m[94m|[0m [1m[94m|[0m         let client = self.client.lock().await;
    [1m[94m|[0m [1m[94m|[0m             [1m[33m^^^^^^[0m
[1m[94m110[0m [1m[94m|[0m [1m[94m|[0m         let mapper = NotionEventMapper::new(self.account_id);
[1m[94m111[0m [1m[94m|[0m [1m[94m|[0m         let mut events = Vec::new();
[1m[94m...[0m   [1m[94m|[0m
[1m[94m135[0m [1m[94m|[0m [1m[94m|[0m         })
[1m[94m136[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `client` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m127[0m [92m~ [0m        }
[1m[94m128[0m [92m+         drop(client);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-github/src/lib.rs:68:5
   [1m[94m|[0m
[1m[94m68[0m [1m[94m|[0m [1m[33m/[0m     pub fn account_id(mut self,[1m[94m...[0m
[1m[94m69[0m [1m[94m|[0m [1m[33m|[0m         self.account_id = id;
[1m[94m70[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m71[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m68[0m [1m[94m| [0m    pub[92m const[0m fn account_id(mut self, id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `focus-sync` (lib) generated 4 warnings (run `cargo clippy --fix --lib -p focus-sync -- -W clippy::nursery` to apply 3 suggestions)
[1m[92m    Checking[0m focus-storage v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage)
[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/connector-github/src/lib.rs:158:17
    [1m[94m|[0m
[1m[94m150[0m [1m[94m|[0m       async fn ensure_login(&self, client: &GitHubClient) -> Result<String> {
    [1m[94m|[0m [1m[94m ___________________________________________________________________________-[0m
[1m[94m151[0m [1m[94m|[0m [1m[94m|[0m         {
[1m[94m152[0m [1m[94m|[0m [1m[94m|[0m             let g = self.login.lock().await;
[1m[94m153[0m [1m[94m|[0m [1m[94m|[0m             if let Some(l) = g.as_ref() {
[1m[94m...[0m   [1m[94m|[0m
[1m[94m158[0m [1m[94m|[0m [1m[94m|[0m         let mut g = self.login.lock().await;
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^[0m
[1m[94m159[0m [1m[94m|[0m [1m[94m|[0m         *g = Some(user.login.clone());
[1m[94m160[0m [1m[94m|[0m [1m[94m|[0m         Ok(user.login)
[1m[94m161[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `g` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m159[0m [92m~ [0m        *g = Some(user.login.clone());
[1m[94m160[0m [92m+         drop(g);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: future cannot be sent between threads safely[0m
   [1m[94m--> [0mcrates/connector-gcal/src/api.rs:115:10
    [1m[94m|[0m
[1m[94m115[0m [1m[94m|[0m [1m[94m...[0m-> Result<R, ConnectorError> {
    [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mfuture returned by `post_json` is not `Send`[0m
    [1m[94m|[0m
[1m[92mnote[0m: captured value is not `Send` because `&` references cannot be sent unless their referent is `Sync`
   [1m[94m--> [0mcrates/connector-gcal/src/api.rs:114:9
    [1m[94m|[0m
[1m[94m114[0m [1m[94m|[0m         body: &T,
    [1m[94m|[0m         [1m[92m^^^^[0m [1m[92mhas type `&T` which is not `Send`, because `T` is not `Sync`[0m
    [1m[94m= [0m[1mnote[0m: `T` doesn't implement `std::marker::Sync`
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#future_not_send
    [1m[94m= [0m[1mnote[0m: `-W clippy::future-not-send` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::future_not_send)]`

[1m[33mwarning[0m: `connector-notion` (lib) generated 12 warnings (run `cargo clippy --fix --lib -p connector-notion -- -W clippy::nursery` to apply 9 suggestions)
[1m[33mwarning[0m: `connector-testkit` (lib) generated 1 warning (run `cargo clippy --fix --lib -p connector-testkit -- -W clippy::nursery` to apply 1 suggestion)
[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
  [1m[94m--> [0mcrates/connector-gcal/src/auth.rs:39:48
   [1m[94m|[0m
[1m[94m39[0m [1m[94m|[0m [1m[94m...[0mrialize, PartialEq)]
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq
   [1m[94m= [0m[1mnote[0m: `-W clippy::derive-partial-eq-without-eq` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::derive_partial_eq_without_eq)]`

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
  [1m[94m--> [0mcrates/connector-gcal/src/auth.rs:60:9
   [1m[94m|[0m
[1m[94m60[0m [1m[94m|[0m [1m[33m/[0m [1m[94m...[0m   match self.expires_at {
[1m[94m61[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       Some(exp) => now >= e[1m[94m...[0m
[1m[94m62[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       None => {
[1m[94m63[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           if self.refresh_t[1m[94m...[0m
[1m[94m...[0m  [1m[33m|[0m
[1m[94m69[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   }
   [1m[94m|[0m [1m[33m|_______^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
   [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`
[1m[96mhelp[0m: try
   [1m[94m|[0m
[1m[94m60[0m [92m~ [0m        [92mself.expires_at.map_or_else(|| if self.refresh_token.is_some() {[0m
[1m[94m61[0m [92m+                     now - self.issued_at >= Duration::seconds(STALE_IF_NO_EXPIRY_SECS)[0m
[1m[94m62[0m [92m+                 } else {[0m
[1m[94m63[0m [92m+                     false[0m
[1m[94m64[0m [92m+                 }, |exp| now >= exp - Duration::seconds(30))[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-gcal/src/auth.rs:91:5
   [1m[94m|[0m
[1m[94m91[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_token(token: GC[1m[94m...[0m
[1m[94m92[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m93[0m [1m[94m|[0m [1m[33m|[0m             inner: Mutex::new(S[1m[94m...[0m
[1m[94m94[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m95[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m91[0m [1m[94m| [0m    pub[92m const[0m fn with_token(token: GCalToken) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/connector-gcal/src/auth.rs:225:5
    [1m[94m|[0m
[1m[94m225[0m [1m[94m|[0m [1m[33m/[0m     pub fn config(&self) -> &G[1m[94m...[0m
[1m[94m226[0m [1m[94m|[0m [1m[33m|[0m         &self.config
[1m[94m227[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m225[0m [1m[94m| [0m    pub[92m const[0m fn config(&self) -> &GCalAuthConfig {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-gcal/src/lib.rs:62:5
   [1m[94m|[0m
[1m[94m62[0m [1m[94m|[0m [1m[33m/[0m     pub fn account_id(mut self,[1m[94m...[0m
[1m[94m63[0m [1m[94m|[0m [1m[33m|[0m         self.account_id = id;
[1m[94m64[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m65[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m62[0m [1m[94m| [0m    pub[92m const[0m fn account_id(mut self, id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/connector-gcal/src/lib.rs:140:17
    [1m[94m|[0m
[1m[94m134[0m [1m[94m|[0m       async fn refresh_client_token(&self) -> Result<()> {
    [1m[94m|[0m [1m[94m ________________________________________________________-[0m
[1m[94m135[0m [1m[94m|[0m [1m[94m|[0m         let tok = self
[1m[94m136[0m [1m[94m|[0m [1m[94m|[0m             .token_store
[1m[94m137[0m [1m[94m|[0m [1m[94m|[0m             .load()
[1m[94m...[0m   [1m[94m|[0m
[1m[94m140[0m [1m[94m|[0m [1m[94m|[0m         let mut c = self.client.lock().await;
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^[0m
[1m[94m141[0m [1m[94m|[0m [1m[94m|[0m         c.set_access_token(tok.access_token);
[1m[94m142[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m143[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `c` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m140[0m [92m~ [0m        
[1m[94m141[0m [92m+         self.client.lock().await.set_access_token(tok.access_token);[0m
[1m[94m142[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/connector-gcal/src/lib.rs:169:9
    [1m[94m|[0m
[1m[94m169[0m [1m[94m|[0m         GCalConnector::builder(c[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
    [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-strava/src/api.rs:20:5
   [1m[94m|[0m
[1m[94m20[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(http: Client) ->[1m[94m...[0m
[1m[94m21[0m [1m[94m|[0m [1m[33m|[0m         Self { http }
[1m[94m22[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m20[0m [1m[94m| [0m    pub[92m const[0m fn new(http: Client) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-strava/src/auth.rs:81:5
   [1m[94m|[0m
[1m[94m81[0m [1m[94m|[0m [1m[33m/[0m     pub fn new() -> Self {
[1m[94m82[0m [1m[94m|[0m [1m[33m|[0m         Self
[1m[94m83[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m81[0m [1m[94m| [0m    pub[92m const[0m fn new() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-strava/src/events.rs:14:5
   [1m[94m|[0m
[1m[94m14[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(account_id: Uuid[1m[94m...[0m
[1m[94m15[0m [1m[94m|[0m [1m[33m|[0m         Self { account_id }
[1m[94m16[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m14[0m [1m[94m| [0m    pub[92m const[0m fn new(account_id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-fitbit/src/api.rs:19:5
   [1m[94m|[0m
[1m[94m19[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(http: Client) ->[1m[94m...[0m
[1m[94m20[0m [1m[94m|[0m [1m[33m|[0m         Self { http }
[1m[94m21[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m19[0m [1m[94m| [0m    pub[92m const[0m fn new(http: Client) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `connector-github` (lib) generated 4 warnings (run `cargo clippy --fix --lib -p connector-github -- -W clippy::nursery` to apply 2 suggestions)
[1m[92m    Checking[0m focus-webhook-server v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-readwise/src/api.rs:19:5
   [1m[94m|[0m
[1m[94m19[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(http: Client) ->[1m[94m...[0m
[1m[94m20[0m [1m[94m|[0m [1m[33m|[0m         Self { http }
[1m[94m21[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m19[0m [1m[94m| [0m    pub[92m const[0m fn new(http: Client) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-strava/src/lib.rs:58:5
   [1m[94m|[0m
[1m[94m58[0m [1m[94m|[0m [1m[33m/[0m     pub fn account_id(mut self,[1m[94m...[0m
[1m[94m59[0m [1m[94m|[0m [1m[33m|[0m         self.account_id = id;
[1m[94m60[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m61[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m58[0m [1m[94m| [0m    pub[92m const[0m fn account_id(mut self, id: uuid::Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-fitbit/src/auth.rs:77:5
   [1m[94m|[0m
[1m[94m77[0m [1m[94m|[0m [1m[33m/[0m     pub fn new() -> Self {
[1m[94m78[0m [1m[94m|[0m [1m[33m|[0m         Self
[1m[94m79[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m77[0m [1m[94m| [0m    pub[92m const[0m fn new() -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-readwise/src/events.rs:14:5
   [1m[94m|[0m
[1m[94m14[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(account_id: Uuid[1m[94m...[0m
[1m[94m15[0m [1m[94m|[0m [1m[33m|[0m         Self { account_id }
[1m[94m16[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m14[0m [1m[94m| [0m    pub[92m const[0m fn new(account_id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: redundant clone[0m
  [1m[94m--> [0mcrates/connector-readwise/src/events.rs:47:33
   [1m[94m|[0m
[1m[94m47[0m [1m[94m|[0m [1m[94m...[0m id: h.id.clone(),
   [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
   [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
  [1m[94m--> [0mcrates/connector-readwise/src/events.rs:47:29
   [1m[94m|[0m
[1m[94m47[0m [1m[94m|[0m [1m[94m...[0m   id: h.id.clone(),
   [1m[94m|[0m           [1m[92m^^^^[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone
   [1m[94m= [0m[1mnote[0m: `-W clippy::redundant-clone` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::redundant_clone)]`

[1m[33mwarning[0m[1m: redundant clone[0m
  [1m[94m--> [0mcrates/connector-readwise/src/events.rs:84:33
   [1m[94m|[0m
[1m[94m84[0m [1m[94m|[0m [1m[94m...[0m id: a.id.clone(),
   [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
   [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
  [1m[94m--> [0mcrates/connector-readwise/src/events.rs:84:29
   [1m[94m|[0m
[1m[94m84[0m [1m[94m|[0m [1m[94m...[0m   id: a.id.clone(),
   [1m[94m|[0m           [1m[92m^^^^[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/connector-readwise/src/models.rs:19:52
   [1m[94m|[0m
[1m[94m19[0m [1m[94m|[0m [1m[94m...[0m) -> Vec<Article> {
   [1m[94m|[0m             [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
  [1m[94m--> [0mcrates/connector-readwise/src/models.rs:20:9
   [1m[94m|[0m
[1m[94m20[0m [1m[94m|[0m [1m[33m/[0m [1m[94m...[0m   if let Some(results) = js[1m[94m...[0m
[1m[94m21[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       results
[1m[94m22[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           .iter()
[1m[94m23[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           .filter_map(|doc| {
[1m[94m...[0m  [1m[33m|[0m
[1m[94m46[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       vec![]
[1m[94m47[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   }
   [1m[94m|[0m [1m[33m|_______^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
   [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`
[1m[96mhelp[0m: try
   [1m[94m|[0m
[1m[94m20[0m [92m~ [0m        [92mjson.get("results").and_then(|r| r.as_array()).map_or_else(|| vec![], |results| results[0m
[1m[94m21[0m [92m+                 .iter()[0m
[1m[94m22[0m [92m+                 .filter_map(|doc| {[0m
[1m[94m23[0m [92m+                     Some(Article {[0m
[1m[94m24[0m [92m+                         id: doc.get("id")?.as_str()?.into(),[0m
[1m[94m25[0m [92m+                         title: doc.get("title")?.as_str()?.into(),[0m
[1m[94m26[0m [92m+                         author: doc.get("author").and_then(|a| a.as_str()).map(|s| s.into()),[0m
[1m[94m27[0m [92m+                         source_url: doc[0m
[1m[94m28[0m [92m+                             .get("source_url")[0m
[1m[94m29[0m [92m+                             .and_then(|u| u.as_str())[0m
[1m[94m30[0m [92m+                             .map(|s| s.into()),[0m
[1m[94m31[0m [92m+                         cover_image_url: doc[0m
[1m[94m32[0m [92m+                             .get("cover_image_url")[0m
[1m[94m33[0m [92m+                             .and_then(|u| u.as_str())[0m
[1m[94m34[0m [92m+                             .map(|s| s.into()),[0m
[1m[94m35[0m [92m+                         published_date: doc[0m
[1m[94m36[0m [92m+                             .get("published_date")[0m
[1m[94m37[0m [92m+                             .and_then(|d| d.as_str())[0m
[1m[94m38[0m [92m+                             .map(|s| s.into()),[0m
[1m[94m39[0m [92m+                         created_at: doc.get("created_at")?.as_str()?.into(),[0m
[1m[94m40[0m [92m+                         updated_at: doc.get("updated_at")?.as_str()?.into(),[0m
[1m[94m41[0m [92m+                     })[0m
[1m[94m42[0m [92m+                 })[0m
[1m[94m43[0m [92m+                 .collect())[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/connector-readwise/src/models.rs:24:26
   [1m[94m|[0m
[1m[94m24[0m [1m[94m|[0m                     Some(Article {
   [1m[94m|[0m                          [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-fitbit/src/events.rs:14:5
   [1m[94m|[0m
[1m[94m14[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(account_id: Uuid[1m[94m...[0m
[1m[94m15[0m [1m[94m|[0m [1m[33m|[0m         Self { account_id }
[1m[94m16[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m14[0m [1m[94m| [0m    pub[92m const[0m fn new(account_id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: redundant clone[0m
  [1m[94m--> [0mcrates/connector-fitbit/src/events.rs:49:40
   [1m[94m|[0m
[1m[94m49[0m [1m[94m|[0m [1m[94m...[0mgged.name.clone(),
   [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
   [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
  [1m[94m--> [0mcrates/connector-fitbit/src/events.rs:49:29
   [1m[94m|[0m
[1m[94m49[0m [1m[94m|[0m [1m[94m...[0m   id: logged.name.clone(),
   [1m[94m|[0m           [1m[92m^^^^^^^^^^^[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone
   [1m[94m= [0m[1mnote[0m: `-W clippy::redundant-clone` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::redundant_clone)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
 [1m[94m--> [0mcrates/focus-webhook-server/src/lib.rs:5:1
  [1m[94m|[0m
[1m[94m5[0m [1m[94m|[0m pub fn serve() {}
  [1m[94m|[0m [1m[33m^^^^^^^^^^^^^^^^^[0m
  [1m[94m|[0m
  [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
  [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
  [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
  [1m[94m|[0m
[1m[94m5[0m [1m[94m| [0mpub[92m const[0m fn serve() {}
  [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/connector-readwise/src/models.rs:63:52
   [1m[94m|[0m
[1m[94m63[0m [1m[94m|[0m [1m[94m...[0m) -> Vec<Highlight> {
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
  [1m[94m--> [0mcrates/connector-readwise/src/models.rs:64:9
   [1m[94m|[0m
[1m[94m64[0m [1m[94m|[0m [1m[33m/[0m [1m[94m...[0m   if let Some(results) = js[1m[94m...[0m
[1m[94m65[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       results
[1m[94m66[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           .iter()
[1m[94m67[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           .filter_map(|h| {
[1m[94m...[0m  [1m[33m|[0m
[1m[94m80[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       vec![]
[1m[94m81[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   }
   [1m[94m|[0m [1m[33m|_______^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
[1m[96mhelp[0m: try
   [1m[94m|[0m
[1m[94m64[0m [92m~ [0m        [92mjson.get("results").and_then(|r| r.as_array()).map_or_else(|| vec![], |results| results[0m
[1m[94m65[0m [92m+                 .iter()[0m
[1m[94m66[0m [92m+                 .filter_map(|h| {[0m
[1m[94m67[0m [92m+                     Some(Highlight {[0m
[1m[94m68[0m [92m+                         id: h.get("id")?.as_str()?.into(),[0m
[1m[94m69[0m [92m+                         text: h.get("text")?.as_str()?.into(),[0m
[1m[94m70[0m [92m+                         note: h.get("note").and_then(|n| n.as_str()).map(|s| s.into()),[0m
[1m[94m71[0m [92m+                         document_id: h.get("document_id")?.as_str()?.into(),[0m
[1m[94m72[0m [92m+                         color: h.get("color").and_then(|c| c.as_str()).map(|s| s.into()),[0m
[1m[94m73[0m [92m+                         created_at: h.get("created_at")?.as_str()?.into(),[0m
[1m[94m74[0m [92m+                         updated_at: h.get("updated_at")?.as_str()?.into(),[0m
[1m[94m75[0m [92m+                     })[0m
[1m[94m76[0m [92m+                 })[0m
[1m[94m77[0m [92m+                 .collect())[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/connector-readwise/src/models.rs:68:26
   [1m[94m|[0m
[1m[94m68[0m [1m[94m|[0m                     Some(Highlight {
   [1m[94m|[0m                          [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-readwise/src/lib.rs:46:5
   [1m[94m|[0m
[1m[94m46[0m [1m[94m|[0m [1m[33m/[0m     pub fn account_id(mut self,[1m[94m...[0m
[1m[94m47[0m [1m[94m|[0m [1m[33m|[0m         self.account_id = id;
[1m[94m48[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m49[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m46[0m [1m[94m| [0m    pub[92m const[0m fn account_id(mut self, id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `focus-webhook-server` (lib) generated 1 warning (run `cargo clippy --fix --lib -p focus-webhook-server -- -W clippy::nursery` to apply 1 suggestion)
[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/connector-fitbit/src/models.rs:25:9
   [1m[94m|[0m
[1m[94m25[0m [1m[94m|[0m         Activity {
   [1m[94m|[0m         [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/connector-fitbit/src/models.rs:81:9
   [1m[94m|[0m
[1m[94m81[0m [1m[94m|[0m         Sleep { sleep, summary }
   [1m[94m|[0m         [1m[33m^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/connector-fitbit/src/models.rs:122:9
    [1m[94m|[0m
[1m[94m122[0m [1m[94m|[0m         HeartRate { heart_data }
    [1m[94m|[0m         [1m[33m^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-fitbit/src/lib.rs:58:5
   [1m[94m|[0m
[1m[94m58[0m [1m[94m|[0m [1m[33m/[0m     pub fn account_id(mut self,[1m[94m...[0m
[1m[94m59[0m [1m[94m|[0m [1m[33m|[0m         self.account_id = id;
[1m[94m60[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m61[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m58[0m [1m[94m| [0m    pub[92m const[0m fn account_id(mut self, id: uuid::Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/connector-strava/src/lib.rs:219:13
    [1m[94m|[0m
[1m[94m218[0m [1m[94m|[0m       async fn sync(&self, _cursor: Option<String>) -> Result<SyncOutcome> {
    [1m[94m|[0m [1m[94m __________________________________________________________________________-[0m
[1m[94m219[0m [1m[94m|[0m [1m[94m|[0m         let client = self.client.lock().await;
    [1m[94m|[0m [1m[94m|[0m             [1m[33m^^^^^^[0m
[1m[94m220[0m [1m[94m|[0m [1m[94m|[0m         let mapper = StravaEventMapper::new(self.account_id);
[1m[94m221[0m [1m[94m|[0m [1m[94m|[0m         let mut events = Vec::new();
[1m[94m...[0m   [1m[94m|[0m
[1m[94m239[0m [1m[94m|[0m [1m[94m|[0m         })
[1m[94m240[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `client` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m231[0m [92m~ [0m        }
[1m[94m232[0m [92m+         drop(client);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/connector-readwise/src/lib.rs:112:13
    [1m[94m|[0m
[1m[94m111[0m [1m[94m|[0m       async fn sync(&self, _cursor: Option<String>) -> Result<SyncOutcome> {
    [1m[94m|[0m [1m[94m __________________________________________________________________________-[0m
[1m[94m112[0m [1m[94m|[0m [1m[94m|[0m         let client = self.client.lock().await;
    [1m[94m|[0m [1m[94m|[0m             [1m[33m^^^^^^[0m
[1m[94m113[0m [1m[94m|[0m [1m[94m|[0m         let mapper = ReadwiseEventMapper::new(self.account_id);
[1m[94m114[0m [1m[94m|[0m [1m[94m|[0m         let mut events = Vec::new();
[1m[94m...[0m   [1m[94m|[0m
[1m[94m138[0m [1m[94m|[0m [1m[94m|[0m         })
[1m[94m139[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `client` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m130[0m [92m~ [0m        }
[1m[94m131[0m [92m+         drop(client);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/connector-fitbit/src/lib.rs:134:13
    [1m[94m|[0m
[1m[94m133[0m [1m[94m|[0m       async fn sync(&self, _cursor: Option<String>) -> Result<SyncOutcome> {
    [1m[94m|[0m [1m[94m __________________________________________________________________________-[0m
[1m[94m134[0m [1m[94m|[0m [1m[94m|[0m         let client = self.client.lock().await;
    [1m[94m|[0m [1m[94m|[0m             [1m[33m^^^^^^[0m
[1m[94m135[0m [1m[94m|[0m [1m[94m|[0m         let mapper = FitbitEventMapper::new(self.account_id);
[1m[94m136[0m [1m[94m|[0m [1m[94m|[0m         let mut events = Vec::new();
[1m[94m...[0m   [1m[94m|[0m
[1m[94m168[0m [1m[94m|[0m [1m[94m|[0m         })
[1m[94m169[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `client` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m160[0m [92m~ [0m        }
[1m[94m161[0m [92m+         drop(client);[0m
    [1m[94m|[0m

[1m[33mwarning[0m: `connector-strava` (lib) generated 5 warnings (run `cargo clippy --fix --lib -p connector-strava -- -W clippy::nursery` to apply 4 suggestions)
[1m[33mwarning[0m: `connector-gcal` (lib) generated 8 warnings (run `cargo clippy --fix --lib -p connector-gcal -- -W clippy::nursery` to apply 5 suggestions)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/audit_store.rs:28:5
   [1m[94m|[0m
[1m[94m28[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(conn: Arc<Mutex<[1m[94m...[0m
[1m[94m29[0m [1m[94m|[0m [1m[33m|[0m         Self { conn }
[1m[94m30[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m28[0m [1m[94m| [0m    pub[92m const[0m fn new(conn: Arc<Mutex<Connection>>) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
  [1m[94m--> [0mcrates/connector-canvas/src/auth.rs:32:48
   [1m[94m|[0m
[1m[94m32[0m [1m[94m|[0m [1m[94m...[0mrialize, PartialEq)]
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq
   [1m[94m= [0m[1mnote[0m: `-W clippy::derive-partial-eq-without-eq` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::derive_partial_eq_without_eq)]`

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
  [1m[94m--> [0mcrates/connector-canvas/src/auth.rs:52:9
   [1m[94m|[0m
[1m[94m52[0m [1m[94m|[0m [1m[33m/[0m [1m[94m...[0m   match self.expires_at {
[1m[94m53[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       Some(exp) => Utc::now[1m[94m...[0m
[1m[94m54[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       None => {
[1m[94m...[0m  [1m[33m|[0m
[1m[94m65[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   }
   [1m[94m|[0m [1m[33m|_______^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
   [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`
[1m[96mhelp[0m: try
   [1m[94m|[0m
[1m[94m52[0m [92m~ [0m        [92mself.expires_at.map_or_else(|| if self.refresh_token.is_some() {[0m
[1m[94m53[0m [92m+                     Utc::now() - self.issued_at >= Duration::seconds(STALE_IF_NO_EXPIRY_SECS)[0m
[1m[94m54[0m [92m+                 } else {[0m
[1m[94m55[0m [92m+                     false[0m
[1m[94m56[0m [92m+                 }, |exp| Utc::now() >= exp - Duration::seconds(30))[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-canvas/src/auth.rs:87:5
   [1m[94m|[0m
[1m[94m87[0m [1m[94m|[0m [1m[33m/[0m     pub fn with_token(token: Ca[1m[94m...[0m
[1m[94m88[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m89[0m [1m[94m|[0m [1m[33m|[0m             inner: Mutex::new(S[1m[94m...[0m
[1m[94m90[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m91[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m87[0m [1m[94m| [0m    pub[92m const[0m fn with_token(token: CanvasToken) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m: `connector-readwise` (lib) generated 12 warnings (run `cargo clippy --fix --lib -p connector-readwise -- -W clippy::nursery` to apply 9 suggestions)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/auth.rs:206:5
    [1m[94m|[0m
[1m[94m206[0m [1m[94m|[0m [1m[33m/[0m     pub fn config(&self) -> &C[1m[94m...[0m
[1m[94m207[0m [1m[94m|[0m [1m[33m|[0m         &self.config
[1m[94m208[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m206[0m [1m[94m| [0m    pub[92m const[0m fn config(&self) -> &CanvasAuthConfig {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/audit_store.rs:85:17
   [1m[94m|[0m
[1m[94m84[0m [1m[94m|[0m           tokio::task::spawn_blocking(move || -> Result<()> {
   [1m[94m|[0m [1m[94m ___________________________________________________________-[0m
[1m[94m85[0m [1m[94m|[0m [1m[94m|[0m             let guard = conn.blocking_lock();
   [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m86[0m [1m[94m|[0m [1m[94m|[0m             let s = serde_json::to_string(&new_payload).contex[1m[94m...[0m
[1m[94m87[0m [1m[94m|[0m [1m[94m|[0m             guard
[1m[94m...[0m  [1m[94m|[0m
[1m[94m93[0m [1m[94m|[0m [1m[94m|[0m             Ok(())
[1m[94m94[0m [1m[94m|[0m [1m[94m|[0m         })
   [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
   [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
   [1m[94m|[0m
[1m[94m92[0m [92m~ [0m                .context("tamper update")?;
[1m[94m93[0m [92m+             drop(guard);[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/cursor_store.rs:19:17
   [1m[94m|[0m
[1m[94m18[0m [1m[94m|[0m           tokio::task::spawn_blocking(move || -> Result<Option<String>> {
   [1m[94m|[0m [1m[94m _______________________________________________________________________-[0m
[1m[94m19[0m [1m[94m|[0m [1m[94m|[0m             let guard = conn.blocking_lock();
   [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m20[0m [1m[94m|[0m [1m[94m|[0m             let row: Option<String> = guard
[1m[94m21[0m [1m[94m|[0m [1m[94m|[0m                 .query_row(
[1m[94m...[0m  [1m[94m|[0m
[1m[94m29[0m [1m[94m|[0m [1m[94m|[0m             Ok(row)
[1m[94m30[0m [1m[94m|[0m [1m[94m|[0m         })
   [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
   [1m[94m|[0m
[1m[94m28[0m [92m~ [0m                .context("select connector_cursors")?;
[1m[94m29[0m [92m+             drop(guard);[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/cursor_store.rs:42:17
   [1m[94m|[0m
[1m[94m41[0m [1m[94m|[0m           tokio::task::spawn_blocking(move || -> Result<()> {
   [1m[94m|[0m [1m[94m ___________________________________________________________-[0m
[1m[94m42[0m [1m[94m|[0m [1m[94m|[0m             let guard = conn.blocking_lock();
   [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m43[0m [1m[94m|[0m [1m[94m|[0m             guard
[1m[94m44[0m [1m[94m|[0m [1m[94m|[0m                 .execute(
[1m[94m...[0m  [1m[94m|[0m
[1m[94m54[0m [1m[94m|[0m [1m[94m|[0m             Ok(())
[1m[94m55[0m [1m[94m|[0m [1m[94m|[0m         })
   [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
   [1m[94m|[0m
[1m[94m53[0m [92m~ [0m                .context("upsert connector_cursors")?;
[1m[94m54[0m [92m+             drop(guard);[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/cursor_store.rs:61:1
   [1m[94m|[0m
[1m[94m61[0m [1m[94m|[0m [1m[33m/[0m /// Concrete newtype alias for [1m[94m...[0m
[1m[94m62[0m [1m[94m|[0m [1m[33m|[0m /// [`SqliteAdapter`] directly;[1m[94m...[0m
[1m[94m63[0m [1m[94m|[0m [1m[33m|[0m /// match the naming pattern of[1m[94m...[0m
[1m[94m64[0m [1m[94m|[0m [1m[33m|[0m /// that prefer referring to th[1m[94m...[0m
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
   [1m[94m= [0m[1mnote[0m: `-W clippy::too-long-first-doc-paragraph` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::too_long_first_doc_paragraph)]`

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/event_dedup.rs:23:17
   [1m[94m|[0m
[1m[94m22[0m [1m[94m|[0m           tokio::task::spawn_blocking(move || {
   [1m[94m|[0m [1m[94m _____________________________________________-[0m
[1m[94m23[0m [1m[94m|[0m [1m[94m|[0m             let guard = conn.blocking_lock();
   [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m24[0m [1m[94m|[0m [1m[94m|[0m             let now = Utc::now().timestamp();
[1m[94m...[0m  [1m[94m|[0m
[1m[94m37[0m [1m[94m|[0m [1m[94m|[0m             Ok::<bool, DedupeError>(exists)
[1m[94m38[0m [1m[94m|[0m [1m[94m|[0m         })
   [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
   [1m[94m|[0m
[1m[94m23[0m [92m~ [0m            
[1m[94m24[0m [92m+             let exists = conn.blocking_lock().unwrap_or(false);[0m
[1m[94m25[0m [1m[94m|[0m             let now = Utc::now().timestamp();
[1m[94m26[0m [1m[94m|[0m
[1m[94m27[0m [1m[94m|[0m             // Check if hash_key exists AND has not expired (first_seen_at + ttl_sec > now)
[1m[94m28[0m [92m~ [0m            
   [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/event_dedup.rs:48:17
   [1m[94m|[0m
[1m[94m47[0m [1m[94m|[0m           tokio::task::spawn_blocking(move || {
   [1m[94m|[0m [1m[94m _____________________________________________-[0m
[1m[94m48[0m [1m[94m|[0m [1m[94m|[0m             let guard = conn.blocking_lock();
   [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m49[0m [1m[94m|[0m [1m[94m|[0m             let now = Utc::now().timestamp();
[1m[94m...[0m  [1m[94m|[0m
[1m[94m58[0m [1m[94m|[0m [1m[94m|[0m             Ok(())
[1m[94m59[0m [1m[94m|[0m [1m[94m|[0m         })
   [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
   [1m[94m|[0m
[1m[94m56[0m [92m~ [0m                .map_err(|e| DedupeError::DatabaseError(format!("insert dedup: {e}")))?;
[1m[94m57[0m [92m+             drop(guard);[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/event_dedup.rs:69:17
   [1m[94m|[0m
[1m[94m68[0m [1m[94m|[0m           tokio::task::spawn_blocking(move || {
   [1m[94m|[0m [1m[94m _____________________________________________-[0m
[1m[94m69[0m [1m[94m|[0m [1m[94m|[0m             let guard = conn.blocking_lock();
   [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m70[0m [1m[94m|[0m [1m[94m|[0m
[1m[94m71[0m [1m[94m|[0m [1m[94m|[0m             let count = guard
[1m[94m...[0m  [1m[94m|[0m
[1m[94m78[0m [1m[94m|[0m [1m[94m|[0m             Ok(count)
[1m[94m79[0m [1m[94m|[0m [1m[94m|[0m         })
   [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
   [1m[94m|[0m
[1m[94m76[0m [92m~ [0m                .map_err(|e| DedupeError::DatabaseError(format!("purge: {e}")))?;
[1m[94m77[0m [92m+             drop(guard);[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/event_store.rs:25:5
   [1m[94m|[0m
[1m[94m25[0m [1m[94m|[0m [1m[33m/[0m     if let Some(rest) = s.strip[1m[94m...[0m
[1m[94m26[0m [1m[94m|[0m [1m[33m|[0m         EventType::Custom(rest.[1m[94m...[0m
[1m[94m27[0m [1m[94m|[0m [1m[33m|[0m     } else {
[1m[94m28[0m [1m[94m|[0m [1m[33m|[0m         EventType::Custom(s.to_[1m[94m...[0m
[1m[94m29[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m [1m[33mhelp: try: `s.strip_prefix("Custom:").map_or_else(|| EventType::Custom(s.to_string()), |rest| EventType::Custom(rest.to_string()))`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
   [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/event_store.rs:37:17
   [1m[94m|[0m
[1m[94m36[0m [1m[94m|[0m           tokio::task::spawn_blocking(move || -> Result<()> {
   [1m[94m|[0m [1m[94m ___________________________________________________________-[0m
[1m[94m37[0m [1m[94m|[0m [1m[94m|[0m             let guard = conn.blocking_lock();
   [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m38[0m [1m[94m|[0m [1m[94m|[0m             let raw_ref_json = match &event.raw_ref {
[1m[94m39[0m [1m[94m|[0m [1m[94m|[0m                 Some(r) => Some(serde_json::to_string(r).conte[1m[94m...[0m
[1m[94m...[0m  [1m[94m|[0m
[1m[94m63[0m [1m[94m|[0m [1m[94m|[0m             Ok(())
[1m[94m64[0m [1m[94m|[0m [1m[94m|[0m         })
   [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
   [1m[94m|[0m
[1m[94m62[0m [92m~ [0m                .context("insert event")?;
[1m[94m63[0m [92m+             drop(guard);[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-storage/src/sqlite/event_store.rs:77:17
    [1m[94m|[0m
[1m[94m 76[0m [1m[94m|[0m           tokio::task::spawn_blocking(move || -> Result<Vec<NormalizedEvent>> {
    [1m[94m|[0m [1m[94m _____________________________________________________________________________-[0m
[1m[94m 77[0m [1m[94m|[0m [1m[94m|[0m             let guard = conn.blocking_lock();
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m 78[0m [1m[94m|[0m [1m[94m|[0m             let sql = "SELECT event_id, connector_id, account_id, event_type, oc[1m[94m...[0m
[1m[94m 79[0m [1m[94m|[0m [1m[94m|[0m                        effective_at, dedupe_key, confidence, payload, raw_ref \
[1m[94m...[0m   [1m[94m|[0m
[1m[94m134[0m [1m[94m|[0m [1m[94m|[0m             Ok(out)
[1m[94m135[0m [1m[94m|[0m [1m[94m|[0m         })
    [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m 84[0m [92m~ [0m            let mut stmt = guard.prepare(sql).context("prepare since_cursor")?;
[1m[94m 85[0m [92m+             drop(guard);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-storage/src/sqlite/event_store.rs:145:13
    [1m[94m|[0m
[1m[94m144[0m [1m[94m|[0m       tokio::task::spawn_blocking(move || -> Result<Option<NormalizedEvent>> {
    [1m[94m|[0m [1m[94m ____________________________________________________________________________-[0m
[1m[94m145[0m [1m[94m|[0m [1m[94m|[0m         let guard = conn.blocking_lock();
    [1m[94m|[0m [1m[94m|[0m             [1m[33m^^^^^[0m
[1m[94m146[0m [1m[94m|[0m [1m[94m|[0m         let row = guard
[1m[94m147[0m [1m[94m|[0m [1m[94m|[0m             .query_row(
[1m[94m...[0m   [1m[94m|[0m
[1m[94m189[0m [1m[94m|[0m [1m[94m|[0m         }))
[1m[94m190[0m [1m[94m|[0m [1m[94m|[0m     })
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m168[0m [92m~ [0m            .context("query get_by_id")?;
[1m[94m169[0m [92m+         drop(guard);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/penalty_store.rs:14:1
   [1m[94m|[0m
[1m[94m14[0m [1m[94m|[0m [1m[33m/[0m fn tier_to_str(t: EscalationTie[1m[94m...[0m
[1m[94m15[0m [1m[94m|[0m [1m[33m|[0m     match t {
[1m[94m16[0m [1m[94m|[0m [1m[33m|[0m         EscalationTier::Clear =[1m[94m...[0m
[1m[94m17[0m [1m[94m|[0m [1m[33m|[0m         EscalationTier::Warning[1m[94m...[0m
[1m[94m...[0m  [1m[33m|[0m
[1m[94m21[0m [1m[94m|[0m [1m[33m|[0m }
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m14[0m [1m[94m| [0m[92mconst [0mfn tier_to_str(t: EscalationTier) -> &'static str {
   [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-storage/src/sqlite/penalty_store.rs:137:21
    [1m[94m|[0m
[1m[94m136[0m [1m[94m|[0m           tokio::task::spawn_blocking(move || -> Result<()> {
    [1m[94m|[0m [1m[94m ___________________________________________________________-[0m
[1m[94m137[0m [1m[94m|[0m [1m[94m|[0m             let mut guard = conn.blocking_lock();
    [1m[94m|[0m [1m[94m|[0m                     [1m[33m^^^^^[0m
[1m[94m138[0m [1m[94m|[0m [1m[94m|[0m             let tx = guard.transaction().context("begin penalt[1m[94m...[0m
[1m[94m139[0m [1m[94m|[0m [1m[94m|[0m             let mut state = load_sync(&tx, user_id)?;
[1m[94m...[0m   [1m[94m|[0m
[1m[94m146[0m [1m[94m|[0m [1m[94m|[0m             Ok(())
[1m[94m147[0m [1m[94m|[0m [1m[94m|[0m         })
    [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m138[0m [92m~ [0m            let tx = guard.transaction().context("begin penalty.apply")?;
[1m[94m139[0m [92m+             drop(guard);[0m
    [1m[94m|[0m

[1m[33mwarning[0m: `connector-fitbit` (lib) generated 9 warnings (run `cargo clippy --fix --lib -p connector-fitbit -- -W clippy::nursery` to apply 8 suggestions)
[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/rule_store.rs:52:13
   [1m[94m|[0m
[1m[94m51[0m [1m[94m|[0m       tokio::task::spawn_blocking(move || -> Result<()> {
   [1m[94m|[0m [1m[94m _______________________________________________________-[0m
[1m[94m52[0m [1m[94m|[0m [1m[94m|[0m         let guard = conn.blocking_lock();
   [1m[94m|[0m [1m[94m|[0m             [1m[33m^^^^^[0m
[1m[94m53[0m [1m[94m|[0m [1m[94m|[0m         let trigger_json = serde_json::to_string(&rule.tri[1m[94m...[0m
[1m[94m54[0m [1m[94m|[0m [1m[94m|[0m         let conditions_json =
[1m[94m...[0m  [1m[94m|[0m
[1m[94m82[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m83[0m [1m[94m|[0m [1m[94m|[0m     })
   [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
   [1m[94m|[0m
[1m[94m81[0m [92m~ [0m            .context("upsert rule")?;
[1m[94m82[0m [92m+         drop(guard);[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-storage/src/sqlite/rule_store.rs:93:17
    [1m[94m|[0m
[1m[94m 92[0m [1m[94m|[0m           tokio::task::spawn_blocking(move || -> Result<Option<Rule>> {
    [1m[94m|[0m [1m[94m _____________________________________________________________________-[0m
[1m[94m 93[0m [1m[94m|[0m [1m[94m|[0m             let guard = conn.blocking_lock();
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m 94[0m [1m[94m|[0m [1m[94m|[0m             let row = guard
[1m[94m 95[0m [1m[94m|[0m [1m[94m|[0m                 .query_row(
[1m[94m...[0m   [1m[94m|[0m
[1m[94m120[0m [1m[94m|[0m [1m[94m|[0m             Ok(Some(row_to_rule(row)?))
[1m[94m121[0m [1m[94m|[0m [1m[94m|[0m         })
    [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m116[0m [92m~ [0m                .context("query rule.get")?;
[1m[94m117[0m [92m+             drop(guard);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-storage/src/sqlite/rule_store.rs:129:17
    [1m[94m|[0m
[1m[94m128[0m [1m[94m|[0m           tokio::task::spawn_blocking(move || -> Result<Vec<Rule>> {
    [1m[94m|[0m [1m[94m __________________________________________________________________-[0m
[1m[94m129[0m [1m[94m|[0m [1m[94m|[0m             let guard = conn.blocking_lock();
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m130[0m [1m[94m|[0m [1m[94m|[0m             let mut stmt = guard
[1m[94m131[0m [1m[94m|[0m [1m[94m|[0m                 .prepare(
[1m[94m...[0m   [1m[94m|[0m
[1m[94m158[0m [1m[94m|[0m [1m[94m|[0m             Ok(out)
[1m[94m159[0m [1m[94m|[0m [1m[94m|[0m         })
    [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m136[0m [92m~ [0m                .context("prepare list_enabled")?;
[1m[94m137[0m [92m+             drop(guard);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/task_store.rs:32:5
   [1m[94m|[0m
[1m[94m32[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(conn: Arc<Mutex<[1m[94m...[0m
[1m[94m33[0m [1m[94m|[0m [1m[33m|[0m         Self { conn }
[1m[94m34[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m32[0m [1m[94m| [0m    pub[92m const[0m fn new(conn: Arc<Mutex<Connection>>) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-storage/src/sqlite/task_store.rs:46:1
   [1m[94m|[0m
[1m[94m46[0m [1m[94m|[0m [1m[33m/[0m fn status_tag(status: &TaskStat[1m[94m...[0m
[1m[94m47[0m [1m[94m|[0m [1m[33m|[0m     match status {
[1m[94m48[0m [1m[94m|[0m [1m[33m|[0m         TaskStatus::Pending => [1m[94m...[0m
[1m[94m49[0m [1m[94m|[0m [1m[33m|[0m         TaskStatus::Scheduled {[1m[94m...[0m
[1m[94m...[0m  [1m[33m|[0m
[1m[94m54[0m [1m[94m|[0m [1m[33m|[0m }
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m46[0m [1m[94m| [0m[92mconst [0mfn status_tag(status: &TaskStatus) -> &'static str {
   [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
  [1m[94m--> [0mcrates/connector-canvas/src/models.rs:78:48
   [1m[94m|[0m
[1m[94m78[0m [1m[94m|[0m [1m[94m...[0mrialize, PartialEq)]
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-storage/src/sqlite/wallet_store.rs:157:21
    [1m[94m|[0m
[1m[94m156[0m [1m[94m|[0m           tokio::task::spawn_blocking(move || -> Result<()> {
    [1m[94m|[0m [1m[94m ___________________________________________________________-[0m
[1m[94m157[0m [1m[94m|[0m [1m[94m|[0m             let mut guard = conn.blocking_lock();
    [1m[94m|[0m [1m[94m|[0m                     [1m[33m^^^^^[0m
[1m[94m158[0m [1m[94m|[0m [1m[94m|[0m             let tx = guard.transaction().context("begin wallet[1m[94m...[0m
[1m[94m159[0m [1m[94m|[0m [1m[94m|[0m             let mut wallet = load_wallet_sync(&tx, user_id)?;
[1m[94m...[0m   [1m[94m|[0m
[1m[94m166[0m [1m[94m|[0m [1m[94m|[0m             Ok(())
[1m[94m167[0m [1m[94m|[0m [1m[94m|[0m         })
    [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m158[0m [92m~ [0m            let tx = guard.transaction().context("begin wallet.apply")?;
[1m[94m159[0m [92m+             drop(guard);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
  [1m[94m--> [0mcrates/connector-canvas/src/models.rs:93:48
   [1m[94m|[0m
[1m[94m93[0m [1m[94m|[0m [1m[94m...[0mrialize, PartialEq)]
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:145:48
    [1m[94m|[0m
[1m[94m145[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:185:48
    [1m[94m|[0m
[1m[94m185[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:205:48
    [1m[94m|[0m
[1m[94m205[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:267:48
    [1m[94m|[0m
[1m[94m267[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:283:48
    [1m[94m|[0m
[1m[94m283[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:301:48
    [1m[94m|[0m
[1m[94m301[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:321:48
    [1m[94m|[0m
[1m[94m321[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:341:48
    [1m[94m|[0m
[1m[94m341[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m: `focus-storage` (lib) generated 20 warnings (run `cargo clippy --fix --lib -p focus-storage -- -W clippy::nursery` to apply 4 suggestions)
[1m[92m    Checking[0m focus-demo-seed v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed)
[1m[92m    Checking[0m focus-ir v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir)
[1m[92m    Checking[0m focus-eval v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval)
[1m[92m    Checking[0m focus-backup v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-backup)
[1m[92m    Checking[0m focus-replay v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-replay)
[1m[92m    Checking[0m focus-mcp-server v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server)
[1m[92m    Checking[0m focus-connectors-mock-familycontrols v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors-mock-familycontrols)
[1m[92m    Checking[0m focus-rule-suggester v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester)
[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:363:48
    [1m[94m|[0m
[1m[94m363[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:379:48
    [1m[94m|[0m
[1m[94m379[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:397:48
    [1m[94m|[0m
[1m[94m397[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:415:48
    [1m[94m|[0m
[1m[94m415[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:429:48
    [1m[94m|[0m
[1m[94m429[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
   [1m[94m--> [0mcrates/connector-canvas/src/models.rs:485:48
    [1m[94m|[0m
[1m[94m485[0m [1m[94m|[0m [1m[94m...[0malize, PartialEq)]
    [1m[94m|[0m           [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
  [1m[94m--> [0mcrates/connector-canvas/src/lib.rs:27:1
   [1m[94m|[0m
[1m[94m27[0m [1m[94m|[0m [1m[33m/[0m /// Defensive cap on per-course[1m[94m...[0m
[1m[94m28[0m [1m[94m|[0m [1m[33m|[0m /// Canvas doesn't bound page c[1m[94m...[0m
[1m[94m29[0m [1m[94m|[0m [1m[33m|[0m /// Link header points back at [1m[94m...[0m
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
   [1m[94m= [0m[1mnote[0m: `-W clippy::too-long-first-doc-paragraph` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::too_long_first_doc_paragraph)]`
[1m[96mhelp[0m: add an empty line
   [1m[94m|[0m
[1m[94m27[0m [1m[94m|[0m /// Defensive cap on per-course assignment/submission/announcement pagination.
[1m[94m28[0m [92m+ ///[0m
   [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/connector-canvas/src/lib.rs:62:5
   [1m[94m|[0m
[1m[94m62[0m [1m[94m|[0m [1m[33m/[0m     pub fn account_id(mut self,[1m[94m...[0m
[1m[94m63[0m [1m[94m|[0m [1m[33m|[0m         self.account_id = id;
[1m[94m64[0m [1m[94m|[0m [1m[33m|[0m         self
[1m[94m65[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m62[0m [1m[94m| [0m    pub[92m const[0m fn account_id(mut self, id: Uuid) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/connector-canvas/src/lib.rs:191:17
    [1m[94m|[0m
[1m[94m184[0m [1m[94m|[0m     #[async_instrumented]
    [1m[94m|[0m     [1m[94m---------------------[0m [1m[94mtemporary `c` is currently being dropped at the end of its contained scope[0m
[1m[94m...[0m
[1m[94m191[0m [1m[94m|[0m         let mut c = self.client.[1m[94m...[0m
    [1m[94m|[0m                 [1m[33m^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m191[0m [92m~ [0m        
[1m[94m192[0m [92m+         self.client.lock().await.set_access_token(tok.access_token);[0m
[1m[94m193[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/connector-canvas/src/lib.rs:221:9
    [1m[94m|[0m
[1m[94m221[0m [1m[94m|[0m         CanvasConnector::builder[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
    [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:38:5
   [1m[94m|[0m
[1m[94m38[0m [1m[94m|[0m [1m[33m/[0m     pub fn kind_name(&self) -> [1m[94m...[0m
[1m[94m39[0m [1m[94m|[0m [1m[33m|[0m         match self {
[1m[94m40[0m [1m[94m|[0m [1m[33m|[0m             SyntheticEventKind:[1m[94m...[0m
[1m[94m41[0m [1m[94m|[0m [1m[33m|[0m             SyntheticEventKind:[1m[94m...[0m
[1m[94m...[0m  [1m[33m|[0m
[1m[94m46[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m38[0m [1m[94m| [0m    pub[92m const[0m fn kind_name(&self) -> &'static str {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:40:13
   [1m[94m|[0m
[1m[94m40[0m [1m[94m|[0m [1m[94m...[0m     SyntheticEventKind::AppLa[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:41:13
   [1m[94m|[0m
[1m[94m41[0m [1m[94m|[0m [1m[94m...[0m     SyntheticEventKind::Scree[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:42:13
   [1m[94m|[0m
[1m[94m42[0m [1m[94m|[0m [1m[94m...[0m     SyntheticEventKind::Inter[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:43:13
   [1m[94m|[0m
[1m[94m43[0m [1m[94m|[0m [1m[94m...[0m     SyntheticEventKind::Emerg[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:44:13
   [1m[94m|[0m
[1m[94m44[0m [1m[94m|[0m [1m[94m...[0m     SyntheticEventKind::Inter[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:54:13
   [1m[94m|[0m
[1m[94m54[0m [1m[94m|[0m [1m[94m...[0m     SyntheticEventKind::AppLa[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:64:13
   [1m[94m|[0m
[1m[94m64[0m [1m[94m|[0m [1m[94m...[0m     SyntheticEventKind::Scree[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:74:13
   [1m[94m|[0m
[1m[94m74[0m [1m[94m|[0m [1m[94m...[0m     SyntheticEventKind::Inter[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:84:13
   [1m[94m|[0m
[1m[94m84[0m [1m[94m|[0m [1m[94m...[0m     SyntheticEventKind::Emerg[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:94:13
   [1m[94m|[0m
[1m[94m94[0m [1m[94m|[0m [1m[94m...[0m     SyntheticEventKind::Inter[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/time_source.rs:26:5
   [1m[94m|[0m
[1m[94m26[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(initial: DateTim[1m[94m...[0m
[1m[94m27[0m [1m[94m|[0m [1m[33m|[0m         Self {
[1m[94m28[0m [1m[94m|[0m [1m[33m|[0m             current: Mutex::new[1m[94m...[0m
[1m[94m29[0m [1m[94m|[0m [1m[33m|[0m         }
[1m[94m30[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m26[0m [1m[94m| [0m    pub[92m const[0m fn new(initial: DateTime<Utc>) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/lib.rs:43:9
   [1m[94m|[0m
[1m[94m43[0m [1m[94m|[0m         ConnectorError::Schema(e.[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/lib.rs:100:17
    [1m[94m|[0m
[1m[94m 99[0m [1m[94m|[0m       pub fn load_scenario(&self, scenario: &str) -> std::result::Result<(), MockError> {
    [1m[94m|[0m [1m[94m _______________________________________________________________________________________-[0m
[1m[94m100[0m [1m[94m|[0m [1m[94m|[0m         let mut schedule = self.schedule.lock().expect("schedule poisoned");
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^^^^[0m
[1m[94m101[0m [1m[94m|[0m [1m[94m|[0m         *schedule = SyntheticEventSchedule::from_scenario(scenario)?;
[1m[94m102[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m103[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `schedule` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m101[0m [92m~ [0m        *schedule = SyntheticEventSchedule::from_scenario(scenario)?;
[1m[94m102[0m [92m+         drop(schedule);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/lib.rs:172:17
    [1m[94m|[0m
[1m[94m170[0m [1m[94m|[0m       async fn sync(&self, _cursor: Option<String>) -> Result<SyncOutcome> {
    [1m[94m|[0m [1m[94m __________________________________________________________________________-[0m
[1m[94m171[0m [1m[94m|[0m [1m[94m|[0m         let now = self.time_source.now();
[1m[94m172[0m [1m[94m|[0m [1m[94m|[0m         let mut schedule = self.schedule.lock().expect("schedule poisoned");
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^^^^[0m
[1m[94m...[0m   [1m[94m|[0m
[1m[94m194[0m [1m[94m|[0m [1m[94m|[0m         Ok(outcome)
[1m[94m195[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `schedule` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m179[0m [92m~ [0m            schedule.dequeue();
[1m[94m180[0m [92m+             drop(schedule);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-connectors-mock-familycontrols/src/lib.rs:191:17
    [1m[94m|[0m
[1m[94m170[0m [1m[94m|[0m       async fn sync(&self, _cursor: Option<String>) -> Result<SyncOutcome> {
    [1m[94m|[0m [1m[94m __________________________________________________________________________-[0m
[1m[94m171[0m [1m[94m|[0m [1m[94m|[0m         let now = self.time_source.now();
[1m[94m172[0m [1m[94m|[0m [1m[94m|[0m         let mut schedule = self.schedule.lock().expect("schedule poisoned");
[1m[94m...[0m   [1m[94m|[0m
[1m[94m191[0m [1m[94m|[0m [1m[94m|[0m         let mut last = self.last_cursor.lock().expect("cursor poisoned");
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^[0m
[1m[94m...[0m   [1m[94m|[0m
[1m[94m194[0m [1m[94m|[0m [1m[94m|[0m         Ok(outcome)
[1m[94m195[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `last` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m192[0m [92m~ [0m        *last = Some(next_cursor);
[1m[94m193[0m [92m+         drop(last);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: redundant clone[0m
   [1m[94m--> [0mcrates/focus-rule-suggester/src/lib.rs:137:26
    [1m[94m|[0m
[1m[94m137[0m [1m[94m|[0m             audit_records.clone(),
    [1m[94m|[0m                          [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
    [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
   [1m[94m--> [0mcrates/focus-rule-suggester/src/lib.rs:137:13
    [1m[94m|[0m
[1m[94m137[0m [1m[94m|[0m             audit_records.clone(),
    [1m[94m|[0m             [1m[92m^^^^^^^^^^^^^[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone
    [1m[94m= [0m[1mnote[0m: `-W clippy::redundant-clone` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::redundant_clone)]`

[1m[33mwarning[0m: `focus-connectors-mock-familycontrols` (lib) generated 16 warnings (run `cargo clippy --fix --lib -p focus-connectors-mock-familycontrols -- -W clippy::nursery` to apply 13 suggestions)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-mcp-server/src/tools.rs:26:5
   [1m[94m|[0m
[1m[94m26[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(adapter: focus_s[1m[94m...[0m
[1m[94m27[0m [1m[94m|[0m [1m[33m|[0m         Self { adapter }
[1m[94m28[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m26[0m [1m[94m| [0m    pub[92m const[0m fn new(adapter: focus_storage::SqliteAdapter) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: redundant clone[0m
  [1m[94m--> [0mcrates/focus-mcp-server/src/tools.rs:97:29
   [1m[94m|[0m
[1m[94m97[0m [1m[94m|[0m [1m[94m...[0m: adapter.clone(),
   [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
   [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
  [1m[94m--> [0mcrates/focus-mcp-server/src/tools.rs:97:22
   [1m[94m|[0m
[1m[94m97[0m [1m[94m|[0m [1m[94m...[0m   adapter: adapter.clone(),
   [1m[94m|[0m                [1m[92m^^^^^^^[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone
   [1m[94m= [0m[1mnote[0m: `-W clippy::redundant-clone` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::redundant_clone)]`

[1m[33mwarning[0m: `focus-rule-suggester` (lib) generated 1 warning (run `cargo clippy --fix --lib -p focus-rule-suggester -- -W clippy::nursery` to apply 1 suggestion)
[1m[33mwarning[0m[1m: function call inside of `ok_or`[0m
  [1m[94m--> [0mcrates/focus-backup/src/tar_builder.rs:70:30
   [1m[94m|[0m
[1m[94m70[0m [1m[94m|[0m [1m[94m...[0mon.ok_or("manifest.json not found in tar".to_string())?;
   [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: try: `ok_or_else(|| "manifest.json not found in tar".to_string())`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#or_fun_call
   [1m[94m= [0m[1mnote[0m: `-W clippy::or-fun-call` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::or_fun_call)]`

[1m[33mwarning[0m[1m: function call inside of `ok_or`[0m
  [1m[94m--> [0mcrates/focus-backup/src/tar_builder.rs:71:30
   [1m[94m|[0m
[1m[94m71[0m [1m[94m|[0m [1m[94m...[0msh.ok_or("manifest.json.sha256 not found in tar".to_string())?;
   [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: try: `ok_or_else(|| "manifest.json.sha256 not found in tar".to_string())`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#or_fun_call

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-backup/src/lib.rs:59:9
   [1m[94m|[0m
[1m[94m59[0m [1m[94m|[0m         BackupError::Storage(e.to[1m[94m...[0m
   [1m[94m|[0m         [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-backup/src/lib.rs:119:5
    [1m[94m|[0m
[1m[94m119[0m [1m[94m|[0m [1m[33m/[0m     pub fn total(&self) -> usi[1m[94m...[0m
[1m[94m120[0m [1m[94m|[0m [1m[33m|[0m         self.audit_count
[1m[94m121[0m [1m[94m|[0m [1m[33m|[0m             + self.event_count
[1m[94m122[0m [1m[94m|[0m [1m[33m|[0m             + self.rule_count
[1m[94m...[0m   [1m[33m|[0m
[1m[94m126[0m [1m[94m|[0m [1m[33m|[0m             + self.template_count
[1m[94m127[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m119[0m [1m[94m| [0m    pub[92m const[0m fn total(&self) -> usize {
    [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-webhook-server/src/handler.rs:27:13
   [1m[94m|[0m
[1m[94m27[0m [1m[94m|[0m [1m[94m...[0m       WebhookError::Signature[1m[94m...[0m
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
   [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-webhook-server/src/handler.rs:28:13
   [1m[94m|[0m
[1m[94m28[0m [1m[94m|[0m [1m[94m...[0m       WebhookError::UnknownCo[1m[94m...[0m
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
  [1m[94m--> [0mcrates/focus-webhook-server/src/handler.rs:29:13
   [1m[94m|[0m
[1m[94m29[0m [1m[94m|[0m [1m[94m...[0m       WebhookError::Processin[1m[94m...[0m
   [1m[94m|[0m           [1m[33m^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this pattern matching can be expressed using equality[0m
   [1m[94m--> [0mcrates/focus-demo-seed/src/lib.rs:112:12
    [1m[94m|[0m
[1m[94m112[0m [1m[94m|[0m [1m[94m...[0mif let Some("demo") = payload.get("source").and_then(|v| v.as_str()) {
    [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: try: `payload.get("source").and_then(|v| v.as_str()) == Some("demo")`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#equatable_if_let
    [1m[94m= [0m[1mnote[0m: `-W clippy::equatable-if-let` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::equatable_if_let)]`

[1m[33mwarning[0m: `connector-canvas` (lib) generated 24 warnings (run `cargo clippy --fix --lib -p connector-canvas -- -W clippy::nursery` to apply 22 suggestions)
[1m[33mwarning[0m[1m: all if blocks contain the same code at the start[0m
   [1m[94m--> [0mcrates/focus-replay/src/lib.rs:329:9
    [1m[94m|[0m
[1m[94m329[0m [1m[94m|[0m [1m[33m/[0m [1m[94m...[0m   if self.diffs.is_empty() {
[1m[94m330[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       md.push_str("## Differences\n\n");
    [1m[94m|[0m [1m[33m|____________________________________________^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#branches_sharing_code
    [1m[94m= [0m[1mnote[0m: `-W clippy::branches-sharing-code` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::branches_sharing_code)]`
[1m[96mhelp[0m: consider moving these statements before the if
    [1m[94m|[0m
[1m[94m329[0m [92m~ [0m        [92mmd.push_str("## Differences\n\n");[0m
[1m[94m330[0m [92m+         if self.diffs.is_empty() {[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: multiply and add expressions can be calculated more efficiently and accurately[0m
  [1m[94m--> [0mcrates/focus-webhook-server/src/rate_limit.rs:29:23
   [1m[94m|[0m
[1m[94m29[0m [1m[94m|[0m [1m[94m...[0m = (self.tokens + elapsed * self.refill_per_sec).mi[1m[94m...[0m
   [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: consider using: `elapsed.mul_add(self.refill_per_sec, self.tokens)`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#suboptimal_flops
   [1m[94m= [0m[1mnote[0m: `-W clippy::suboptimal-flops` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::suboptimal_flops)]`

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
  [1m[94m--> [0mcrates/focus-webhook-server/src/rate_limit.rs:59:17
   [1m[94m|[0m
[1m[94m58[0m [1m[94m|[0m       pub fn allow(&self, ip: IpAddr) -> bool {
   [1m[94m|[0m [1m[94m _____________________________________________-[0m
[1m[94m59[0m [1m[94m|[0m [1m[94m|[0m         let mut buckets = self.buckets.write().u[1m[94m...[0m
   [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^^^[0m
[1m[94m60[0m [1m[94m|[0m [1m[94m|[0m         let bucket = buckets
[1m[94m61[0m [1m[94m|[0m [1m[94m|[0m             .entry(ip)
[1m[94m62[0m [1m[94m|[0m [1m[94m|[0m             .or_insert_with(|| TokenBucket::new([1m[94m...[0m
[1m[94m63[0m [1m[94m|[0m [1m[94m|[0m         bucket.allow()
[1m[94m64[0m [1m[94m|[0m [1m[94m|[0m     }
   [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `buckets` is currently being dropped at the end of its contained scope[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
   [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: merge the temporary construction with its single usage
   [1m[94m|[0m
[1m[94m59[0m [92m~ [0m        
[1m[94m60[0m [92m+         let bucket = self.buckets.write().unwrap().or_insert_with(|| TokenBucket::new(100.0, 100.0 / 60.0));[0m
[1m[94m61[0m [92m~ [0m        
   [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-webhook-server/src/main.rs:183:17
    [1m[94m|[0m
[1m[94m182[0m [1m[94m|[0m [1m[94m/[0m     {
[1m[94m183[0m [1m[94m|[0m [1m[94m|[0m         let mut metrics = stat[1m[94m...[0m
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^^^[0m
[1m[94m184[0m [1m[94m|[0m [1m[94m|[0m         let health = metrics
[1m[94m185[0m [1m[94m|[0m [1m[94m|[0m             .entry(connector_i[1m[94m...[0m
[1m[94m...[0m   [1m[94m|[0m
[1m[94m200[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `metrics` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m183[0m [92m~ [0m        
[1m[94m184[0m [92m+         let health = state.health_metrics.write().unwrap().or_insert(ConnectorHealth {[0m
[1m[94m185[0m [92m+                 last_received_at: None,[0m
[1m[94m186[0m [92m+                 hmac_success_count: 0,[0m
[1m[94m187[0m [92m+                 hmac_failure_count: 0,[0m
[1m[94m188[0m [92m+                 last_hour_count: 0,[0m
[1m[94m189[0m [92m+             });[0m
[1m[94m190[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-webhook-server/src/main.rs:258:17
    [1m[94m|[0m
[1m[94m257[0m [1m[94m|[0m [1m[94m/[0m     {
[1m[94m258[0m [1m[94m|[0m [1m[94m|[0m         let mut metrics = stat[1m[94m...[0m
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^^^[0m
[1m[94m259[0m [1m[94m|[0m [1m[94m|[0m         let health = metrics
[1m[94m260[0m [1m[94m|[0m [1m[94m|[0m             .entry(connector_i[1m[94m...[0m
[1m[94m...[0m   [1m[94m|[0m
[1m[94m275[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `metrics` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m258[0m [92m~ [0m        
[1m[94m259[0m [92m+         let health = state.health_metrics.write().unwrap().or_insert(ConnectorHealth {[0m
[1m[94m260[0m [92m+                 last_received_at: None,[0m
[1m[94m261[0m [92m+                 hmac_success_count: 0,[0m
[1m[94m262[0m [92m+                 hmac_failure_count: 0,[0m
[1m[94m263[0m [92m+                 last_hour_count: 0,[0m
[1m[94m264[0m [92m+             });[0m
[1m[94m265[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m: `focus-backup` (lib) generated 4 warnings (run `cargo clippy --fix --lib -p focus-backup -- -W clippy::nursery` to apply 2 suggestions)
[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-webhook-server/src/main.rs:379:17
    [1m[94m|[0m
[1m[94m378[0m [1m[94m|[0m [1m[94m/[0m     {
[1m[94m379[0m [1m[94m|[0m [1m[94m|[0m         let mut status = state[1m[94m...[0m
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^^[0m
[1m[94m380[0m [1m[94m|[0m [1m[94m|[0m         let entry = status.ent[1m[94m...[0m
[1m[94m381[0m [1m[94m|[0m [1m[94m|[0m             plugin_id: plugin_[1m[94m...[0m
[1m[94m...[0m   [1m[94m|[0m
[1m[94m395[0m [1m[94m|[0m [1m[94m|[0m         entry.is_running = true;
[1m[94m396[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `status` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m379[0m [92m~ [0m        
[1m[94m380[0m [92m+         let entry = state.plugin_status.write().unwrap().or_insert(PluginExecStatus {[0m
[1m[94m381[0m [92m+             plugin_id: plugin_id.clone(),[0m
[1m[94m382[0m [92m+             is_running: false,[0m
[1m[94m383[0m [92m+             last_poll_at: None,[0m
[1m[94m384[0m [92m+         });[0m
[1m[94m385[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m[1m: function call inside of `or_insert`[0m
   [1m[94m--> [0mcrates/focus-webhook-server/src/main.rs:380:53
    [1m[94m|[0m
[1m[94m380[0m [1m[94m|[0m           let entry = status.entry(plugin_id.clone()).or_i[1m[94m...[0m
    [1m[94m|[0m [1m[33m _____________________________________________________^[0m
[1m[94m381[0m [1m[94m|[0m [1m[33m|[0m             plugin_id: plugin_id.clone(),
[1m[94m382[0m [1m[94m|[0m [1m[33m|[0m             is_running: false,
[1m[94m383[0m [1m[94m|[0m [1m[33m|[0m             last_poll_at: None,
[1m[94m384[0m [1m[94m|[0m [1m[33m|[0m         });
    [1m[94m|[0m [1m[33m|__________^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#or_fun_call
    [1m[94m= [0m[1mnote[0m: `-W clippy::or-fun-call` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::or_fun_call)]`
[1m[96mhelp[0m: try
    [1m[94m|[0m
[1m[94m380[0m [92m~ [0m        let entry = status.entry(plugin_id.clone()).[92mor_insert_with(|| PluginExecStatus {[0m
[1m[94m381[0m [92m+             plugin_id: plugin_id.clone(),[0m
[1m[94m382[0m [92m+             is_running: false,[0m
[1m[94m383[0m [92m+             last_poll_at: None,[0m
[1m[94m384[0m [92m~         })[0m;
    [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-webhook-server/src/main.rs:474:5
    [1m[94m|[0m
[1m[94m474[0m [1m[94m|[0m [1m[33m/[0m     fn new(status: Arc<RwLock<[1m[94m...[0m
[1m[94m475[0m [1m[94m|[0m [1m[33m|[0m         Self { status, plugin_[1m[94m...[0m
[1m[94m476[0m [1m[94m|[0m [1m[33m|[0m     }
    [1m[94m|[0m [1m[33m|_____^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m474[0m [1m[94m| [0m    [92mconst [0mfn new(status: Arc<RwLock<HashMap<String, PluginExecStatus>>>, plugin_id: String) -> Self {
    [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m: `focus-mcp-server` (lib) generated 2 warnings (run `cargo clippy --fix --lib -p focus-mcp-server -- -W clippy::nursery` to apply 2 suggestions)
[1m[33mwarning[0m: `focus-demo-seed` (lib) generated 1 warning (run `cargo clippy --fix --lib -p focus-demo-seed -- -W clippy::nursery` to apply 1 suggestion)
[1m[33mwarning[0m: `focus-replay` (lib) generated 1 warning
[1m[33mwarning[0m: `focus-webhook-server` (bin "focalpoint-webhook-server") generated 10 warnings (run `cargo clippy --fix --bin "focalpoint-webhook-server" -p focus-webhook-server -- -W clippy::nursery` to apply 5 suggestions)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-eval/src/lib.rs:65:5
   [1m[94m|[0m
[1m[94m65[0m [1m[94m|[0m [1m[33m/[0m     pub fn new(inner: Arc<Mutex[1m[94m...[0m
[1m[94m66[0m [1m[94m|[0m [1m[33m|[0m         Self { inner, cap }
[1m[94m67[0m [1m[94m|[0m [1m[33m|[0m     }
   [1m[94m|[0m [1m[33m|_____^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m65[0m [1m[94m| [0m    pub[92m const[0m fn new(inner: Arc<Mutex<Vec<PrioritizedDecision>>>, cap: usize) -> Self {
   [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: use Option::map_or instead of an if let/else[0m
   [1m[94m--> [0mcrates/focus-eval/src/lib.rs:400:39
    [1m[94m|[0m
[1m[94m400[0m [1m[94m|[0m   [1m[94m...[0m   let should_fire = match [1m[94m...[0m
    [1m[94m|[0m [1m[33m _________________________^[0m
[1m[94m401[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       None => true,
[1m[94m402[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       Some(last_fire) => {
[1m[94m403[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           let elapsed = no[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m406[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   };
    [1m[94m|[0m [1m[33m|_______^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
    [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`
[1m[96mhelp[0m: try
    [1m[94m|[0m
[1m[94m400[0m [92m~ [0m                    let should_fire = [92mrate_limit_guard.map_or(true, |last_fire| {[0m
[1m[94m401[0m [92m+                             let elapsed = now_instant.duration_since(last_fire);[0m
[1m[94m402[0m [92m+                             elapsed.as_secs() >= 3600 // 1 hour[0m
[1m[94m403[0m [92m~                         })[0m;
    [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-eval/src/lib.rs:512:1
    [1m[94m|[0m
[1m[94m512[0m [1m[94m|[0m [1m[33m/[0m fn action_variant_name(action:[1m[94m...[0m
[1m[94m513[0m [1m[94m|[0m [1m[33m|[0m     match action {
[1m[94m514[0m [1m[94m|[0m [1m[33m|[0m         Action::GrantCredit { [1m[94m...[0m
[1m[94m515[0m [1m[94m|[0m [1m[33m|[0m         Action::DeductCredit {[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m525[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m512[0m [1m[94m| [0m[92mconst [0mfn action_variant_name(action: &Action) -> &'static str {
    [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-eval/src/lib.rs:550:17
    [1m[94m|[0m
[1m[94m549[0m [1m[94m|[0m       async fn append(&self, event: NormalizedEvent) -> anyhow::Result<()> {
    [1m[94m|[0m [1m[94m __________________________________________________________________________-[0m
[1m[94m550[0m [1m[94m|[0m [1m[94m|[0m         let mut g = self
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^[0m
[1m[94m551[0m [1m[94m|[0m [1m[94m|[0m             .inner
[1m[94m552[0m [1m[94m|[0m [1m[94m|[0m             .lock()
[1m[94m...[0m   [1m[94m|[0m
[1m[94m558[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m559[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `g` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
    [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m557[0m [92m~ [0m        g.push(event);
[1m[94m558[0m [92m+         drop(g);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-eval/src/lib.rs:567:13
    [1m[94m|[0m
[1m[94m566[0m [1m[94m|[0m       ) -> anyhow::Result<Vec<NormalizedEvent>> {
    [1m[94m|[0m [1m[94m _______________________________________________-[0m
[1m[94m567[0m [1m[94m|[0m [1m[94m|[0m         let g = self
    [1m[94m|[0m [1m[94m|[0m             [1m[33m^[0m
[1m[94m568[0m [1m[94m|[0m [1m[94m|[0m             .inner
[1m[94m569[0m [1m[94m|[0m [1m[94m|[0m             .lock()
[1m[94m...[0m   [1m[94m|[0m
[1m[94m585[0m [1m[94m|[0m [1m[94m|[0m         Ok(out)
[1m[94m586[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `g` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
    [1m[94m|[0m
[1m[94m567[0m [92m~ [0m        
[1m[94m568[0m [92m+         let mut out = self[0m
[1m[94m569[0m [92m+             .inner[0m
[1m[94m570[0m [92m+             .lock()[0m
[1m[94m571[0m [92m+             .map_err(|e| anyhow::anyhow!("poisoned: {e}"))?.collect();[0m
[1m[94m572[0m [92m~ [0m        
    [1m[94m|[0m

[1m[33mwarning[0m[1m: use Option::map_or instead of an if let/else[0m
   [1m[94m--> [0mcrates/focus-eval/src/lib.rs:573:25
    [1m[94m|[0m
[1m[94m573[0m [1m[94m|[0m               .filter(|e| match [1m[94m...[0m
    [1m[94m|[0m [1m[33m _________________________^[0m
[1m[94m574[0m [1m[94m|[0m [1m[33m|[0m                 Some(c) => e.o[1m[94m...[0m
[1m[94m575[0m [1m[94m|[0m [1m[33m|[0m                 None => true,
[1m[94m576[0m [1m[94m|[0m [1m[33m|[0m             })
    [1m[94m|[0m [1m[33m|_____________^[0m [1m[33mhelp: try: `cursor.map_or(true, |c| e.occurred_at.to_rfc3339().as_str() > c)`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-eval/src/lib.rs:653:17
    [1m[94m|[0m
[1m[94m652[0m [1m[94m|[0m       async fn apply(&self, user_id: Uuid, mutation: WalletMutation) -> anyhow::Result<()> {
    [1m[94m|[0m [1m[94m __________________________________________________________________________________________-[0m
[1m[94m653[0m [1m[94m|[0m [1m[94m|[0m         let mut g = self
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^[0m
[1m[94m654[0m [1m[94m|[0m [1m[94m|[0m             .inner
[1m[94m655[0m [1m[94m|[0m [1m[94m|[0m             .lock()
[1m[94m...[0m   [1m[94m|[0m
[1m[94m660[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m661[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `g` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m659[0m [92m~ [0m            .map_err(|e| anyhow::anyhow!("wallet apply: {e}"))?;
[1m[94m660[0m [92m+         drop(g);[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
   [1m[94m--> [0mcrates/focus-eval/src/lib.rs:689:17
    [1m[94m|[0m
[1m[94m688[0m [1m[94m|[0m       async fn apply(&self, user_id: Uuid, mutation: PenaltyMutation) -> anyhow::Result<()> {
    [1m[94m|[0m [1m[94m ___________________________________________________________________________________________-[0m
[1m[94m689[0m [1m[94m|[0m [1m[94m|[0m         let mut g = self
    [1m[94m|[0m [1m[94m|[0m                 [1m[33m^[0m
[1m[94m690[0m [1m[94m|[0m [1m[94m|[0m             .inner
[1m[94m691[0m [1m[94m|[0m [1m[94m|[0m             .lock()
[1m[94m...[0m   [1m[94m|[0m
[1m[94m696[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m697[0m [1m[94m|[0m [1m[94m|[0m     }
    [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `g` is currently being dropped at the end of its contained scope[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
    [1m[94m|[0m
[1m[94m695[0m [92m~ [0m            .map_err(|e| anyhow::anyhow!("penalty apply: {e}"))?;
[1m[94m696[0m [92m+         drop(g);[0m
    [1m[94m|[0m

[1m[33mwarning[0m: `focus-eval` (lib) generated 8 warnings (run `cargo clippy --fix --lib -p focus-eval -- -W clippy::nursery` to apply 2 suggestions)
[1m[33mwarning[0m: `focus-mcp-server` (bin "focalpoint-mcp-server") generated 2 warnings (2 duplicates)
[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ir/src/lib.rs:154:27
    [1m[94m|[0m
[1m[94m154[0m [1m[94m|[0m [1m[94m...[0ms: Vec<ConditionIr> },
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
    [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ir/src/lib.rs:157:26
    [1m[94m|[0m
[1m[94m157[0m [1m[94m|[0m [1m[94m...[0ms: Vec<ConditionIr> },
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ir/src/lib.rs:160:26
    [1m[94m|[0m
[1m[94m160[0m [1m[94m|[0m [1m[94m...[0mn: Box<ConditionIr> },
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ir/src/lib.rs:218:36
    [1m[94m|[0m
[1m[94m218[0m [1m[94m|[0m [1m[94m...[0mons: Vec<ActionIr> },
    [1m[94m|[0m             [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m: `focus-ir` (lib) generated 4 warnings (run `cargo clippy --fix --lib -p focus-ir -- -W clippy::nursery` to apply 4 suggestions)
[1m[92m    Checking[0m focus-lang v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang)
[1m[92m    Checking[0m focus-transpilers v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers)
[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
  [1m[94m--> [0mcrates/focus-transpilers/src/graph_transpiler.rs:34:48
   [1m[94m|[0m
[1m[94m34[0m [1m[94m|[0m [1m[94m...[0mrialize, PartialEq)]
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq
   [1m[94m= [0m[1mnote[0m: `-W clippy::derive-partial-eq-without-eq` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::derive_partial_eq_without_eq)]`

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-transpilers/src/graph_transpiler.rs:59:1
   [1m[94m|[0m
[1m[94m59[0m [1m[94m|[0m [1m[33m/[0m fn default_zoom() -> f64 {
[1m[94m60[0m [1m[94m|[0m [1m[33m|[0m     1.0
[1m[94m61[0m [1m[94m|[0m [1m[33m|[0m }
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m59[0m [1m[94m| [0m[92mconst [0mfn default_zoom() -> f64 {
   [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: multiply and add expressions can be calculated more efficiently and accurately[0m
   [1m[94m--> [0mcrates/focus-transpilers/src/graph_transpiler.rs:193:24
    [1m[94m|[0m
[1m[94m193[0m [1m[94m|[0m [1m[94m...[0my: action_start_y + 100.0 * i as f64,
    [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: consider using: `100.0f64.mul_add(i as f64, action_start_y)`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#suboptimal_flops
    [1m[94m= [0m[1mnote[0m: `-W clippy::suboptimal-flops` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::suboptimal_flops)]`

[1m[33mwarning[0m[1m: you are deriving `PartialEq` and can implement `Eq`[0m
  [1m[94m--> [0mcrates/focus-transpilers/src/wizard_transpiler.rs:13:48
   [1m[94m|[0m
[1m[94m13[0m [1m[94m|[0m [1m[94m...[0mrialize, PartialEq)]
   [1m[94m|[0m             [1m[33m^^^^^^^^^[0m [1m[33mhelp: consider deriving `Eq` as well: `PartialEq, Eq`[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#derive_partial_eq_without_eq

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-transpilers/src/wizard_transpiler.rs:42:1
   [1m[94m|[0m
[1m[94m42[0m [1m[94m|[0m [1m[33m/[0m fn default_enabled() -> bool {
[1m[94m43[0m [1m[94m|[0m [1m[33m|[0m     true
[1m[94m44[0m [1m[94m|[0m [1m[33m|[0m }
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m42[0m [1m[94m| [0m[92mconst [0mfn default_enabled() -> bool {
   [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m: `focus-transpilers` (lib) generated 5 warnings (run `cargo clippy --fix --lib -p focus-transpilers -- -W clippy::nursery` to apply 5 suggestions)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1584:1
     [1m[94m|[0m
[1m[94m1584[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1585[0m [1m[94m|[0m [1m[33m|[0m     17506
[1m[94m1586[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
     [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
     [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1584[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_alwaysonapi_tick() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1589:1
     [1m[94m|[0m
[1m[94m1589[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1590[0m [1m[94m|[0m [1m[33m|[0m     15205
[1m[94m1591[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1589[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_auditapi_head_hash() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1594:1
     [1m[94m|[0m
[1m[94m1594[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1595[0m [1m[94m|[0m [1m[33m|[0m     22797
[1m[94m1596[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1594[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_auditapi_recent() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1599:1
     [1m[94m|[0m
[1m[94m1599[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1600[0m [1m[94m|[0m [1m[33m|[0m     39883
[1m[94m1601[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1599[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_auditapi_verify_chain() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1604:1
     [1m[94m|[0m
[1m[94m1604[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1605[0m [1m[94m|[0m [1m[33m|[0m     64902
[1m[94m1606[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1604[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_backupapi_create() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1609:1
     [1m[94m|[0m
[1m[94m1609[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1610[0m [1m[94m|[0m [1m[33m|[0m     4004
[1m[94m1611[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1609[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_backupapi_restore() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1614:1
     [1m[94m|[0m
[1m[94m1614[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1615[0m [1m[94m|[0m [1m[33m|[0m     25167
[1m[94m1616[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1614[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_connectorapi_connect_canvas() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1619:1
     [1m[94m|[0m
[1m[94m1619[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1620[0m [1m[94m|[0m [1m[33m|[0m     19411
[1m[94m1621[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1619[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_connectorapi_connect_gcal() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1624:1
     [1m[94m|[0m
[1m[94m1624[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1625[0m [1m[94m|[0m [1m[33m|[0m     43521
[1m[94m1626[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1624[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_connectorapi_connect_github() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1629:1
     [1m[94m|[0m
[1m[94m1629[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1630[0m [1m[94m|[0m [1m[33m|[0m     6107
[1m[94m1631[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1629[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_datalifecycleapi_wipe_all() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1634:1
     [1m[94m|[0m
[1m[94m1634[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1635[0m [1m[94m|[0m [1m[33m|[0m     32751
[1m[94m1636[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1634[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_demoseedapi_reset() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1639:1
     [1m[94m|[0m
[1m[94m1639[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1640[0m [1m[94m|[0m [1m[33m|[0m     58087
[1m[94m1641[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1639[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_demoseedapi_seed() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1644:1
     [1m[94m|[0m
[1m[94m1644[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1645[0m [1m[94m|[0m [1m[33m|[0m     89
[1m[94m1646[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1644[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_evalapi_tick() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1649:1
     [1m[94m|[0m
[1m[94m1649[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1650[0m [1m[94m|[0m [1m[33m|[0m     33686
[1m[94m1651[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1649[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_always_on() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1654:1
     [1m[94m|[0m
[1m[94m1654[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1655[0m [1m[94m|[0m [1m[33m|[0m     17901
[1m[94m1656[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1654[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_app_version() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1659:1
     [1m[94m|[0m
[1m[94m1659[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1660[0m [1m[94m|[0m [1m[33m|[0m     43630
[1m[94m1661[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1659[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_audit() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1664:1
     [1m[94m|[0m
[1m[94m1664[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1665[0m [1m[94m|[0m [1m[33m|[0m     62331
[1m[94m1666[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1664[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_backup() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1669:1
     [1m[94m|[0m
[1m[94m1669[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1670[0m [1m[94m|[0m [1m[33m|[0m     38360
[1m[94m1671[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1669[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_connector() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1674:1
     [1m[94m|[0m
[1m[94m1674[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1675[0m [1m[94m|[0m [1m[33m|[0m     4295
[1m[94m1676[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1674[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_data_lifecycle() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1679:1
     [1m[94m|[0m
[1m[94m1679[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1680[0m [1m[94m|[0m [1m[33m|[0m     13954
[1m[94m1681[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1679[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_demo_seed() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1684:1
     [1m[94m|[0m
[1m[94m1684[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1685[0m [1m[94m|[0m [1m[33m|[0m     18039
[1m[94m1686[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1684[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_eval() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1689:1
     [1m[94m|[0m
[1m[94m1689[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1690[0m [1m[94m|[0m [1m[33m|[0m     55459
[1m[94m1691[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1689[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_generate_bubble() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1694:1
     [1m[94m|[0m
[1m[94m1694[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1695[0m [1m[94m|[0m [1m[33m|[0m     52989
[1m[94m1696[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1694[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_host_events() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1699:1
     [1m[94m|[0m
[1m[94m1699[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1700[0m [1m[94m|[0m [1m[33m|[0m     37207
[1m[94m1701[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1699[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_mascot_state() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1704:1
     [1m[94m|[0m
[1m[94m1704[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1705[0m [1m[94m|[0m [1m[33m|[0m     11848
[1m[94m1706[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1704[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_mutations() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1709:1
     [1m[94m|[0m
[1m[94m1709[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1710[0m [1m[94m|[0m [1m[33m|[0m     42702
[1m[94m1711[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1709[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_penalty() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1714:1
     [1m[94m|[0m
[1m[94m1714[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1715[0m [1m[94m|[0m [1m[33m|[0m     50783
[1m[94m1716[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1714[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_policy() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1719:1
     [1m[94m|[0m
[1m[94m1719[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1720[0m [1m[94m|[0m [1m[33m|[0m     65039
[1m[94m1721[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1719[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_propose_rule_from_nl() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1724:1
     [1m[94m|[0m
[1m[94m1724[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1725[0m [1m[94m|[0m [1m[33m|[0m     5154
[1m[94m1726[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1724[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_push_mascot_event() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1729:1
     [1m[94m|[0m
[1m[94m1729[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1730[0m [1m[94m|[0m [1m[33m|[0m     3356
[1m[94m1731[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1729[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_rituals() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1734:1
     [1m[94m|[0m
[1m[94m1734[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1735[0m [1m[94m|[0m [1m[33m|[0m     31253
[1m[94m1736[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1734[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_rules() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1739:1
     [1m[94m|[0m
[1m[94m1739[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1740[0m [1m[94m|[0m [1m[33m|[0m     14259
[1m[94m1741[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1739[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_rules_dsl() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1744:1
     [1m[94m|[0m
[1m[94m1744[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1745[0m [1m[94m|[0m [1m[33m|[0m     51428
[1m[94m1746[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1744[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_set_calendar_host() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1749:1
     [1m[94m|[0m
[1m[94m1749[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1750[0m [1m[94m|[0m [1m[33m|[0m     29162
[1m[94m1751[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1749[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_set_coaching() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1754:1
     [1m[94m|[0m
[1m[94m1754[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1755[0m [1m[94m|[0m [1m[33m|[0m     17300
[1m[94m1756[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1754[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_suggester() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1759:1
     [1m[94m|[0m
[1m[94m1759[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1760[0m [1m[94m|[0m [1m[33m|[0m     64609
[1m[94m1761[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1759[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_sync() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1764:1
     [1m[94m|[0m
[1m[94m1764[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1765[0m [1m[94m|[0m [1m[33m|[0m     46807
[1m[94m1766[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1764[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_tasks() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1769:1
     [1m[94m|[0m
[1m[94m1769[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1770[0m [1m[94m|[0m [1m[33m|[0m     29947
[1m[94m1771[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1769[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_templates() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1774:1
     [1m[94m|[0m
[1m[94m1774[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1775[0m [1m[94m|[0m [1m[33m|[0m     5330
[1m[94m1776[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1774[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_focalpointcore_wallet() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1779:1
     [1m[94m|[0m
[1m[94m1779[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1780[0m [1m[94m|[0m [1m[33m|[0m     61935
[1m[94m1781[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1779[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_hosteventapi_emit() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1784:1
     [1m[94m|[0m
[1m[94m1784[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1785[0m [1m[94m|[0m [1m[33m|[0m     49148
[1m[94m1786[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1784[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_penaltyapi_apply() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1789:1
     [1m[94m|[0m
[1m[94m1789[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1790[0m [1m[94m|[0m [1m[33m|[0m     38703
[1m[94m1791[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1789[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_penaltyapi_load() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1794:1
     [1m[94m|[0m
[1m[94m1794[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1795[0m [1m[94m|[0m [1m[33m|[0m     45177
[1m[94m1796[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1794[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_penaltyapi_quote_bypass() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1799:1
     [1m[94m|[0m
[1m[94m1799[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1800[0m [1m[94m|[0m [1m[33m|[0m     12840
[1m[94m1801[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1799[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_policyapi_build_from_recent_decisions() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1804:1
     [1m[94m|[0m
[1m[94m1804[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1805[0m [1m[94m|[0m [1m[33m|[0m     52957
[1m[94m1806[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1804[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_ritualsapi_capture_intention() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1809:1
     [1m[94m|[0m
[1m[94m1809[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1810[0m [1m[94m|[0m [1m[33m|[0m     33629
[1m[94m1811[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1809[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_ritualsapi_generate_evening_shutdown() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1814:1
     [1m[94m|[0m
[1m[94m1814[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1815[0m [1m[94m|[0m [1m[33m|[0m     17766
[1m[94m1816[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1814[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_ritualsapi_generate_monthly_retro() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1819:1
     [1m[94m|[0m
[1m[94m1819[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1820[0m [1m[94m|[0m [1m[33m|[0m     52570
[1m[94m1821[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1819[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_ritualsapi_generate_morning_brief() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1824:1
     [1m[94m|[0m
[1m[94m1824[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1825[0m [1m[94m|[0m [1m[33m|[0m     1913
[1m[94m1826[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1824[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_ritualsapi_generate_weekly_review() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1829:1
     [1m[94m|[0m
[1m[94m1829[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1830[0m [1m[94m|[0m [1m[33m|[0m     14878
[1m[94m1831[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1829[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_rulemutation_set_enabled() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1834:1
     [1m[94m|[0m
[1m[94m1834[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1835[0m [1m[94m|[0m [1m[33m|[0m     32993
[1m[94m1836[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1834[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_rulemutation_upsert() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1839:1
     [1m[94m|[0m
[1m[94m1839[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1840[0m [1m[94m|[0m [1m[33m|[0m     31405
[1m[94m1841[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1839[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_rulequery_list_enabled() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1844:1
     [1m[94m|[0m
[1m[94m1844[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1845[0m [1m[94m|[0m [1m[33m|[0m     56064
[1m[94m1846[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1844[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_suggesterapi_apply() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1849:1
     [1m[94m|[0m
[1m[94m1849[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1850[0m [1m[94m|[0m [1m[33m|[0m     47445
[1m[94m1851[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1849[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_suggesterapi_dismiss() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1854:1
     [1m[94m|[0m
[1m[94m1854[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1855[0m [1m[94m|[0m [1m[33m|[0m     49027
[1m[94m1856[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1854[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_suggesterapi_fetch() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1859:1
     [1m[94m|[0m
[1m[94m1859[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1860[0m [1m[94m|[0m [1m[33m|[0m     57835
[1m[94m1861[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1859[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_syncapi_connectors() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1864:1
     [1m[94m|[0m
[1m[94m1864[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1865[0m [1m[94m|[0m [1m[33m|[0m     8704
[1m[94m1866[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1864[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_syncapi_tick() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1869:1
     [1m[94m|[0m
[1m[94m1869[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1870[0m [1m[94m|[0m [1m[33m|[0m     51725
[1m[94m1871[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1869[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_taskapi_add() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1874:1
     [1m[94m|[0m
[1m[94m1874[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1875[0m [1m[94m|[0m [1m[33m|[0m     46930
[1m[94m1876[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1874[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_taskapi_list() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1879:1
     [1m[94m|[0m
[1m[94m1879[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1880[0m [1m[94m|[0m [1m[33m|[0m     33523
[1m[94m1881[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1879[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_taskapi_mark_done() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1884:1
     [1m[94m|[0m
[1m[94m1884[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1885[0m [1m[94m|[0m [1m[33m|[0m     16303
[1m[94m1886[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1884[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_taskapi_remove() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1889:1
     [1m[94m|[0m
[1m[94m1889[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1890[0m [1m[94m|[0m [1m[33m|[0m     29206
[1m[94m1891[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1889[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_templateapi_install() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1894:1
     [1m[94m|[0m
[1m[94m1894[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1895[0m [1m[94m|[0m [1m[33m|[0m     44689
[1m[94m1896[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1894[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_templateapi_list_bundled() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1899:1
     [1m[94m|[0m
[1m[94m1899[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1900[0m [1m[94m|[0m [1m[33m|[0m     45227
[1m[94m1901[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1899[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_walletapi_apply_mutation() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1904:1
     [1m[94m|[0m
[1m[94m1904[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1905[0m [1m[94m|[0m [1m[33m|[0m     2507
[1m[94m1906[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1904[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_walletapi_load() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1909:1
     [1m[94m|[0m
[1m[94m1909[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1910[0m [1m[94m|[0m [1m[33m|[0m     56285
[1m[94m1911[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1909[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_constructor_coachingconfig_new() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1914:1
     [1m[94m|[0m
[1m[94m1914[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1915[0m [1m[94m|[0m [1m[33m|[0m     23567
[1m[94m1916[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1914[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_constructor_focalpointcore_new() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0m/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/focus-ffi-e05ef0f80c9e186b/out/focus_ffi.uniffi.rs:1919:1
     [1m[94m|[0m
[1m[94m1919[0m [1m[94m|[0m [1m[33m/[0m pub extern "C" fn r#uniffi_fo[1m[94m...[0m
[1m[94m1920[0m [1m[94m|[0m [1m[33m|[0m     799
[1m[94m1921[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1919[0m [1m[94m| [0mpub[92m const[0m extern "C" fn r#uniffi_focus_ffi_checksum_method_calendarhost_list_events() -> u16 {
     [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:109:9
    [1m[94m|[0m
[1m[94m109[0m [1m[94m|[0m         FfiError::Storage(e.to_s[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
    [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:167:36
    [1m[94m|[0m
[1m[94m167[0m [1m[94m|[0m [1m[94m...[0mident => Pose::Confident,
    [1m[94m|[0m             [1m[33m^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:168:38
    [1m[94m|[0m
[1m[94m168[0m [1m[94m|[0m [1m[94m...[0maging => Pose::Encouraging,
    [1m[94m|[0m             [1m[33m^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:169:42
    [1m[94m|[0m
[1m[94m169[0m [1m[94m|[0m [1m[94m...[0mnking => Pose::CuriousThinking,
    [1m[94m|[0m             [1m[33m^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:170:41
    [1m[94m|[0m
[1m[94m170[0m [1m[94m|[0m [1m[94m...[0mhLove => Pose::SternToughLove,
    [1m[94m|[0m             [1m[33m^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:171:38
    [1m[94m|[0m
[1m[94m171[0m [1m[94m|[0m [1m[94m...[0matory => Pose::Celebratory,
    [1m[94m|[0m             [1m[33m^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:172:45
    [1m[94m|[0m
[1m[94m172[0m [1m[94m|[0m [1m[94m...[0minted => Pose::SleepyDisappoi[1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:173:31
    [1m[94m|[0m
[1m[94m173[0m [1m[94m|[0m [1m[94m...[0m:Idle => Pose::Idle,
    [1m[94m|[0m             [1m[33m^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:181:37
    [1m[94m|[0m
[1m[94m181[0m [1m[94m|[0m [1m[94m...[0mutral => Emotion::Neutral,
    [1m[94m|[0m             [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:182:35
    [1m[94m|[0m
[1m[94m182[0m [1m[94m|[0m [1m[94m...[0mHappy => Emotion::Happy,
    [1m[94m|[0m             [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:183:35
    [1m[94m|[0m
[1m[94m183[0m [1m[94m|[0m [1m[94m...[0mProud => Emotion::Proud,
    [1m[94m|[0m             [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:184:39
    [1m[94m|[0m
[1m[94m184[0m [1m[94m|[0m [1m[94m...[0merned => Emotion::Concerned,
    [1m[94m|[0m             [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:185:35
    [1m[94m|[0m
[1m[94m185[0m [1m[94m|[0m [1m[94m...[0mStern => Emotion::Stern,
    [1m[94m|[0m             [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:186:37
    [1m[94m|[0m
[1m[94m186[0m [1m[94m|[0m [1m[94m...[0mcited => Emotion::Excited,
    [1m[94m|[0m             [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:187:35
    [1m[94m|[0m
[1m[94m187[0m [1m[94m|[0m [1m[94m...[0mTired => Emotion::Tired,
    [1m[94m|[0m             [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:188:34
    [1m[94m|[0m
[1m[94m188[0m [1m[94m|[0m [1m[94m...[0m:Warm => Emotion::Warm,
    [1m[94m|[0m             [1m[33m^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:195:9
    [1m[94m|[0m
[1m[94m195[0m [1m[94m|[0m         MascotState {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:207:53
    [1m[94m|[0m
[1m[94m207[0m [1m[94m|[0m [1m[94m...[0m} => CoreMascotEvent::RuleFir[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:209:17
    [1m[94m|[0m
[1m[94m209[0m [1m[94m|[0m [1m[94m...[0m     CoreMascotEvent::StreakI[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:211:50
    [1m[94m|[0m
[1m[94m211[0m [1m[94m|[0m [1m[94m...[0m} => CoreMascotEvent::StreakR[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:212:53
    [1m[94m|[0m
[1m[94m212[0m [1m[94m|[0m [1m[94m...[0m} => CoreMascotEvent::CreditE[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:213:55
    [1m[94m|[0m
[1m[94m213[0m [1m[94m|[0m [1m[94m...[0m} => CoreMascotEvent::BypassS[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:214:55
    [1m[94m|[0m
[1m[94m214[0m [1m[94m|[0m [1m[94m...[0m} => CoreMascotEvent::Penalty[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:216:17
    [1m[94m|[0m
[1m[94m216[0m [1m[94m|[0m [1m[94m...[0m     CoreMascotEvent::AppLaun[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:219:17
    [1m[94m|[0m
[1m[94m219[0m [1m[94m|[0m [1m[94m...[0m     CoreMascotEvent::FocusSe[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:222:17
    [1m[94m|[0m
[1m[94m222[0m [1m[94m|[0m [1m[94m...[0m     CoreMascotEvent::FocusSe[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:224:42
    [1m[94m|[0m
[1m[94m224[0m [1m[94m|[0m [1m[94m...[0mn => CoreMascotEvent::DailyCh[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:226:17
    [1m[94m|[0m
[1m[94m226[0m [1m[94m|[0m [1m[94m...[0m     CoreMascotEvent::SleepDe[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:228:34
    [1m[94m|[0m
[1m[94m228[0m [1m[94m|[0m [1m[94m...[0me => CoreMascotEvent::Idle,
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:288:54
    [1m[94m|[0m
[1m[94m288[0m [1m[94m|[0m [1m[94m...[0mt } => CoreAction::GrantCredi[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:289:55
    [1m[94m|[0m
[1m[94m289[0m [1m[94m|[0m [1m[94m...[0mt } => CoreAction::DeductCred[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:293:18
    [1m[94m|[0m
[1m[94m293[0m [1m[94m|[0m [1m[94m...[0m  } => CoreAction::Block {
    [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:298:51
    [1m[94m|[0m
[1m[94m298[0m [1m[94m|[0m [1m[94m...[0me } => CoreAction::Unblock { [1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:299:56
    [1m[94m|[0m
[1m[94m299[0m [1m[94m|[0m [1m[94m...[0me } => CoreAction::StreakIncr[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:300:52
    [1m[94m|[0m
[1m[94m300[0m [1m[94m|[0m [1m[94m...[0me } => CoreAction::StreakRese[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:301:50
    [1m[94m|[0m
[1m[94m301[0m [1m[94m|[0m [1m[94m...[0me } => CoreAction::Notify(mes[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:387:13
    [1m[94m|[0m
[1m[94m387[0m [1m[94m|[0m [1m[94m...[0m     WalletMutationDto::Grant[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:392:13
    [1m[94m|[0m
[1m[94m392[0m [1m[94m|[0m [1m[94m...[0m     WalletMutationDto::Spend[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:395:13
    [1m[94m|[0m
[1m[94m395[0m [1m[94m|[0m [1m[94m...[0m     WalletMutationDto::Strea[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:398:13
    [1m[94m|[0m
[1m[94m398[0m [1m[94m|[0m [1m[94m...[0m     WalletMutationDto::Strea[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:399:13
    [1m[94m|[0m
[1m[94m399[0m [1m[94m|[0m [1m[94m...[0m     WalletMutationDto::SetMu[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:448:1
    [1m[94m|[0m
[1m[94m448[0m [1m[94m|[0m [1m[33m/[0m fn tier_name(t: EscalationTier[1m[94m...[0m
[1m[94m449[0m [1m[94m|[0m [1m[33m|[0m     match t {
[1m[94m450[0m [1m[94m|[0m [1m[33m|[0m         EscalationTier::Clear [1m[94m...[0m
[1m[94m451[0m [1m[94m|[0m [1m[33m|[0m         EscalationTier::Warnin[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m455[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m448[0m [1m[94m| [0m[92mconst [0mfn tier_name(t: EscalationTier) -> &'static str {
    [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:474:13
    [1m[94m|[0m
[1m[94m474[0m [1m[94m|[0m [1m[94m...[0m     PenaltyMutationDto::Esca[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:477:13
    [1m[94m|[0m
[1m[94m477[0m [1m[94m|[0m [1m[94m...[0m     PenaltyMutationDto::Spen[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:478:13
    [1m[94m|[0m
[1m[94m478[0m [1m[94m|[0m [1m[94m...[0m     PenaltyMutationDto::Gran[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:479:13
    [1m[94m|[0m
[1m[94m479[0m [1m[94m|[0m [1m[94m...[0m     PenaltyMutationDto::AddL[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:487:13
    [1m[94m|[0m
[1m[94m487[0m [1m[94m|[0m [1m[94m...[0m     PenaltyMutationDto::Clea[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:488:13
    [1m[94m|[0m
[1m[94m488[0m [1m[94m|[0m [1m[94m...[0m     PenaltyMutationDto::SetS[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:491:13
    [1m[94m|[0m
[1m[94m491[0m [1m[94m|[0m [1m[94m...[0m     PenaltyMutationDto::Clea[1m[94m...[0m
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:649:1
    [1m[94m|[0m
[1m[94m649[0m [1m[94m|[0m [1m[33m/[0m fn kind_name(k: &CoreScheduleW[1m[94m...[0m
[1m[94m650[0m [1m[94m|[0m [1m[33m|[0m     match k {
[1m[94m651[0m [1m[94m|[0m [1m[33m|[0m         CoreScheduleWindowKind[1m[94m...[0m
[1m[94m652[0m [1m[94m|[0m [1m[33m|[0m         CoreScheduleWindowKind[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m656[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m649[0m [1m[94m| [0m[92mconst [0mfn kind_name(k: &CoreScheduleWindowKind) -> &'static str {
    [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:658:1
    [1m[94m|[0m
[1m[94m658[0m [1m[94m|[0m [1m[33m/[0m fn slip_name(r: &CoreSlipReaso[1m[94m...[0m
[1m[94m659[0m [1m[94m|[0m [1m[33m|[0m     match r {
[1m[94m660[0m [1m[94m|[0m [1m[33m|[0m         CoreSlipReason::Skippe[1m[94m...[0m
[1m[94m661[0m [1m[94m|[0m [1m[33m|[0m         CoreSlipReason::Deferr[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m665[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m658[0m [1m[94m| [0m[92mconst [0mfn slip_name(r: &CoreSlipReason) -> &'static str {
    [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:669:9
    [1m[94m|[0m
[1m[94m669[0m [1m[94m|[0m         TopPriorityLineDto {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:681:9
    [1m[94m|[0m
[1m[94m681[0m [1m[94m|[0m         ScheduleWindowLineDto {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:692:9
    [1m[94m|[0m
[1m[94m692[0m [1m[94m|[0m         SchedulePreviewDto {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:702:9
    [1m[94m|[0m
[1m[94m702[0m [1m[94m|[0m         MorningBriefDto {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:719:9
    [1m[94m|[0m
[1m[94m719[0m [1m[94m|[0m         ShippedTaskDto {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:730:9
    [1m[94m|[0m
[1m[94m730[0m [1m[94m|[0m         SlippedTaskDto {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:741:9
    [1m[94m|[0m
[1m[94m741[0m [1m[94m|[0m         EveningShutdownDto {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:756:9
    [1m[94m|[0m
[1m[94m756[0m [1m[94m|[0m         RuleSummaryDto {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:767:9
    [1m[94m|[0m
[1m[94m767[0m [1m[94m|[0m         StreakSnapshotDto {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:777:9
    [1m[94m|[0m
[1m[94m777[0m [1m[94m|[0m         WeeklyReviewDto {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:801:9
    [1m[94m|[0m
[1m[94m801[0m [1m[94m|[0m         MonthDeltaDto {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:812:9
    [1m[94m|[0m
[1m[94m812[0m [1m[94m|[0m         MonthlyRetroDto {
    [1m[94m|[0m         [1m[33m^^^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
   [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:871:1
    [1m[94m|[0m
[1m[94m871[0m [1m[94m|[0m [1m[33m/[0m /// [`CalendarPort`] implement[1m[94m...[0m
[1m[94m872[0m [1m[94m|[0m [1m[33m|[0m /// callback. Round-trips ISO8[1m[94m...[0m
[1m[94m873[0m [1m[94m|[0m [1m[33m|[0m /// and parses them back into [1m[94m...[0m
[1m[94m874[0m [1m[94m|[0m [1m[33m|[0m /// supported by the host shim[1m[94m...[0m
[1m[94m875[0m [1m[94m|[0m [1m[33m|[0m /// write to them), so both re[1m[94m...[0m
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph
    [1m[94m= [0m[1mnote[0m: `-W clippy::too-long-first-doc-paragraph` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::too_long_first_doc_paragraph)]`

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1055:17
     [1m[94m|[0m
[1m[94m1046[0m [1m[94m|[0m       pub fn apply_mutation(&self, m: WalletMutationDto) -> Result<(), FfiError> {
     [1m[94m|[0m [1m[94m ________________________________________________________________________________-[0m
[1m[94m1047[0m [1m[94m|[0m [1m[94m|[0m         let now = Utc::now();
[1m[94m1048[0m [1m[94m|[0m [1m[94m|[0m         let core = m.into_core(now)?;
[1m[94m1049[0m [1m[94m|[0m [1m[94m|[0m         let adapter = self.ctx.adapter.clone();
[1m[94m...[0m    [1m[94m|[0m
[1m[94m1055[0m [1m[94m|[0m [1m[94m|[0m         let mut chain = self
     [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m...[0m    [1m[94m|[0m
[1m[94m1067[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m1068[0m [1m[94m|[0m [1m[94m|[0m     }
     [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `chain` is currently being dropped at the end of its contained scope[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
     [1m[94m= [0m[1mnote[0m: `-W clippy::significant-drop-tightening` implied by `-W clippy::nursery`
     [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::significant_drop_tightening)]`
[1m[96mhelp[0m: merge the temporary construction with its single usage
     [1m[94m|[0m
[1m[94m1055[0m [92m~ [0m        
[1m[94m1056[0m [92m+         self[0m
[1m[94m1057[0m [92m+             .ctx[0m
[1m[94m1058[0m [92m+             .audit[0m
[1m[94m1059[0m [92m+             .chain[0m
[1m[94m1060[0m [92m+             .lock()[0m
[1m[94m1061[0m [92m+             .map_err(|e| FfiError::Storage(format!("audit chain poisoned: {e}")))?.append([0m
[1m[94m1062[0m [92m+             "wallet.mutation",[0m
[1m[94m1063[0m [92m+             self.ctx.user_id.to_string(),[0m
[1m[94m1064[0m [92m+             serde_json::json!({"at": now.to_rfc3339()}),[0m
[1m[94m1065[0m [92m+             now,[0m
[1m[94m1066[0m [92m+         );[0m
[1m[94m1067[0m [92m~ [0m        
     [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1126:17
     [1m[94m|[0m
[1m[94m1118[0m [1m[94m|[0m       pub fn apply(&self, m: PenaltyMutationDto) -> Result<(), FfiError> {
     [1m[94m|[0m [1m[94m ________________________________________________________________________-[0m
[1m[94m1119[0m [1m[94m|[0m [1m[94m|[0m         let now = Utc::now();
[1m[94m1120[0m [1m[94m|[0m [1m[94m|[0m         let core = m.into_core()?;
[1m[94m1121[0m [1m[94m|[0m [1m[94m|[0m         let adapter = self.ctx.adapter.clone();
[1m[94m...[0m    [1m[94m|[0m
[1m[94m1126[0m [1m[94m|[0m [1m[94m|[0m         let mut chain = self
     [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m...[0m    [1m[94m|[0m
[1m[94m1138[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m1139[0m [1m[94m|[0m [1m[94m|[0m     }
     [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `chain` is currently being dropped at the end of its contained scope[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
     [1m[94m|[0m
[1m[94m1126[0m [92m~ [0m        
[1m[94m1127[0m [92m+         self[0m
[1m[94m1128[0m [92m+             .ctx[0m
[1m[94m1129[0m [92m+             .audit[0m
[1m[94m1130[0m [92m+             .chain[0m
[1m[94m1131[0m [92m+             .lock()[0m
[1m[94m1132[0m [92m+             .map_err(|e| FfiError::Storage(format!("audit chain poisoned: {e}")))?.append([0m
[1m[94m1133[0m [92m+             "penalty.mutation",[0m
[1m[94m1134[0m [92m+             self.ctx.user_id.to_string(),[0m
[1m[94m1135[0m [92m+             serde_json::json!({"at": now.to_rfc3339()}),[0m
[1m[94m1136[0m [92m+             now,[0m
[1m[94m1137[0m [92m+         );[0m
[1m[94m1138[0m [92m~ [0m        
     [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1156:13
     [1m[94m|[0m
[1m[94m1155[0m [1m[94m|[0m       ) -> Result<EnforcementPolicySummary, FfiError> {
     [1m[94m|[0m [1m[94m _____________________________________________________-[0m
[1m[94m1156[0m [1m[94m|[0m [1m[94m|[0m         let recent = self
     [1m[94m|[0m [1m[94m|[0m             [1m[33m^^^^^^[0m
[1m[94m1157[0m [1m[94m|[0m [1m[94m|[0m             .ctx
[1m[94m1158[0m [1m[94m|[0m [1m[94m|[0m             .recent_decisions
[1m[94m...[0m    [1m[94m|[0m
[1m[94m1186[0m [1m[94m|[0m [1m[94m|[0m         })
[1m[94m1187[0m [1m[94m|[0m [1m[94m|[0m     }
     [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `recent` is currently being dropped at the end of its contained scope[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
     [1m[94m|[0m
[1m[94m1166[0m [92m~ [0m        let slice: Vec<PrioritizedDecision> = recent.iter().rev().take(n).cloned().collect();
[1m[94m1167[0m [92m+         drop(recent);[0m
     [1m[94m|[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1373:1
     [1m[94m|[0m
[1m[94m1373[0m [1m[94m|[0m [1m[33m/[0m fn rigidity_tag(r: &focus_dom[1m[94m...[0m
[1m[94m1374[0m [1m[94m|[0m [1m[33m|[0m     match r {
[1m[94m1375[0m [1m[94m|[0m [1m[33m|[0m         focus_domain::Rigidit[1m[94m...[0m
[1m[94m1376[0m [1m[94m|[0m [1m[33m|[0m         focus_domain::Rigidit[1m[94m...[0m
[1m[94m...[0m    [1m[33m|[0m
[1m[94m1379[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1373[0m [1m[94m| [0m[92mconst [0mfn rigidity_tag(r: &focus_domain::Rigidity) -> &'static str {
     [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1381:1
     [1m[94m|[0m
[1m[94m1381[0m [1m[94m|[0m [1m[33m/[0m fn task_status_tag(s: &CoreTa[1m[94m...[0m
[1m[94m1382[0m [1m[94m|[0m [1m[33m|[0m     match s {
[1m[94m1383[0m [1m[94m|[0m [1m[33m|[0m         CoreTaskStatus::Pendi[1m[94m...[0m
[1m[94m1384[0m [1m[94m|[0m [1m[33m|[0m         CoreTaskStatus::Sched[1m[94m...[0m
[1m[94m...[0m    [1m[33m|[0m
[1m[94m1389[0m [1m[94m|[0m [1m[33m|[0m }
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m1381[0m [1m[94m| [0m[92mconst [0mfn task_status_tag(s: &CoreTaskStatus) -> &'static str {
     [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1393:36
     [1m[94m|[0m
[1m[94m1393[0m [1m[94m|[0m       let (deadline_iso, rigidity) = matc[1m[94m...[0m
     [1m[94m|[0m [1m[33m ____________________________________^[0m
[1m[94m1394[0m [1m[94m|[0m [1m[33m|[0m         Some(w) => (
[1m[94m1395[0m [1m[94m|[0m [1m[33m|[0m             Some(w.to_rfc3339()),
[1m[94m1396[0m [1m[94m|[0m [1m[33m|[0m             rigidity_tag(&t.deadline.ri[1m[94m...[0m
[1m[94m...[0m    [1m[33m|[0m
[1m[94m1400[0m [1m[94m|[0m [1m[33m|[0m         None => (None, rigidity_tag(&t.[1m[94m...[0m
[1m[94m1401[0m [1m[94m|[0m [1m[33m|[0m     };
     [1m[94m|[0m [1m[33m|_____^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
     [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
     [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`
[1m[96mhelp[0m: try
     [1m[94m|[0m
[1m[94m1393[0m [92m~ [0m    let (deadline_iso, rigidity) = [92mt.deadline.when.map_or_else(|| (None, rigidity_tag(&t.deadline.rigidity).to_string()), |w| ([0m
[1m[94m1394[0m [92m+             Some(w.to_rfc3339()),[0m
[1m[94m1395[0m [92m+             rigidity_tag(&t.deadline.rigidity).to_string(),[0m
[1m[94m1396[0m [92m~         ))[0m;
     [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1676:17
     [1m[94m|[0m
[1m[94m1598[0m [1m[94m|[0m       pub fn connect_canvas(&self, instance_url: String, code: String) -> Result<(), FfiError> {
     [1m[94m|[0m [1m[94m ______________________________________________________________________________________________-[0m
[1m[94m1599[0m [1m[94m|[0m [1m[94m|[0m         use connector_canvas::auth::{CanvasAuthConfig, CanvasOAuth2, KeychainStore, TokenStore};
[1m[94m1600[0m [1m[94m|[0m [1m[94m|[0m         use connector_canvas::CanvasConnector;
[1m[94m...[0m    [1m[94m|[0m
[1m[94m1676[0m [1m[94m|[0m [1m[94m|[0m         let mut chain = self
     [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m...[0m    [1m[94m|[0m
[1m[94m1691[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m1692[0m [1m[94m|[0m [1m[94m|[0m     }
     [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `chain` is currently being dropped at the end of its contained scope[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
     [1m[94m|[0m
[1m[94m1676[0m [92m~ [0m        
[1m[94m1677[0m [92m+         self[0m
[1m[94m1678[0m [92m+             .ctx[0m
[1m[94m1679[0m [92m+             .audit[0m
[1m[94m1680[0m [92m+             .chain[0m
[1m[94m1681[0m [92m+             .lock()[0m
[1m[94m1682[0m [92m+             .map_err(|e| FfiError::Storage(format!("audit chain poisoned: {e}")))?.append([0m
[1m[94m1683[0m [92m+             "connector.canvas.connected",[0m
[1m[94m1684[0m [92m+             account,[0m
[1m[94m1685[0m [92m+             serde_json::json!({[0m
[1m[94m1686[0m [92m+                 "at": now.to_rfc3339(),[0m
[1m[94m1687[0m [92m+                 "instance": cleaned,[0m
[1m[94m1688[0m [92m+             }),[0m
[1m[94m1689[0m [92m+             now,[0m
[1m[94m1690[0m [92m+         );[0m
[1m[94m1691[0m [92m~ [0m        
     [1m[94m|[0m

[1m[33mwarning[0m[1m: redundant clone[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1658:46
     [1m[94m|[0m
[1m[94m1658[0m [1m[94m|[0m [1m[94m...[0mase_url.clone())
     [1m[94m|[0m           [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
     [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1658:38
     [1m[94m|[0m
[1m[94m1658[0m [1m[94m|[0m [1m[94m...[0m::builder(base_url.clone())
     [1m[94m|[0m              [1m[92m^^^^^^^^[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone
     [1m[94m= [0m[1mnote[0m: `-W clippy::redundant-clone` implied by `-W clippy::nursery`
     [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::redundant_clone)]`

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1763:17
     [1m[94m|[0m
[1m[94m1697[0m [1m[94m|[0m       pub fn connect_gcal(&self, code: String) -> Result<(), FfiError> {
     [1m[94m|[0m [1m[94m ______________________________________________________________________-[0m
[1m[94m1698[0m [1m[94m|[0m [1m[94m|[0m         use connector_gcal::auth::{GCalAuthConfig, GCalOAuth2, KeychainSt[1m[94m...[0m
[1m[94m1699[0m [1m[94m|[0m [1m[94m|[0m
[1m[94m1700[0m [1m[94m|[0m [1m[94m|[0m         if code.trim().is_empty() {
[1m[94m...[0m    [1m[94m|[0m
[1m[94m1763[0m [1m[94m|[0m [1m[94m|[0m         let mut chain = self
     [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m...[0m    [1m[94m|[0m
[1m[94m1778[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m1779[0m [1m[94m|[0m [1m[94m|[0m     }
     [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `chain` is currently being dropped at the end of its contained scope[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
     [1m[94m|[0m
[1m[94m1763[0m [92m~ [0m        
[1m[94m1764[0m [92m+         self[0m
[1m[94m1765[0m [92m+             .ctx[0m
[1m[94m1766[0m [92m+             .audit[0m
[1m[94m1767[0m [92m+             .chain[0m
[1m[94m1768[0m [92m+             .lock()[0m
[1m[94m1769[0m [92m+             .map_err(|e| FfiError::Storage(format!("audit chain poisoned: {e}")))?.append([0m
[1m[94m1770[0m [92m+             "connector.gcal.connected",[0m
[1m[94m1771[0m [92m+             account,[0m
[1m[94m1772[0m [92m+             serde_json::json!({[0m
[1m[94m1773[0m [92m+                 "at": now.to_rfc3339(),[0m
[1m[94m1774[0m [92m+                 "identity": identity,[0m
[1m[94m1775[0m [92m+             }),[0m
[1m[94m1776[0m [92m+             now,[0m
[1m[94m1777[0m [92m+         );[0m
[1m[94m1778[0m [92m~ [0m        
     [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1844:17
     [1m[94m|[0m
[1m[94m1786[0m [1m[94m|[0m       pub fn connect_github(&self, pat: String) -> Result<(), FfiError> {
     [1m[94m|[0m [1m[94m _______________________________________________________________________-[0m
[1m[94m1787[0m [1m[94m|[0m [1m[94m|[0m         use connector_github::api::{GitHubClient, DEFAULT_BASE_URL};
[1m[94m1788[0m [1m[94m|[0m [1m[94m|[0m         use connector_github::auth::{GitHubToken, KeychainStore, TokenStore};
[1m[94m1789[0m [1m[94m|[0m [1m[94m|[0m         use focus_connectors::ConnectorError;
[1m[94m...[0m    [1m[94m|[0m
[1m[94m1844[0m [1m[94m|[0m [1m[94m|[0m         let mut chain = self
     [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m...[0m    [1m[94m|[0m
[1m[94m1859[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m1860[0m [1m[94m|[0m [1m[94m|[0m     }
     [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `chain` is currently being dropped at the end of its contained scope[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
     [1m[94m|[0m
[1m[94m1844[0m [92m~ [0m        
[1m[94m1845[0m [92m+         self[0m
[1m[94m1846[0m [92m+             .ctx[0m
[1m[94m1847[0m [92m+             .audit[0m
[1m[94m1848[0m [92m+             .chain[0m
[1m[94m1849[0m [92m+             .lock()[0m
[1m[94m1850[0m [92m+             .map_err(|e| FfiError::Storage(format!("audit chain poisoned: {e}")))?.append([0m
[1m[94m1851[0m [92m+             "connector.github.connected",[0m
[1m[94m1852[0m [92m+             account,[0m
[1m[94m1853[0m [92m+             serde_json::json!({[0m
[1m[94m1854[0m [92m+                 "at": now.to_rfc3339(),[0m
[1m[94m1855[0m [92m+                 "login": login,[0m
[1m[94m1856[0m [92m+             }),[0m
[1m[94m1857[0m [92m+             now,[0m
[1m[94m1858[0m [92m+         );[0m
[1m[94m1859[0m [92m~ [0m        
     [1m[94m|[0m

[1m[33mwarning[0m[1m: first doc comment paragraph is too long[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1863:1
     [1m[94m|[0m
[1m[94m1863[0m [1m[94m|[0m [1m[33m/[0m /// Event → Rule → Action eva[1m[94m...[0m
[1m[94m1864[0m [1m[94m|[0m [1m[33m|[0m /// [`focus_eval::RuleEvaluat[1m[94m...[0m
[1m[94m1865[0m [1m[94m|[0m [1m[33m|[0m /// this right after `SyncApi[1m[94m...[0m
[1m[94m1866[0m [1m[94m|[0m [1m[33m|[0m /// wallet / penalty / policy[1m[94m...[0m
     [1m[94m|[0m [1m[33m|_^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#too_long_first_doc_paragraph

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:1926:21
     [1m[94m|[0m
[1m[94m1925[0m [1m[94m|[0m           self.ctx.runtime.block_on(async move {
     [1m[94m|[0m [1m[94m ______________________________________________-[0m
[1m[94m1926[0m [1m[94m|[0m [1m[94m|[0m             let mut guard = sync.lock().await;
     [1m[94m|[0m [1m[94m|[0m                     [1m[33m^^^^^[0m
[1m[94m1927[0m [1m[94m|[0m [1m[94m|[0m             let report = guard.tick(Utc::now()).a[1m[94m...[0m
[1m[94m1928[0m [1m[94m|[0m [1m[94m|[0m             SyncReportDto {
[1m[94m...[0m    [1m[94m|[0m
[1m[94m1937[0m [1m[94m|[0m [1m[94m|[0m         })
     [1m[94m|[0m [1m[94m|_________-[0m [1m[94mtemporary `guard` is currently being dropped at the end of its contained scope[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: drop the temporary after the end of its last usage
     [1m[94m|[0m
[1m[94m1927[0m [92m~ [0m            let report = guard.tick(Utc::now()).await;
[1m[94m1928[0m [92m+             drop(guard);[0m
     [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:2021:17
     [1m[94m|[0m
[1m[94m1961[0m [1m[94m|[0m       pub fn emit(&self, dto: HostEventDto) -> Result<(), FfiError> {
     [1m[94m|[0m [1m[94m ___________________________________________________________________-[0m
[1m[94m1962[0m [1m[94m|[0m [1m[94m|[0m         let event_type_raw = dto.event_type.trim();
[1m[94m1963[0m [1m[94m|[0m [1m[94m|[0m         if event_type_raw.is_empty() {
[1m[94m1964[0m [1m[94m|[0m [1m[94m|[0m             return Err(FfiError::InvalidArgument(
[1m[94m...[0m    [1m[94m|[0m
[1m[94m2021[0m [1m[94m|[0m [1m[94m|[0m         let mut chain = self
     [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^[0m
[1m[94m...[0m    [1m[94m|[0m
[1m[94m2036[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m2037[0m [1m[94m|[0m [1m[94m|[0m     }
     [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `chain` is currently being dropped at the end of its contained scope[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
     [1m[94m|[0m
[1m[94m2021[0m [92m~ [0m        
[1m[94m2022[0m [92m+         self[0m
[1m[94m2023[0m [92m+             .ctx[0m
[1m[94m2024[0m [92m+             .audit[0m
[1m[94m2025[0m [92m+             .chain[0m
[1m[94m2026[0m [92m+             .lock()[0m
[1m[94m2027[0m [92m+             .map_err(|e| FfiError::Storage(format!("audit chain poisoned: {e}")))?.append([0m
[1m[94m2028[0m [92m+             "host.event.emitted",[0m
[1m[94m2029[0m [92m+             self.ctx.user_id.to_string(),[0m
[1m[94m2030[0m [92m+             serde_json::json!({[0m
[1m[94m2031[0m [92m+                 "event_type": event_type_raw,[0m
[1m[94m2032[0m [92m+                 "at": now.to_rfc3339(),[0m
[1m[94m2033[0m [92m+             }),[0m
[1m[94m2034[0m [92m+             now,[0m
[1m[94m2035[0m [92m+         );[0m
[1m[94m2036[0m [92m~ [0m        
     [1m[94m|[0m

[1m[33mwarning[0m[1m: redundant clone[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:2013:37
     [1m[94m|[0m
[1m[94m2013[0m [1m[94m|[0m [1m[94m...[0m= event.clone();
     [1m[94m|[0m           [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
     [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:2013:32
     [1m[94m|[0m
[1m[94m2013[0m [1m[94m|[0m [1m[94m...[0mr_append = event.clone();
     [1m[94m|[0m               [1m[92m^^^^^[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:2183:38
     [1m[94m|[0m
[1m[94m2183[0m [1m[94m|[0m [1m[94m...[0mcus => NudgeKindDto::StartFo[1m[94m...[0m
     [1m[94m|[0m           [1m[33m^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:2184:37
     [1m[94m|[0m
[1m[94m2184[0m [1m[94m|[0m [1m[94m...[0meak => NudgeKindDto::TakeBreak,
     [1m[94m|[0m           [1m[33m^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:2185:42
     [1m[94m|[0m
[1m[94m2185[0m [1m[94m|[0m [1m[94m...[0mine => NudgeKindDto::ReviewD[1m[94m...[0m
     [1m[94m|[0m           [1m[33m^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:2186:40
     [1m[94m|[0m
[1m[94m2186[0m [1m[94m|[0m [1m[94m...[0misk => NudgeKindDto::StreakA[1m[94m...[0m
     [1m[94m|[0m           [1m[33m^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:2187:36
     [1m[94m|[0m
[1m[94m2187[0m [1m[94m|[0m [1m[94m...[0mown => NudgeKindDto::WindDown,
     [1m[94m|[0m           [1m[33m^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: this could be a `const fn`[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:2475:5
     [1m[94m|[0m
[1m[94m2475[0m [1m[94m|[0m [1m[33m/[0m     pub fn fetch(&self, _wind[1m[94m...[0m
[1m[94m2476[0m [1m[94m|[0m [1m[33m|[0m         // Placeholder: in pr[1m[94m...[0m
[1m[94m2477[0m [1m[94m|[0m [1m[33m|[0m         // For now, return em[1m[94m...[0m
[1m[94m2478[0m [1m[94m|[0m [1m[33m|[0m         Ok(Vec::new())
[1m[94m2479[0m [1m[94m|[0m [1m[33m|[0m     }
     [1m[94m|[0m [1m[33m|_____^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
[1m[96mhelp[0m: make the function `const`
     [1m[94m|[0m
[1m[94m2475[0m [1m[94m| [0m    pub[92m const[0m fn fetch(&self, _window_days: u32) -> Result<Vec<RuleSuggestionDto>, FfiError> {
     [1m[94m|[0m         [92m+++++[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:2484:17
     [1m[94m|[0m
[1m[94m2481[0m [1m[94m|[0m       pub fn apply(&self, suggestion_id: String) -> Result<(), FfiError> {
     [1m[94m|[0m [1m[94m ________________________________________________________________________-[0m
[1m[94m2482[0m [1m[94m|[0m [1m[94m|[0m         // In production: deserialize proposed rule from suggestion and call
[1m[94m2483[0m [1m[94m|[0m [1m[94m|[0m         // rules_mut().upsert() to persist it. For now, accept idempotently.
[1m[94m2484[0m [1m[94m|[0m [1m[94m|[0m         let mut dismissed = self
     [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^^^^^[0m
[1m[94m...[0m    [1m[94m|[0m
[1m[94m2489[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m2490[0m [1m[94m|[0m [1m[94m|[0m     }
     [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `dismissed` is currently being dropped at the end of its contained scope[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
     [1m[94m|[0m
[1m[94m2484[0m [92m~ [0m        
[1m[94m2485[0m [92m+         self[0m
[1m[94m2486[0m [92m+             .dismissed[0m
[1m[94m2487[0m [92m+             .lock()[0m
[1m[94m2488[0m [92m+             .map_err(|e| FfiError::Poisoned(format!("dismissed lock: {}", e)))?.remove(&suggestion_id);[0m
[1m[94m2489[0m [92m~ [0m        
     [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:2493:17
     [1m[94m|[0m
[1m[94m2492[0m [1m[94m|[0m       pub fn dismiss(&self, suggestion_id: String) -> Result<(), FfiError> {
     [1m[94m|[0m [1m[94m __________________________________________________________________________-[0m
[1m[94m2493[0m [1m[94m|[0m [1m[94m|[0m         let mut dismissed = self
     [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^^^^^[0m
[1m[94m2494[0m [1m[94m|[0m [1m[94m|[0m             .dismissed
[1m[94m2495[0m [1m[94m|[0m [1m[94m|[0m             .lock()
[1m[94m...[0m    [1m[94m|[0m
[1m[94m2498[0m [1m[94m|[0m [1m[94m|[0m         Ok(())
[1m[94m2499[0m [1m[94m|[0m [1m[94m|[0m     }
     [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `dismissed` is currently being dropped at the end of its contained scope[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
     [1m[94m|[0m
[1m[94m2493[0m [92m~ [0m        
[1m[94m2494[0m [92m+         self[0m
[1m[94m2495[0m [92m+             .dismissed[0m
[1m[94m2496[0m [92m+             .lock()[0m
[1m[94m2497[0m [92m+             .map_err(|e| FfiError::Poisoned(format!("dismissed lock: {}", e)))?.insert(suggestion_id);[0m
[1m[94m2498[0m [92m~ [0m        
     [1m[94m|[0m

[1m[33mwarning[0m[1m: temporary with significant `Drop` can be early dropped[0m
    [1m[94m--> [0mcrates/focus-ffi/src/lib.rs:2698:17
     [1m[94m|[0m
[1m[94m2697[0m [1m[94m|[0m       pub fn push_mascot_event(&self, event: MascotEvent) -> MascotState {
     [1m[94m|[0m [1m[94m ________________________________________________________________________-[0m
[1m[94m2698[0m [1m[94m|[0m [1m[94m|[0m         let mut machine = self.mascot.lock().expect("mascot mutex poisoned");
     [1m[94m|[0m [1m[94m|[0m                 [1m[33m^^^^^^^[0m
[1m[94m2699[0m [1m[94m|[0m [1m[94m|[0m         let core_event: CoreMascotEvent = event.into();
[1m[94m2700[0m [1m[94m|[0m [1m[94m|[0m         let next = machine.on_event(core_event);
[1m[94m2701[0m [1m[94m|[0m [1m[94m|[0m         MascotState::from(next)
[1m[94m2702[0m [1m[94m|[0m [1m[94m|[0m     }
     [1m[94m|[0m [1m[94m|_____-[0m [1m[94mtemporary `machine` is currently being dropped at the end of its contained scope[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mnote[0m: this might lead to unnecessary resource contention
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#significant_drop_tightening
[1m[96mhelp[0m: merge the temporary construction with its single usage
     [1m[94m|[0m
[1m[94m2698[0m [92m~ [0m        
[1m[94m2699[0m [92m+         let next = self.mascot.lock().expect("mascot mutex poisoned").on_event(core_event);[0m
[1m[94m2700[0m [1m[94m|[0m         let core_event: CoreMascotEvent = event.into();
[1m[94m2701[0m [92m~ [0m        
     [1m[94m|[0m

[1m[33mwarning[0m: `focus-ffi` (lib) generated 155 warnings (run `cargo clippy --fix --lib -p focus-ffi -- -W clippy::nursery` to apply 141 suggestions)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
  [1m[94m--> [0mcrates/focus-lang/src/bulk.rs:46:1
   [1m[94m|[0m
[1m[94m46[0m [1m[94m|[0m [1m[33m/[0m fn default_true() -> bool {
[1m[94m47[0m [1m[94m|[0m [1m[33m|[0m     true
[1m[94m48[0m [1m[94m|[0m [1m[33m|[0m }
   [1m[94m|[0m [1m[33m|_^[0m
   [1m[94m|[0m
   [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
   [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
   [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
   [1m[94m|[0m
[1m[94m46[0m [1m[94m| [0m[92mconst [0mfn default_true() -> bool {
   [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-lang/src/lib.rs:361:19
    [1m[94m|[0m
[1m[94m361[0m [1m[94m|[0m     AllOf(Vec<Box<ConditionData>>),
    [1m[94m|[0m                   [1m[33m^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self
    [1m[94m= [0m[1mnote[0m: `-W clippy::use-self` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::use_self)]`

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-lang/src/lib.rs:362:19
    [1m[94m|[0m
[1m[94m362[0m [1m[94m|[0m     AnyOf(Vec<Box<ConditionData>>),
    [1m[94m|[0m                   [1m[33m^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m[1m: unnecessary structure name repetition[0m
   [1m[94m--> [0mcrates/focus-lang/src/lib.rs:363:13
    [1m[94m|[0m
[1m[94m363[0m [1m[94m|[0m     Not(Box<ConditionData>),
    [1m[94m|[0m             [1m[33m^^^^^^^^^^^^^[0m [1m[33mhelp: use the applicable keyword: `Self`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#use_self

[1m[33mwarning[0m: `focus-lang` (lib) generated 4 warnings (run `cargo clippy --fix --lib -p focus-lang -- -W clippy::nursery` to apply 4 suggestions)
[1m[92m    Checking[0m focus-cli v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
 [1m[94m--> [0mcrates/focus-cli/src/lib.rs:5:1
  [1m[94m|[0m
[1m[94m5[0m [1m[94m|[0m pub fn run() {}
  [1m[94m|[0m [1m[33m^^^^^^^^^^^^^^^[0m
  [1m[94m|[0m
  [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
  [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
  [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
  [1m[94m|[0m
[1m[94m5[0m [1m[94m| [0mpub[92m const[0m fn run() {}
  [1m[94m|[0m     [92m+++++[0m

[1m[33mwarning[0m: `focus-cli` (lib) generated 1 warning (run `cargo clippy --fix --lib -p focus-cli -- -W clippy::nursery` to apply 1 suggestion)
[1m[33mwarning[0m[1m: this could be a `const fn`[0m
   [1m[94m--> [0mcrates/focus-cli/src/replay.rs:121:1
    [1m[94m|[0m
[1m[94m121[0m [1m[94m|[0m [1m[33m/[0m fn parse_fpl_ruleset(content: [1m[94m...[0m
[1m[94m122[0m [1m[94m|[0m [1m[33m|[0m     // Stub: in production, th[1m[94m...[0m
[1m[94m123[0m [1m[94m|[0m [1m[33m|[0m     // For now, return an empt[1m[94m...[0m
[1m[94m124[0m [1m[94m|[0m [1m[33m|[0m     let _ = content;
[1m[94m125[0m [1m[94m|[0m [1m[33m|[0m     Ok(Vec::new())
[1m[94m126[0m [1m[94m|[0m [1m[33m|[0m }
    [1m[94m|[0m [1m[33m|_^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#missing_const_for_fn
    [1m[94m= [0m[1mnote[0m: `-W clippy::missing-const-for-fn` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::missing_const_for_fn)]`
[1m[96mhelp[0m: make the function `const`
    [1m[94m|[0m
[1m[94m121[0m [1m[94m| [0m[92mconst [0mfn parse_fpl_ruleset(content: &str) -> anyhow::Result<Vec<Rule>> {
    [1m[94m|[0m [92m+++++[0m

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
   [1m[94m--> [0mcrates/focus-cli/src/main.rs:839:40
    [1m[94m|[0m
[1m[94m839[0m [1m[94m|[0m   [1m[94m...[0m   let deadline_obj = if le[1m[94m...[0m
    [1m[94m|[0m [1m[33m __________________________^[0m
[1m[94m840[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       match chrono::DateTi[1m[94m...[0m
[1m[94m841[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           Ok(dt) => {
[1m[94m842[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m               let utc = dt[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m851[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       focus_planning::Dead[1m[94m...[0m
[1m[94m852[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   };
    [1m[94m|[0m [1m[33m|_______^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
    [1m[94m= [0m[1mnote[0m: `-W clippy::option-if-let-else` implied by `-W clippy::nursery`
    [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::option_if_let_else)]`
[1m[96mhelp[0m: try
    [1m[94m|[0m
[1m[94m839[0m [92m~ [0m                    let deadline_obj = [92mtask_yaml.deadline.as_ref().map_or_else(focus_planning::Deadline::none, |deadline_str| match chrono::DateTime::parse_from_rfc3339(deadline_str) {[0m
[1m[94m840[0m [92m+                             Ok(dt) => {[0m
[1m[94m841[0m [92m+                                 let utc = dt.with_timezone(&Utc);[0m
[1m[94m842[0m [92m+                                 focus_planning::Deadline {[0m
[1m[94m843[0m [92m+                                     when: Some(utc),[0m
[1m[94m844[0m [92m+                                     rigidity: focus_domain::Rigidity::Soft,[0m
[1m[94m845[0m [92m+                                 }[0m
[1m[94m846[0m [92m+                             }[0m
[1m[94m847[0m [92m+                             Err(_) => focus_planning::Deadline::none(),[0m
[1m[94m848[0m [92m~                         })[0m;
    [1m[94m|[0m

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
   [1m[94m--> [0mcrates/focus-cli/src/main.rs:840:25
    [1m[94m|[0m
[1m[94m840[0m [1m[94m|[0m [1m[33m/[0m [1m[94m...[0m   match chrono::DateTime::[1m[94m...[0m
[1m[94m841[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       Ok(dt) => {
[1m[94m842[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           let utc = dt.wit[1m[94m...[0m
[1m[94m843[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           focus_planning::[1m[94m...[0m
[1m[94m...[0m   [1m[33m|[0m
[1m[94m848[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       Err(_) => focus_plan[1m[94m...[0m
[1m[94m849[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   }
    [1m[94m|[0m [1m[33m|_______^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
[1m[96mhelp[0m: try
    [1m[94m|[0m
[1m[94m840[0m [92m~ [0m                        [92mchrono::DateTime::parse_from_rfc3339(deadline_str).map_or_else(|_| focus_planning::Deadline::none(), |dt| {[0m
[1m[94m841[0m [92m+                                 let utc = dt.with_timezone(&Utc);[0m
[1m[94m842[0m [92m+                                 focus_planning::Deadline {[0m
[1m[94m843[0m [92m+                                     when: Some(utc),[0m
[1m[94m844[0m [92m+                                     rigidity: focus_domain::Rigidity::Soft,[0m
[1m[94m845[0m [92m+                                 }[0m
[1m[94m846[0m [92m+                             })[0m
    [1m[94m|[0m

[1m[33mwarning[0m[1m: redundant clone[0m
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:1207:59
     [1m[94m|[0m
[1m[94m1207[0m [1m[94m|[0m [1m[94m...[0mr, rule.clone()))?;
     [1m[94m|[0m           [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
     [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:1207:55
     [1m[94m|[0m
[1m[94m1207[0m [1m[94m|[0m [1m[94m...[0me(&adapter, rule.clone()))?;
     [1m[94m|[0m                [1m[92m^^^^[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone
     [1m[94m= [0m[1mnote[0m: `-W clippy::redundant-clone` implied by `-W clippy::nursery`
     [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::redundant_clone)]`

[1m[33mwarning[0m[1m: use Option::map_or instead of an if let/else[0m
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:1344:43
     [1m[94m|[0m
[1m[94m1344[0m [1m[94m|[0m   [1m[94m...[0m   let action_kind = if le[1m[94m...[0m
     [1m[94m|[0m [1m[33m _________________________^[0m
[1m[94m1345[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       match action {
[1m[94m1346[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           focus_rules::Ac[1m[94m...[0m
[1m[94m1347[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           focus_rules::Ac[1m[94m...[0m
[1m[94m...[0m    [1m[33m|[0m
[1m[94m1360[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       "None"
[1m[94m1361[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   };
     [1m[94m|[0m [1m[33m|_______^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
[1m[96mhelp[0m: try
     [1m[94m|[0m
[1m[94m1344[0m [92m~ [0m                        let action_kind = [92mr.actions.first().map_or("None", |action| match action {[0m
[1m[94m1345[0m [92m+                                 focus_rules::Action::GrantCredit { .. } => "GrantCredit",[0m
[1m[94m1346[0m [92m+                                 focus_rules::Action::DeductCredit { .. } => "DeductCredit",[0m
[1m[94m1347[0m [92m+                                 focus_rules::Action::Block { .. } => "Block",[0m
[1m[94m1348[0m [92m+                                 focus_rules::Action::Unblock { .. } => "Unblock",[0m
[1m[94m1349[0m [92m+                                 focus_rules::Action::StreakIncrement(_) => "StreakIncrement",[0m
[1m[94m1350[0m [92m+                                 focus_rules::Action::StreakReset(_) => "StreakReset",[0m
[1m[94m1351[0m [92m+                                 focus_rules::Action::Notify(_) => "Notify",[0m
[1m[94m1352[0m [92m+                                 focus_rules::Action::EmergencyExit { .. } => "EmergencyExit",[0m
[1m[94m1353[0m [92m+                                 focus_rules::Action::Intervention { .. } => "Intervention",[0m
[1m[94m1354[0m [92m+                                 focus_rules::Action::ScheduledUnlockWindow { .. } => {[0m
[1m[94m1355[0m [92m+                                     "ScheduledUnlockWindow"[0m
[1m[94m1356[0m [92m+                                 }[0m
[1m[94m1357[0m [92m~                             })[0m;
     [1m[94m|[0m

[1m[33mwarning[0m[1m: use Option::map_or instead of an if let/else[0m
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:1362:38
     [1m[94m|[0m
[1m[94m1362[0m [1m[94m|[0m   [1m[94m...[0m   let amount = if let Som[1m[94m...[0m
     [1m[94m|[0m [1m[33m ____________________^[0m
[1m[94m1363[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       match action {
[1m[94m1364[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           focus_rules::Ac[1m[94m...[0m
[1m[94m1365[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           | focus_rules::[1m[94m...[0m
[1m[94m...[0m    [1m[33m|[0m
[1m[94m1369[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       None
[1m[94m1370[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   };
     [1m[94m|[0m [1m[33m|_______^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
[1m[96mhelp[0m: try
     [1m[94m|[0m
[1m[94m1362[0m [92m~ [0m                        let amount = [92mr.actions.first().map_or(None, |action| match action {[0m
[1m[94m1363[0m [92m+                                 focus_rules::Action::GrantCredit { amount }[0m
[1m[94m1364[0m [92m+                                 | focus_rules::Action::DeductCredit { amount } => Some(*amount),[0m
[1m[94m1365[0m [92m+                                 _ => None,[0m
[1m[94m1366[0m [92m~                             })[0m;
     [1m[94m|[0m

[1m[33mwarning[0m[1m: use Option::map_or instead of an if let/else[0m
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:1401:43
     [1m[94m|[0m
[1m[94m1401[0m [1m[94m|[0m   [1m[94m...[0m   let action_kind = if le[1m[94m...[0m
     [1m[94m|[0m [1m[33m _________________________^[0m
[1m[94m1402[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       match action {
[1m[94m1403[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           focus_rules::Ac[1m[94m...[0m
[1m[94m1404[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           focus_rules::Ac[1m[94m...[0m
[1m[94m...[0m    [1m[33m|[0m
[1m[94m1417[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       "None"
[1m[94m1418[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   };
     [1m[94m|[0m [1m[33m|_______^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
[1m[96mhelp[0m: try
     [1m[94m|[0m
[1m[94m1401[0m [92m~ [0m                        let action_kind = [92mr.actions.first().map_or("None", |action| match action {[0m
[1m[94m1402[0m [92m+                                 focus_rules::Action::GrantCredit { .. } => "GrantCredit",[0m
[1m[94m1403[0m [92m+                                 focus_rules::Action::DeductCredit { .. } => "DeductCredit",[0m
[1m[94m1404[0m [92m+                                 focus_rules::Action::Block { .. } => "Block",[0m
[1m[94m1405[0m [92m+                                 focus_rules::Action::Unblock { .. } => "Unblock",[0m
[1m[94m1406[0m [92m+                                 focus_rules::Action::StreakIncrement(_) => "StreakIncrement",[0m
[1m[94m1407[0m [92m+                                 focus_rules::Action::StreakReset(_) => "StreakReset",[0m
[1m[94m1408[0m [92m+                                 focus_rules::Action::Notify(_) => "Notify",[0m
[1m[94m1409[0m [92m+                                 focus_rules::Action::EmergencyExit { .. } => "EmergencyExit",[0m
[1m[94m1410[0m [92m+                                 focus_rules::Action::Intervention { .. } => "Intervention",[0m
[1m[94m1411[0m [92m+                                 focus_rules::Action::ScheduledUnlockWindow { .. } => {[0m
[1m[94m1412[0m [92m+                                     "ScheduledUnlockWindow"[0m
[1m[94m1413[0m [92m+                                 }[0m
[1m[94m1414[0m [92m~                             })[0m;
     [1m[94m|[0m

[1m[33mwarning[0m[1m: use Option::map_or instead of an if let/else[0m
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:1419:38
     [1m[94m|[0m
[1m[94m1419[0m [1m[94m|[0m   [1m[94m...[0m   let amount = if let Som[1m[94m...[0m
     [1m[94m|[0m [1m[33m ____________________^[0m
[1m[94m1420[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       match action {
[1m[94m1421[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           focus_rules::Ac[1m[94m...[0m
[1m[94m1422[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m           | focus_rules::[1m[94m...[0m
[1m[94m...[0m    [1m[33m|[0m
[1m[94m1426[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m       None
[1m[94m1427[0m [1m[94m|[0m [1m[33m|[0m [1m[94m...[0m   };
     [1m[94m|[0m [1m[33m|_______^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else
[1m[96mhelp[0m: try
     [1m[94m|[0m
[1m[94m1419[0m [92m~ [0m                        let amount = [92mr.actions.first().map_or(None, |action| match action {[0m
[1m[94m1420[0m [92m+                                 focus_rules::Action::GrantCredit { amount }[0m
[1m[94m1421[0m [92m+                                 | focus_rules::Action::DeductCredit { amount } => Some(*amount),[0m
[1m[94m1422[0m [92m+                                 _ => None,[0m
[1m[94m1423[0m [92m~                             })[0m;
     [1m[94m|[0m

[1m[33mwarning[0m[1m: redundant clone[0m
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:1540:36
     [1m[94m|[0m
[1m[94m1540[0m [1m[94m|[0m [1m[94m...[0mpurpose.clone(),
     [1m[94m|[0m           [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
     [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:1540:29
     [1m[94m|[0m
[1m[94m1540[0m [1m[94m|[0m [1m[94m...[0m   reason: purpose.clone(),
     [1m[94m|[0m               [1m[92m^^^^^^^[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone

[1m[33mwarning[0m[1m: redundant clone[0m
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:1576:36
     [1m[94m|[0m
[1m[94m1576[0m [1m[94m|[0m [1m[94m...[0mpurpose.clone(),
     [1m[94m|[0m           [1m[33m^^^^^^^^[0m [1m[33mhelp: remove this[0m
     [1m[94m|[0m
[1m[92mnote[0m: this value is dropped without further use
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:1576:29
     [1m[94m|[0m
[1m[94m1576[0m [1m[94m|[0m [1m[94m...[0m   reason: purpose.clone(),
     [1m[94m|[0m               [1m[92m^^^^^^^[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#redundant_clone

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:1875:5
     [1m[94m|[0m
[1m[94m1875[0m [1m[94m|[0m [1m[33m/[0m     if let Some(paren_pos) = [1m[94m...[0m
[1m[94m1876[0m [1m[94m|[0m [1m[33m|[0m         prefix[..paren_pos].t[1m[94m...[0m
[1m[94m1877[0m [1m[94m|[0m [1m[33m|[0m     } else {
[1m[94m1878[0m [1m[94m|[0m [1m[33m|[0m         prefix.to_string()
[1m[94m1879[0m [1m[94m|[0m [1m[33m|[0m     }
     [1m[94m|[0m [1m[33m|_____^[0m [1m[33mhelp: try: `prefix.find('(').map_or_else(|| prefix.to_string(), |paren_pos| prefix[..paren_pos].to_string())`[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else

[1m[33mwarning[0m[1m: use Option::map_or_else instead of an if let/else[0m
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:2828:13
     [1m[94m|[0m
[1m[94m2828[0m [1m[94m|[0m [1m[33m/[0m             match chars.next() {
[1m[94m2829[0m [1m[94m|[0m [1m[33m|[0m                 None => Strin[1m[94m...[0m
[1m[94m2830[0m [1m[94m|[0m [1m[33m|[0m                 Some(first) =[1m[94m...[0m
[1m[94m2831[0m [1m[94m|[0m [1m[33m|[0m             }
     [1m[94m|[0m [1m[33m|_____________^[0m [1m[33mhelp: try: `chars.next().map_or_else(String::new, |first| first.to_uppercase().collect::<String>() + chars.as_str())`[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#option_if_let_else

[1m[33mwarning[0m[1m: all if blocks contain the same code at the start[0m
    [1m[94m--> [0mcrates/focus-cli/src/main.rs:2873:5
     [1m[94m|[0m
[1m[94m2873[0m [1m[94m|[0m [1m[33m/[0m     if json {
[1m[94m2874[0m [1m[94m|[0m [1m[33m|[0m         // Output is already[1m[94m...[0m
[1m[94m2875[0m [1m[94m|[0m [1m[33m|[0m         println!("{}", result);
     [1m[94m|[0m [1m[33m|_______________________________^[0m
     [1m[94m|[0m
     [1m[94m= [0m[1mhelp[0m: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.95.0/index.html#branches_sharing_code
     [1m[94m= [0m[1mnote[0m: `-W clippy::branches-sharing-code` implied by `-W clippy::nursery`
     [1m[94m= [0m[1mhelp[0m: to override `-W clippy::nursery` add `#[allow(clippy::branches_sharing_code)]`
[1m[96mhelp[0m: consider moving these statements before the if
     [1m[94m|[0m
[1m[94m2873[0m [92m~ [0m    [92mprintln!("{}", result);[0m
[1m[94m2874[0m [92m+     if json {[0m
     [1m[94m|[0m

[1m[33mwarning[0m: `focus-cli` (bin "focus") generated 13 warnings (run `cargo clippy --fix --bin "focus" -p focus-cli -- -W clippy::nursery` to apply 4 suggestions)
[1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`dev` profile [unoptimized + debuginfo]]8;;\ target(s) in 16.47s
EXIT=0
