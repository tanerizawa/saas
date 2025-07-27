#[cfg(test)]
mod api_tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::{json, Value};
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::application::services::AuthService;
    use crate::domain::entities::{User, UserRole, UserStatus};
    use crate::domain::repositories::UserRepository;
    use crate::domain::value_objects::{Email, UserId};
    use crate::infrastructure::web::{router, AppState};
    use crate::shared::errors::AppResult;
    
    // Mock repositories and services
    use crate::tests::mocks::{MockUserRepository, MockAuthService};
    use mockall::predicate::*;
    use std::sync::Arc;
    use chrono::Utc;

    // Helper function to create a test app state with mock services
    fn create_test_state() -> AppState {
        let user_repo = Arc::new(MockUserRepository::new());
        let auth_service = Arc::new(MockAuthService::new());
        
        AppState {
            user_repository: user_repo as Arc<dyn UserRepository + Send + Sync>,
            auth_service: auth_service as Arc<dyn AuthService + Send + Sync>,
            // Add other needed services here, such as company repository, license repository, etc.
        }
    }

    // Test health check endpoint
    #[tokio::test]
    async fn test_health_check() -> AppResult<()> {
        // Create test app with router
        let app_state = create_test_state();
        let app = router::create_router(app_state);
        
        // Create a request to the health endpoint
        let request = Request::builder()
            .uri("/api/health")
            .method("GET")
            .body(Body::empty())
            .unwrap();
        
        // Send the request to our app
        let response = app.oneshot(request).await.unwrap();
        
        // Check status code
        assert_eq!(response.status(), StatusCode::OK);
        
        // Check response body
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        
        assert_eq!(body["status"], "ok");
        
        Ok(())
    }
    
    // Test user registration endpoint
    #[tokio::test]
    async fn test_user_registration() -> AppResult<()> {
        // Setup mocks
        let mut user_repo = MockUserRepository::new();
        let mut auth_service = MockAuthService::new();
        
        // Mock email check to return no existing user
        user_repo.expect_find_by_email()
            .returning(|_| Ok(None));
        
        // Mock password hashing
        auth_service.expect_hash_password()
            .returning(|_| Ok("hashed_password".to_string()));
            
        // Mock user save to succeed
        user_repo.expect_save()
            .returning(|_| Ok(()));
        
        let app_state = AppState {
            user_repository: Arc::new(user_repo) as Arc<dyn UserRepository + Send + Sync>,
            auth_service: Arc::new(auth_service) as Arc<dyn AuthService + Send + Sync>,
            // Add other needed services here
        };
        
        let app = router::create_router(app_state);
        
        // Create registration request
        let request_body = json!({
            "email": "new_user@example.com",
            "password": "Secure123!",
            "full_name": "New User"
        });
        
        let request = Request::builder()
            .uri("/api/auth/register")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(request_body.to_string()))
            .unwrap();
        
        // Send the request
        let response = app.oneshot(request).await.unwrap();
        
        // Check response
        assert_eq!(response.status(), StatusCode::CREATED);
        
        Ok(())
    }
    
    // Test user login endpoint
    #[tokio::test]
    async fn test_user_login() -> AppResult<()> {
        // Setup mocks
        let mut user_repo = MockUserRepository::new();
        let mut auth_service = MockAuthService::new();
        
        // Create a test user
        let user_id = UserId::new();
        let user = User {
            id: user_id.clone(),
            email: Email::new("test_user@example.com").unwrap(),
            password_hash: "hashed_password".to_string(),
            full_name: "Test User".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Mock finding the user
        user_repo.expect_find_by_email()
            .returning(move |_| Ok(Some(user.clone())));
        
        // Mock password verification
        auth_service.expect_verify_password()
            .returning(|_, _| Ok(true));
            
        // Mock token generation
        auth_service.expect_generate_token()
            .returning(|_| Ok("test_jwt_token".to_string()));
        
        let app_state = AppState {
            user_repository: Arc::new(user_repo) as Arc<dyn UserRepository + Send + Sync>,
            auth_service: Arc::new(auth_service) as Arc<dyn AuthService + Send + Sync>,
            // Add other needed services here
        };
        
        let app = router::create_router(app_state);
        
        // Create login request
        let request_body = json!({
            "email": "test_user@example.com",
            "password": "Secure123!"
        });
        
        let request = Request::builder()
            .uri("/api/auth/login")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(request_body.to_string()))
            .unwrap();
        
        // Send the request
        let response = app.oneshot(request).await.unwrap();
        
        // Check response
        assert_eq!(response.status(), StatusCode::OK);
        
        // Verify response body contains a token
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        
        assert_eq!(body["token"], "test_jwt_token");
        
        Ok(())
    }
    
    // Test user profile endpoint (authenticated endpoint)
    #[tokio::test]
    async fn test_get_user_profile() -> AppResult<()> {
        // Setup mocks
        let mut user_repo = MockUserRepository::new();
        let mut auth_service = MockAuthService::new();
        
        // Create a test user
        let user_id = UserId::new();
        let user = User {
            id: user_id.clone(),
            email: Email::new("test_user@example.com").unwrap(),
            password_hash: "hashed_password".to_string(),
            full_name: "Test User".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Mock token validation
        auth_service.expect_validate_token()
            .returning(move |_| Ok(user_id.to_string()));
            
        // Mock finding the user by ID
        user_repo.expect_find_by_id()
            .returning(move |_| Ok(Some(user.clone())));
        
        let app_state = AppState {
            user_repository: Arc::new(user_repo) as Arc<dyn UserRepository + Send + Sync>,
            auth_service: Arc::new(auth_service) as Arc<dyn AuthService + Send + Sync>,
            // Add other needed services here
        };
        
        let app = router::create_router(app_state);
        
        // Create profile request with auth token
        let request = Request::builder()
            .uri("/api/users/me")
            .method("GET")
            .header("Authorization", "Bearer test_jwt_token")
            .body(Body::empty())
            .unwrap();
        
        // Send the request
        let response = app.oneshot(request).await.unwrap();
        
        // Check response
        assert_eq!(response.status(), StatusCode::OK);
        
        // Verify response body contains user info
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body_bytes).unwrap();
        
        assert_eq!(body["email"], "test_user@example.com");
        assert_eq!(body["full_name"], "Test User");
        
        Ok(())
    }
    
    // Test unauthorized access
    #[tokio::test]
    async fn test_unauthorized_access() -> AppResult<()> {
        // Setup mocks
        let user_repo = MockUserRepository::new();
        let mut auth_service = MockAuthService::new();
        
        // Mock token validation to fail
        auth_service.expect_validate_token()
            .returning(|_| Err(crate::shared::errors::AppError::Unauthorized("Invalid token".to_string())));
        
        let app_state = AppState {
            user_repository: Arc::new(user_repo) as Arc<dyn UserRepository + Send + Sync>,
            auth_service: Arc::new(auth_service) as Arc<dyn AuthService + Send + Sync>,
            // Add other needed services here
        };
        
        let app = router::create_router(app_state);
        
        // Create request with invalid auth token
        let request = Request::builder()
            .uri("/api/users/me")
            .method("GET")
            .header("Authorization", "Bearer invalid_token")
            .body(Body::empty())
            .unwrap();
        
        // Send the request
        let response = app.oneshot(request).await.unwrap();
        
        // Check response - should be unauthorized
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        
        Ok(())
    }
}
