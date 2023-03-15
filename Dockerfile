# syntax=docker/dockerfile:1

ARG BUILD_TARGET=x86_64-unknown-linux-gnu
ARG PRODUCTION_PLATFORM=linux/amd64

FROM rust:1.68 as builder
ARG BUILD_TARGET
RUN rustup target add $BUILD_TARGET
RUN apt-get update && apt-get install -y \
    gcc-x86-64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY . .
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
RUN cargo install --target $BUILD_TARGET --locked --path .

FROM --platform=$PRODUCTION_PLATFORM gcr.io/distroless/cc as production
COPY --from=builder /usr/local/cargo/bin/keto_recipes_api /app/keto_recipes_api
EXPOSE 3000
ENTRYPOINT ["/app/keto_recipes_api"]
