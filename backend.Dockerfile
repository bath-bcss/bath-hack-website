FROM docker.io/rust:1.76-bookworm as builder

WORKDIR /usr/src/bhw

COPY . .

RUN cargo install --path packages/backend --root /usr

FROM docker.io/debian:bookworm-slim

COPY --from=builder --chmod=0755 /usr/bin/bhw-backend /opt/bhw-backend

RUN apt-get update && apt-get install -y libpq5

CMD ["/opt/bhw-backend"]
