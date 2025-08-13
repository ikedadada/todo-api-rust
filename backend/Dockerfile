FROM rust:1.89.0-alpine AS build

WORKDIR /app

RUN apk update && apk add --no-cache musl-dev build-base

COPY . .

RUN cargo build --release

FROM alpine:latest

COPY --from=build /work/target/release/todo-api-rust /usr/local/bin/todo-api-rust

CMD [ "/usr/local/bin/todo-api-rust" ]
