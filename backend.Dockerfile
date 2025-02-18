FROM docker.io/rust:1-bookworm as builder

WORKDIR /usr/src/bhw

COPY . .

RUN cargo install --path packages/backend --root /usr --no-default-features

FROM docker.io/debian:bookworm-slim

COPY --from=builder --chmod=0755 /usr/bin/bhw-backend /opt/bhw-backend

RUN apt-get update && apt-get install -y ca-certificates

CMD ["/opt/bhw-backend"]
