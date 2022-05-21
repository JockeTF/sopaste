FROM docker.io/library/archlinux:latest AS build

ARG TARGET=x86_64-unknown-linux-musl
WORKDIR /app

RUN pacman -Syu --noconfirm musl rust rust-musl \
 && useradd -ms /bin/bash builder \
 && chown builder:builder /app

USER builder

COPY Cargo.* ./
RUN mkdir src \
 && touch src/lib.rs \
 && cargo build --release --target=$TARGET \
 && rm -rf src

COPY . .
RUN cargo build --release --target=$TARGET \
 && mv target/$TARGET/release/sopaste target


FROM scratch

COPY --from=build /app/target/sopaste .

USER 65534:65534
CMD ["/sopaste"]
