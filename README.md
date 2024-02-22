# Bath Hack Website 2024

This is the monorepo for the 2024 Bath Hack Website, written in Rust (using Wasm on the frontend).

## Developing
To work on this repo, make sure you have a `stable` Rustup toolchain installed. You'll also need [Trunk](https://trunkrs.dev/) to work on the frontend.

For the backend, you'll need a local instance of PostgreSQL and Redis.

If you have Podman (or Docker), you can easily start these:

```
podman run --name postgres -p 54320:5432 -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=bhw docker.io/postgres
```

```
podman run -d --name redis -p 63790:6379 docker.io/redis/redis-stack-server:latest
```

> Warning: do **not** use these commands for production containers; these are _insecure_ configurations. Make sure to use _real_ passwords and persistent storage for Postgres (preferably also for Redis).

Copy `.env.example` to be `.env` and make sure all the variables have sensible values.

Then, `cd` into `packages/backend` to run the migrations:

```
diesel migration run --database-url $BHW_DATABASE_URL
```

Then, `cd` into `packages/frontend` and run:

```
trunk server --config packages/frontend/Trunk.toml
```

And in a new/parallel terminal, `cd` into the project root and run:

```
cargo run -p bhw-backend
```

## Deploying
This can be done in a couple of ways. If you're using Kubernetes, see the `k8s` directory for a `kustomize`-compatible configuration. You'll need to set some secrets manually, so make sure to look through the files before deploying them.

## License
Project created by Pal Kerecsenyi.

Licensed under GNU GPL 3.0 (see LICENSE.md).

For future BCSS committees: feel free to use, modify, or discard this project however you'd like. If you need any help, email pk760@bath.ac.uk or pal@palk.me.
