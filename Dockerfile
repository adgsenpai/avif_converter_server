# Use the official Rust image as the base image
FROM rust:latest

# Install NASM
RUN apt-get update && \
    apt-get install -y nasm && \
    apt-get clean

# Set a working directory within the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY . . 

# Build the Rust application
RUN cargo build --release

# Specify the command to run your Rust application
CMD ["cargo", "run", "--release"]
