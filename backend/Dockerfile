FROM rust:slim-trixie AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:trixie-slim

WORKDIR /app

COPY --from=builder /app/target/release/main ./main
COPY --from=builder /app/target/release/migration ./migration
COPY --from=builder /app/target/release/seeder ./seeder

CMD ["./main"] 