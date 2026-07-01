# Agents Guide

This repository is a Rust workspace for an Axum-based backend.

## What to know first

- Prefer reading `README.md` and `API design.md` before changing behavior.
- Treat the workspace as a multi-crate project; changes may need to be made in more than one crate.
- Keep edits small and consistent with the existing code style.

## Common commands

- `task help`
- `task`
- `task local`
- `task run`
- `task check`
- `task test`
- `task fmt`
- `task pre`
- `task pipeline`

## Database and schema work

- Use the `task mig:*` commands for migration changes.
- Regenerate SeaORM entities with `task entity` when schema changes require it.
- Be careful with destructive migration commands because they may delete data.

## API conventions

- Follow the CRUD patterns described in `API design.md`.
- Keep endpoint names, status codes, and validation behavior aligned with the existing conventions.
- Prefer explicit request and response types over ad hoc JSON handling.

## Working rules

- Do not overwrite user changes unless explicitly asked.
- Avoid destructive git or filesystem commands unless they are clearly required.
- Run the relevant checks after code changes when possible.
- If something is ambiguous, inspect the codebase first instead of guessing.
