# Bath Hack Website 2024

This is the monorepo for the 2024 Bath Hack Website, written in Rust (using Wasm on the frontend).

## Developing
To work on this repo, make sure you have a `stable` Rustup toolchain installed. You'll also need [Trunk](https://trunkrs.dev/) to work on the frontend.

For the backend, you'll need a local instance of PostgreSQL and Redis.

If you have Podman (or Docker), you can easily start these:

```
podman run -d --name postgres -p 54320:5432 -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=bhw docker.io/postgres
```

```
podman run -d --name redis -p 63790:6379 docker.io/redis/redis-stack-server:latest
```

> Warning: do **not** use these commands for production containers; these are _insecure_ configurations. Make sure to use _real_ passwords and persistent storage for Postgres (preferably also for Redis).

Copy `.env.example` to be `.env` and make sure all the variables have sensible values.

Then, `cd` into `packages/frontend` and run:

```
trunk serve
```

This command will start up a live-reloading server for the frontend.

In a new/parallel terminal, `cd` into the project root and run:

```
cargo run -p bhw-backend --no-default-features
```

This last command will run the migrations on your database and then start up the backend server (this doesn't live-reload).

## Formatting
Frontend Yew macros can be tricky for Rust's built-in `rustfmt` to handle. Instead, we use [`yew-fmt`](https://github.com/schvv31n/yew-fmt), which is a drop-in replacement that handles Yew code nicely.

## Deploying
This can be done in a couple of ways. If you're using Kubernetes, see the `k8s` directory for a `kustomize`-compatible configuration. You'll need to set some secrets manually, so make sure to look through the files before deploying them.

### Images
The frontend landing page has a lot of images, and we serve these in the efficient [webp](https://en.wikipedia.org/wiki/WebP) format at the smallest reasonable scale.

You can easily generate these images (in `packages/frontend/img`) using the `mogrify` CLI (part of ImageMagick):

```
mogrify -format webp -resize 400 packages/frontend/img/*.JPG
```

## License
Project created by Pal Kerecsenyi.

Licensed under GNU GPL 3.0 (see LICENSE.md).

For future BCSS committees: feel free to use, modify, or discard this project however you'd like. If you need any help, email pk760@bath.ac.uk or pal@palk.me.
