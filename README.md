# askly

simple CRUD api for QnA service written in rust

## Tech used

- warp (web framework)
- sqlx (db interface)
- shuttle (cloud platform for easy deployment)

## Run locally (without shuttle)

Rust and postgresql is required to run locally

```bash
# setup env vars
cp .env.example .env

cargo run --bin http
```

## Run locally (with shuttle)

cargo-shuttle is required (refer shuttle.rs docs)

```bash
# setup env vars
cp .env.example Secrets.toml

cargo shuttle run
```
