# PROGRESS — chronological journal

## Session 1

1. Inspected git state of both repos. Target: branch `dev`, HEAD `a54e38c`, clean except
   untracked `.idea/`. Reference: main checkout on `main` (old arch); adapted architecture
   confirmed at worktree `/home/alex_goat/.local/share/opencode/worktree/41dece/witty-tiger`
   @ `8b82629` (`refactor/clean-architecture-rewrite`), no extra local commits.
2. Read every source file of the reference (main/lib/container/api/domain/data/infrastructure)
   and of the target (all crates). Reference is a single-crate onion app: use cases with
   Request/Response models, `Container` composition root, settings/logging/errors in
   infrastructure::core, thin actix handlers, postgres data layer.
3. Baseline verification executed (see VERIFICATION.md): check OK, test OK (0 tests),
   clippy 3 pre-existing warnings, HTTP smoke verified on running binary
   (200 `{"message":"Hello, Alice!"}`, 400 `Invalid name length` for 2-byte name,
   200 for 2-char/4-byte Cyrillic name → validation is byte-length ≥ 3).
4. Created `.ai-handoff/` skeleton: START_HERE, TASK_SPEC, ARCHITECTURE_MAP, DECISIONS (D-01..D-08),
   IMPLEMENTATION_PLAN (Stages 1–2 COMPLETED).

## Session 1 (continued) — implementation of Stages 3–10

5. Stage 4: rewrote `domain` crate — `errors.rs` (`GreeterError`, thiserror, same message),
   `use_cases/mod.rs` + `use_cases/hello.rs` (`HelloUseCase::new`/`execute`, `HelloRequest`
   Deserialize, `HelloResponse` Serialize+Deserialize, byte-length ≥3 rule), 4 unit tests;
   deleted old `greeter.rs`; added serde to domain deps.
6. Stage 3/5: created `container` crate (`Container { hello_use_case: Arc<HelloUseCase> }`,
   `new()` + `Default`) and `infrastructure` crate (`settings.rs` with env config + defaults
   0.0.0.0:3000/0.0.0.0:50051 + hermetic test, `logging_config.rs` env_logger); root
   `Cargo.toml` gained both members, `resolver = "2"`, fully centralized `[workspace.dependencies]`.
7. Stage 6: rewrote `api/http` — `create_app(container) -> Router` factory, `serve(addr,
   container, shutdown)` with graceful shutdown, `RouteError` (IntoResponse → 400 + message),
   thin handler using domain models directly; tower-oneshot integration tests.
8. Stage 7: rewrote `api/grpc` — `serve(...)` with `serve_with_shutdown`, `GreeterService`
   holding the shared use case, proto↔domain conversions, `Status::invalid_argument` mapping;
   service tests + real wire tests over TCP (`tests/wire.rs`, tokio-stream dev-dep);
   `GreeterService`/`greeter_service` re-exported for composition; tonic-generated lints
   scoped-allowed on the `proto` module only.
9. Stage 8: rewrote `cli` — config → logging → `Arc<Container>` → `tokio::try_join!` of both
   servers with ctrl_c graceful shutdown; README fully rewritten (architecture, config, run,
   test, extension guide); module doc comments everywhere.
10. Stage 9: full verification suite passes — fmt, check, 11 tests, clippy `-D warnings`
    (exit 0); runtime smoke preserved all baseline behavior incl. byte semantics; SIGINT clean
    exit with ports released. Removed unused deps (`serde` from api/http, `log` from
    infrastructure); bumped axum lock 0.7.2 → 0.7.5 within "0.7" for graceful-shutdown API.
11. Stage 10: final review — no stale references, dependency hygiene checked, 17-dimension
    comparison against the reference completed; deliberate deviations documented in DECISIONS.
