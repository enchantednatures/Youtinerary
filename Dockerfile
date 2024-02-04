# Use the official Rust image as the base
FROM rust:1.75 as build


# Create a new empty shell project

WORKDIR /youtinerary
RUN USER=root cargo new --bin api
RUN USER=root cargo new --lib auth
RUN USER=root cargo new --lib components 
RUN USER=root cargo new --lib pages
RUN USER=root cargo new --lib state
RUN USER=root cargo new --bin webapp

# Copy your project's Cargo.toml and Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./api/Cargo.toml ./api/Cargo.toml
COPY ./auth/Cargo.toml ./auth/Cargo.toml

# Cache dependencies
RUN cargo build --bin api --release
RUN rm api/src/*.rs
RUN rm auth/src/*.rs

# Copy your source code

COPY ./.sqlx ./.sqlx
COPY ./config/ /config
COPY ./api/src ./api/src
COPY ./auth/src ./auth/src
COPY ./api/migrations ./api/migrations

# Build the release binary
RUN rm ./target/release/deps/api*
ENV SQLX_OFFLINE true
ENV RUST_LOG=info,axum::rejection=trace 

RUN cargo install --path api
# Start a new stage for the runtime image
FROM debian:bullseye-slim as runtime

# Install necessary runtime libraries for PostgreSQL
RUN apt-get update && \
    apt-get install -y libpq-dev ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the release binary from the build stage
COPY --from=build /usr/local/cargo/bin/api /usr/local/bin/api

# Expose the port the API will run on
EXPOSE 6969
# Run the API

COPY ./config ./config
ENV ENVIRONMENT production
ENV RUST_LOG=info,axum::rejection=trace 
CMD ["api"]
