````markdown
# Backend Task Plan

This document outlines the project phases, task plans and current implementation status for the SaaS UMKM backend.

## Project Phases Overview

- ✅ **Phase 1**: Initial Setup & Core Infrastructure
- ✅ **Phase 2**: User Management & Authentication
- ✅ **Phase 3**: License Management
- ✅ **Phase 4**: Performance Optimization
- ✅ **Phase 5**: File Management
- ✅ **Phase 6**: Financial Management
- � **Phase 7**: Reporting & Analytics (Next Phase)
- 🔲 **Phase 8**: Multi-tenancy Enhancements

## PHASE 7 - REPORTING & ANALYTICS TASKS

### New Implementation Tasks

1. **Dashboard & Data Visualization**

   - Create endpoints for dashboard data aggregation
   - Implement time-series data analysis for financial trends
   - Add summary statistics endpoints for key business metrics

2. **Advanced Financial Reporting**

   - Implement detailed financial reports with filtering options
   - Create income/expense analysis by categories and periods
   - Add cash flow and balance sheet reporting capabilities

3. **Data Export Functionality**

   - Implement PDF export using wkhtmltopdf integration
   - Add Excel/CSV export for transaction data
   - Create scheduled report generation

4. **Notification System**

   - Implement event-based notification system for important financial events
   - Add email notifications using lettre or external API
   - Create in-app notification center with WebSocket support

### Ongoing Fix Tasks

1. **Fix LicenseRepository Implementation**

   - Complete implementation of all required methods in `PostgresLicenseRepository`
   - Fix mapping of database fields to struct fields in `sqlx::query_as!` macros
   - Resolve the issue with the `business_id` field not found in License struct

2. **Fix Middleware Issues**

   - Either implement the missing rate limiter module or remove references to it
   - Update middleware imports and usages in `main.rs`

3. **Service Layer Configuration**
   - Fix the `cache_service` not found in scope in `main.rs`
   - Update service builder code to properly handle different middleware layers

## Implementation Notes for Phase 7

### Dashboard Data Example

```rust
// In reporting_service.rs
impl ReportingService {
    async fn generate_dashboard_data(&self, user_id: Uuid) -> Result<DashboardData, AppError> {
        let now = Utc::now();
        let month_start = now.with_day(1).unwrap().with_hour(0).unwrap()
            .with_minute(0).unwrap().with_second(0).unwrap();
        let prev_month_start = month_start.checked_sub_months(1).unwrap();
        
        // Get current month data
        let current_month = self.aggregate_financial_data(user_id, month_start, now).await?;
        
        // Get previous month data for comparison
        let prev_month = self.aggregate_financial_data(
            user_id, prev_month_start, month_start.checked_sub_days(1).unwrap()
        ).await?;
        
        // Calculate trends
        let income_trend = calculate_percentage_change(
            prev_month.total_income, current_month.total_income
        );
        let expense_trend = calculate_percentage_change(
            prev_month.total_expenses, current_month.total_expenses
        );
        
        Ok(DashboardData {
            period: format!("{}", now.format("%B %Y")),
            total_income: current_month.total_income,
            total_expenses: current_month.total_expenses,
            net_profit: current_month.total_income - current_month.total_expenses,
            income_trend,
            expense_trend,
            categories: current_month.categories,
            daily_transactions: self.get_daily_transaction_counts(user_id, month_start, now).await?,
        })
    }
}
```

### PDF Export Example

```rust
async fn export_financial_report_pdf(
    user_id: Uuid, 
    start_date: DateTime<Utc>, 
    end_date: DateTime<Utc>
) -> Result<Vec<u8>, AppError> {
    // Generate report HTML
    let transactions = fetch_transactions(user_id, start_date, end_date).await?;
    let accounts = fetch_accounts(user_id).await?;
    let summary = calculate_summary(&transactions, &accounts);
    
    // Generate HTML using handlebars or similar
    let html = render_report_template(
        "financial_report", 
        &json!({
            "title": "Financial Report",
            "period": format!("{} - {}", 
                      start_date.format("%b %d, %Y"),
                      end_date.format("%b %d, %Y")),
            "summary": summary,
            "transactions": transactions,
            "accounts": accounts,
            "generated_at": Utc::now().format("%b %d, %Y %H:%M:%S").to_string(),
        })
    )?;
    
    // Convert HTML to PDF
    let pdf_bytes = html_to_pdf(&html)?;
    
    Ok(pdf_bytes)
}
```

## Testing Approach

1. Run `cargo check` to verify compilation succeeds
2. Run `cargo test` to verify tests pass
3. Run `cargo run --bin migrate` to verify database migrations work
4. Run `cargo run --bin server` to start the server and test API endpoints

## Completed Phases

### Phase 6 - Financial Management

✅ Implemented comprehensive financial management system with:
- Transaction and Account models in domain layer
- FinancialRepository with PostgreSQL implementation and caching
- API endpoints for transaction and account management
- Financial reporting and summary endpoints
- Database migrations with triggers for automatic balance updates
- Proper validation and security for all financial operations

### Phase 5 - File Management

✅ Implemented comprehensive file handling with:
- Secure file upload with validation
- Streaming file download
- Repository pattern for file metadata
- Database migrations for files table
- Integration with authentication middleware
