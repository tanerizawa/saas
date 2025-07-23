# 🚀 Production Environment Setup Guide

This guide provides instructions for setting up and managing the production environment for the SaaS UMKM platform.

## 📋 Prerequisites

- Kubernetes cluster (GKE, AKS, or EKS)
- kubectl CLI tool
- Docker
- GitHub Container Registry access
- Domain name with DNS control

## 🔧 Initial Setup

### Setting up the Kubernetes Cluster

1. **Create Kubernetes Cluster**

   ```bash
   # For GKE
   gcloud container clusters create saas-umkm-cluster \
     --num-nodes=3 \
     --machine-type=e2-standard-2 \
     --region=asia-southeast1 \
     --enable-autoscaling \
     --min-nodes=3 \
     --max-nodes=6
   ```

2. **Connect to the cluster**

   ```bash
   gcloud container clusters get-credentials saas-umkm-cluster --region=asia-southeast1
   ```

3. **Apply namespace configuration**

   ```bash
   kubectl apply -f infrastructure/kubernetes/config.yaml
   ```

### Setting up Container Registry

1. **Configure Docker for GitHub Container Registry**

   ```bash
   echo $GITHUB_TOKEN | docker login ghcr.io -u $GITHUB_USERNAME --password-stdin
   ```

2. **Grant access to the Kubernetes cluster**

   ```bash
   kubectl create secret docker-registry ghcr-creds \
     --namespace=saas-umkm \
     --docker-server=ghcr.io \
     --docker-username=$GITHUB_USERNAME \
     --docker-password=$GITHUB_TOKEN
   ```

## 🚀 Deployment

### Database Setup

1. **Apply database manifests**

   ```bash
   kubectl apply -f infrastructure/kubernetes/postgres-statefulset.yaml
   kubectl apply -f infrastructure/kubernetes/redis-deployment.yaml
   ```

2. **Wait for database pods to be ready**

   ```bash
   kubectl wait --for=condition=ready pod -l app=postgres -n saas-umkm --timeout=120s
   kubectl wait --for=condition=ready pod -l app=redis -n saas-umkm --timeout=60s
   ```

3. **Run database migrations**

   ```bash
   kubectl apply -f infrastructure/kubernetes/db-migration-job.yaml
   ```

### Application Deployment

1. **Deploy backend and frontend**

   ```bash
   kubectl apply -f infrastructure/kubernetes/backend-deployment.yaml
   kubectl apply -f infrastructure/kubernetes/frontend-deployment.yaml
   ```

2. **Apply ingress configuration**

   ```bash
   kubectl apply -f infrastructure/kubernetes/ingress.yaml
   ```

3. **Verify deployment**

   ```bash
   kubectl get pods -n saas-umkm
   kubectl get services -n saas-umkm
   kubectl get ingress -n saas-umkm
   ```

## 📊 Monitoring

### Setting up Prometheus and Grafana

1. **Deploy monitoring namespace and components**

   ```bash
   kubectl apply -f infrastructure/monitoring/prometheus.yaml
   kubectl apply -f infrastructure/monitoring/grafana.yaml
   ```

2. **Access Grafana dashboard**

   Navigate to `https://monitoring.saas-umkm.id` and login with the admin credentials.

3. **Import dashboards**

   Import the pre-configured dashboards from the `infrastructure/monitoring/dashboards` directory.

## 🔄 Scaling

### Horizontal Pod Autoscaling

1. **Enable HPA for backend**

   ```bash
   kubectl autoscale deployment backend -n saas-umkm --cpu-percent=70 --min=3 --max=10
   ```

2. **Enable HPA for frontend**

   ```bash
   kubectl autoscale deployment frontend -n saas-umkm --cpu-percent=70 --min=3 --max=10
   ```

3. **Check autoscaling status**

   ```bash
   kubectl get hpa -n saas-umkm
   ```

## 🔒 Security

### TLS Certificates

TLS certificates are automatically managed by cert-manager. To check the status:

```bash
kubectl get certificates -n saas-umkm
kubectl get certificaterequests -n saas-umkm
```

### Secrets Management

Secrets are managed through Kubernetes secrets. Avoid storing sensitive values in config files or code.

```bash
# To update a secret
kubectl create secret generic auth-secrets \
  --namespace=saas-umkm \
  --from-literal=jwt-secret=<new-jwt-secret> \
  --from-literal=totp-secret=<new-totp-secret> \
  --dry-run=client -o yaml | kubectl apply -f -
```

## 🔍 Troubleshooting

### Common Issues

1. **Pods in CrashLoopBackOff**

   Check the logs:

   ```bash
   kubectl logs <pod-name> -n saas-umkm
   ```

2. **Database connection issues**

   Verify connectivity:

   ```bash
   kubectl exec -it <backend-pod-name> -n saas-umkm -- curl postgres:5432
   ```

3. **Ingress not working**

   Check ingress controller logs:

   ```bash
   kubectl logs -l app=nginx-ingress-controller -n ingress-nginx
   ```

## 📚 Additional Resources

- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Prometheus Documentation](https://prometheus.io/docs/introduction/overview/)
- [Grafana Documentation](https://grafana.com/docs/)

## 🔄 CI/CD Pipeline

The CI/CD pipeline is configured in GitHub Actions. The workflow is defined in `.github/workflows/ci-cd.yaml`.

The pipeline performs the following steps:

1. Run tests for backend and frontend
2. Build and push Docker images
3. Deploy to Kubernetes cluster

To trigger a manual deployment, use the "Run workflow" feature in GitHub Actions.
