# Kubernetes Deployment Guide

## Overview

This guide covers deploying PoolAI on Kubernetes clusters. Kubernetes provides orchestration, scaling, and high availability for PoolAI deployments.

## Prerequisites

- Kubernetes cluster 1.24+
- kubectl configured
- Helm 3.0+ (optional, for Helm charts)
- StorageClass for persistent volumes

## Quick Start

### Basic Deployment

Create `k8s/deployment.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: poolai
  labels:
    app: poolai
spec:
  replicas: 1
  selector:
    matchLabels:
      app: poolai
  template:
    metadata:
      labels:
        app: poolai
    spec:
      containers:
      - name: poolai
        image: poolai:latest
        ports:
        - containerPort: 8080
          name: http
        - containerPort: 8443
          name: https
        env:
        - name: RUST_LOG
          value: "info"
        - name: POOLAI_CONFIG_PATH
          value: "/config/config.toml"
        volumeMounts:
        - name: data
          mountPath: /data
        - name: config
          mountPath: /config
        resources:
          requests:
            memory: "2Gi"
            cpu: "1"
          limits:
            memory: "4Gi"
            cpu: "2"
        livenessProbe:
          httpGet:
            path: /api/v1/health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /api/v1/health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: poolai-data
      - name: config
        configMap:
          name: poolai-config
---
apiVersion: v1
kind: Service
metadata:
  name: poolai
spec:
  selector:
    app: poolai
  ports:
  - name: http
    port: 80
    targetPort: 8080
  - name: https
    port: 443
    targetPort: 8443
  type: LoadBalancer
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: poolai-data
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 100Gi
---
apiVersion: v1
kind: ConfigMap
metadata:
  name: poolai-config
data:
  config.toml: |
    [server]
    host = "0.0.0.0"
    port = 8080
    
    [raid]
    mode = "distributed"
```

Deploy:

```bash
kubectl apply -f k8s/deployment.yaml
```

## Distributed RAID Deployment

For multi-node distributed RAID:

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: poolai-raft
spec:
  serviceName: poolai-raft
  replicas: 3
  selector:
    matchLabels:
      app: poolai-raft
  template:
    metadata:
      labels:
        app: poolai-raft
    spec:
      containers:
      - name: poolai
        image: poolai:latest
        env:
        - name: NODE_ID
          valueFrom:
            fieldRef:
              fieldPath: metadata.name
        - name: RAFT_CLUSTER
          value: "poolai-raft-0.poolai-raft:8080,poolai-raft-1.poolai-raft:8080,poolai-raft-2.poolai-raft:8080"
        volumeMounts:
        - name: data
          mountPath: /data
  volumeClaimTemplates:
  - metadata:
      name: data
    spec:
      accessModes: [ "ReadWriteOnce" ]
      resources:
        requests:
          storage: 100Gi
---
apiVersion: v1
kind: Service
metadata:
  name: poolai-raft
spec:
  clusterIP: None
  selector:
    app: poolai-raft
  ports:
  - port: 8080
    name: http
```

## Ingress Configuration

For external access with TLS:

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: poolai-ingress
  annotations:
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
spec:
  ingressClassName: nginx
  tls:
  - hosts:
    - poolai.example.com
    secretName: poolai-tls
  rules:
  - host: poolai.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: poolai
            port:
              number: 80
```

## Helm Chart

Create `helm/poolai/Chart.yaml`:

```yaml
apiVersion: v2
name: poolai
description: PoolAI - Distributed AI Mining Pool Management System
version: 0.1.0
appVersion: "0.1.0"
```

Create `helm/poolai/values.yaml`:

```yaml
replicaCount: 1

image:
  repository: poolai
  tag: latest
  pullPolicy: IfNotPresent

service:
  type: LoadBalancer
  port: 80
  httpsPort: 443

ingress:
  enabled: false
  className: nginx
  annotations: {}
  hosts:
    - host: poolai.example.com
      paths:
        - path: /
          pathType: Prefix
  tls: []

resources:
  requests:
    memory: "2Gi"
    cpu: "1"
  limits:
    memory: "4Gi"
    cpu: "2"

persistence:
  enabled: true
  size: 100Gi
  storageClass: ""

config:
  rust_log: "info"
```

Install:

```bash
helm install poolai ./helm/poolai
```

## Monitoring

### Prometheus ServiceMonitor

```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: poolai
spec:
  selector:
    matchLabels:
      app: poolai
  endpoints:
  - port: http
    path: /metrics
    interval: 30s
```

### Grafana Dashboard

Import dashboard from `docs/monitoring/grafana-dashboard.json`.

## Scaling

### Horizontal Pod Autoscaler

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: poolai-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: poolai
  minReplicas: 1
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

## Security

### Pod Security Policy

```yaml
apiVersion: policy/v1beta1
kind: PodSecurityPolicy
metadata:
  name: poolai-psp
spec:
  privileged: false
  allowPrivilegeEscalation: false
  requiredDropCapabilities:
    - ALL
  volumes:
    - 'configMap'
    - 'persistentVolumeClaim'
  runAsUser:
    rule: 'MustRunAsNonRoot'
  seLinux:
    rule: 'RunAsAny'
  fsGroup:
    rule: 'RunAsAny'
```

### Network Policies

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: poolai-netpol
spec:
  podSelector:
    matchLabels:
      app: poolai
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
    ports:
    - protocol: TCP
      port: 8080
  egress:
  - to:
    - podSelector:
        matchLabels:
          app: poolai-raft
    ports:
    - protocol: TCP
      port: 8080
```

## Troubleshooting

### Check Pod Status

```bash
kubectl get pods -l app=poolai
kubectl describe pod <pod-name>
kubectl logs <pod-name>
```

### Check Services

```bash
kubectl get svc
kubectl describe svc poolai
```

### Port Forward for Debugging

```bash
kubectl port-forward pod/<pod-name> 8080:8080
```

## Backup and Restore

### Backup

```bash
# Backup PVC
kubectl exec -it <pod-name> -- tar czf /tmp/backup.tar.gz /data
kubectl cp <pod-name>:/tmp/backup.tar.gz ./backup.tar.gz
```

### Restore

```bash
# Copy backup to pod
kubectl cp ./backup.tar.gz <pod-name>:/tmp/backup.tar.gz

# Restore
kubectl exec -it <pod-name> -- tar xzf /tmp/backup.tar.gz -C /
```

