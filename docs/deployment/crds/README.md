# PoolAI Custom Resource Definitions (CRDs)

This directory contains Kubernetes Custom Resource Definitions (CRDs) for PoolAI resources.

## Installation

Install all CRDs:

```bash
kubectl apply -f poolai-worker-crd.yaml
kubectl apply -f poolai-vm-crd.yaml
kubectl apply -f poolai-tenant-crd.yaml
```

Or install all at once:

```bash
kubectl apply -f .
```

## CRD Resources

### PoolAIWorker

Manages worker deployments in Kubernetes.

**Example:**

```yaml
apiVersion: poolai.io/v1
kind: PoolAIWorker
metadata:
  name: my-worker
spec:
  image: poolai/worker:v1.0.0
  replicas: 3
  resources:
    cpu: "500m"
    memory: "512Mi"
    gpu: 1
```

### PoolAIVM

Manages VM instances in Kubernetes.

**Example:**

```yaml
apiVersion: poolai.io/v1
kind: PoolAIVM
metadata:
  name: my-vm
spec:
  image: poolai/vm:v1.0.0
  resources:
    cpu: "1"
    memory: "2Gi"
  storage:
    size: "20Gi"
    storage_class: "ssd"
```

### PoolAITenant

Manages tenant configurations with resource quotas.

**Example:**

```yaml
apiVersion: poolai.io/v1
kind: PoolAITenant
metadata:
  name: tenant-abc
spec:
  active: true
  quotas:
    max_workers: 10
    max_memory_mb: 1024
    max_cpu_cores: 4
    max_storage_mb: 10000
```

## Verification

Verify CRDs are installed:

```bash
kubectl get crds | grep poolai
```

Expected output:

```
poolaiworkers.poolai.io
poolaivms.poolai.io
poolaitenants.poolai.io
```

## Uninstallation

Remove all CRDs:

```bash
kubectl delete -f .
```

**Note:** This will also delete all custom resources of these types.
