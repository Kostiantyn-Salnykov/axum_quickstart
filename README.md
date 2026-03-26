Simple CRUD operations:

| Method | Endpoint          | Description                                                    | Status OK | Status FAIL |
|--------|-------------------|----------------------------------------------------------------|-----------|-------------|
| POST   | /{entity}/        | Create one                                                     | 201       | 400, 422    |
| GET    | /{entity}/{id}/   | Retrieve one                                                   | 200       | 404         |
| POST   | /{entity}/search/ | List з searching/filters/pagination/sorting/projection in body | 200       | 422         |
| PUT    | /{entity}/{id}/   | Replace one                                                    | 200       | 400, 422    |
| PATCH  | /{entity}/{id}/   | Partial update one                                             | 200       | 404         |
| DELETE | /{entity}/{id}/   | Delete one                                                     | 204       | 404         |
| HEAD   | /{entity}/{id}/   | Check existance without retrieving                             | 200       | 404         |

---

Batch operations:

| Method | Endpoint         | Description                    | Status OK | Status FAIL |
|--------|------------------|--------------------------------|-----------|-------------|
| POST   | /{entity}/batch/ | Create many                    | 201       | Array[422]  |
| PUT    | /{entity}/batch/ | Upsert many (update or create) | 200       | Array[422]  |
| PATCH  | /{entity}/batch/ | Partial update many            | 200       | Array[422]  |
| DELETE | /{entity}/batch/ | Delete many                    | 204       | -           |


Possible invariants:

| Method | Endpoint                        | Description                                             | Status OK | Status FAIL |
|--------|---------------------------------|---------------------------------------------------------|-----------|-------------|
| POST   | /{entity}/{id}/actions/{action} | Domain action (such as `publish`, `archive`, `approve`) | 200, 202  | 400, 422    |
| PUT    | /{entity}/export/               | File generation                                         | 200, 202  | 400, 422    |
| PATCH  | /{entity}/import/               | Bulk upload from file                                   | 200, 202  | 400, 422    |


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
-o infrastructure/src/seaorm/entities `
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