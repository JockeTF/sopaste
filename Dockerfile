FROM archlinux:base-devel AS build

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
 && strip target/$TARGET/release/sopaste


FROM scratch

ARG TARGET=x86_64-unknown-linux-musl
WORKDIR /app

COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /app/target/$TARGET/release/sopaste .

USER nobody

CMD ["/app/sopaste"]
