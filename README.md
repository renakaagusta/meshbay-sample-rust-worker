# meshbay-sample-rust-worker

Rust worker for the Meshbay vs Railway side-by-side test, meant to be run as a cron job: it connects to MongoDB,
inserts one document into `items`, prints a line and exits.

Both platforms run cron as "start the process on schedule", so this binary does its one job and returns. There is no
HTTP server, no Axum and no long-lived process.

## Environment

| Variable | Required | Default | Description |
|---|---|---|---|
| `MONGO_URL` | yes | none | MongoDB connection string, e.g. `mongodb://user:password@host:27017/meshbay` |

The database name comes from the path of `MONGO_URL`; when the URL has no path the worker uses `meshbay`.

## What it inserts

```json
{"source": "cron", "at": "<BSON datetime of this run>"}
```

into the `items` collection (created implicitly by the first insert).

## Run locally

```sh
MONGO_URL=mongodb://localhost:27017/meshbay cargo run
```

Build a release binary:

```sh
cargo build --release
./target/release/meshbay-sample-rust-worker
```

Rust is pinned by `rust-toolchain.toml` (`stable`, minimal profile). Continuous integration on GitHub Actions runs
`cargo build --release`, so a red build means the crate does not compile.
