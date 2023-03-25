# Build the runtime container
FROM debian:buster-slim
WORKDIR /app
COPY target/release/dedupfeed /app/dedupfeed
ENTRYPOINT ["/app/dedupfeed"]