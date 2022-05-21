FROM docker.io/library/rust:alpine AS build

WORKDIR /app

RUN apk add build-base \
 && adduser -D builder \
 && chown builder:builder /app

USER builder

COPY Cargo.* ./
RUN mkdir src \
 && touch src/lib.rs \
 && cargo build --release \
 && rm -rf src

COPY . .
RUN cargo build --release


FROM scratch

COPY --from=build /app/target/release/sopaste .

USER 65534:65534
CMD ["/sopaste"]
