# TASK_SPEC — Persistent copy of the task requirements

## Architectural objective

Refactor the FINAL APPLICATION source of the target repository so its logical and physical
architecture corresponds, as closely as reasonably possible, to the adapted Actix clean/onion
architecture (reference commit `8b82629` on branch `refactor/clean-architecture-rewrite`).
The reference is a guide, not an immutable spec; justified Rust/dual-transport adaptations are
expected and must be documented (approach in Actix → why not reproduced → chosen alternative →
trade-offs → consequences).

## Repositories

- MODIFY (only this): `/home/alex_goat/CLionProjects/rust-grpc-app-template`
- READ ONLY (never modify; do not merge its PR; do not switch branches):
  `/home/alex_goat/CLionProjects/actix-clean-architecture-app-template` — adapted architecture
  available in worktree `/home/alex_goat/.local/share/opencode/worktree/41dece/witty-tiger` @ 8b82629.

## Restrictions

- Never overwrite/discard existing user changes; no destructive git ops; no commits/push without
  explicit authorization.
- Do not refactor Cookiecutter/Jinja template mechanics; edit source files in their actual
  locations; only minimal wrapper adjustments.
- Do not add PostgreSQL/Diesel/Redis/Kafka or other tech merely because the reference has them.
- Do not upgrade dependencies without a concrete reason; no breaking protocol changes.

## Technology preservation

- Keep Axum (HTTP) and Tonic+Prost (gRPC) as two independent transport adapters over shared
  application logic. Protobuf package/service/RPC/message names and field numbers unchanged.
- HTTP layer: routing, JSON, validation, thin handlers, HTTP error mapping, middleware.
- gRPC layer: tonic services, proto conversion at the boundary, interceptors if needed, Status mapping.
- Application layer must not depend on axum/tonic/proto types.

## Workspace design requirements

Evaluate crate boundaries as an independent dimension. Candidate responsibilities: domain,
application/use cases, data contracts, infrastructure, HTTP API, gRPC API, bootstrap. Prefer
boundaries that enforce meaningful architecture via compile-time dependencies; avoid artificial
fragmentation and one-crate-per-layer automatism. Record the decision, alternatives, reasoning.

## Cross-cutting requirements

- One composition root (the reference's `Container` concept) providing shared dependencies to
  BOTH servers; no duplicated initialization; idiomatic Rust DI (no DI framework).
- Configuration: explicit, testable, extensible; defaults HTTP `0.0.0.0:3000`, gRPC `0.0.0.0:50051`.
- Consistent logging approach (reference uses env_logger/log with LOG_LEVEL).
- Validation: business validation in the application/domain boundary; transport parsing at the
  transport boundary. Preserve existing validation semantics (BYTE length ≥ 3).

## Behavioral preservation (verified at baseline)

- `POST /greeter/hello` with `{"name": <≥3 bytes>}` → 200 `{"message":"Hello, {name}!"}`
- name shorter than 3 bytes → HTTP 400 body `Invalid name length`; gRPC `INVALID_ARGUMENT`.
- gRPC `greeter.Greeter/Hello`, request field `name`, response field `message`.
- Preserve all other existing functionality/contracts discovered during investigation.

## Error architecture

Application-level errors defined independently of HTTP and gRPC; each transport translates at
its boundary. Do not leak infrastructure details through external responses.

## Documentation

Update application-level documentation (README) and module docs to describe the actual
architecture: layers, responsibilities, dependency direction, crate/module organization,
composition, HTTP/gRPC integration, runtime/lifecycle, configuration, testing, extension points.
English only; remove obsolete descriptions.

## Verification requirements

After refactoring, actually execute and record in `.ai-handoff/VERIFICATION.md`:
- `cargo fmt --check`
- `cargo check --workspace --all-targets`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- HTTP behavior checks (success + validation failure) and gRPC checks (success + status mapping).
Never report a command as successful unless actually executed and passed.

## Cross-session continuity requirements (mandatory)

Maintain `.ai-handoff/` with START_HERE.md, TASK_SPEC.md, CURRENT_STATE.md (live execution
cursor), ARCHITECTURE_MAP.md, DECISIONS.md, IMPLEMENTATION_PLAN.md, PROGRESS.md,
VERIFICATION.md, RESUME_PROMPT.txt. Atomic operations with checkpoint discipline; the next AI
model must be able to resume the exact next operation without re-doing investigation/planning.

## Definition of done

Logical architecture coherent (responsibilities, relationships, dependency inversion,
transport-independent application logic, separated infrastructure). Physical architecture
coherent (deliberate crates/modules, consistent naming, sensible visibility). Dual-transport
architecture intact with preserved protocol contracts. Idiomatic Rust, verified, documented,
fully resumable via `.ai-handoff/`.
