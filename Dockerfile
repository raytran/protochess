# Rust
FROM rust as builder
WORKDIR /usr/src/protochess
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path ./protochess-server-rs

# Install npm
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get install -y nodejs

#Build frontend SPA
WORKDIR /usr/src/protochess/protochess-front
RUN npm install
RUN npm run build

# Bundle Stage
FROM scratch
# Copy in warp
COPY --from=builder /usr/local/cargo/bin/protochess-server-rs .
# Copy in built frontend SPA
COPY --from=builder /usr/src/protochess/protochess-front/dist ./dist
USER 1000
EXPOSE 3030
CMD ["./protochess-server-rs"]
