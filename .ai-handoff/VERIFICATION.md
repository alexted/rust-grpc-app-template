# VERIFICATION — actual command results

## Baseline (target @ a54e38c, before any change) — all executed in /home/alex_goat/CLionProjects/rust-grpc-app-template

| Date | Command | Result | Notes |
|---|---|---|---|
| 2026-11-11 | `cargo check --workspace --all-targets` | PASS | clean build |
| 2026-11-11 | `cargo test --workspace` | PASS | 0 tests exist |
| 2026-11-11 | `cargo clippy --workspace --all-targets --all-features` | 3 pre-existing warnings | `double_must_use` ×2 in `my_service_api_grpc` (tonic async_trait macro), `result_large_err` in gRPC handler, workspace resolver="1" defaulting note |
| 2026-11-11 | `cargo build` + run binary + `curl -X POST localhost:3000/greeter/hello` | behavior verified | `{"name":"Alice"}` → 200 `{"message":"Hello, Alice!"}`; `{"name":"ab"}` → 400 body `Invalid name length`; `{"name":"аб"}` → 200 (byte semantics) |

## Post-refactoring (all in /home/alex_goat/CLionProjects/rust-grpc-app-template)

| Date | Command | Result | Notes |
|---|---|---|---|
| 2026-11-11 | `cargo fmt --check` | PASS | after `cargo fmt --all` |
| 2026-11-11 | `cargo check --workspace --all-targets` | PASS | 6 crates |
| 2026-11-11 | `cargo test --workspace` | PASS | 11 tests: domain 4 (incl. byte-vs-char), http adapter 2, http integration 2, grpc service 2, grpc wire-over-TCP 2, settings 1 |
| 2026-11-11 | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS (exit 0) | tonic-generated-code lints (`result_large_err`, `double_must_use`) scoped-allowed on the generated `proto` modules only |
| 2026-11-11 | Runtime smoke (rebuilt binary) | behavior preserved | startup logs both addresses; `{"name":"Bob"}` → 200 `{"message":"Hello, Bob!"}`; `{"name":"xy"}` → 400 `Invalid name length`; `{"name":"аб"}` → 200 (byte semantics); both ports LISTEN; SIGINT → graceful exit, ports released |
| 2026-11-11 | gRPC wire tests (client↔server over real TCP, ephemeral port) | PASS | 200-class reply `Hello, Alice!` and `INVALID_ARGUMENT`/`Invalid name length` verified on the wire |

## Unverified
- Nothing known. `grpcurl` is not installed on this machine; gRPC verification was performed by
  the automated wire tests instead (real client over real TCP socket).
- Cargo.lock: axum 0.7.2 → 0.7.5 within declared "0.7" (needed for `Serve::with_graceful_shutdown`).
