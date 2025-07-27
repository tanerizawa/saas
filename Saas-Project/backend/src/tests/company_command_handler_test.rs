#[cfg(test)]
mod company_command_handler_tests {
    use async_trait::async_trait;
    use mockall::predicate::*;
    use mockall::*;
    use std::sync::Arc;
    
    use crate::application::commands::{CreateCompanyCommand, UpdateCompanyCommand, DeleteCompanyCommand};
    use crate::application::command_handlers::{CompanyCommandHandler, CompanyCommandHandlerImpl};
    use crate::domain::entities::{Company};
    use crate::domain::repositories::CompanyRepository;
    use crate::domain::value_objects::CompanyId;
    use crate::shared::errors::AppResult;
    
    // Mock company repository
    mock! {
        CompanyRepo {}
        
        #[async_trait]
        impl CompanyRepository for CompanyRepo {
            async fn find_by_id(&self, id: &CompanyId) -> AppResult<Option<Company>>;
            async fn list_all(&self, limit: Option<i64>, offset: Option<i64>) -> AppResult<Vec<Company>>;
            async fn count_all(&self) -> AppResult<i64>;
            async fn save(&self, company: &Company) -> AppResult<()>;
            async fn delete(&self, id: &CompanyId) -> AppResult<()>;
            async fn search(&self, query: &str, limit: Option<i64>, offset: Option<i64>) -> AppResult<Vec<Company>>;
        }
    }
    
    #[tokio::test]
    async fn test_create_company() -> AppResult<()> {
        // Setup
        let mut mock_repo = MockCompanyRepo::new();
        
        // Configure mock to verify save is called with the right data
        mock_repo
            .expect_save()
            .withf(|company| {
                company.name == "New Company" 
                // Add more field verifications based on your Company struct
            })
            .times(1)
            .returning(|_| Ok(()));
            
        let handler = CompanyCommandHandlerImpl::new(Arc::new(mock_repo));
        
        // Execute command
        let command = CreateCompanyCommand {
            name: "New Company".to_string(),
            // Add other fields as needed
        };
        
        let result = handler.handle(command).await?;
        
        // Verify result contains a valid CompanyId
        assert!(!result.id.to_string().is_empty());
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_update_company() -> AppResult<()> {
        // Setup
        let mut mock_repo = MockCompanyRepo::new();
        let company_id = CompanyId::new();
        let existing_company = Company {
            id: company_id.clone(),
            name: "Original Name".to_string(),
            // Add other fields as needed
        };
        
        // Configure mock to return the existing company
        mock_repo
            .expect_find_by_id()
            .with(eq(company_id.clone()))
            .times(1)
            .returning(move |_| Ok(Some(existing_company.clone())));
            
        // Configure mock to verify save is called with updated data
        mock_repo
            .expect_save()
            .withf(move |company| {
                company.id == company_id && 
                company.name == "Updated Name"
                // Verify other fields as needed
            })
            .times(1)
            .returning(|_| Ok(()));
            
        let handler = CompanyCommandHandlerImpl::new(Arc::new(mock_repo));
        
        // Execute command
        let command = UpdateCompanyCommand {
            id: company_id,
            name: Some("Updated Name".to_string()),
            // Add other fields as needed, using Option to indicate fields that may or may not be updated
        };
        
        let result = handler.handle(command).await?;
        
        // Verify success
        assert!(result.success);
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_delete_company() -> AppResult<()> {
        // Setup
        let mut mock_repo = MockCompanyRepo::new();
        let company_id = CompanyId::new();
        
        // Configure mock to verify delete is called with the right id
        mock_repo
            .expect_delete()
            .with(eq(company_id.clone()))
            .times(1)
            .returning(|_| Ok(()));
            
        let handler = CompanyCommandHandlerImpl::new(Arc::new(mock_repo));
        
        // Execute command
        let command = DeleteCompanyCommand {
            id: company_id,
        };
        
        let result = handler.handle(command).await?;
        
        // Verify success
        assert!(result.success);
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_update_nonexistent_company() -> AppResult<()> {
        // Setup
        let mut mock_repo = MockCompanyRepo::new();
        let company_id = CompanyId::new();
        
        // Configure mock to return None (company not found)
        mock_repo
            .expect_find_by_id()
            .with(eq(company_id.clone()))
            .times(1)
            .returning(|_| Ok(None));
            
        let handler = CompanyCommandHandlerImpl::new(Arc::new(mock_repo));
        
        // Execute command
        let command = UpdateCompanyCommand {
            id: company_id,
            name: Some("Updated Name".to_string()),
            // Add other fields as needed
        };
        
        let result = handler.handle(command).await;
        
        // Verify that an error is returned
        assert!(result.is_err());
        
        Ok(())
    }
}
