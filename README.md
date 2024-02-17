# Bath Hack Website 2024

This is the monorepo for the 2024 Bath Hack Website, written in Rust (using Wasm on the frontend).

## Developing
To work on this repo, make sure you have a `stable` Rustup toolchain installed. You'll also need [Trunk](https://trunkrs.dev/) to work on the frontend.

For the backend, you'll need a local instance of PostgreSQL and Redis.

If you have Podman (or Docker), you can easily start these:

```
podman run --name postgres -p 5432:5432 -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres docker.io/postgres
```

```
podman run -d --name redis -p 6379:6379 redis/redis-stack-server:latest
```

Ensure you've created the `bhw` database inside your new local PostgreSQL instance.

Copy `.env.example` to be `.env` and make sure all the variables have sensible values.

Then, `cd` into `packages/backend` to run the migrations:

```
diesel migration run --database-url $BHW_DATABASE_URL
```

Finally, `cd` back to the project root, and simultaneously (e.g. in two terminal tabs) start both the frontend and backend:

```
trunk server --config packages/frontend/Trunk.toml
```

```
cargo run -p bhw-backend
```

## License
Project created by Pal Kerecsenyi.

Licensed under GNU GPL 3.0 (see LICENSE.md).

For future BCSS committees: feel free to use, modify, or discard this project however you'd like. If you need any help, email pk760@bath.ac.uk or pal@palk.me.
