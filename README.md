# rust-grpc-app-template

A Rust back-end service template exposing the same application over two independent
transports — HTTP (Axum) and gRPC (Tonic/Prost) — organized as a clean/onion architecture:
thin transport adapters over a shared, transport-independent application core.

## Architecture

The codebase follows the layered clean/onion model (adapted from the Actix clean-architecture
reference): dependencies point strictly inward, and the innermost layer knows nothing about
HTTP, gRPC, or the environment.

```
api/http ──▶ container ──▶ domain ◀──┐
api/grpc ──▶ container ──▶ domain    │   (transports never depend on each other)
cli ──▶ api/*, container, infrastructure
infrastructure (settings, logging) — no application knowledge
```

- **domain** (`my_service_domain`) — the innermost layer: the use cases with their request and
  response models (`domain::use_cases`), and the shared, transport-independent error vocabulary
  (`domain::errors`). It only depends on serde/thiserror; it never mentions HTTP or gRPC types.
  Business validation lives here (a name must be at least 3 **bytes** long).
- **container** (`my_service_container`) — the composition root. `Container` constructs the
  application components once per process and shares them with every transport adapter.
  New dependencies (repositories, clients) are wired here.
- **infrastructure** (`my_service_infrastructure`) — process-level technical concerns:
  configuration (`settings::AppConfig`, read once from the environment) and logging
  (`logging_config::init_logging`, `env_logger` with `RUST_LOG`/`LOG_LEVEL`). This layer knows
  nothing about the application.
- **api/http** (`my_service_api_http`) — the HTTP adapter: `create_app(container)` builds the
  Axum router; handlers are thin — they parse the JSON body into the use-case request model,
  execute the use case from the shared `Container`, and map its response/error onto HTTP
  (validation failures → `400 Bad Request`).
- **api/grpc** (`my_service_api_grpc`) — the gRPC adapter: `GreeterService` converts the
  protobuf messages into the use-case models, executes the same use case, and maps errors onto
  gRPC statuses (`INVALID_ARGUMENT`). The generated Prost code stays inside this crate.
- **cli** (`my_service`) — the process entrypoint and lifecycle boundary: reads the settings,
  initializes logging, builds the composition root and runs both servers concurrently with a
  graceful `ctrl_c` shutdown.

### Extending the application

1. Add a use case in `domain/src/use_cases/` (request/response models + operation object with
   `execute`).
2. Register it in `container/src/lib.rs` if it has injectable dependencies.
3. Add a route in `api/http/src/` and/or a service method in `api/grpc/src/` — keep the
   adapters thin: parse → execute → map.

## Configuration

Every setting comes from an environment variable (optionally via a `.env` file); defaults in
parentheses:

| Variable   | Purpose                        | Default         |
|------------|--------------------------------|-----------------|
| `HTTP_HOST` | HTTP server host              | `0.0.0.0`       |
| `HTTP_PORT` | HTTP server port              | `3000`          |
| `GRPC_HOST` | gRPC server host              | `0.0.0.0`       |
| `GRPC_PORT` | gRPC server port              | `50051`         |
| `LOG_LEVEL` | log filter when `RUST_LOG` unset | `INFO`       |

## Running

```sh
cargo run
# HTTP listening on 0.0.0.0:3000, gRPC on 0.0.0.0:50051
```

Try it:

```sh
curl -X POST localhost:3000/greeter/hello \
  -H 'content-type: application/json' \
  -d '{"name":"Alice"}'
# {"message":"Hello, Alice!"}
```

The gRPC service is `greeter.Greeter` with the `Hello(HelloRequest) returns (HelloReply)` RPC
(protobuf schema in `api/grpc/src/proto/greeter.proto`).

## Testing

```sh
cargo test --workspace
```

- `domain` — use-case unit tests (including byte-vs-character validation semantics).
- `api/http` — endpoint tests through the router in memory (`tower::ServiceExt::oneshot`).
- `api/grpc` — service tests plus wire tests that run a real client against the server over TCP.

## Checks

```sh
cargo fmt --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
