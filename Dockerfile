FROM rust:1.66-alpine as build

# Install dependencies
RUN apk add --no-cache \
        build-base \
        cmake \
        git \
        openssh-client

WORKDIR /app

COPY . .

RUN cargo build --release


FROM alpine:latest as install

RUN apk add --no-cache ca-certificates bash

COPY --from=build /app/target/release/json2file /usr/local/bin/json2file

RUN chmod +x /usr/local/bin/json2file

COPY entrypoint.sh /entrypoint.sh

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
