# START HERE — Architectural Refactoring of rust-grpc-app-template

## Overall objective

Comprehensively refactor the Rust HTTP+gRPC application at
`/home/alex_goat/CLionProjects/rust-grpc-app-template` so that its architecture matches the
adapted Actix clean/onion architecture from the reference repository
(`/home/alex_goat/CLionProjects/actix-clean-architecture-app-template`, branch
`refactor/clean-architecture-rewrite`, commit `8b82629` "Refactor to the FastAPI-style clean
architecture"), adapted for a dual-transport (Axum HTTP + Tonic gRPC) Rust workspace.

All details are in the sibling handoff documents. Read them in this order:

1. `TASK_SPEC.md` — full task requirements (scope, restrictions, definition of done).
2. `CURRENT_STATE.md` — **the execution cursor**: what is done, what is active, exact next action.
3. `IMPLEMENTATION_PLAN.md` — ordered stages with statuses.
4. `ARCHITECTURE_MAP.md` — investigation findings: actual architecture of both apps + correspondence.
5. `DECISIONS.md` — architectural decision log (final vs tentative).
6. `PROGRESS.md` — chronological journal of completed operations.
7. `VERIFICATION.md` — actual command results (baseline + post-refactoring).

## Repository facts (verified)

- Target repo: `/home/alex_goat/CLionProjects/rust-grpc-app-template`
  - Original revision: `a54e38c163d154ee34bdd64646b86d9b15f8e175` (branch `dev`, clean tree except
    untracked `.idea/`).
  - Do NOT create commits or push without explicit user authorization.
- Reference repo (READ ONLY — never modify, never switch branches):
  - Main checkout: `/home/alex_goat/CLionProjects/actix-clean-architecture-app-template` on `main`.
  - The adapted architecture is checked out in a git worktree:
    `/home/alex_goat/.local/share/opencode/worktree/41dece/witty-tiger` at `8b82629`
    (branch `refactor/clean-architecture-rewrite`). Local branch has NO extra commits beyond the PR.

## Baseline verification (already executed, before any change)

- `cargo check --workspace --all-targets` — OK.
- `cargo test --workspace` — OK (0 tests exist).
- `cargo clippy --workspace --all-targets --all-features` — 3 pre-existing warnings
  (`double_must_use` ×2 in `my_service_api_grpc` from the tonic async_trait macro;
  `result_large_err` in the gRPC handler; workspace resolver defaulting note). These are
  PRE-EXISTING, not introduced by the refactoring.
- Runtime smoke (app built from `a54e38c`, since stopped):
  - `POST /greeter/hello {"name":"Alice"}` → 200 `{"message":"Hello, Alice!"}`
  - `POST /greeter/hello {"name":"ab"}` → 400, body `Invalid name length`
  - `POST /greeter/hello {"name":"аб"}` (2 chars = 4 bytes) → 200 (validation is BYTE length ≥ 3)

## Current implementation stage

ALL STAGES COMPLETE (1–10) — implemented, verified, and documented. The working tree holds the
complete refactoring as uncommitted changes (no commits without authorization). Read
`CURRENT_STATE.md` for the final architecture summary and `VERIFICATION.md` for executed
results.

## Mandatory continuation protocol (for a new AI session)

1. Read this file, then `TASK_SPEC.md`, then `CURRENT_STATE.md`.
2. Read the `IN_PROGRESS` / next stage in `IMPLEMENTATION_PLAN.md`.
3. Check `git -C /home/alex_goat/CLionProjects/rust-grpc-app-template status` and reconcile with
   `CURRENT_STATE.md` section I. If the tree matches the checkpoint, continue with the recorded
   NEXT ACTION immediately — do NOT restart the investigation, do NOT recreate the plan.
4. Execute the exact NEXT ACTION from `CURRENT_STATE.md` section H.
5. After each atomic operation: verify, update `CURRENT_STATE.md`, `PROGRESS.md`, `VERIFICATION.md`.

## Definition of done (summary)

Logical + physical architecture aligned with the reference (use cases, composition root,
settings, logging, error vocabulary, thin transport adapters), dual-transport independence
preserved, all existing behavior preserved (routes, wire format, 400/INVALID_ARGUMENT mapping,
byte-length validation), `cargo fmt --check`, `cargo check --workspace --all-targets`,
`cargo test --workspace`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`
all pass (except pre-existing grpc macro warnings, to be eliminated or explicitly recorded),
handoff documents kept current.
