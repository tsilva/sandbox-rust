FROM ubuntu:22.04

# Create non-root user
RUN groupadd --gid 1000 devuser \
    && useradd --uid 1000 --gid devuser --shell /bin/bash --create-home devuser

# Install basic dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    build-essential \
    git \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
USER devuser
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/home/devuser/.cargo/bin:$PATH

# Create workspace directory
USER root
RUN mkdir -p /workspace && chown devuser:devuser /workspace

# Set working directory
USER devuser
WORKDIR /workspace