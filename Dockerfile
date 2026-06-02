# syntax=docker/dockerfile:1
# ============================================================
# Stage 1 — Build the Vue/Vite frontend
# ============================================================
FROM oven/bun:1.3-alpine AS frontend-builder

WORKDIR /app

COPY web/package.json web/bun.lock ./
RUN bun install --frozen-lockfile

COPY web/ .
RUN bun run build

# ============================================================
# Stage 2 — Build the Rust API binary (fully static MUSL)
# ============================================================
FROM rust:1.85-alpine3.21 AS rust-builder

WORKDIR /app

RUN apk add --no-cache musl-dev binutils && \
    rustup target add x86_64-unknown-linux-musl

# Cache dependencies: copy manifests + dummy sources
COPY Cargo.toml Cargo.lock ./

COPY backend/api/Cargo.toml backend/api/
COPY backend/db/Cargo.toml backend/db/
COPY backend/domain/Cargo.toml backend/domain/
COPY backend/ingestion/Cargo.toml backend/ingestion/

RUN mkdir -p backend/api/src backend/db/src backend/domain/src backend/ingestion/src && \
    echo "fn main() {}" > backend/api/src/main.rs && \
    touch backend/db/src/lib.rs && \
    touch backend/domain/src/lib.rs && \
    touch backend/ingestion/src/lib.rs

RUN cargo build --release --target x86_64-unknown-linux-musl --bin api

# Real sources
COPY backend/ ./backend/

RUN cargo build --release --target x86_64-unknown-linux-musl --bin api && \
    strip target/x86_64-unknown-linux-musl/release/api

# ============================================================
# Stage 3 — Runtime directories and system files
# ============================================================
FROM alpine:3.21 AS prep

RUN mkdir -p /incoming /library /tmp && \
    adduser -D -u 1000 -h /tmp bouquinerie

# ============================================================
# Stage 4 — Final scratch image
# ============================================================
FROM scratch AS final

ARG BUILD_DATE
ARG VERSION=0.1.0
ARG COMMIT_SHA

LABEL org.opencontainers.image.title="Bouquinerie"
LABEL org.opencontainers.image.description="Ebook management platform"
LABEL org.opencontainers.image.version=${VERSION}
LABEL org.opencontainers.image.created=${BUILD_DATE}
LABEL org.opencontainers.image.revision=${COMMIT_SHA}

# Runtime directories
COPY --from=prep /incoming /incoming
COPY --from=prep /library /library
COPY --from=prep /tmp /tmp

# System files (CA certs, user/group)
COPY --from=prep /etc/ssl/cert.pem /etc/ssl/cert.pem
COPY --from=prep /etc/passwd /etc/passwd
COPY --from=prep /etc/group /etc/group

# Frontend static assets
COPY --from=frontend-builder /app/dist /web/dist

# Rust API binary
COPY --from=rust-builder /app/target/x86_64-unknown-linux-musl/release/api /api

# Busybox (fully static musl build) — shell, ls, ps, wget, etc.
COPY --from=busybox:musl /bin/busybox /bin/busybox
RUN ["/bin/busybox", "--install", "-s", "/bin"]

# Default environment
ENV FRONTEND_DIST=/web/dist
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000
ENV LIBRARY_PATH=/library
ENV WATCHED_DIRS=/incoming
ENV INGEST_DEBOUNCE_SECS=5
ENV SUPPORTED_FORMATS=epub,pdf,mobi,azw3,m4b,mp3
ENV LOG_LEVEL=api=info,ingestion=info,tower_http=info

EXPOSE 3000

USER bouquinerie

ENTRYPOINT ["/api"]
