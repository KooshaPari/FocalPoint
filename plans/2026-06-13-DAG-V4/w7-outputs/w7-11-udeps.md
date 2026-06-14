W7-11: cargo udeps
[1m[92m    Blocking[0m waiting for file lock on package cache
[1m[92m    Blocking[0m waiting for file lock on package cache
[1m[92m    Blocking[0m waiting for file lock on package cache
[1m[92m   Compiling[0m proc-macro2 v1.0.106
[1m[92m   Compiling[0m quote v1.0.45
[1m[92m   Compiling[0m unicode-ident v1.0.24
[1m[92m   Compiling[0m serde_core v1.0.228
[1m[92m   Compiling[0m libc v0.2.186
[1m[92m    Checking[0m cfg-if v1.0.4
[1m[92m   Compiling[0m serde v1.0.228
[1m[92m    Checking[0m memchr v2.8.0
error: the option `Z` is only accepted on the nightly compiler

error: the option `Z` is only accepted on the nightly compiler

help: consider switching to a nightly toolchain: `rustup default nightly`

note: selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>

note: for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>

error: the option `Z` is only accepted on the nightly compiler
error: 1 nightly option were parsed


help: consider switching to a nightly toolchain: `rustup default nightly`

note: selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>

note: for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>

error: 1 nightly option were parsed

[1m[91merror[0m: could not compile `proc-macro2` (build script)

Caused by:
  process didn't exit successfully: `rustc --crate-name build_script_build --edition=2021 /Users/kooshapari/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/proc-macro2-1.0.106/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no --cfg 'feature="default"' --cfg 'feature="proc-macro"' --cfg 'feature="span-locations"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "nightly", "proc-macro", "span-locations"))' -C metadata=0e461e817152177c -C extra-filename=-5829337c173b52e1 --out-dir /Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/proc-macro2-5829337c173b52e1 -L dependency=/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 1)
[1m[33mwarning[0m: build failed, waiting for other jobs to finish...
[1m[91merror[0m: could not compile `memchr` (lib)

Caused by:
  process didn't exit successfully: `rustc --crate-name memchr --edition=2021 /Users/kooshapari/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C split-debuginfo=unpacked --cfg 'feature="alloc"' --cfg 'feature="default"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("alloc", "core", "default", "libc", "logging", "rustc-dep-of-std", "std", "use_std"))' -C metadata=86df18f1b0d2e2d3 -C extra-filename=-88df81f5fb812ad6 --out-dir /Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps -L dependency=/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 1)
help: consider switching to a nightly toolchain: `rustup default nightly`

note: selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>

note: for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>

error: 1 nightly option were parsed

[1m[91merror[0m: could not compile `libc` (build script)

Caused by:
  process didn't exit successfully: `rustc --crate-name build_script_build --edition=2021 /Users/kooshapari/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.186/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no '--allow=clippy::used_underscore_binding' --allow=unused_qualifications '--warn=clippy::unnecessary_semicolon' '--allow=clippy::unnecessary_cast' '--allow=clippy::uninlined_format_args' '--warn=clippy::ptr_as_ptr' '--allow=clippy::non_minimal_cfg' '--allow=clippy::missing_safety_doc' '--warn=clippy::map_unwrap_or' '--warn=clippy::manual_assert' '--allow=clippy::identity_op' '--warn=clippy::explicit_iter_loop' '--allow=clippy::expl_impl_clone_on_copy' --cfg 'feature="default"' --cfg 'feature="extra_traits"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("align", "const-extern-fn", "default", "extra_traits", "rustc-dep-of-std", "rustc-std-workspace-core", "std", "use_std"))' -C metadata=abd477088ab662c6 -C extra-filename=-4d64efc98897b67a --out-dir /Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/libc-4d64efc98897b67a -L dependency=/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 1)
error: the option `Z` is only accepted on the nightly compiler

help: consider switching to a nightly toolchain: `rustup default nightly`

note: selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>

note: for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>

error: 1 nightly option were parsed

[1m[91merror[0m: could not compile `unicode-ident` (lib)

Caused by:
  process didn't exit successfully: `rustc --crate-name unicode_ident --edition=2021 /Users/kooshapari/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/unicode-ident-1.0.24/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=2161cd28b098a302 -C extra-filename=-3e0d08f935476298 --out-dir /Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps -L dependency=/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 1)
error: the option `Z` is only accepted on the nightly compiler
error: the option `Z` is only accepted on the nightly compiler

help: consider switching to a nightly toolchain: `rustup default nightly`

note: selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>

note: for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>

error: 1 nightly option were parsed

[1m[91merror[0m: could not compile `quote` (build script)

Caused by:
  process didn't exit successfully: `rustc --crate-name build_script_build --edition=2021 /Users/kooshapari/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/quote-1.0.45/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no --cfg 'feature="default"' --cfg 'feature="proc-macro"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "proc-macro"))' -C metadata=d0907865c71cf196 -C extra-filename=-41c28335ff6cfe51 --out-dir /Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/quote-41c28335ff6cfe51 -L dependency=/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 1)

help: consider switching to a nightly toolchain: `rustup default nightly`

note: selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>

note: for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>

error: 1 nightly option were parsed

[1m[91merror[0m: could not compile `cfg-if` (lib)

Caused by:
  process didn't exit successfully: `rustc --crate-name cfg_if --edition=2018 /Users/kooshapari/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cfg-if-1.0.4/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C split-debuginfo=unpacked --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("core", "rustc-dep-of-std"))' -C metadata=56120df17a04a0a2 -C extra-filename=-a3e5604381f7a3e6 --out-dir /Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps -L dependency=/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 1)
error: the option `Z` is only accepted on the nightly compiler

help: consider switching to a nightly toolchain: `rustup default nightly`

note: selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>

note: for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>

error: 1 nightly option were parsed

error: the option `Z` is only accepted on the nightly compiler

help: consider switching to a nightly toolchain: `rustup default nightly`

note: selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>

note: for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>

error: 1 nightly option were parsed

[1m[91merror[0m: could not compile `serde_core` (build script)

Caused by:
  process didn't exit successfully: `rustc --crate-name build_script_build --edition=2021 /Users/kooshapari/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no --cfg 'feature="alloc"' --cfg 'feature="default"' --cfg 'feature="rc"' --cfg 'feature="result"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("alloc", "default", "rc", "result", "std", "unstable"))' -C metadata=cf6ec2a371d0b149 -C extra-filename=-94017e25195e1106 --out-dir /Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/serde_core-94017e25195e1106 -L dependency=/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 1)
[1m[91merror[0m: could not compile `serde` (build script)

Caused by:
  process didn't exit successfully: `rustc --crate-name build_script_build --edition=2021 /Users/kooshapari/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no --cfg 'feature="alloc"' --cfg 'feature="default"' --cfg 'feature="derive"' --cfg 'feature="rc"' --cfg 'feature="serde_derive"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("alloc", "default", "derive", "rc", "serde_derive", "std", "unstable"))' -C metadata=c7fef20f00ca1f7f -C extra-filename=-86f9dd86108b772c --out-dir /Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/serde-86f9dd86108b772c -L dependency=/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 1)
error: the option `Z` is only accepted on the nightly compiler

help: consider switching to a nightly toolchain: `rustup default nightly`

note: selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>

note: for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>

error: 1 nightly option were parsed

[1m[91merror[0m: could not compile `libc` (build script)

Caused by:
  process didn't exit successfully: `rustc --crate-name build_script_build --edition=2021 /Users/kooshapari/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.186/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no '--allow=clippy::used_underscore_binding' --allow=unused_qualifications '--warn=clippy::unnecessary_semicolon' '--allow=clippy::unnecessary_cast' '--allow=clippy::uninlined_format_args' '--warn=clippy::ptr_as_ptr' '--allow=clippy::non_minimal_cfg' '--allow=clippy::missing_safety_doc' '--warn=clippy::map_unwrap_or' '--warn=clippy::manual_assert' '--allow=clippy::identity_op' '--warn=clippy::explicit_iter_loop' '--allow=clippy::expl_impl_clone_on_copy' --cfg 'feature="default"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("align", "const-extern-fn", "default", "extra_traits", "rustc-dep-of-std", "rustc-std-workspace-core", "std", "use_std"))' -C metadata=2f12c28a8553b93f -C extra-filename=-715a8dd9deacd898 --out-dir /Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/libc-715a8dd9deacd898 -L dependency=/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 1)
error: the option `Z` is only accepted on the nightly compiler

help: consider switching to a nightly toolchain: `rustup default nightly`

note: selecting a toolchain with `+toolchain` arguments require a rustup proxy; see <https://rust-lang.github.io/rustup/concepts/index.html>

note: for more information about Rust's stability policy, see <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#unstable-features>

error: 1 nightly option were parsed

[1m[91merror[0m: could not compile `serde_core` (build script)

Caused by:
  process didn't exit successfully: `rustc --crate-name build_script_build --edition=2021 /Users/kooshapari/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no --cfg 'feature="alloc"' --cfg 'feature="default"' --cfg 'feature="result"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("alloc", "default", "rc", "result", "std", "unstable"))' -C metadata=d3cf33c1fa448b79 -C extra-filename=-eaece20129898af1 --out-dir /Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/build/serde_core-eaece20129898af1 -L dependency=/Users/kooshapari/CodeProjects/Phenotype/repos/target/debug/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 1)
EXIT=0
