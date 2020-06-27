# Rust
FROM rust as builder
WORKDIR /usr/src/protochess
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path ./protochess-server-rs

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/protochess-server-rs .
USER 1000
EXPOSE 3030
CMD ["./protochess-server-rs"]
