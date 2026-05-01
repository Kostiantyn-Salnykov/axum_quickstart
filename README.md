# axum_quickstart

## Setup

Rename example.env to .env and fill in the values.

### Main commands:
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

✅Check formatting without changing files
```shell
task fmt-check
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

⛔Clean database migration:
```shell
task mig:reset
```
(⚠️deletes the data and schemas defined in the migration files.)


⛔Down to 0 and up to the latest:
```shell
task mig:fresh
```
(⚠️deletes the data)
