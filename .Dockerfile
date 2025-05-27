# Build stage - specify platform for cross-compilation from macOS
FROM --platform=linux/amd64 rust:1.87-bullseye as builder

# Install required dependencies
RUN apt-get update && apt-get install -y \
    musl-dev \
    musl-tools \
    build-essential \
    gcc-x86-64-linux-gnu \
    wget \
    unzip \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Add to your existing Dockerfile after apt-get install
RUN apt-get update && apt-get install -y \
    musl-dev \
    musl-tools \
    build-essential \
    gcc-x86-64-linux-gnu \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Install PyTorch via pip
RUN pip3 install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cpu

# Set environment variable to use Python PyTorch
ENV LIBTORCH_USE_PYTORCH=1

# Set LibTorch environment variables
ENV LIBTORCH=/app/libtorch
ENV LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
ENV LIBTORCH_BYPASS_VERSION_CHECK=1 

# Add the musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Set cross-compilation environment variables
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-gnu-gcc
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
ENV CXX_x86_64_unknown_linux_musl=x86_64-linux-gnu-g++

# Set torch download environment (use CPU version)
ENV TORCH_CUDA_VERSION=cpu

# Copy workspace root files first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Copy all workspace members
COPY atelier ./atelier
COPY atelier-core ./atelier-core
COPY atelier-dcm ./atelier-dcm
COPY atelier-generators ./atelier-generators
COPY atelier-results ./atelier-results
COPY atelier-synth ./atelier-synth
COPY benches ./benches
COPY examples ./examples

# Create a dummy main.rs in the correct location for dependency caching
RUN mkdir -p atelier/src && echo "fn main() {}" > atelier/src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --target x86_64-unknown-linux-musl --release -p atelier

# Remove the dummy main.rs
RUN rm atelier/src/main.rs

# Copy the actual source code
COPY atelier/src ./atelier/src

# Build the actual binary
RUN cargo build --target x86_64-unknown-linux-musl --release -p atelier

# Runtime stage - need to include libtorch libraries
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libgomp1 \
    && rm -rf /var/lib/apt/lists/*

# Copy libtorch libraries
# COPY --from=builder /app/libtorch/lib /usr/local/lib

# Copy the statically linked binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/atelier /app

# Set library path
ENV LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

# Set the binary as the entrypoint
ENTRYPOINT ["/app"]
