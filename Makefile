.PHONY: help \
        fmt fmt-fix clippy test build run \
        sqlx-prepare sqlx-check \
        up down restart logs ps \
        migrate \
        kill-4000 \
        ci docker-build

# -------------------------
# Config (override if needed)
# -------------------------
DATABASE_URL ?= postgres://acid:acid@localhost:5432/acidcomics
JWT_SECRET   ?= acidcomics_super_secret_key_2026_do_not_share_98as7d9as8d7a9s8d

help:
	@echo "Targets:"
	@echo "  up             Start Postgres (docker-compose)"
	@echo "  migrate        Apply migrations to DB"
	@echo "  run            Run API locally (SQLX_OFFLINE=true)"
	@echo "  restart        Restart stack (down + up)"
	@echo "  logs           Tail logs"
	@echo "  ps             Show compose status"
	@echo "  kill-4000      Kill process using port 4000"
	@echo "  fmt            Check formatting"
	@echo "  fmt-fix        Auto-format"
	@echo "  clippy         Clippy with -D warnings"
	@echo "  test           Run tests (SQLX_OFFLINE=true)"
	@echo "  sqlx-prepare   Regenerate .sqlx cache (needs DB running)"
	@echo "  sqlx-check     Verify .sqlx cache is up to date"
	@echo "  docker-build   Build API Docker image"
	@echo "  ci             Local CI: fmt + clippy + test + docker-build"

# -------------------------
# Docker / DB
# -------------------------
up:
	docker-compose up -d

down:
	docker-compose down

restart: down up

logs:
	docker-compose logs -f --tail=200

ps:
	docker-compose ps

# Apply SQL migrations to the running DB
migrate:
	@echo "Applying migrations to $(DATABASE_URL)"
	@DATABASE_URL=$(DATABASE_URL) cargo sqlx migrate run

# Kill whatever is bound to port 4000 (common local trap)
kill-4000:
	@echo "Killing process on :4000 if any..."
	@bash -lc 'PID=$$(ss -ltnp | awk "/:4000/ {print \$$$$(NF)}" | sed -E "s/.*pid=([0-9]+).*/\\1/" | head -n1); \
	if [ -n "$$PID" ]; then echo "Killing PID $$PID"; kill -9 $$PID; else echo "Nothing on :4000"; fi'

# -------------------------
# Rust checks
# -------------------------
fmt:
	cargo fmt --all -- --check

fmt-fix:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	SQLX_OFFLINE=true cargo test --all

build:
	SQLX_OFFLINE=true cargo build

# Run locally (expects DB already up; uses env vars)
run:
	@echo "Running API locally on :4000 (SQLX_OFFLINE=true)"
	@DATABASE_URL=$(DATABASE_URL) JWT_SECRET=$(JWT_SECRET) SQLX_OFFLINE=true cargo run

# -------------------------
# SQLx offline cache
# -------------------------
# Generates/updates .sqlx based on real DB schema (DB MUST be reachable)
sqlx-prepare:
	@echo "Regenerating .sqlx cache using $(DATABASE_URL)"
	@SQLX_OFFLINE=false DATABASE_URL=$(DATABASE_URL) cargo sqlx prepare

# Checks if .sqlx matches current queries
sqlx-check:
	@SQLX_OFFLINE=true cargo sqlx prepare --check

# -------------------------
# Docker image
# -------------------------
docker-build:
	docker build -t acidcomics-api .

# "Local CI" command
ci: fmt clippy test docker-build