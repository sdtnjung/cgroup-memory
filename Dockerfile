FROM rust:slim as builder
WORKDIR /usr/home/app

COPY . .

# RUN cargo bench

RUN cargo install cargo-nextest@0.9.78

# Build test archive
RUN cargo nextest archive --archive-file test_memory_max_100m.tar.zst --features test_memory_max_100m

FROM gcr.io/distroless/cc-debian12:debug-nonroot

# Copy source code (required for nextest)
COPY . .

# Copy nextest binary
COPY --from=builder /usr/local/cargo/bin/cargo-nextest /usr/local/bin/cargo-nextest

# Copy tests archive
COPY --from=builder /usr/home/app/test_memory_max_100m.tar.zst test_memory_max_100m.tar.zst

ENTRYPOINT []