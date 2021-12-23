FROM rust:1-slim-buster as builder

WORKDIR /app

RUN apt-get update -y; apt-get install -y pkg-config libssl-dev libpg-dev


COPY . .

RUN cargo build --release
RUN cargo install diesel_cli --no-default-features --features postgres

# second stage.
FROM debian:buster-slim

COPY --from=builder /app/target/release/form-website .
COPY --from=builder ~/.cargo/bin/ .
COPY --from=builder /app/migrations .
COPY --from=builder .env .
COPY --from=builder .diesel.toml .

EXPOSE 8000

CMD ["diesel migration run && ./form-website"]


