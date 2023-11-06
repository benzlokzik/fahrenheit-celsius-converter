# official Rust image as a builder
FROM rust:1.70 as builder

# new empty project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy my manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# copy the source code
COPY ./src ./src

# remove the dummy target
RUN rm ./target/release/deps/fahrenheit_celsius_kelvin_converter*

# set the build to be statically linked
RUN RUSTFLAGS='-C target-feature=+crt-static'

# build for release
RUN cargo build --release

# final base image, use the same Debian version as the builder
FROM debian:bullseye-slim

# copy the build artifact from the build stage
COPY --from=builder /app/target/release/fahrenheit_celsius_kelvin_converter .

# set the startup command to run binary
CMD ["./fahrenheit_celsius_kelvin_converter"]
