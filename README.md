# Bouquinerie

[![CI](https://github.com/cparadis777/Bouquinerie/actions/workflows/ci.yml/badge.svg)](https://github.com/cparadis777/Bouquinerie/actions/workflows/ci.yml)
[![License: AGPLv3](https://img.shields.io/badge/license-AGPLv3-blue.svg)](LICENSE)

Bouquinerie (French for "bookstore") is a personal ebook library manager. It
catalogs ebooks (epub, pdf, mobi, azw3, m4b, mp3) and serves them through a
web UI.

**Built with:** Rust (Axum, SeaORM) on the backend and Vue 3 / TypeScript on
the frontend.

> This is an early-stage project. It is not ready for use.

---

## Quick start

```bash
# Prerequisites: Rust 1.85+, bun, docker compose (for Postgres), task

# 1. Install pre-commit hooks (prek)
cargo install prek && prek install

# 2. Start Postgres
docker compose -f .devcontainer/docker-compose.yml up -d postgres

# 3. Set up environment
cp .env.example .env

# 4. Run database migrations
cargo run --bin api

# 5. Seed sample data (optional)
task db:seed

# 6. Start the backend
task api:run

# 7. In another terminal, start the frontend
task frontend:dev
```

Open http://localhost:5173 in your browser.

---

## Architecture

```
┌──────────────┐    ┌───────────────────┐    ┌──────────┐
│  Vue 3 SPA   │───▶│  Rust API (Axum)  │───▶│ Postgres │
│  TypeScript  │    │  OpenAPI / Swagger │   │          │
│  Vite        │    │  SeaORM            │    └──────────┘
└──────────────┘    │  File ingestion    │
                    └───────────────────┘
```

- **`backend/api/`** — Axum web server, serves the API + frontend static files
- **`backend/db/`** — SeaORM entities, migrations, and connection management
- **`backend/domain/`** — Shared domain models
- **`backend/ingestion/`** — File watcher and ebook metadata extraction
- **`web/`** — Vue 3 SPA with Pinia, Vue Router, Reka UI

---

## Commands

| Command | Description |
|---|---|
| `task api:run` | Start the Rust API server on `:3000` |
| `task frontend:dev` | Start Vite dev server on `:5173` (proxies `/api` to `:3000`) |
| `task frontend:build` | Build the frontend for production |
| `task frontend:generate:api` | Regenerate TypeScript types from the API's OpenAPI spec |
| `task db:seed` | Seed the database with sample data |
| `task db:entities` | Regenerate SeaORM entities from the database |
| `task frontend:lint` | Type-check the frontend (`vue-tsc --noEmit`) |
| `cargo test` | Run backend tests |

See `Taskfile.yml` for the full list.

---

## Docker

```bash
# Build the production image (~15 MB, scratch + busybox)
docker build -t bouquinerie .

# Run with a Postgres container
docker compose -f .devcontainer/docker-compose.yml up -d postgres
docker run -d --rm -p 3000:3000 --env-file .env bouquinerie
```

---

## API docs

When the backend is running, visit http://localhost:3000/docs for the Swagger UI.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions, coding
guidelines, and the PR workflow.

---

## License

AGPLv3 — see [LICENSE](LICENSE).
