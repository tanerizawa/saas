#[cfg(test)]
mod refresh_token_tests {
    use super::mocks::{MockCompanyRepository, MockLicenseRepository, MockUserRepository};
    use crate::infrastructure::web::handlers::{self, AppStateType};
    use crate::services::auth::AuthService;
    use crate::domain::entities::{User, UserRole, UserStatus};
    use crate::domain::value_objects::{Email, UserId};
    use crate::config::{AppConfig, SmtpConfig, ExternalApiConfig};
    use axum::extract::State;
    use axum::{Json, http::StatusCode};
    use serde_json::json;
    use std::sync::Arc;

    #[derive(Clone)]
    struct DummyState {
        user_repo: Arc<MockUserRepository>,
        company_repo: Arc<MockCompanyRepository>,
        license_repo: Arc<MockLicenseRepository>,
        auth_service: AuthService,
        config: AppConfig,
    }

    impl AppStateType for DummyState {
        fn company_repository(&self) -> &Arc<dyn crate::domain::repositories::CompanyRepository + Send + Sync> {
            &self.company_repo
        }
        fn user_repository(&self) -> &Arc<dyn crate::domain::repositories::UserRepository + Send + Sync> {
            &self.user_repo
        }
        fn license_repository(&self) -> &Arc<dyn crate::infrastructure::repositories::LicenseRepository + Send + Sync> {
            &self.license_repo
        }
        fn auth_service(&self) -> &AuthService {
            &self.auth_service
        }
        fn config(&self) -> &AppConfig {
            &self.config
        }
        fn cache_service(&self) -> &Option<crate::infrastructure::cache::CacheService> {
            &None
        }
    }

    fn default_config() -> AppConfig {
        AppConfig {
            database_url: "sqlite://test".into(),
            app_host: "127.0.0.1".into(),
            app_port: 0,
            jwt_secret: "secret".into(),
            jwt_expires_in: "15m".into(),
            jwt_refresh_expires_in: "7d".into(),
            redis_url: None,
            upload_dir: "/tmp".into(),
            max_file_size: 0,
            smtp: SmtpConfig {
                host: "".into(),
                port: 0,
                username: "".into(),
                password: "".into(),
                from_email: "".into(),
            },
            external_apis: ExternalApiConfig {
                oss_api_url: "".into(),
                oss_api_key: "".into(),
                oss_api_secret: "".into(),
                midtrans_server_key: "".into(),
                midtrans_client_key: "".into(),
                midtrans_is_production: false,
            },
            cors_origins: vec![],
            rate_limiter: None,
            enable_compression: false,
        }
    }

    #[tokio::test]
    async fn test_refresh_token_success() {
        let email = Email::new("test@example.com").unwrap();
        let user = User {
            id: UserId::new(),
            email: email.clone(),
            password_hash: "hash".into(),
            full_name: "Test".into(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: Some(chrono::Utc::now()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_find_by_id().returning(move |_| Ok(Some(user.clone())));

        let state = Arc::new(DummyState {
            user_repo: Arc::new(user_repo),
            company_repo: Arc::new(MockCompanyRepository::new()),
            license_repo: Arc::new(MockLicenseRepository::new()),
            auth_service: AuthService::new("secret".into()),
            config: default_config(),
        });

        let tokens = state.auth_service().generate_tokens(&user).unwrap();

        let payload = Json(json!({"refresh_token": tokens.refresh_token}));
        let resp = handlers::auth::refresh_token(State(state.clone()), payload).await.unwrap();
        let body = resp.0;
        assert!(body.get("access_token").is_some());
        assert!(body.get("refresh_token").is_some());
    }

    #[tokio::test]
    async fn test_refresh_token_invalid() {
        let state = Arc::new(DummyState {
            user_repo: Arc::new(MockUserRepository::new()),
            company_repo: Arc::new(MockCompanyRepository::new()),
            license_repo: Arc::new(MockLicenseRepository::new()),
            auth_service: AuthService::new("secret".into()),
            config: default_config(),
        });

        let payload = Json(json!({"refresh_token": "bad"}));
        let result = handlers::auth::refresh_token(State(state.clone()), payload).await;
        assert!(result.is_err());
        let (status, _) = result.err().unwrap();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
}
