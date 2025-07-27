# API Documentation

This directory contains the API documentation for the SaaS UMKM Backend.

## OpenAPI Specification

The API is documented using the OpenAPI 3.0 specification in the `openapi.yaml` file. This specification describes all available endpoints, request/response schemas, authentication methods, and examples.

## Viewing the Documentation

### Option 1: Swagger UI

When running the backend in development mode, Swagger UI is available at:

```
http://localhost:8080/docs
```

This provides an interactive documentation interface where you can:
- Browse available endpoints
- View request/response schemas
- Execute API requests directly from the browser
- See examples and response codes

### Option 2: Redoc

For a more user-friendly documentation view, Redoc is available at:

```
http://localhost:8080/redoc
```

### Option 3: External Tools

You can also use external tools to view and interact with the OpenAPI specification:

1. **Swagger Editor**: Copy the content of `openapi.yaml` to [Swagger Editor](https://editor.swagger.io/)
2. **Postman**: Import the `openapi.yaml` file into Postman
3. **Insomnia**: Import the `openapi.yaml` file into Insomnia

## Authentication

Most API endpoints require authentication using JWT Bearer tokens. To authenticate:

1. Obtain a token using the `/api/auth/login` endpoint
2. Include the token in the `Authorization` header of subsequent requests:
   ```
   Authorization: Bearer <your_token>
   ```

## Rate Limiting

API requests are subject to rate limiting to prevent abuse. The current limits are:

- 60 requests per minute for authenticated users
- 20 requests per minute for unauthenticated users

When a rate limit is exceeded, the API will respond with a 429 Too Many Requests status code.

## Versioning

The API version is included in the URL path. The current version is `v1`:

```
/api/v1/...
```

## Error Handling

The API uses standard HTTP status codes and returns error details in a consistent format:

```json
{
  "error": "Error type",
  "message": "Detailed error message",
  "status": 400,
  "timestamp": "2025-07-01T12:00:00Z"
}
```
