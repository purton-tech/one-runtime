VERSION 0.8

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
    ARG PROJECT_NAME
    ARG ISLANDS_PACKAGE
    ARG ISLANDS_WASM
    FROM +devcontainer
    WORKDIR /workspace
    COPY . .
    RUN cd /workspace/crates/${PROJECT_NAME}-assets && mkdir -p dist && tailwind-extra -i ./input.css -o ./dist/tailwind.css
    RUN rustup target add wasm32-unknown-unknown
    RUN cargo build -p ${ISLANDS_PACKAGE} --target wasm32-unknown-unknown --release && \
        wasm-bindgen \
          target/wasm32-unknown-unknown/release/${ISLANDS_WASM}.wasm \
          --target web \
          --out-dir crates/${PROJECT_NAME}-assets/dist
    RUN cargo fmt --check
    RUN cargo clippy --workspace --all-targets -- -D warnings

# Compile the workspace once as static musl binaries, then export all of the
# known binaries from the shared release output.
build:
    ARG PROJECT_NAME
    ARG APP_BINARY
    ARG ISLANDS_PACKAGE
    ARG ISLANDS_WASM
    FROM +devcontainer
    WORKDIR /workspace
    COPY . .
    RUN cd /workspace/crates/${PROJECT_NAME}-assets && mkdir -p dist && tailwind-extra -i ./input.css -o ./dist/tailwind.css
    RUN rustup target add wasm32-unknown-unknown
    RUN cargo build -p ${ISLANDS_PACKAGE} --target wasm32-unknown-unknown --release && \
        wasm-bindgen \
          target/wasm32-unknown-unknown/release/${ISLANDS_WASM}.wasm \
          --target web \
          --out-dir crates/${PROJECT_NAME}-assets/dist
    RUN rustup target add x86_64-unknown-linux-musl
    RUN cargo build --workspace --exclude ${ISLANDS_PACKAGE} --release --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/${APP_BINARY} /${APP_BINARY}
    SAVE ARTIFACT crates/${PROJECT_NAME}-assets/dist /workspace/crates/${PROJECT_NAME}-assets/dist
    SAVE ARTIFACT crates/${PROJECT_NAME}-assets/images /workspace/crates/${PROJECT_NAME}-assets/images

# Package a selected binary into a scratch image tagged with the binary name.
image:
    ARG PROJECT_NAME
    ARG APP_BINARY
    ARG ISLANDS_PACKAGE
    ARG ISLANDS_WASM
    ARG REGISTRY
    ARG BINARY=$APP_BINARY
    ARG TAG=latest
    FROM scratch
    COPY +certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
    COPY (+build/$BINARY --PROJECT_NAME=$PROJECT_NAME --APP_BINARY=$APP_BINARY --ISLANDS_PACKAGE=$ISLANDS_PACKAGE --ISLANDS_WASM=$ISLANDS_WASM) /app
    COPY (+build/workspace/crates/${PROJECT_NAME}-assets/dist --PROJECT_NAME=$PROJECT_NAME --APP_BINARY=$APP_BINARY --ISLANDS_PACKAGE=$ISLANDS_PACKAGE --ISLANDS_WASM=$ISLANDS_WASM) /workspace/crates/${PROJECT_NAME}-assets/dist
    COPY (+build/workspace/crates/${PROJECT_NAME}-assets/images --PROJECT_NAME=$PROJECT_NAME --APP_BINARY=$APP_BINARY --ISLANDS_PACKAGE=$ISLANDS_PACKAGE --ISLANDS_WASM=$ISLANDS_WASM) /workspace/crates/${PROJECT_NAME}-assets/images
    USER 65532:65532
    ENTRYPOINT ["/app"]
    SAVE IMAGE --push $REGISTRY/$BINARY:$TAG

# Package the dbmate migrations into a one-shot image that runs `dbmate up`
# at startup. Attach this via Stack's `init` section so migrations complete
# before the main service starts.
migration-image:
    ARG PROJECT_NAME
    ARG REGISTRY
    ARG TAG=latest
    FROM ghcr.io/amacneil/dbmate:2.26.0
    COPY crates/db/migrations /migrations
    ENTRYPOINT ["dbmate", "--no-dump-schema", "--migrations-dir", "/migrations", "up"]
    SAVE IMAGE --push $REGISTRY/${PROJECT_NAME}-migrations:$TAG

release-candidate:
    ARG PROJECT_NAME=one-runtime
    ARG APP_BINARY=one-runtime
    ARG ISLANDS_PACKAGE=one-runtime-islands
    ARG ISLANDS_WASM=one_runtime_islands
    ARG REGISTRY=ghcr.io/purton-tech
    ARG TAG
    BUILD +checks --PROJECT_NAME=$PROJECT_NAME --ISLANDS_PACKAGE=$ISLANDS_PACKAGE --ISLANDS_WASM=$ISLANDS_WASM
    BUILD +image --PROJECT_NAME=$PROJECT_NAME --BINARY=$APP_BINARY --REGISTRY=$REGISTRY --TAG=$TAG
    BUILD +migration-image --PROJECT_NAME=$PROJECT_NAME --REGISTRY=$REGISTRY --TAG=$TAG

# Build the currently packaged application image plus migrations.
all:
    ARG PROJECT_NAME=one-runtime
    ARG APP_BINARY=one-runtime
    ARG ISLANDS_PACKAGE=one-runtime-islands
    ARG ISLANDS_WASM=one_runtime_islands
    ARG REGISTRY=ghcr.io/purton-tech
    BUILD +checks --PROJECT_NAME=$PROJECT_NAME --ISLANDS_PACKAGE=$ISLANDS_PACKAGE --ISLANDS_WASM=$ISLANDS_WASM
    BUILD +image --PROJECT_NAME=$PROJECT_NAME --BINARY=$APP_BINARY --REGISTRY=$REGISTRY --TAG=latest
    BUILD +migration-image --PROJECT_NAME=$PROJECT_NAME --REGISTRY=$REGISTRY --TAG=latest
