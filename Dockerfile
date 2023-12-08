FROM docker.io/library/rust:alpine AS build

WORKDIR /sopaste

RUN apk add build-base xz \
 && adduser -D builder \
 && chown builder .

USER builder

COPY Cargo.* ./
RUN mkdir src \
 && touch src/lib.rs \
 && cargo build --release \
 && cargo vendor --versioned-dirs \
 && rm -rf src vendor/*/lib

COPY --chown=builder . .
ENV XZ_OPT --block-size=16MiB --threads=0
RUN tar -cJf static/sopaste.tar.xz --exclude=target /sopaste
RUN cargo build --features=source --release


FROM scratch

COPY --from=build /sopaste/target/release/sopaste .

USER 65534:65534
CMD ["/sopaste"]
