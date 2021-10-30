FROM rust:slim-bullseye AS build

WORKDIR /app

RUN apt-get update \
 && apt-get dist-upgrade -y \
 && chown nobody:nogroup /app

USER nobody

COPY Cargo.* ./
RUN mkdir src \
 && touch src/lib.rs \
 && cargo build --release \
 && rm -rf src

COPY . .
RUN cargo build --release \
 && strip target/release/sopaste


FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update \
 && apt-get dist-upgrade -y \
 && apt-get clean

COPY --from=build /app/target/release/sopaste bin

USER nobody
CMD ["./bin"]
