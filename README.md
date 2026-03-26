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
