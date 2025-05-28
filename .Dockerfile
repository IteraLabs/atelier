# Build stage - specify platform for cross-compilation from macOS
FROM rust:1.87-bullseye AS builder

# Install required dependencies
RUN apt-get update && apt-get install -y \
    musl-dev \
    musl-tools \
    build-essential \
    gcc-x86-64-linux-gnu \
    wget \
    unzip \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Install torch using python & pip
pip install torch==2.6.0 torchvision==0.21.0 torchaudio==2.6.0 \
    --index-url https://download.pytorch.org/whl/cpu

# Set the working directory
WORKDIR /app

# Set environment variables for LibTorch and cross-compilation
ENV LIBTORCH=/app/libtorch
ENV LIBTORCH_USE_PYTORCH=1
ENV LIBTORCH_BYPASS_VERSION_CHECK=1
ENV LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

ENV TORCH_CUDA_VERSION=cpu
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-gnu-gcc
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
ENV CXX_x86_64_unknown_linux_musl=x86_64-linux-gnu-g++

# Add the musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Copy workspace configuration files first (for better layer caching)
COPY Cargo.toml ./

# Copy all workspace member directories (including atelier)
COPY atelier ./atelier
COPY atelier-core ./atelier-core
COPY atelier-dcm ./atelier-dcm
COPY atelier-generators ./atelier-generators
COPY atelier-results ./atelier-results
COPY atelier-synth ./atelier-synth

# Build the final binary
RUN cargo build --target x86_64-unknown-linux-musl --release -p atelier

# To be dropped into a Bash shell inside the container
CMD ["/bin/bash"]

