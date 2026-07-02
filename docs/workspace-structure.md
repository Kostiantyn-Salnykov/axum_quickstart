# Workspace and Module Structure

This repository is organized as a Rust workspace with a layered, hexagonal style.

## Workspace crates

The workspace is defined in the root `Cargo.toml` and contains these members:

- `app_config`
- `application`
- `api_http`
- `bootstrap`
- `domain`
- `infrastructure`
- `infrastructure/migrations`

The workspace uses `bootstrap` as the default member, so running the project usually starts from the application entrypoint crate.

## Dependency direction

The intended dependency flow is:

`domain` -> `application` -> `infrastructure` -> `bootstrap`

`api_http` sits on top of `application` and exposes the HTTP interface.

In practice:

- `domain` contains core value objects and domain errors.
- `application` contains use cases, ports, and application-level models.
- `infrastructure` contains concrete adapters for persistence, Redis, security, health checks, and Casbin.
- `api_http` contains routers, handlers, request/response types, docs, and HTTP state wiring.
- `bootstrap` wires configuration, database connections, Redis, services, and the HTTP server together.
- `app_config` owns application settings loading and derived URLs.
- `infrastructure/migrations` contains SeaORM migrations and is kept as a separate workspace crate.

## Crate overview

### `domain`

Core business types that should stay free from framework and infrastructure concerns.

Typical contents:

- value objects
- domain-specific enums
- domain errors

### `application`

Application use cases and ports.

This crate contains:

- auth use cases
- authorization use cases
- rate limiting ports and policies
- user retrieval and search use cases
- system health-check use cases
- shared result and query types

Important rule:

- application code depends on abstractions, not on concrete infrastructure adapters.

### `infrastructure`

Concrete runtime adapters.

This crate contains:

- SeaORM persistence adapters;
- Redis adapters;
- JWT token manager;
- Password hashing (Argon2);
- Health check adapters;
- Casbin authorization adapter;

For authorization specifically:

- `infrastructure/src/adapters/authorization/casbin_enforcer.rs`
  - Casbin enforcer adapter
- `infrastructure/src/adapters/redis/policy_store.rs`
  - Redis-backed policy storage adapter
- `infrastructure/casbin/model.conf`
  - Casbin model definition

### `api_http`

The HTTP interface crate.

This crate contains:

- Axum router composition;
- Request handlers;
- Request and response DTOs;
- API docs;
- HTTP-level state;
- Middlewares;

Important submodules:

- `auth`
- `authorization` is not exposed as a public HTTP module yet, but its state is available through `AppState`
- `health_check`
- `users`
- `docs`

### `bootstrap`

Application startup crate.

This crate is responsible for:

- Loading configuration;
- Initializing tracing;
- Connecting to PostgreSQL;
- Creating Redis clients;
- Wiring application services;
- Starting the Axum server;

The entrypoint is:

- `bootstrap/src/main.rs`

The wiring layer is:

- `bootstrap/src/wiring/`

### `app_config`

Configuration crate.

It owns:

- `Settings`
- environment loading
- derived URLs like database and Redis URLs
- config-related tests

### `infrastructure/migrations`

SeaORM migrations live here as a separate crate inside the workspace.

This keeps schema evolution isolated from the main runtime crates.

## Module layout by crate

### `application/src`

Main modules:

- `auth` - application use cases for login, logout, registration, refresh, and token verification.
- `authorization` - authorization use cases, ports, policy models, and policy lifecycle logic.
- `errors` - shared application error type used across use cases and ports.
- `rate_limit` - rate limiting policies and the port used by the application layer.
- `search` - generic search use cases and query/result abstractions.
- `system` - system-level use cases such as health checks.
- `users` - user-related use cases, repositories, search models, and result types.

`authorization` is further split into:

- `action` - supported authorization actions like read, create, update, delete, and manage.
- `access_role` - higher-level roles that map to one or more allowed actions.
- `enforcer_port` - abstraction over the authorization engine implementation.
- `policy` - authorization policy data structures and effect definitions.
- `policy_lifecycle` - use cases for granting and revoking access to resources.
- `policy_management` - CRUD-style policy management over the storage layer.
- `repository_port` - abstraction for storing and listing policy records.
- `resource` - generic subject and resource request models for authorization checks.
- `result` - authorization result wrapper returned by the use case.
- `service` - the main authorization checking service.
- `use_case` - trait definition for the authorization use case.

### `api_http/src`

Main modules:

- `auth` - HTTP routes and handlers for authentication flows.
- `docs` - OpenAPI and schema documentation modules.
- `health_check` - HTTP health-check endpoints.
- `middlewares` - shared HTTP middleware functions.
- `state` - shared application state injected into request handlers.
- `users` - HTTP routes, handlers, and DTOs for user endpoints.

  Each feature module usually contains:
  - `mod.rs` - the module entrypoint that ties the feature files together.
  - `router.rs` - the Axum router setup for the feature endpoints.
  - `handler.rs` - the request handlers that execute the feature logic.
  - `request.rs` - the request DTOs and validation shapes for incoming payloads.
  - `response.rs` - the response DTOs returned by the HTTP handlers.

### `infrastructure/src`

Main modules:

- `adapters` - concrete implementations for persistence, Redis, security, health checks, and authorization.

  Adapter groups:
  - `adapters/auth` or `adapters/security` - token management and password hashing adapters.
  - `adapters/authorization` - Casbin-based authorization adapters.
  - `adapters/health` - health-check adapters for Redis and Postgres.
  - `adapters/persistence` - SeaORM adapters and persistence helpers.
  - `adapters/redis` - Redis clients and Redis-backed adapters.

### `bootstrap/src`

Main modules:

- `main.rs` - application entrypoint that loads config, builds the state, and starts the HTTP server.
- `wiring/` - composition root that creates service graphs for each subsystem.

The `wiring` module is split by concern:

- `auth` - wires authentication services and security adapters.
- `authorization` - wires Casbin, Redis policy storage, and authorization use cases.
- `system` - wires system-level services like health checks.
- `users` - wires user retrieval and search use cases.
- `app` - wires the full application state together.

## Practical reading order

If you want to understand the project quickly, read in this order:

1. `README.md`
2. `API design.md`
3. `docs/workspace-structure.md`
4. `docs/authorization.md`

That gives you:

- the API conventions
- the workspace layout
- the authorization design
- the Casbin / Redis flow
