#[cfg(test)]
mod api_endpoints_tests {
    use std::sync::Arc;
    use actix_web::{test, web, App, http::header};
    use sqlx::PgPool;
    
    use crate::domain::entities::{User, UserRole, UserStatus};
    use crate::domain::value_objects::{Email, UserId};
    use crate::application::services::AuthService;
    use crate::infrastructure::auth::JwtAuthService;
    use crate::infrastructure::repositories::PostgresUserRepository;
    use crate::middleware::auth::AuthMiddleware;
    
    // Helper function to create a test application with actual database
    async fn setup_test_app() -> (actix_web::test::TestApp, PgPool, String) {
        // Setup test database
        let schema_name = format!("test_{}", uuid::Uuid::new_v4().as_simple());
        
        // Get database URL from environment or use a default test database
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://saas_user:saas_password@localhost:5432/saas_test_db".to_string());

        // Connect to the test database
        let pool = PgPool::connect(&database_url).await.unwrap();

        // Create a new schema for this test and setup tables
        sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name))
            .execute(&pool)
            .await
            .unwrap();

        // Set the search path to our new schema
        sqlx::query(&format!("SET search_path TO {}", schema_name))
            .execute(&pool)
            .await
            .unwrap();

        // Create necessary tables for the test
        sqlx::query(
            r#"
            CREATE TABLE users (
                id UUID PRIMARY KEY,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                full_name TEXT NOT NULL,
                role TEXT NOT NULL,
                status TEXT NOT NULL,
                email_verified BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TIMESTAMPTZ NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        // Create and configure app
        let user_repository = Arc::new(PostgresUserRepository::new(pool.clone()));
        let auth_service = Arc::new(JwtAuthService::new("test_secret_key"));

        // Create the application
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(user_repository.clone()))
                .app_data(web::Data::new(auth_service.clone()))
                .service(
                    web::scope("/api")
                        .wrap(AuthMiddleware::new(auth_service.clone()))
                        .service(crate::infrastructure::web::routes::users_routes())
                        .service(crate::infrastructure::web::routes::auth_routes())
                )
        ).await;

        (app, pool, schema_name)
    }

    // Helper to clean up resources
    async fn teardown(pool: &PgPool, schema_name: &str) {
        // Drop the schema to clean up
        sqlx::query(&format!("DROP SCHEMA IF EXISTS {} CASCADE", schema_name))
            .execute(pool)
            .await
            .unwrap();
        
        // Close the connection pool to prevent potential memory leaks
        pool.close().await;
    }

    // Test user registration endpoint
    #[actix_web::test]
    async fn test_user_registration() {
        // Setup
        let (app, pool, schema_name) = setup_test_app().await;
        
        // Create test data
        let registration_data = serde_json::json!({
            "email": "newuser@example.com",
            "password": "securePassword123!",
            "full_name": "New Test User"
        });
        
        // Execute request
        let req = test::TestRequest::post()
            .uri("/api/auth/register")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(registration_data.to_string())
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        
        // Verify response
        assert_eq!(resp.status().as_u16(), 201); // Created status
        
        // Verify user was created in the database
        let user_repository = PostgresUserRepository::new(pool.clone());
        let email = Email::new("newuser@example.com").unwrap();
        let user = user_repository.find_by_email(&email).await.unwrap();
        
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.full_name, "New Test User");
        assert_eq!(user.status, UserStatus::PendingVerification);
        
        // Clean up
        teardown(&pool, &schema_name).await;
    }
    
    // Test user login endpoint
    #[actix_web::test]
    async fn test_user_login() {
        // Setup
        let (app, pool, schema_name) = setup_test_app().await;
        let user_repository = PostgresUserRepository::new(pool.clone());
        let auth_service = JwtAuthService::new("test_secret_key");
        
        // Create a test user
        let email = Email::new("testuser@example.com").unwrap();
        let password = "securePassword123!";
        let password_hash = auth_service.hash_password(password).await.unwrap();
        
        let user = User {
            id: UserId::new(),
            email: email.clone(),
            password_hash,
            full_name: "Test User".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        user_repository.save(&user).await.unwrap();
        
        // Execute login request
        let login_data = serde_json::json!({
            "email": "testuser@example.com",
            "password": "securePassword123!"
        });
        
        let req = test::TestRequest::post()
            .uri("/api/auth/login")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(login_data.to_string())
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        
        // Verify response
        assert_eq!(resp.status().as_u16(), 200);
        
        // Parse response body and verify token exists
        let body = test::read_body(resp).await;
        let response: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert!(response.get("token").is_some());
        assert!(response["token"].as_str().unwrap().len() > 0);
        
        // Clean up
        teardown(&pool, &schema_name).await;
    }
    
    // Test protected endpoint
    #[actix_web::test]
    async fn test_protected_endpoint() {
        // Setup
        let (app, pool, schema_name) = setup_test_app().await;
        let user_repository = PostgresUserRepository::new(pool.clone());
        let auth_service = JwtAuthService::new("test_secret_key");
        
        // Create a test user
        let user_id = UserId::new();
        let email = Email::new("testuser@example.com").unwrap();
        let password = "securePassword123!";
        let password_hash = auth_service.hash_password(password).await.unwrap();
        
        let user = User {
            id: user_id.clone(),
            email: email.clone(),
            password_hash,
            full_name: "Test User".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        user_repository.save(&user).await.unwrap();
        
        // Generate token for user
        let token = auth_service.generate_token(&user_id).await.unwrap();
        
        // Test accessing a protected endpoint with the token
        let req = test::TestRequest::get()
            .uri("/api/users/me")
            .insert_header((header::AUTHORIZATION, format!("Bearer {}", token)))
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        
        // Verify successful access
        assert_eq!(resp.status().as_u16(), 200);
        
        // Parse response and verify it's the correct user
        let body = test::read_body(resp).await;
        let response: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(response["email"].as_str().unwrap(), "testuser@example.com");
        assert_eq!(response["full_name"].as_str().unwrap(), "Test User");
        
        // Test accessing protected endpoint without token
        let req = test::TestRequest::get()
            .uri("/api/users/me")
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        
        // Verify unauthorized access
        assert_eq!(resp.status().as_u16(), 401);
        
        // Clean up
        teardown(&pool, &schema_name).await;
    }
}
