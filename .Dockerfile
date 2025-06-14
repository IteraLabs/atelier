# ---------------------------------------------------------------- STAGE 1: builder --- #
# ---------------------------------------------------------------- ---------------- --- #
FROM rust:1.87-bullseye AS builder

# Set working directory
WORKDIR /app

# Install system dependencies needed for tch-rs and libtorch
RUN apt-get update && apt-get install -y \
    build-essential \
    wget \
    unzip \
    python3 \
    python3-pip \
    ca-certificates \
    pkg-config \
    vim \
    && rm -rf /var/lib/apt/lists/*

# Download and unzip libtorch (CPU-only version, adjust version as needed)
RUN pip3 install torch==2.7 \
  --index-url https://download.pytorch.org/whl/cpu

# Set enviroments
ENV LIBTORCH=/app/libtorch
ENV LIBTORCH_INCLUDE=/app/libtorch
ENV LIBTORCH_LIB=/app/libtorch
ENV LIBTORCH_USE_PYTORCH=1
ENV LIBTORCH_BYPASS_VERSION_CHECK=1
ENV LD_LIBRARY_PATH=/app/libtorch/lib
ENV TORCH_CUDA_VERSION=cpu

# Copy Workspace Files
COPY Cargo.toml ./
COPY atelier-rs ./atelier-rs
COPY atelier-data ./atelier-data
COPY atelier-dcml ./atelier-dcml
COPY atelier-generators ./atelier-generators
COPY atelier-results ./atelier-results
COPY atelier-synth ./atelier-synth
COPY examples ./examples
COPY benches ./benches

# Build the Rust binary in release mode
RUN cargo build --release --bin synthetizer

# ----------------------------------------------------------------- STAGE 2: runner --- #
# ----------------------------------------------------------------- --------------- --- #
FROM debian:bullseye-slim AS runner

# Set Working Directory
WORKDIR /app

# Install minimal vm with system dependencies and tooling
RUN apt-get update && apt-get install -y \
  libgomp1 \
  vim

# Get the binary, the templates and dependencies from Builder
COPY --from=builder /app/target/release/synthetizer /usr/local/bin/synthetizer
COPY --from=builder /app/atelier-synth/templates ./templates
COPY --from=builder /usr/local/lib/python3*/dist-packages/torch/lib /usr/local/libtorch

# Add locations in the new environment
ENV LIBTORCH=/usr/local/libtorch
ENV LIBTORCH_INCLUDE=/usr/local/libtorch
ENV LIBTORCH_LIB=/usr/local/libtorch
ENV LIBTORCH_USE_PYTORCH=0
ENV LIBTORCH_BYPASS_VERSION_CHECK=1
ENV LD_LIBRARY_PATH=/usr/local/libtorch
ENV TORCH_CUDA_VERSION=cpu

# Service entrypoint
ENTRYPOINT ["/usr/local/bin/synthetizer"]
