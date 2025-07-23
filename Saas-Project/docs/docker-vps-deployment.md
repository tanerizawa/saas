# Panduan Deployment SaaS UMKM di VPS Ubuntu

Dokumen ini menjelaskan langkah-langkah untuk men-deploy platform SaaS UMKM di VPS Ubuntu menggunakan Docker dan optimasi untuk VPS dengan spesifikasi 8GB RAM.

## Perbandingan Opsi Deployment

| Opsi           | Kelebihan                                                                          | Kekurangan                                                        | Rekomendasi Untuk                              |
| -------------- | ---------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ---------------------------------------------- |
| **Docker**     | - Setup mudah<br>- Isolasi aplikasi<br>- Resource efisien<br>- Portabilitas tinggi | - Overhead container (minimal)<br>- Perlu memahami Docker         | **VPS 8GB RAM** (pilihan utama)                |
| **Kubernetes** | - Orkestrasi canggih<br>- Auto-scaling<br>- Self-healing                           | - Kompleks<br>- Resource intensif<br>- Overkill untuk skala kecil | Cluster multi-node, skala besar                |
| **Bare Metal** | - Performa maksimal<br>- Tidak ada overhead virtualisasi                           | - Setup kompleks<br>- Sulit maintenance<br>- Migrasi sulit        | Kasus khusus yang membutuhkan performa ekstrem |

## Rekomendasi: Docker

Berdasarkan spesifikasi VPS dengan 8GB RAM yang disebutkan dalam instruksi proyek, **Docker** adalah pilihan optimal karena:

1. Memberikan isolasi aplikasi dengan overhead minimal
2. Mempermudah deployment dan manajemen dependensi
3. Memungkinkan scaling horizontal di masa depan
4. Mengoptimalkan resource dengan containerisasi

## Prasyarat

- VPS dengan Ubuntu 20.04 LTS atau 22.04 LTS
- Minimal 8GB RAM (sesuai spesifikasi proyek)
- Minimal 2 vCPU
- Minimal 25GB SSD Storage
- Akses root atau sudo
- Domain (opsional namun direkomendasikan untuk HTTPS)

## 1. Persiapan Server

### Update sistem dan instal paket dasar

```bash
# Update package list dan upgrade sistem
sudo apt update && sudo apt upgrade -y

# Instal paket-paket yang dibutuhkan
sudo apt install -y curl git build-essential openssl libssl-dev pkg-config ufw
```

### Instal Docker dan Docker Compose

```bash
# Install Docker menggunakan script resmi
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER
# Log out dan log in kembali untuk menerapkan perubahan grup

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/download/v2.24.6/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

# Verifikasi instalasi
docker --version
docker-compose --version

# Start dan enable Docker service
sudo systemctl start docker
sudo systemctl enable docker
```

### Instal Caddy (Web Server dengan HTTPS otomatis)

```bash
sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list
sudo apt update
sudo apt install caddy
```

## 2. Konfigurasi Firewall

```bash
# Konfigurasi UFW firewall
sudo ufw allow ssh
sudo ufw allow http
sudo ufw allow https
sudo ufw enable
```

## 3. Clone Repository dan Konfigurasi

```bash
# Clone repository
git clone https://github.com/yourusername/Saas-Project.git
cd Saas-Project

# Setup file environment
cp backend/.env.example backend/.env
cp frontend/.env.example frontend/.env.local

# Edit file konfigurasi sesuai kebutuhan
nano backend/.env
nano frontend/.env.local
```

## 4. Optimasi Docker untuk VPS dengan 8GB RAM

Buat file `docker-compose.vps.yml` khusus untuk VPS:

```bash
nano docker-compose.vps.yml
```

Isi dengan konfigurasi berikut:

