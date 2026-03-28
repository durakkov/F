FROM rust:1.84 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd -m -u 10001 woxel
WORKDIR /app
COPY --from=builder /app/target/release/woxel /usr/local/bin/woxel
COPY migrations ./migrations
USER woxel
CMD ["/usr/local/bin/woxel"]
