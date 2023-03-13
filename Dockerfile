# syntax=docker/dockerfile:1

FROM rust:1.68 as builder
WORKDIR /src
COPY . .
RUN cargo install --path .

FROM gcr.io/distroless/cc as production
COPY --from=builder /usr/local/cargo/bin/keto_recipes_api /app/keto_recipes_api
EXPOSE 3000
ENTRYPOINT ["/app/keto_recipes_api"]