```yaml
version: "3.8"

services:
  postgres:
    image: postgres:15-alpine
    container_name: saas-postgres
    restart: unless-stopped
    environment:
      POSTGRES_USER: saas_user
      POSTGRES_PASSWORD: saas_password
      POSTGRES_DB: saas_umkm_db
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./config/postgres:/docker-entrypoint-initdb.d
    ports:
      - "5432:5432"
    command: >
      postgres
      -c shared_buffers=1GB
      -c effective_cache_size=3GB
      -c maintenance_work_mem=256MB
      -c work_mem=32MB
      -c max_connections=50
      -c random_page_cost=1.1
      -c effective_io_concurrency=200
      -c max_worker_processes=2
      -c max_parallel_workers_per_gather=1
      -c max_parallel_workers=2
      -c wal_buffers=16MB
      -c checkpoint_timeout=15min
      -c checkpoint_completion_target=0.9
    deploy:
      resources:
        limits:
          cpus: "0.5"
          memory: 2G

  redis:
    image: redis:7-alpine
    container_name: saas-redis
    restart: unless-stopped
    command: redis-server --maxmemory 512mb --maxmemory-policy allkeys-lru
    volumes:
      - redis_data:/data
    ports:
      - "6379:6379"
    deploy:
      resources:
        limits:
          cpus: "0.2"
          memory: 512M

  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile
    container_name: saas-backend
    restart: unless-stopped
    depends_on:
      - postgres
      - redis
    environment:
      - DATABASE_URL=postgresql://saas_user:saas_password@postgres:5432/saas_umkm_db
      - REDIS_URL=redis://redis:6379
      - APP_HOST=0.0.0.0
      - APP_PORT=8000
      - RUST_LOG=info
      - JWT_SECRET=${JWT_SECRET}
      - ENABLE_RATE_LIMITING=false
      - ENABLE_COMPRESSION=true
    ports:
      - "8000:8000"
    deploy:
      resources:
        limits:
          cpus: "1"
          memory: 3G

  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile
    container_name: saas-frontend
    restart: unless-stopped
    environment:
      - NEXT_PUBLIC_API_URL=http://localhost:8000/api/v1
      - NODE_ENV=production
    ports:
      - "3000:3000"
    depends_on:
      - backend
    deploy:
      resources:
        limits:
          cpus: "0.3"
          memory: 1G

volumes:
  postgres_data:
  redis_data:
```

## 5. Konfigurasi Akses ke Aplikasi

Karena server VPS Anda sudah menggunakan Webmin/Virtualmin dengan WordPress, berikut beberapa opsi untuk mengakses aplikasi SaaS UMKM tanpa mengganggu konfigurasi yang ada:

### Opsi 1: Akses Melalui Port Forwarding

Ini adalah opsi paling sederhana, Anda cukup mengakses aplikasi langsung melalui port yang telah dikonfigurasi dalam `docker-compose.vps.yml`:

- Frontend: `http://your-vps-ip:3000`
- Backend API: `http://your-vps-ip:8000`

Pastikan port tersebut dibuka pada firewall:

```bash
sudo ufw allow 3000
sudo ufw allow 8000
```

### Opsi 2: Menggunakan Virtualmin sebagai Proxy

Jika Anda ingin menggunakan subdomain untuk aplikasi SaaS UMKM, Anda bisa membuat virtual server baru di Virtualmin dan mengonfigurasinya sebagai proxy:

1. Buka Webmin/Virtualmin
2. Buat Virtual Server baru (misal: saas.yourdomain.com)
3. Di menu "Services", pilih "Configure Website"
4. Pilih "Proxy Directives" dan tambahkan:

```
ProxyPass / http://localhost:3000/
ProxyPassReverse / http://localhost:3000/

ProxyPass /api http://localhost:8000/api
ProxyPassReverse /api http://localhost:8000/api
```

### Opsi 3: Konfigurasi Nginx pada Port Berbeda

Jika Anda ingin menggunakan Nginx sebagai reverse proxy tanpa mengganggu konfigurasi Apache/Nginx yang ada:

1. Instal Nginx jika belum ada:

```bash
sudo apt update
sudo apt install nginx
```

