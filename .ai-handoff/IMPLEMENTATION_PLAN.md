# IMPLEMENTATION PLAN

Statuses: `NOT_STARTED` | `IN_PROGRESS` | `COMPLETED` | `BLOCKED`

## Stage 1 — Baseline and investigation — COMPLETED
- Git state of both repos recorded; reference worktree @8b82629 confirmed as latest intentional state.
- Full source inspection of both apps; architecture map written.
- Baseline executed: `cargo check` OK, `cargo test` OK (0 tests), clippy 3 pre-existing warnings,
  HTTP runtime smoke verified (200/400/byte semantics).
- Handoff skeleton created.

## Stage 2 — Architectural correspondence — COMPLETED
- `ARCHITECTURE_MAP.md` + `DECISIONS.md` D-01..D-08 written.

## Stage 3 — Workspace and crate architecture — COMPLETED
- `container` crate (`my_service_container`) created — `Container { hello_use_case: Arc<HelloUseCase> }`.
- `infrastructure` crate (`my_service_infrastructure`) created — `settings.rs` (AppConfig:
  HTTP_HOST/HTTP_PORT default 0.0.0.0:3000, GRPC_HOST/GRPC_PORT default 0.0.0.0:50051,
  LOG_LEVEL default INFO; `get_config()`), `logging_config.rs` (`init_logging`).
- Root `Cargo.toml`: members extended; `resolver = "2"`; all dependency versions centralized
  in `[workspace.dependencies]`. Dependency graph matches ARCHITECTURE_MAP §4.

## Stage 4 — Core application architecture (domain crate) — COMPLETED
- `domain/src/errors.rs` — `GreeterError` (thiserror), message preserved.
- `domain/src/use_cases/mod.rs` + `hello.rs` — `HelloUseCase::new()`, `execute(HelloRequest)`,
  `HelloRequest{name}` (Deserialize), `HelloResponse{message}` (Serialize+Deserialize);
  byte-length validation ≥3; 4 unit tests incl. byte-vs-char semantics.
- Old `domain/src/greeter.rs` deleted (`hello` free fn, `HelloInput/Output`, `GreeterResult`).
- `cargo test -p my_service_domain` passes.

## Stage 5 — Data and infrastructure — COMPLETED
- Infrastructure crate implemented; no data layer introduced (D-05); settings test passes.

## Stage 6 — HTTP integration — COMPLETED
- `api/http/src/lib.rs` — `create_app(container) -> Router` (factory), `serve(addr, container,
  shutdown)` with graceful shutdown, `RouteError` wrapping `GreeterError` → 400 + message.
- `api/http/src/greeter.rs` — thin handler using the shared `HelloUseCase` and domain models.
- Integration tests (`tower` oneshot): 200 + JSON, 400 + message. Pass.

## Stage 7 — gRPC integration — COMPLETED
- `api/grpc/src/lib.rs` — `serve(addr, container, shutdown)` with `serve_with_shutdown`;
  `GreeterService` + `greeter_service` constructor re-exported (needed by wire tests).
- `api/grpc/src/greeter.rs` — service holding the shared use case, proto↔domain conversions,
  `Status::invalid_argument` mapping; service-level tests + real wire tests over TCP
  (`tests/wire.rs`, tokio-stream dev-dep). Pass. Proto/build.rs untouched.

## Stage 8 — Runtime and cross-cutting — COMPLETED
- `cli/src/main.rs` — config → logging → `Arc<Container>` → both servers with ctrl_c shutdown,
  `tokio::try_join!`, anyhow error propagation. Runtime smoke verified (startup logs,
  200/400/unicode HTTP, ports listening, SIGINT clean exit, ports released).
- README fully rewritten; module doc comments in every source file.

## Stage 9 — Verification and cleanup — COMPLETED
- `cargo fmt --check` PASS; `cargo check --workspace --all-targets` PASS;
  `cargo test --workspace` PASS (11 tests); `cargo clippy --workspace --all-targets
  --all-features -- -D warnings` PASS (exit 0).
- Runtime smoke PASS; unused direct deps removed; axum lock 0.7.2 → 0.7.5 within "0.7".

## Stage 10 — Independent final architecture review — COMPLETED
- Final tree, stale-reference grep, dependency hygiene pass, 17-dimension comparison against
  the reference (see PROGRESS.md). Deliberate deviations recorded in DECISIONS.md (D-01..D-09).
  No unnecessary discrepancies found requiring correction.
- Compare result vs reference across the 17 dimensions from the task; correct unnecessary
  discrepancies; finalize all handoff documents.
