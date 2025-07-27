#!/bin/bash
set -e

# Create config directory structure for Grafana
mkdir -p config/grafana/dashboards
mkdir -p config/grafana/datasources

# Create datasource configuration for Grafana
cat > config/grafana/datasources/datasource.yml << EOF
apiVersion: 1

datasources:
  - name: InfluxDB
    type: influxdb
    access: proxy
    url: http://influxdb:8086
    database: k6
    isDefault: true
    editable: true
EOF

# Create dashboard for k6 results
cat > config/grafana/dashboards/dashboard.yml << EOF
apiVersion: 1

providers:
  - name: 'K6 Dashboard'
    orgId: 1
    folder: ''
    type: file
    disableDeletion: false
    editable: true
    options:
      path: /etc/grafana/provisioning/dashboards/k6-dashboard.json
EOF

# Download k6 dashboard for Grafana
curl -o config/grafana/dashboards/k6-dashboard.json https://raw.githubusercontent.com/grafana/k6/master/grafana/dashboards/k6-load-testing-results.json

echo "Grafana configuration created successfully!"
echo "Run 'docker-compose up' to start the performance testing environment."
