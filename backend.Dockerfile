FROM docker.io/rust:1.76-bookworm as builder

WORKDIR /usr/src/bhw

COPY . .

RUN cargo install --path packages/backend --root /usr

FROM docker.io/alpine:3 as runner

WORKDIR /usr/bin
COPY --from=builder --chmod=0755 /usr/bin/bhw-backend /usr/bin/bhw-backend

CMD ["/usr/bin/bhw-backend"]
