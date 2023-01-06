FROM rust:1.49-alpine as build

# Install dependencies
RUN apk add --no-cache \
        build-base \
        cmake \
        git \
        openssh-client

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml file
COPY Cargo.toml .

# Copy the source code
COPY . .

# Rebuild the project
RUN cargo build --release

# Install the binary
FROM alpine:latest as install

RUN apk add --no-cache \
        ca-certificates

COPY --from=build /app/target/release/json2file /usr/local/bin/

RUN chmod +x /usr/local/bin/json2file

COPY entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
