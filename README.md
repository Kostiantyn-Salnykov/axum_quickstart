Create .env file inside root of the project:
```dotenv
POSTGRES_DB=postgres
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_HOST=127.0.0.1
POSTGRES_PORT=5432

SERVER_PORT=9999

PGADMIN_LISTEN_PORT=8080

REDIS_PORT=8000
REDIS_INSIGHT_PORT=8001
REDIS_HOST=127.0.0.1

LOG_LEVEL=trace,tower_http=info,sea_orm=info,axum=info,api_http=info
```

Set environment variables to current powershell session:
```powershell
Get-Content .env | ForEach-Object {
if ($_ -match '^\s*([^#][^=]+)=(.*)$') {
[System.Environment]::SetEnvironmentVariable($matches[1].Trim(), $matches[2].Trim())
}
}
```

Regenerate sea_orm entities:
```powershell
sea-orm-cli generate entity `
  -u "postgres://${env:POSTGRES_USER}:${env:POSTGRES_PASSWORD}@${env:POSTGRES_HOST}:${env:POSTGRES_PORT}/${env:POSTGRES_DB}" `
-o infrastructure/src/adapters/persistence/seaorm/entities `
--ignore-tables migrations
```

Update database schema:
```powershell
sea-orm-cli migrate up -d infrastructure/migrations
```

Rollback database schema:
```powershell
sea-orm-cli migrate down -d infrastructure/migrations
```

```powershell
sea-orm-cli migrate generate <NAME> -d infrastructure/migrations
```

Taskfile commands:
```powershell
task
task run
task check
task test
task fmt
task fmt-check
task clippy
task pre
task pipeline
task local
task mig:up
task mig:down -- 1
task mig:fresh
task mig:reset
task mig:status
task mig:generate NAME=create_posts
task mig:entity
task mig -- up
```
