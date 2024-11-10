# Use a multi-stage build to optimize final image size

# Stage 1: Build the Rust backend
FROM rust:latest AS backend-builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Rust source code
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src

# Build the Rust binary
RUN cargo build --release

# Stage 2: Build the frontend
FROM node:latest AS frontend-builder

# Set the working directory for frontend
WORKDIR /app/frontend

# Copy the frontend source code
COPY ./frontend/package*.json ./
COPY ./frontend/ ./

# Install dependencies and build the frontend
RUN npm install
RUN npm run build

# Stage 3: Final image with both backend and frontend artifacts
FROM ubuntu:22.04

# Install minimal dependencies to run the Rust binary
RUN apt-get update && apt-get install -y \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory for the final container
WORKDIR /app

# Copy the Rust binary from the backend build stage
COPY --from=backend-builder /app/target/release/steel /app/steel

# Copy the frontend build output from the frontend build stage
COPY --from=frontend-builder /app/frontend/build /app/frontend/build

# Expose the port your server will run on
EXPOSE 3000

# Run the Rust binary as the main application
CMD ["./steel"]
