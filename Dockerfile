# Use the latest version of the Rust base image
FROM rust:latest

# Set the working directory in the container to /my
WORKDIR /usr/src/app

# Copy the Rust project files to the working directory
COPY . .

ARG API_KEY
ARG API_URL
ENV API_KEY=$API_KEY
ENV API_URL=$API_URL

# Build the Rust app
RUN cargo build