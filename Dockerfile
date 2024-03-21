# Use the latest version of the Rust base image
FROM rust:latest
ARG API_KEY
ARG API_URL
ENV API_KEY=$API_KEY
ENV API_URL=$API_URL
ENV RUST_LOG=task
COPY ./ ./
RUN cargo build --release
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
