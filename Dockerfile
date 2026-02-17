FROM rust:latest AS builder

ARG TARGET=x86_64-unknown-linux-gnu

RUN rustup target add ${TARGET}

# Install cross-compilation toolchains
RUN apt-get update && apt-get install -y \
    gcc-mingw-w64-x86-64 \
    gcc-aarch64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo build --release --target ${TARGET}
RUN cargo test --release --target ${TARGET} || true

# Output binary to /out
RUN mkdir -p /out && cp target/${TARGET}/release/mermaid-ascii-rust* /out/ 2>/dev/null || true
