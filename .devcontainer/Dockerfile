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

# Install WABT (Includes wat2wasm)
RUN apt-get update && apt-get install -y wabt

# Install NVM
ENV NVM_DIR /usr/local/nvm
ENV NODE_VERSION 18.16.0

RUN mkdir -p $NVM_DIR

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash

# Add NVM to bash profile
RUN echo "export NVM_DIR=\"$NVM_DIR\"" >> /etc/bash.bashrc \
    && echo "[ -s \"$NVM_DIR/nvm.sh\" ] && \. \"$NVM_DIR/nvm.sh\"" >> /etc/bash.bashrc \
    && echo "[ -s \"$NVM_DIR/bash_completion\" ] && \. \"$NVM_DIR/bash_completion\"" >> /etc/bash.bashrc

# Install default Node.js version
SHELL ["/bin/bash", "-c"]
RUN source $NVM_DIR/nvm.sh \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && nvm use default

# Move Wasmer installation after user creation and before workspace setup
USER devuser
RUN curl https://get.wasmer.io -sSfL | sh

# Add Wasmer to devuser's PATH
ENV PATH="/home/devuser/.wasmer/bin:${PATH}"

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/home/devuser/.cargo/bin:$PATH

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && \
    chmod +x /home/devuser/.cargo/bin/wasm-pack

# Create workspace directory
USER root
RUN mkdir -p /workspace && chown devuser:devuser /workspace

# Set working directory
USER devuser
WORKDIR /workspace
