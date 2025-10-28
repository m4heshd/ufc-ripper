# =====================
# ===== BUILDER =======
# =====================
FROM rust:1.90.0-bookworm AS builder-base

# Init
WORKDIR /ufcr

# Setup Node.js
RUN apt-get update && apt-get install -y curl ca-certificates gnupg && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash -
RUN apt-get install -y nodejs && rm -rf /var/lib/apt/lists/*
RUN node -v && npm -v

# Setup cargo-chef
RUN cargo install cargo-chef

# Generate the cargo-chef recipe
FROM builder-base AS planner

# Init
WORKDIR /ufcr

# Copy complete project files
COPY . .

# Create cargo-chef recipe
RUN cargo chef prepare --recipe-path recipe.json

# Cache dependencies and build
FROM builder-base AS builder

# Init
WORKDIR /ufcr

# Compile and cache Rust dependencies
COPY --from=planner /ufcr/recipe.json .
RUN cargo chef cook --profile dist --recipe-path recipe.json
RUN cargo chef cook --release --package pack --recipe-path recipe.json

# Cache Node.js dependencies
COPY --from=planner /ufcr/package.json .
COPY --from=planner /ufcr/package-lock.json .
RUN npm install

# Setup project
COPY . .

# Build
RUN npm run build
RUN cargo build-linux
RUN cargo pack-linux

# =====================
# ===== RUNTIME =======
# =====================
FROM ubuntu:24.04

# Meta
LABEL \
  "name"="ufc-ripper" \
  "maintainer"="Mahesh Bandara Wijerathna <m4heshd@gmail.com> (m4heshd)"

# Init
WORKDIR /ufcr

# Environment variables
ENV RUN_ENV=container

# Setup app
COPY --from=builder /ufcr/package/linux/ .
RUN chmod +x ./ufc-ripper

# Ports
EXPOSE 8383

# Volumes
VOLUME ["/ufcr/config"]
VOLUME ["/downloads"]

# Start
CMD ["./ufc-ripper"]

