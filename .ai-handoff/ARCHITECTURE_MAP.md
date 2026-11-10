# ARCHITECTURE MAP — investigation findings & component correspondence

## 1. Reference architecture (Actix, commit 8b82629 — the ADAPTED implementation)

Single crate `actix-clean-architecture` (source under `{{cookiecutter.repo_name}}/src/`):

```
src/
├── main.rs            # entrypoint: get_config() → init_logging(config) → init_database(config)
│                      #   → Arc<Container> → HttpServer with create_app(container)
├── lib.rs             # pub mod api; container; data; domain; infrastructure;
├── container.rs       # Composition root: Container { Arc<TodoRepository>, Arc<ServiceContextRepository> }
├── api/               # HTTP layer: thin handlers; each builds a use case per request from
│   ├── mod.rs         #   web::Data<Repository> and calls execute(); routes(cfg)
│   └── todos.rs       #   POST/GET /todos, GET/DELETE /todos/{id}
├── domain/            # Innermost layer
│   ├── interfaces/    #   empty placeholder for domain-owned ports
│   └── use_cases/     #   base.rs: BaseUseCase<Request, Response> trait { async fn execute(...) -> Result<Response, OtherError> }
│       └── todos/     #   one file per use case: XxxUseCase{new(repo)} + XxxRequest/XxxResponse + From<DTO>
├── data/              # repositories + DTOs (persistence boundary)
│   ├── base.rs        #   BaseRepository trait, RepositoryResult/RepositoryError, ResultPaging, QueryParams
│   ├── todos/         #   TodoRepository, TodoDTO, TodoFilters
│   └── service_context/
├── infrastructure/
│   ├── core/
│   │   ├── settings.rs          # AppConfig from env (dotenv), get_config() -> &'static (OnceLock)
│   │   ├── logging_config.rs    # init_logging(config): env_logger, LOG_LEVEL default
│   │   ├── application.rs       # create_app(container) → Actix App: app_data + middlewares + routes
│   │   ├── errors/              # OtherError enum (Internal/ExternalService/BadRequest/NotFound/AccessDenied)
│   │   │                        #   + ErrorType machine codes; handlers return OtherError, middleware maps to HTTP
│   │   └── middlewares/         # error_handling (extractor config), log_requests, maintenance, trace_id
│   └── clients/postgres/        # engine.rs OnceLock pool, diesel schema/models
└── tests/test_todos.rs          # integration tests via testcontainers postgres
```

Key design decisions of the ADAPTED architecture (vs the old main branch):
- Use-case oriented domain: one struct per business operation with Request/Response models.
- Explicit composition root `Container` built once per process; parts published as app state;
  endpoints construct use cases from the shared repositories per request.
- Shared error vocabulary `OtherError`; transports/framework map it to responses in one place.
- Settings + logging initialized in `main`, read once per process.
- Thin HTTP layer (handlers only parse requests, execute use cases, map errors).

## 2. Original target architecture (rust-grpc-app-template @ a54e38c)

Cargo workspace: `members = ["cli", "domain", "api/*"]`, default-member `cli`.

- `domain` (my_service_domain): `greeter.rs` — `GreeterError::InvalidNameLength`,
  `HelloInput`/`HelloOutput`, free fn `hello(input)` with `name.len() < 3` byte check.
  No use-case objects, no composition, no config, no logging, no tests.
- `api/http` (my_service_api_http): `serve()` hardcodes `0.0.0.0:3000`; router nests
  `greeter::routes()` at `/greeter`; handler maps domain error → 400 via local `RouteError`
  (anyhow-based); duplicated HTTP DTOs `HelloRequest`/`HelloResponse`.
- `api/grpc` (my_service_api_grpc): `serve()` hardcodes `0.0.0.0:50051`; tonic `Greeter` unit
  struct calls domain fn directly; maps error → `Status::InvalidArgument`; `build.rs` compiles
  `src/proto/greeter.proto` (package `greeter`, service `Greeter`, rpc `Hello`, fields name=1,
  message=1).
