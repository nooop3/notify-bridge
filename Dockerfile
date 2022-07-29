# syntax=docker/dockerfile:1
ARG BUILDER_SUFFIX=enable-crates-mirror

FROM rust:1.62.1-slim-bullseye as builder-disable-crates-mirror
ONBUILD echo "No need for Crates mirror."

FROM rust:1.62.1-slim-bullseye as builder-enable-crates-mirror
ONBUILD COPY <<EOF /usr/src/.cargo/config.toml
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
EOF

WORKDIR /usr/src

# Create a new empty shell project
RUN USER=root cargo new --bin app

# Copy manifests
COPY Cargo.toml Cargo.lock /usr/src/app/

# Set the working directory
WORKDIR /usr/src/app

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/app/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/app/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.16.0 AS runtime 

# Copy application binary from builder image
COPY --from=builder-${BUILDER_SUFFIX} /usr/src/app/target/x86_64-unknown-linux-musl/release/app /usr/local/bin

# Run the application
CMD ["/usr/local/bin/app"]