2. Buat konfigurasi untuk SaaS UMKM pada port yang berbeda (misal: 8080):

```bash
sudo nano /etc/nginx/sites-available/saas-umkm
```

3. Tambahkan konfigurasi berikut:

```
server {
    listen 8080;
    server_name _;

    # Frontend
    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    # API Endpoints
    location /api {
        proxy_pass http://localhost:8000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

## 6. Jalankan Aplikasi

```bash
# Jalankan dengan docker-compose
docker-compose -f docker-compose.vps.yml up -d

# Periksa status container
docker-compose -f docker-compose.vps.yml ps

# Lihat logs
docker-compose -f docker-compose.vps.yml logs -f
```

## 7. Migrasi Database dan Pembuatan Akun Default

```bash
# Jalankan migrasi database
docker-compose -f docker-compose.vps.yml exec backend cargo run --bin migrate

# Buat akun default
docker-compose -f docker-compose.vps.yml exec backend cargo run --bin create-defaults
```

## 8. Monitoring dan Maintenance

### Setup monitoring dasar

```bash
# Instal netdata untuk monitoring (opsional, sangat ringan)
bash <(curl -Ss https://my-netdata.io/kickstart.sh)
```

### Setup backup otomatis

```bash
# Buat direktori backup
mkdir -p ~/backups

# Buat script backup
cat > ~/backup-db.sh << 'EOF'
#!/bin/bash
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BACKUP_DIR=~/backups
mkdir -p $BACKUP_DIR

# Backup database
docker exec saas-postgres pg_dump -U saas_user saas_umkm_db > $BACKUP_DIR/saas_umkm_db_$TIMESTAMP.sql

# Hapus backup yang lebih dari 7 hari
find $BACKUP_DIR -name "saas_umkm_db_*.sql" -type f -mtime +7 -delete
EOF

# Buat script executable
chmod +x ~/backup-db.sh

# Buat jadwal cron untuk backup otomatis setiap hari jam 2 pagi
(crontab -l 2>/dev/null; echo "0 2 * * * ~/backup-db.sh") | crontab -
```

## 9. Update Aplikasi

```bash
# Pull perubahan terbaru dari repositori
git pull

# Rebuild dan restart container
docker-compose -f docker-compose.vps.yml down
docker-compose -f docker-compose.vps.yml up -d --build
```

## Troubleshooting

### Jika backend tidak bisa terhubung dengan database:

1. Periksa network Docker:

   ```bash
   docker network ls
   docker network inspect saas-project_default
   ```

2. Periksa logs database:

   ```bash
   docker logs saas-postgres
   ```

3. Coba connect langsung ke database:
   ```bash
   docker exec -it saas-postgres psql -U saas_user -d saas_umkm_db
   ```

### Jika frontend tidak bisa terhubung ke backend:

1. Pastikan variabel NEXT_PUBLIC_API_URL diatur dengan benar di frontend/.env.local
2. Periksa konfigurasi CORS di backend/src/main.rs

## Perbandingan dengan Opsi Lain

### Kubernetes

Kubernetes memerlukan minimal 16GB RAM untuk berjalan dengan baik, sehingga tidak ideal untuk VPS 8GB. Kubernetes lebih cocok untuk:

- Aplikasi dengan skala besar
- Kebutuhan high availability
- Deployment multi-node

### Bare Metal

Deployment langsung ke VM tanpa containerization bisa memaksimalkan performa, namun:

- Lebih sulit untuk maintenance
- Kesulitan mengelola dependensi
- Sulit melakukan migrasi/backup

## Kesimpulan

Docker adalah pilihan optimal untuk VPS dengan 8GB RAM karena memberikan keseimbangan antara kemudahan deployment, isolasi aplikasi, dan efisiensi resource. Konfigurasi di atas telah dioptimalkan untuk memastikan aplikasi dapat berjalan dengan baik pada VPS dengan spesifikasi tersebut.
