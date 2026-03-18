# The name we'll give to our k3d cluster.
cluster-name := "one-runtime"

dev-init:
    k3d cluster delete {{cluster-name}}
    k3d cluster create {{cluster-name}} --agents 1 -p "40000-40001:40000-40001@agent:0"
    just get-config

dev-setup:
    stack init
    stack deploy --manifest infra-as-code/stack.yaml --profile dev

dev-secrets:
    stack secrets --manifest infra-as-code/stack.yaml --db-host host.docker.internal --db-port 40001 >> .env

# Retrieve the cluster kube config - so kubectl and k9s work.
get-config:
    k3d kubeconfig write {{cluster-name}} --kubeconfig-merge-default
    sed -i "s/127\.0\.0\.1/host.docker.internal/g; s/0\.0\.0\.0/host.docker.internal/g" "$HOME/.kube/config"
    # Disable TLS verification for local dev
    sed -i '/certificate-authority-data/d' "$HOME/.kube/config"
    sed -i '/cluster:/a \ \ \ \ insecure-skip-tls-verify: true' "$HOME/.kube/config"
    echo "✅ kubeconfig updated and TLS verification disabled"
