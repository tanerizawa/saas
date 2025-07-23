<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

# Platform SaaS UMKM - Copilot Instructions

## Project Context
This is a comprehensive SaaS platform for Indonesian SME (UMKM) licensing and business management. The platform helps small and medium enterprises manage their permits, business operations, and financial compliance.

## Architecture Guidelines
- **Backend**: Use Rust with Axum framework following Domain-Driven Design (DDD) and Hexagonal Architecture patterns
- **Frontend**: Use Next.js 14 with TypeScript, App Router, and Tailwind CSS
- **Database**: PostgreSQL with optimized configuration for 8GB RAM VPS
- **Security**: Implement JWT authentication with RBAC, input validation, and audit logging
- **Infrastructure**: Use Docker for containerization, Caddy for web server with automatic HTTPS

## Code Standards

### Rust Backend
- Follow Domain-Driven Design principles with clear domain boundaries
- Use newtype pattern for type safety (e.g., `struct UserId(Uuid)`)
- Implement proper error handling with `Result<T, E>` - avoid `unwrap()` in production
- Use SQLx for database operations with compile-time query verification
- Implement structured logging with tracing crate
- Follow Hexagonal Architecture: separate domain, application, and infrastructure layers

### TypeScript Frontend
- Use strict TypeScript configuration
- Follow Next.js 14 App Router conventions
- Implement proper type definitions for API responses
- Use React Server Components where appropriate
- Implement proper loading states and error boundaries

### Security Requirements
- Never hardcode secrets - use environment variables
- Implement input validation and sanitization
- Use parameterized queries for database operations
- Implement rate limiting and CORS properly
- Hash passwords with Argon2
- Implement audit logging for sensitive operations

### Database Design
- Follow PostgreSQL best practices
- Use proper indexing for performance
- Implement soft deletes for audit trails
- Use transactions for data consistency
- Follow normalization principles while considering performance

### API Design
- Follow RESTful principles
- Use proper HTTP status codes
- Implement consistent error response format
- Version APIs appropriately (/api/v1/)
- Document all endpoints with OpenAPI/Swagger

## Domain Contexts

### Licensing Domain
- NIB (Nomor Induk Berusaha) registration
- SIUP (Surat Izin Usaha Perdagangan)
- TDP (Tanda Daftar Perusahaan)
- NPWP (Tax registration)

### Business Management Domain
- Company profile management
- Document management
- Business verification

### Financial Domain
- Tax management
- Payment processing
- Financial reporting

### User Management Domain
- Authentication and authorization
- Role-based access control
- User profiles and preferences

## Integration Requirements
- Government API integration (OSS - Online Single Submission)
- Payment gateway integration
- Document storage and validation
- Email and notification services

## Performance Targets
- API response time: < 200ms for most endpoints
- Database queries: optimized for 8GB RAM configuration
- Frontend loading: < 2 seconds initial load
- Support for 100+ concurrent users initially

## Deployment Strategy
- Docker containerization for all services
- Docker Compose for development environment
- Caddy for reverse proxy with automatic HTTPS
- PostgreSQL optimization for VPS deployment
- CI/CD pipeline with GitHub Actions

## Monitoring & Observability
- Structured logging with correlation IDs
- Health check endpoints
- Metrics collection (Prometheus)
- Error tracking and alerting
- Database performance monitoring

When generating code, prioritize:
1. Security and data protection
2. Performance and scalability
3. Maintainability and clean architecture
4. Type safety and error handling
5. Documentation and testing
