VERSION 0.8

# This Earthfile builds and packages the application, frontend assets, and
# migration image used in deployment.
#
# Main entrypoints:
# - `all`: build and push the current application image and migration image
# - `release-candidate`: run checks, then build and push tagged release images
#
# Internal targets:
# - `devcontainer`: shared toolchain environment
# - `certs`: CA bundle for scratch images
# - `checks`: CI-style verification plus frontend asset/WASM generation
# - `build`: shared artifact build for binaries and frontend assets
# - `image`: package the web-server application image
# - `migration-image`: package dbmate migrations as a one-shot image
#
# Important args:
# - `REGISTRY`: image registry and namespace prefix
# - `IMAGE_NAME`: application image repository name
# - `TAG`: image tag for release builds

# Build the same toolchain environment as the devcontainer without hardcoding
# the upstream image in two places.
devcontainer:
    FROM DOCKERFILE .devcontainer
    WORKDIR /workspace

certs:
    FROM alpine:3.19
    RUN apk add --no-cache ca-certificates
    SAVE ARTIFACT /etc/ssl/certs/ca-certificates.crt /ca-certificates.crt

# Run the Rust checks that CI enforces inside the shared devcontainer toolchain.
checks:
    FROM +devcontainer
    WORKDIR /workspace
    COPY . .
    RUN cd /workspace/crates/web-assets && mkdir -p dist && tailwind-extra -i ./input.css -o ./dist/tailwind.css
    RUN rustup target add wasm32-unknown-unknown
    RUN cargo build -p web-islands --target wasm32-unknown-unknown --release && \
        wasm-bindgen \
          target/wasm32-unknown-unknown/release/web_islands.wasm \
          --target web \
          --out-dir crates/web-assets/dist
    RUN cargo fmt --check
    RUN cargo clippy --workspace --all-targets -- -D warnings

# Compile the workspace once as static musl binaries, then export all of the
# known runtime artifacts from the shared release output.
build:
    FROM +devcontainer
    WORKDIR /workspace
    COPY . .
    RUN cd /workspace/crates/web-assets && mkdir -p dist && tailwind-extra -i ./input.css -o ./dist/tailwind.css
    RUN rustup target add wasm32-unknown-unknown
    RUN cargo build -p web-islands --target wasm32-unknown-unknown --release && \
        wasm-bindgen \
          target/wasm32-unknown-unknown/release/web_islands.wasm \
          --target web \
          --out-dir crates/web-assets/dist
    RUN rustup target add x86_64-unknown-linux-musl
    RUN cargo build --workspace --exclude web-islands --release --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/web-server /web-server
    SAVE ARTIFACT crates/web-assets/dist /workspace/crates/web-assets/dist
    SAVE ARTIFACT crates/web-assets/images /workspace/crates/web-assets/images

# Package the web-server binary into the main application image.
image:
    ARG REGISTRY
    ARG IMAGE_NAME
    ARG TAG=latest
    FROM scratch
    COPY +certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
    COPY (+build/web-server) /app
    COPY (+build/workspace/crates/web-assets/dist) /workspace/crates/web-assets/dist
    COPY (+build/workspace/crates/web-assets/images) /workspace/crates/web-assets/images
    USER 65532:65532
    ENTRYPOINT ["/app"]
    SAVE IMAGE --push $REGISTRY/$IMAGE_NAME:$TAG

# Package the dbmate migrations into a one-shot image that runs `dbmate up`
# at startup. Attach this via Stack's `init` section so migrations complete
# before the main service starts.
migration-image:
    ARG REGISTRY
    ARG IMAGE_NAME
    ARG TAG=latest
    FROM ghcr.io/amacneil/dbmate:2.26.0
    COPY crates/db/migrations /migrations
    ENTRYPOINT ["dbmate", "--no-dump-schema", "--migrations-dir", "/migrations", "up"]
    SAVE IMAGE --push $REGISTRY/$IMAGE_NAME-migrations:$TAG

release-candidate:
    ARG REGISTRY=ghcr.io/purton-tech
    ARG IMAGE_NAME=one-runtime
    ARG TAG
    BUILD +checks
    BUILD +image --REGISTRY=$REGISTRY --IMAGE_NAME=$IMAGE_NAME --TAG=$TAG
    BUILD +migration-image --REGISTRY=$REGISTRY --IMAGE_NAME=$IMAGE_NAME --TAG=$TAG

# Build the currently packaged application image plus migrations.
all:
    ARG REGISTRY=ghcr.io/purton-tech
    ARG IMAGE_NAME=one-runtime
    BUILD +checks
    BUILD +image --REGISTRY=$REGISTRY --IMAGE_NAME=$IMAGE_NAME --TAG=latest
    BUILD +migration-image --REGISTRY=$REGISTRY --IMAGE_NAME=$IMAGE_NAME --TAG=latest
