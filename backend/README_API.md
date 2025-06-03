# ATS 2.0 Backend API & Migration Structure

## Migration Order (sqlx/Postgres)
1. `20250603160000_enable_uuid.sql` – Enables pgcrypto for UUID support
2. `20250603161000_create_users_table.sql` – Users table
3. `20250603162000_create_jobs_table.sql` – Jobs table
4. `20250603162500_create_application_stage_enum.sql` – Enum for application pipeline stages
5. `20250603163000_create_applications_table.sql` – Applications table (references users, jobs, and enum)

## API Endpoints

### Health
- `GET /health` – Health check

### Users
- `GET /api/users/` – List users
- `POST /api/users/` – Create user
- `GET /api/users/:id` – Get user by ID
- `PUT /api/users/:id` – Update user
- `DELETE /api/users/:id` – Delete user

### Jobs
- `GET /api/jobs/` – List jobs
- `POST /api/jobs/` – Create job
- `GET /api/jobs/:id` – Get job by ID
- `PUT /api/jobs/:id` – Update job
- `DELETE /api/jobs/:id` – Delete job

### Applications
- `GET /api/applications/` – List applications
- `POST /api/applications/` – Create application
- `GET /api/applications/:id` – Get application by ID
- `PUT /api/applications/:id` – Update application
- `DELETE /api/applications/:id` – Delete application
- `PUT /api/applications/:id/stage` – Update application stage

## Tech Stack
- Rust, Axum, sqlx, Postgres
- Organized by feature: `src/models/`, `src/routes/`, `src/db.rs`
- All endpoints return JSON and use connection pool

---

## Minimal CI (GitHub Actions)
Add this as `.github/workflows/ci.yml`:
```yaml
name: CI
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - run: cd backend && cargo build --all --release
      - run: cd backend && cargo test --all
```
