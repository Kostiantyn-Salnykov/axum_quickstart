# axum_quickstart

## Setup

Rename example.env to .env and fill in the values.

### Main commands:
📚List all available commands:
```shell
task help
```

🖥️Run the application in local mode:
```shell
task
```
```shell
task local
```
(ℹ️by default `task` runs a bunch of commands under the hood.)

---

▶️Run the bootstrap application:
```shell
task run
```
🔎Run cargo check and clippy for the whole workspace:
```shell
task check
```

🧪Run all workspace tests:
```shell
task test
```

🧹Format the whole workspace
```shell
task fmt
```

🛡️Run pre-commit checks
```shell
task pre
```

🚀Format, lint, test, build and run in release mode
```shell
task pipeline
```

---
### ORM commands
🔄Regenerate sea_orm entities:
```shell
task entity
```

### Migrations commands:
🔢Update {NN} database migration:
```shell
task mig:up -- {NN}
```

⬇️Rollback {NN} database migration:
```shell
task mig:down -- {NN}
```

↗️Update database migration:
```shell
task mig:up
```

⛔️Rollback database migration:
```shell
task mig:down
```
(⚠️deletes the data and schemas defined in the migration files.)
(ℹ️The `migrations` table still exists.)

⛔Drop all tables from the database, then reapply all migrations:
```shell
task mig:fresh
```
(⚠️deletes the data)


⛔Rollback all applied migrations, then reapply all migrations:
```shell
task mig:refresh
```
(⚠️deletes the data)


⛔Rollback database migration:
```shell
task mig:reset
```
(⚠️deletes the data and schemas defined in the migration files.)
(ℹ️The `migrations` table also deleted.)
