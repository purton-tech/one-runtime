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
# - `image`: package a selected application binary
# - `migration-image`: package dbmate migrations as a one-shot image
#
# Important args:
# - `APP_BINARY`: main server binary to package
# - `ISLANDS_PACKAGE`: WASM crate to compile
# - `ISLANDS_WASM`: generated WASM artifact name

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
    ARG ISLANDS_PACKAGE
    ARG ISLANDS_WASM
    FROM +devcontainer
    WORKDIR /workspace
    COPY . .
    RUN cd /workspace/crates/web-assets && mkdir -p dist && tailwind-extra -i ./input.css -o ./dist/tailwind.css
    RUN rustup target add wasm32-unknown-unknown
    RUN cargo build -p ${ISLANDS_PACKAGE} --target wasm32-unknown-unknown --release && \
        wasm-bindgen \
          target/wasm32-unknown-unknown/release/${ISLANDS_WASM}.wasm \
          --target web \
          --out-dir crates/web-assets/dist
    RUN cargo fmt --check
    RUN cargo clippy --workspace --all-targets -- -D warnings

# Compile the workspace once as static musl binaries, then export all of the
# known binaries from the shared release output.
build:
    ARG APP_BINARY
    ARG ISLANDS_PACKAGE
    ARG ISLANDS_WASM
    FROM +devcontainer
    WORKDIR /workspace
    COPY . .
    RUN cd /workspace/crates/web-assets && mkdir -p dist && tailwind-extra -i ./input.css -o ./dist/tailwind.css
    RUN rustup target add wasm32-unknown-unknown
    RUN cargo build -p ${ISLANDS_PACKAGE} --target wasm32-unknown-unknown --release && \
        wasm-bindgen \
          target/wasm32-unknown-unknown/release/${ISLANDS_WASM}.wasm \
          --target web \
          --out-dir crates/web-assets/dist
    RUN rustup target add x86_64-unknown-linux-musl
    RUN cargo build --workspace --exclude ${ISLANDS_PACKAGE} --release --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/${APP_BINARY} /${APP_BINARY}
    SAVE ARTIFACT crates/web-assets/dist /workspace/crates/web-assets/dist
    SAVE ARTIFACT crates/web-assets/images /workspace/crates/web-assets/images

# Package a selected binary into a scratch image tagged with the binary name.
image:
    ARG APP_BINARY
    ARG ISLANDS_PACKAGE
    ARG ISLANDS_WASM
    ARG REGISTRY
    ARG BINARY=$APP_BINARY
    ARG TAG=latest
    FROM scratch
    COPY +certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
    COPY (+build/$BINARY --APP_BINARY=$APP_BINARY --ISLANDS_PACKAGE=$ISLANDS_PACKAGE --ISLANDS_WASM=$ISLANDS_WASM) /app
    COPY (+build/workspace/crates/web-assets/dist --APP_BINARY=$APP_BINARY --ISLANDS_PACKAGE=$ISLANDS_PACKAGE --ISLANDS_WASM=$ISLANDS_WASM) /workspace/crates/web-assets/dist
    COPY (+build/workspace/crates/web-assets/images --APP_BINARY=$APP_BINARY --ISLANDS_PACKAGE=$ISLANDS_PACKAGE --ISLANDS_WASM=$ISLANDS_WASM) /workspace/crates/web-assets/images
    USER 65532:65532
    ENTRYPOINT ["/app"]
    SAVE IMAGE --push $REGISTRY/$BINARY:$TAG

# Package the dbmate migrations into a one-shot image that runs `dbmate up`
# at startup. Attach this via Stack's `init` section so migrations complete
# before the main service starts.
migration-image:
    ARG REGISTRY
    ARG TAG=latest
    FROM ghcr.io/amacneil/dbmate:2.26.0
    COPY crates/db/migrations /migrations
    ENTRYPOINT ["dbmate", "--no-dump-schema", "--migrations-dir", "/migrations", "up"]
    SAVE IMAGE --push $REGISTRY/web-server-migrations:$TAG

release-candidate:
    ARG APP_BINARY=web-server
    ARG ISLANDS_PACKAGE=web-islands
    ARG ISLANDS_WASM=web_islands
    ARG REGISTRY=ghcr.io/purton-tech
    ARG TAG
    BUILD +checks --ISLANDS_PACKAGE=$ISLANDS_PACKAGE --ISLANDS_WASM=$ISLANDS_WASM
    BUILD +image --APP_BINARY=$APP_BINARY --ISLANDS_PACKAGE=$ISLANDS_PACKAGE --ISLANDS_WASM=$ISLANDS_WASM --BINARY=$APP_BINARY --REGISTRY=$REGISTRY --TAG=$TAG
    BUILD +migration-image --REGISTRY=$REGISTRY --TAG=$TAG

# Build the currently packaged application image plus migrations.
all:
    ARG APP_BINARY=web-server
    ARG ISLANDS_PACKAGE=web-islands
    ARG ISLANDS_WASM=web_islands
    ARG REGISTRY=ghcr.io/purton-tech
    BUILD +checks --ISLANDS_PACKAGE=$ISLANDS_PACKAGE --ISLANDS_WASM=$ISLANDS_WASM
    BUILD +image --APP_BINARY=$APP_BINARY --ISLANDS_PACKAGE=$ISLANDS_PACKAGE --ISLANDS_WASM=$ISLANDS_WASM --BINARY=$APP_BINARY --REGISTRY=$REGISTRY --TAG=latest
    BUILD +migration-image --REGISTRY=$REGISTRY --TAG=latest
