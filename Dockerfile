FROM rust:1-slim-buster as builder

WORKDIR /app

RUN apt-get update -y; apt-get install -y pkg-config libssl-dev libpq-dev

COPY . .

RUN cargo build --release
RUN cargo install diesel_cli --no-default-features --features postgres

# second stage.
FROM debian:buster-slim
WORKDIR /app

COPY --from=builder /app/target/release/form-website .
COPY --from=builder /usr/local/cargo/bin/diesel .
ADD migrations migrations
COPY .env .
COPY diesel.toml .
COPY Rocket.toml .

RUN apt-get update -y; apt-get install -y libssl-dev libpq-dev
RUN chmod +x form-website

EXPOSE 8000

CMD ["./diesel", "migration run"]
CMD ["./form-website"]
