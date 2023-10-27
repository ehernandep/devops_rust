# Ue the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/

# Copy the entire project into the container
COPY . .

# Set the DATABASE_URL environment variable
ENV DATABASE_URL=postgresql://ehernandep:Tonterias4316*@finalproject.csif2sx3kogz.us-east-1.rds.amazonaws.com:5432/finalproject


# Build your Rust application
RUN cargo build --release

# Expose the port your application will run on
EXPOSE 8080

# Run your Rust application
CMD ["cargo", "run"]