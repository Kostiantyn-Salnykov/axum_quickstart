# Authorization / Casbin

This project uses a layered authorization setup built around Casbin and Redis.

## High-level flow

1. HTTP handlers or other application entry points call `AuthorizationService`.
2. `AuthorizationService` delegates the permission check to `AuthorizationEnforcerPort`.
3. The concrete implementation is `CasbinAuthorizationEnforcerAdapter`.
4. Casbin loads its model from `infrastructure/casbin/model.conf`.
5. Casbin loads and stores policies through `RedisPolicyStoreAdapter`.
6. Redis is the source of truth for authorization policies.

## Modules

### `application/src/authorization`

This is the application-level authorization package.

- `service.rs`
  - Contains `AuthorizationService`.
  - This is the main use case for permission checks.
  - It does not know anything about Redis or Casbin internals.

- `enforcer_port.rs`
  - Defines `AuthorizationEnforcerPort`.
  - This is the abstraction for the authorization engine.
  - The current implementation is backed by Casbin.

- `action.rs`
  - Defines the supported actions such as `read`, `create`, `update`, `delete`, and `manage`.

- `resource.rs`
  - Defines `AuthorizationSubject` and `AuthorizationResource`.
  - These are the generic request types used for authorization checks.

- `policy.rs`
  - Defines `AuthorizationPolicy`, `AuthorizationEffect`, and `AuthorizationPolicyId`.
  - This is the shared policy model used across application and infrastructure.

- `result.rs`
  - Defines the authorization result wrapper returned by the use case.

- `policy_lifecycle/`
  - Contains the use case for granting and revoking access.
  - This layer creates policies for a resource and sends them to the enforcer.

- `policy_management/`
  - Contains CRUD-style policy management over the policy repository.
  - This module is currently not part of the active authorization flow, but it remains available for admin-style or debugging use cases.

### `infrastructure/src/adapters/authorization`

- `casbin_enforcer.rs`
  - Contains `CasbinAuthorizationEnforcerAdapter`.
  - This is the concrete adapter for `AuthorizationEnforcerPort`.
  - It creates and owns the Casbin enforcer.
  - It loads the model from `model.conf`.
  - It connects Casbin to the Redis policy adapter.

### `infrastructure/src/adapters/redis`

- `policy_store.rs`
  - Contains `RedisPolicyStoreAdapter`.
  - This adapter stores authorization policies in Redis.
  - It also implements Casbin's `Adapter` trait.
  - That means Casbin can load, save, add, and remove policies directly through Redis.

### `infrastructure/casbin/model.conf`

- Defines the Casbin model.
- It describes how Casbin interprets the request, policy, effect, and matcher sections.
- Policies stored in Redis are evaluated against this model.

### `bootstrap/src/wiring/authorization.rs`

- Wires the authorization graph together.
- Creates the Casbin enforcer adapter.
- Creates `AuthorizationService`.
- Creates `PolicyLifecycleService`.

### `api_http/src/state.rs`

- Exposes authorization services to HTTP handlers through `AppState`.
- The active authorization state contains:
  - `authorize`
  - `lifecycle`

## Policy storage behavior

- Policies are not loaded from `policy.csv`.
- Policies are stored in Redis.
- The Casbin enforcer loads policies from Redis on startup through the Redis adapter.
- When a policy is added or removed through the enforcer, Redis is updated as well.

## Practical usage

- Use `AuthorizationService` when you need to check whether a user can perform an action on a resource.
- Use `PolicyLifecycleUseCase` when you need to grant or revoke access.
- Use `CasbinAuthorizationEnforcerAdapter` only in infrastructure wiring.
- Use `RedisPolicyStoreAdapter` as the Redis-backed policy storage implementation.

