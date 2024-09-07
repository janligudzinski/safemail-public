# Use the official Rust image as the base
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files for the workspace
COPY Cargo.toml Cargo.lock ./

# Copy the individual Cargo.toml files for each project in the workspace
COPY api/Cargo.toml api/
COPY application/Cargo.toml application/
COPY domain/Cargo.toml domain/
COPY infrastructure/Cargo.toml infrastructure/

# Fetch all dependencies for the workspace
RUN cargo fetch

# Copy the entire source code into the container
COPY . .

# Build the api project (which depends on other projects in the workspace)
RUN cargo build --release --package api

# Create a smaller image to run the application
FROM debian:buster-slim

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the binary from the builder image
COPY --from=builder /usr/src/app/target/release/api .

# Expose the port the API will be running on (adjust the port as needed)
EXPOSE 8080

# Command to run the API binary
CMD ["./api"]