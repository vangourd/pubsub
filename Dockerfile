# Use the official Rust image as a base
FROM rust:latest

# Install ZeroMQ
RUN apt-get update && apt-get install -y libzmq3-dev

# Create a new binary project
RUN USER=root cargo new --bin zmq-rust-app
WORKDIR /zmq-rust-app

# Copy the Cargo.toml and Cargo.lock into the container
COPY Cargo.toml Cargo.lock ./

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code into the container
COPY src ./src

# Build the application
RUN cargo build --release

# Command to run the application
CMD ["./target/release/pubsub"]