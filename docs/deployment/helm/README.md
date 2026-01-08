# PoolAI Helm Chart

This Helm chart deploys PoolAI on a Kubernetes cluster.

## Prerequisites

- Kubernetes 1.19+
- Helm 3.0+
- CRDs installed (see `docs/deployment/crds/`)

## Installation

### Install CRDs first

```bash
kubectl apply -f ../crds/
```

### Install PoolAI

```bash
helm install poolai . --namespace poolai --create-namespace
```

### Install with custom values

```bash
helm install poolai . -f my-values.yaml --namespace poolai
```

## Configuration

The following table lists the configurable parameters:

| Parameter | Description | Default |
|-----------|-------------|---------|
| `operator.enabled` | Enable Kubernetes operator | `true` |
| `apiServer.enabled` | Enable API server | `true` |
| `apiServer.replicaCount` | Number of API server replicas | `2` |
| `worker.enabled` | Enable worker deployments | `true` |
| `kubernetes.namespace` | Kubernetes namespace | `poolai` |
| `serviceAccount.create` | Create service account | `true` |
| `rbac.create` | Create RBAC resources | `true` |
| `crds.install` | Install CRDs | `true` |
| `monitoring.enabled` | Enable monitoring | `false` |
| `ingress.enabled` | Enable ingress | `false` |
| `enterprise.enabled` | Enable enterprise features | `false` |

## Examples

### Basic installation

```bash
helm install poolai . --namespace poolai
```

### With enterprise features

```bash
helm install poolai . \
  --set enterprise.enabled=true \
  --namespace poolai
```

### With custom image

```bash
helm install poolai . \
  --set apiServer.image.repository=my-registry/poolai \
  --set apiServer.image.tag=v1.0.0 \
  --namespace poolai
```

## Uninstallation

```bash
helm uninstall poolai --namespace poolai
```

**Note:** CRDs are kept by default. To remove them:

```bash
kubectl delete -f ../crds/
```

## Upgrading

```bash
helm upgrade poolai . --namespace poolai
```

## Troubleshooting

See the main troubleshooting guide: `../../troubleshooting/COMMON_ISSUES.md`
