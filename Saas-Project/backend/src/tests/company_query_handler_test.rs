#[cfg(test)]
mod company_query_handler_tests {
    use async_trait::async_trait;
    use mockall::predicate::*;
    use mockall::*;
    use std::sync::Arc;
    
    use crate::application::queries::{CompanyQuery, ListCompaniesQuery, GetCompanyByIdQuery};
    use crate::application::query_handlers::{CompanyQueryHandler, CompanyQueryHandlerImpl};
    use crate::domain::entities::{Company, Pagination, PaginationResult};
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
    async fn test_get_company_by_id() -> AppResult<()> {
        // Setup
        let mut mock_repo = MockCompanyRepo::new();
        let test_id = CompanyId::new();
        
        // Configure mock
        mock_repo
            .expect_find_by_id()
            .with(eq(test_id.clone()))
            .times(1)
            .returning(move |_| {
                Ok(Some(Company {
                    id: test_id.clone(),
                    name: "Test Company".to_string(),
                    // Add other fields as per your Company struct
                    // ...
                }))
            });
            
        let handler = CompanyQueryHandlerImpl::new(Arc::new(mock_repo));
        
        // Execute query
        let query = GetCompanyByIdQuery { id: test_id };
        let result = handler.handle(query).await?;
        
        // Verify
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Test Company");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_list_companies() -> AppResult<()> {
        // Setup
        let mut mock_repo = MockCompanyRepo::new();
        
        // Configure mocks
        mock_repo
            .expect_list_all()
            .with(eq(Some(10)), eq(Some(0)))
            .times(1)
            .returning(|_, _| {
                let mut companies = Vec::new();
                for i in 0..10 {
                    companies.push(Company {
                        id: CompanyId::new(),
                        name: format!("Company {}", i),
                        // Add other fields as per your Company struct
                        // ...
                    });
                }
                Ok(companies)
            });
            
        mock_repo
            .expect_count_all()
            .times(1)
            .returning(|| Ok(25)); // Total of 25 companies
        
        let handler = CompanyQueryHandlerImpl::new(Arc::new(mock_repo));
        
        // Execute query
        let query = ListCompaniesQuery {
            pagination: Pagination {
                page: 1,
                per_page: 10,
            },
        };
        
        let result = handler.handle(query).await?;
        
        // Verify
        assert_eq!(result.data.len(), 10);
        assert_eq!(result.total, 25);
        assert_eq!(result.page, 1);
        assert_eq!(result.per_page, 10);
        assert_eq!(result.total_pages, 3); // 25 items with 10 per page = 3 pages
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_company_not_found() -> AppResult<()> {
        // Setup
        let mut mock_repo = MockCompanyRepo::new();
        let test_id = CompanyId::new();
        
        // Configure mock to return None (company not found)
        mock_repo
            .expect_find_by_id()
            .with(eq(test_id.clone()))
            .times(1)
            .returning(|_| Ok(None));
            
        let handler = CompanyQueryHandlerImpl::new(Arc::new(mock_repo));
        
        // Execute query
        let query = GetCompanyByIdQuery { id: test_id };
        let result = handler.handle(query).await?;
        
        // Verify
        assert!(result.is_none());
        
        Ok(())
    }
}
