# The name we'll give to our k3d cluster.
cluster-name := "one-runtime"

dev-init:
    k3d cluster delete {{cluster-name}}
    k3d cluster create {{cluster-name}} --agents 1 -p "32760-32761:32760-32761@agent:0"
    just get-config

dev-setup:
    stack init --install-keycloak
    stack deploy --manifest infra-as-code/stack.yaml --profile dev

dev-secrets:
    stack secrets --manifest infra-as-code/stack.yaml --db-host host.docker.internal --db-port 32761 >> .env

tmux:
    scripts/nails code

tmux-site :
    scripts/nails site

codex:
    sudo chown -R vscode:vscode /home/vscode/.codex
    sudo npm install -g @openai/codex

tk:
    tmux kill-server

# Retrieve the cluster kube config - so kubectl and k9s work.
get-config:
    k3d kubeconfig write {{cluster-name}} --kubeconfig-merge-default
    sed -i "s/127\.0\.0\.1/host.docker.internal/g; s/0\.0\.0\.0/host.docker.internal/g" "$HOME/.kube/config"
    # Disable TLS verification for local dev
    sed -i '/certificate-authority-data/d' "$HOME/.kube/config"
    sed -i '/cluster:/a \ \ \ \ insecure-skip-tls-verify: true' "$HOME/.kube/config"
    echo "✅ kubeconfig updated and TLS verification disabled"

## Watch for source code chnaes and run the relevant code generators
watch-db-gen:
    cargo watch -w ./crates/db/queries/ -s './scripts/clorinde'

_watch binary:
    mold -run cargo watch \
        --workdir /workspace/ \
        -w crates/db \
        -w crates/db-gen \
        -w crates/web-server \
        -w crates/web-ui \
        -w crates/web-assets/dist \
        --no-gitignore -x "run --bin web-server"

watch-binary: (_watch "web-server")

watch-tailwind:
    cd /workspace/crates/web-assets && /workspace/scripts/tailwind-crates --input ./input.css --output ./dist/tailwind.css --watch

build-islands:
    ./scripts/build-islands

watch-islands:
    cargo watch \
      -w crates/web-islands \
      -s './scripts/build-islands'
