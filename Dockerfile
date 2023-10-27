# Use a Rust base image
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy your Rust project files into the container
COPY . .

# Build your Rust application
RUN cargo build --verbose

# Define the command to run when the container starts
CMD ["./target/release/soccer"]