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
-o infrastructure/src/orm/entities `
--ignore-tables migrations
```

Update database schema:
```powershell
sea-orm-cli migrate up -d infrastructure/migration
```

Rollback database schema:
```powershell
sea-orm-cli migrate down -d infrastructure/migrations
```

```powershell
sea-orm-cli migrate generate <NAME> -d infrastructure/migrations
```