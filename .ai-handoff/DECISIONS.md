# DECISIONS — architectural decision log

Status markers: `FINAL/IMPLEMENTED`, `FINAL` (decided, implementation pending), `TENTATIVE`.

---

## D-01 (FINAL/IMPLEMENTED) — Keep the Cargo workspace; add `container` and `infrastructure` crates

- Problem: reference is one crate; target already is a workspace (`cli`, `domain`, `api/http`,
  `api/grpc`).
- Alternatives: (a) merge into one crate like the reference; (b) keep workspace as-is and put
  composition + config in `cli`; (c) keep workspace and add small crates for the composition root
  and infrastructure concerns.
- Selected: (c). The workspace gives compile-time enforcement that `api/http` and `api/grpc`
  never depend on each other and that `domain` has no transport deps. New crates:
  `container` (composition root, depends on `domain`) and `infrastructure` (settings + logging,
  no app deps).
- Consequences: both transports depend on `container` + `domain`; `cli` depends on everything;
  dependency direction matches the onion model.

## D-02 (FINAL/IMPLEMENTED) — Use-case objects without the `BaseUseCase` trait

- Problem: reference defines `BaseUseCase<Request, Response>` (async_trait) implemented by every
  use case.
- Why not reproduced: the task says to adapt the concept "only where it makes practical sense"
  and to avoid generic abstraction frameworks for a simple application. The single use case is
  synchronous and there is no polymorphism site; a trait + `async-trait` dependency would be
  abstraction for its own sake.
- Selected: inherent `pub async fn execute(...)`-style method — here a plain
  `pub fn execute(&self, request: HelloRequest) -> Result<HelloResponse, GreeterError>` on
  `HelloUseCase`. Naming and structure mirror the reference (`XxxUseCase::new`,
  `XxxRequest`/`XxxResponse`).
- Consequences: if use cases later need polymorphism (e.g. generic transport handling), a trait
  can be introduced without breaking consumers.

## D-03 (FINAL/IMPLEMENTED) — Shared error vocabulary lives in the `domain` crate

- Problem: reference keeps `OtherError` in `infrastructure::core::errors` while use cases
  (domain) import it — fine in one crate, but in a workspace that inverts the dependency
  direction (inner layer would depend on the outermost crate).
- Selected: `domain::errors::GreeterError` (thiserror), transport-independent. HTTP maps it to
  400 + message; gRPC maps it to `INVALID_ARGUMENT` + message — preserving baseline behavior.
- Consequences: one error type shared by use cases and both adapters; no dependency inversion.

## D-04 (FINAL/IMPLEMENTED) — `Container` holds the shared use case (no repositories)

- Problem: reference `Container` holds repositories; handlers build use cases per request. The
  target has no persistence and therefore no repositories/data layer.
- Selected: `Container { pub hello_use_case: Arc<HelloUseCase> }` constructed once in `cli` and
  shared with both servers. Per-request use-case construction (repository clones) has no analog
  here; sharing a stateless use case via `Arc` is the idiomatic Rust equivalent.
- Consequences: when repositories are added, `Container` grows repository fields and use cases
  are built from them — the composition root seam already exists. No artificial empty
  repository/data layer was introduced.

## D-05 (FINAL/IMPLEMENTED) — No middlewares, no data layer, no postgres/diesel

- Problem: reference has `data/` (repositories/DTOs), postgres client, and four middlewares.
- Why not reproduced: the target has no persistence and no service-context/maintenance concept;
  adding them would be artificial infrastructure (explicitly forbidden).
- Selected: HTTP error mapping happens via a small `RouteError` wrapper implementing
  `IntoResponse` (the axum analog of the reference's error-handling middleware); gRPC maps in the
  service method. Request-logging/trace-id middleware and interceptors are a documented future
  seam, not built now.
- Trade-off: less cross-cutting machinery than the reference; consistent with the app's actual
  needs.

## D-06 (FINAL/IMPLEMENTED) — env_logger/log for logging (like the reference), no tracing stack

- Problem: task asks for consistent logging with trace correlation "where appropriate".
- Selected: `infrastructure::logging_config::init_logging` using `env_logger` + `log` exactly
  like the reference (LOG_LEVEL, RUST_LOG precedence). No tracing/tower-http/tonic-interceptor
  machinery: it would add dependencies and complexity for a template whose handlers share one
  use case; the seam (HTTP middleware / gRPC interceptor) is documented in DECISIONS and README.
- Consequences: minimal deps; startup logging present; correlation can be added later uniformly.

## D-07 (FINAL/IMPLEMENTED) — Use-case models are the HTTP JSON contracts

- Problem: original HTTP adapter duplicated `HelloRequest`/`HelloResponse` DTOs and converted
  them to `HelloInput`/`HelloOutput`.
- Selected: domain `HelloRequest`/`HelloResponse` derive serde and are used directly as axum
  `Json` bodies (exactly like the reference's serde-deriving use-case models). gRPC keeps proto
  types strictly inside `api/grpc` with conversions at the boundary (generated types must not
  leak — a gRPC-specific constraint with no reference analog).
- Consequences: removes duplication; serialization contract owned by the application layer;
  proto isolation preserved.

## D-08 (FINAL/IMPLEMENTED) — Graceful shutdown via ctrl_c in `cli`

- Problem: original `cli` panicked on server error; no shutdown story.
- Selected: both adapters expose `serve(addr, container, shutdown)`; `cli` passes a
  `tokio::signal::ctrl_c()` future to each; `tokio::try_join!` waits for both. Each framework
  shuts down gracefully (axum `with_graceful_shutdown`, tonic `serve_with_shutdown`).
- Consequences: no orchestration framework, lifecycle owned by the CLI (process boundary), clean
  container/resource drop.

## D-09 (FINAL/IMPLEMENTED) — axum lock 0.7.2 → 0.7.5 and scoped lint allowances for generated code

- Problem: (a) axum 0.7.2's `Serve` lacks `with_graceful_shutdown`; (b) tonic-build 0.10's
  generated service code trips `clippy::result_large_err` and `clippy::double_must_use`, which
  become errors under `-D warnings` (they were pre-existing warnings at baseline).
- Selected: (a) `cargo update -p axum@0.7.2` → 0.7.5, still within the declared `"0.7"`
  requirement (not a dependency upgrade); (b) `#[allow(...)]` scoped to the `proto` modules that
  `tonic::include_proto!` expands into — the signatures are fixed by the protobuf contract, so
  the lints cannot be acted upon; handwritten code remains fully linted.
- Consequences: `cargo clippy -- -D warnings` passes workspace-wide; no behavioral change.