- `cli` (my_service): spawns both `serve()` futures in a `tokio::task::JoinSet`; panics on error.
- No configuration module (comment in code says it should be in config), no logging, no tests,
  README is a two-line stub.

## 3. Architectural correspondence (Actix → gRPC target)

| Actix reference | Target representation | Disposition |
|---|---|---|
| `domain/use_cases/*` (BaseUseCase trait + per-op structs) | `domain` crate: `use_cases::hello` — `HelloUseCase` with inherent `execute(HelloRequest) -> Result<HelloResponse, GreeterError>` | Adapt: no trait/async_trait for a single sync use case (task explicitly allows) |
| Use-case Request/Response models (serde derives) | `HelloRequest`/`HelloResponse` in domain crate, serde derives; HTTP uses them directly as JSON bodies; gRPC converts proto↔models at boundary | Adopt (removes duplicated HTTP DTOs) |
| `infrastructure/core/errors` (`OtherError`) | `domain::errors::GreeterError` — the shared transport-independent error vocabulary | Adapt: must live in the innermost crate so both transports + use cases share it without dependency inversion |
| `container.rs` composition root (holds repositories) | `container` crate: `Container { hello_use_case: Arc<HelloUseCase> }`, built once, shared with both servers | Adapt: no repositories exist (no persistence); container holds the shared application component. When repositories appear, container holds them |
| `infrastructure/core/settings.rs` | `infrastructure` crate: `settings::AppConfig` (HTTP_HOST/PORT, GRPC_HOST/PORT, LOG_LEVEL) + `get_config()` OnceLock | Adopt |
| `infrastructure/core/logging_config.rs` | `infrastructure` crate: `logging_config::init_logging` (env_logger, LOG_LEVEL default) | Adopt |
| `infrastructure/core/application.rs` (`create_app`) | `api/http::create_app(container) -> Router` (application factory analog); each adapter owns its binding via `serve(addr, container, shutdown)` | Adapt (two transports; factory per adapter) |
| `api/` thin handlers building use cases per request | Thin handlers holding `Arc<Container>` state, calling `hello_use_case.execute(...)` | Adopt (shared use case instead of per-request build — stateless use case) |
| `data/` repositories/DTOs | — none — (no persistence in target; do NOT invent) | Omit |
| `infrastructure/clients/postgres` | — none — | Omit |
| middlewares (error_handling, log_requests, maintenance, trace_id) | HTTP: `IntoResponse` mapping on a small `RouteError` wrapper at the handler boundary; no maintenance/trace-id (they depend on postgres service-context & actix specifics) | Partially adopt; documented deviation |
| `main.rs` lifecycle | `cli`: config → logging → container → concurrent servers with graceful shutdown (ctrl_c) | Adopt + extend (graceful shutdown) |
| `tests/` | domain unit tests; api/http integration test (tower oneshot); api/grpc service-level test | Extend (baseline had none) |

## 4. Target dependency graph (planned, compile-time enforced)

```
cli ──▶ api/http ──▶ container ──▶ domain
  │   └─▶ api/grpc ──▶ container ──▶ domain
  └─▶ container, infrastructure ──▶ (no app deps)
api/http ─▶ domain (models/errors); api/grpc ─▶ domain
```
Transports never depend on each other; domain depends on nothing app-specific (only serde/thiserror).

## 5. Data flow (after refactoring)

HTTP: `Json<HelloRequest>` (domain model) → `container.hello_use_case.execute(...)`
→ `HelloResponse` → `Json<HelloResponse>`; `GreeterError` → `RouteError` → 400 + message.
gRPC: proto `HelloRequest` → domain `HelloRequest` → `execute` → domain `HelloResponse`
→ proto `HelloReply`; `GreeterError` → `Status::new(InvalidArgument, msg)`.
Both transports share the SAME `HelloUseCase` instance from the one `Container`.
