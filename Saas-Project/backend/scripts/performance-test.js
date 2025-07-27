import http from 'k6/http';
import { check, sleep } from 'k6';
import { Counter, Rate } from 'k6/metrics';

// Custom metrics
const errors = new Counter('errors');
const userCreationRate = new Rate('user_creation_rate');

// Configuration
export const options = {
  // Test scenarios
  scenarios: {
    // Smoke test
    smoke_test: {
      executor: 'constant-vus',
      vus: 1,
      duration: '30s',
      exec: 'smokeTest',
      tags: { test_type: 'smoke' },
    },
    
    // Load test
    load_test: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '2m', target: 50 },  // Ramp up to 50 users over 2 minutes
        { duration: '5m', target: 50 },  // Stay at 50 users for 5 minutes
        { duration: '2m', target: 0 },   // Ramp down to 0 users over 2 minutes
      ],
      exec: 'loadTest',
      tags: { test_type: 'load' },
    },
    
    // Stress test
    stress_test: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '2m', target: 100 },   // Ramp up to 100 users over 2 minutes
        { duration: '5m', target: 100 },   // Stay at 100 users for 5 minutes
        { duration: '2m', target: 200 },   // Ramp up to 200 users over 2 minutes
        { duration: '5m', target: 200 },   // Stay at 200 users for 5 minutes
        { duration: '2m', target: 0 },     // Ramp down to 0 users over 2 minutes
      ],
      exec: 'stressTest',
      tags: { test_type: 'stress' },
    },
    
    // Spike test
    spike_test: {
      executor: 'ramping-vus',
      startVUs: 0,
      stages: [
        { duration: '10s', target: 0 },     // Initialize
        { duration: '1m', target: 0 },      // Stay at 0 users for 1 minute
        { duration: '10s', target: 300 },   // Spike to 300 users in 10 seconds
        { duration: '1m', target: 300 },    // Stay at 300 users for 1 minute
        { duration: '10s', target: 0 },     // Ramp down to 0 users over 10 seconds
        { duration: '1m', target: 0 },      // Stay at 0 users for 1 minute
        { duration: '10s', target: 300 },   // Spike to 300 users again
        { duration: '1m', target: 300 },    // Stay at 300 users for 1 minute
        { duration: '10s', target: 0 },     // Ramp down to 0 users over 10 seconds
      ],
      exec: 'spikeTest',
      tags: { test_type: 'spike' },
    },
  },
  
  // Thresholds
  thresholds: {
    http_req_duration: ['p(95)<500', 'p(99)<1500'],  // 95% of requests should be below 500ms, 99% below 1.5s
    http_req_failed: ['rate<0.01'],                  // Less than 1% of requests should fail
    'user_creation_rate': ['rate>0.9'],             // User creation success rate should be above 90%
  },
};

// Base URL
const BASE_URL = __ENV.API_URL || 'http://localhost:8080/api';
let authToken = '';

// Shared test data
const userData = {
  email: `user${Date.now()}@example.com`,
  password: 'Password123!',
  full_name: 'Test User',
};

// Utility function for random email generation
function randomEmail() {
  return `user${Date.now()}_${Math.floor(Math.random() * 10000)}@example.com`;
}

// Smoke test - basic functionality check
export function smokeTest() {
  // Health check
  const healthRes = http.get(`${BASE_URL}/health`);
  check(healthRes, {
    'health check status is 200': (r) => r.status === 200,
    'health check returns "ok"': (r) => r.json('status') === 'ok',
  });

  // Register a new user
  const registerRes = http.post(`${BASE_URL}/auth/register`, JSON.stringify({
    email: randomEmail(),
    password: userData.password,
    full_name: userData.full_name,
  }), {
    headers: { 'Content-Type': 'application/json' },
  });
  
  check(registerRes, {
    'register status is 201': (r) => r.status === 201,
  });
  
  if (registerRes.status !== 201) {
    errors.add(1);
    userCreationRate.add(0);
  } else {
    userCreationRate.add(1);
  }

  // Login
  const loginRes = http.post(`${BASE_URL}/auth/login`, JSON.stringify({
    email: userData.email,
    password: userData.password,
  }), {
    headers: { 'Content-Type': 'application/json' },
  });
  
  check(loginRes, {
    'login status is 200': (r) => r.status === 200,
    'login has token': (r) => r.json('token') !== undefined,
  });
  
  if (loginRes.status === 200) {
    authToken = loginRes.json('token');
  }

  // Get user profile (authenticated)
  if (authToken) {
    const profileRes = http.get(`${BASE_URL}/users/me`, {
      headers: {
        'Authorization': `Bearer ${authToken}`,
        'Content-Type': 'application/json',
      },
    });
    
    check(profileRes, {
      'profile status is 200': (r) => r.status === 200,
      'profile has correct email': (r) => r.json('email') === userData.email,
    });
  }

  sleep(1);
}

// Load test - normal expected load
export function loadTest() {
  // Register new users
  if (Math.random() < 0.3) {
    const registerRes = http.post(`${BASE_URL}/auth/register`, JSON.stringify({
      email: randomEmail(),
      password: userData.password,
      full_name: userData.full_name,
    }), {
      headers: { 'Content-Type': 'application/json' },
    });
    
    if (registerRes.status !== 201) {
      errors.add(1);
      userCreationRate.add(0);
    } else {
      userCreationRate.add(1);
    }
  }
  
  // Login with existing users
  const loginRes = http.post(`${BASE_URL}/auth/login`, JSON.stringify({
    email: userData.email,
    password: userData.password,
  }), {
    headers: { 'Content-Type': 'application/json' },
  });
  
  if (loginRes.status === 200) {
    authToken = loginRes.json('token');
    
    // User operations (authenticated)
    const profileRes = http.get(`${BASE_URL}/users/me`, {
      headers: {
        'Authorization': `Bearer ${authToken}`,
        'Content-Type': 'application/json',
      },
    });
    
    // List users (admin operation)
    http.get(`${BASE_URL}/users?limit=10&offset=0`, {
      headers: {
        'Authorization': `Bearer ${authToken}`,
        'Content-Type': 'application/json',
      },
    });
    
    // Search users
    http.get(`${BASE_URL}/users/search?q=test&limit=10`, {
      headers: {
        'Authorization': `Bearer ${authToken}`,
        'Content-Type': 'application/json',
      },
    });
  }
  
  sleep(Math.random() * 3 + 1); // Random sleep between 1-4 seconds
}

// Stress test - higher than normal load
export function stressTest() {
  loadTest(); // Reuse load test logic
}

// Spike test - sudden extreme load
export function spikeTest() {
  loadTest(); // Reuse load test logic but with more VUs in configuration
}
