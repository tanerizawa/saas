# Kubernetes Deployment Guide for SaaS UMKM Platform

Panduan ini memberikan instruksi lengkap untuk men-deploy platform SaaS UMKM pada Kubernetes, menggunakan konfigurasi yang tersedia di direktori `infrastructure/kubernetes`.

## Prasyarat

Sebelum melanjutkan, pastikan Anda memiliki tools berikut terinstall dan terkonfigurasi:

- `kubectl` - Kubernetes command-line tool
- Akses ke cluster Kubernetes (AKS, EKS, GKE, atau minikube untuk pengembangan lokal)
- Docker CLI untuk build dan push container images
- Helm (opsional, untuk deployment layanan tambahan)

## Overview Arsitektur

Platform SaaS UMKM mengikuti pola arsitektur monolitik dan terdiri dari komponen-komponen berikut yang di-deploy pada Kubernetes:

1. **Backend Monolitik** - Server API Rust/Axum tunggal dengan semua business logic
2. **Frontend** - Aplikasi Next.js yang di-serve terpisah
3. **PostgreSQL** - Database stateful
4. **Redis** - Cache in-memory untuk optimasi performa
5. **Monitoring Stack** - Prometheus, Grafana, dan tools observability lainnya

## Struktur Konfigurasi Kubernetes

```
infrastructure/kubernetes/
├── backend-deployment.yaml     # Deployment untuk backend monolitik
├── config.yaml                 # ConfigMap dan Secret untuk konfigurasi
├── frontend-deployment.yaml    # Deployment untuk frontend Next.js
├── ingress.yaml                # Ingress untuk external access
├── postgres-statefulset.yaml   # StatefulSet untuk PostgreSQL
├── redis-deployment.yaml       # Deployment untuk Redis cache
├── saas-umkm-monolith.yaml     # Konfigurasi monolitik all-in-one
├── monitoring/                 # Konfigurasi Prometheus dan Grafana
└── service-mesh/               # Konfigurasi untuk service mesh (future)
```

## Deployment Instructions

### 1. Konfigurasi Environment

Pertama, buat namespace untuk aplikasi:

```bash
kubectl create namespace saas-umkm
kubectl config set-context --current --namespace=saas-umkm
```

Terapkan konfigurasi:

```bash
kubectl apply -f infrastructure/kubernetes/config.yaml
```

### 2. Deploy Database

Deploy PostgreSQL StatefulSet:

```bash
kubectl apply -f infrastructure/kubernetes/postgres-statefulset.yaml
```

Tunggu PostgreSQL siap:

```bash
kubectl wait --for=condition=ready pod -l app=postgres --timeout=300s
```

### 3. Deploy Redis Cache

```bash
kubectl apply -f infrastructure/kubernetes/redis-deployment.yaml
```

### 4. Deploy Backend API

```bash
kubectl apply -f infrastructure/kubernetes/backend-deployment.yaml
```

### 5. Deploy Frontend Application

```bash
kubectl apply -f infrastructure/kubernetes/frontend-deployment.yaml
```

### 6. Konfigurasi Ingress

```bash
kubectl apply -f infrastructure/kubernetes/ingress.yaml
```

### 7. Verifikasi Deployment

Periksa bahwa semua pods berjalan:

```bash
kubectl get pods
```

Verifikasi services terekspos:

```bash
kubectl get services
```

Periksa status ingress:

```bash
kubectl get ingress
```

## Advanced Configuration

### Scaling the Application

To scale the backend horizontally:

```bash
kubectl scale deployment backend --replicas=3
```

For the frontend:

```bash
kubectl scale deployment frontend --replicas=3
```

### Monitoring Setup

The monitoring stack can be deployed using:

```bash
kubectl apply -f infrastructure/kubernetes/monitoring/
```

### Service Mesh (In Development)

Service mesh configuration will be available in:

```
infrastructure/kubernetes/service-mesh/
```

## Troubleshooting

### Common Issues and Resolutions

1. **Pods Stuck in Pending State**

   - Check for resource constraints: `kubectl describe pod [pod-name]`
   - Verify PersistentVolumeClaim availability: `kubectl get pvc`

2. **Database Connection Issues**

   - Verify PostgreSQL pod is healthy: `kubectl logs [postgres-pod-name]`
   - Check backend environment variables: `kubectl describe pod [backend-pod-name]`

3. **Ingress Not Working**
   - Verify ingress controller is installed: `kubectl get pods -n ingress-nginx`
   - Check ingress rules: `kubectl describe ingress`

## Production Considerations

For production deployments, consider implementing:

1. **Resource Limits and Requests** - Ensure all deployments have appropriate CPU and memory constraints
2. **Auto-scaling** - Configure Horizontal Pod Autoscaler (HPA) for dynamic scaling
3. **Affinity Rules** - To control pod placement across nodes
4. **Liveness and Readiness Probes** - For better health checking
5. **Pod Disruption Budgets** - To ensure availability during maintenance

## Future Enhancements

- Implement Istio Service Mesh for advanced traffic management
- Setup distributed tracing with Jaeger
- Configure GitOps with ArgoCD or Flux
- Implement canary deployments for reduced risk

---

_Documentation created: July 23, 2025_
_Last updated: July 23, 2025_
