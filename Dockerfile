FROM rust:buster as builder
RUN mkdir /app
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock
COPY src /app/src
WORKDIR /app
RUN cargo build --release

FROM node:buster as runner
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=builder /app/target/release/frpc-webui /usr/local/bin/frpc-webui
RUN mkdir /app
COPY run.sh /app/run.sh
COPY frontend /app/frontend
COPY bin /app/bin
RUN chmod +x /app/run.sh
WORKDIR /app

CMD ["./run.sh"]
