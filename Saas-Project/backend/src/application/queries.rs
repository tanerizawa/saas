#![allow(dead_code)]

use uuid::Uuid;

pub struct GetUserQuery {
    pub user_id: Uuid,
}

impl GetUserQuery {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ListLicensesQuery;

impl ListLicensesQuery {
    pub fn new() -> Self {
        Self
    }
}

// Example usage to avoid "never constructed" warning
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_licenses_query_construction() {
        let _query = ListLicensesQuery::new();
    }
}
