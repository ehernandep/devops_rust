# Use a Rust base image
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy your Rust project files into the container
COPY . .

# Set the DATABASE_URL environment variable
ENV DATABASE_URL=postgresql://ehernandep:Tonterias4316*@finalproject.csif2sx3kogz.us-east-1.rds.amazonaws.com:5432/finalproject


# Build your Rust application
RUN cargo build --verbose

# Define the command to run when the container starts
CMD ["./target/release/soccer"]