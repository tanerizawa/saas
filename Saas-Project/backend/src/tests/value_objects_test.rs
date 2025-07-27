#[cfg(test)]
mod value_objects_tests {
    use crate::domain::value_objects::*;
    use uuid::Uuid;

    #[test]
    fn test_email_validation() {
        // Valid email addresses
        assert!(Email::new("user@example.com").is_ok());
        assert!(Email::new("user.name+tag@example.co.uk").is_ok());
        assert!(Email::new("user123@subdomain.example.org").is_ok());
        
        // Invalid email addresses
        assert!(Email::new("").is_err());
        assert!(Email::new("invalid").is_err());
        assert!(Email::new("invalid@").is_err());
        assert!(Email::new("@example.com").is_err());
        assert!(Email::new("user@.com").is_err());
        assert!(Email::new("user@example").is_err());
        assert!(Email::new("user@.").is_err());
        
        // Test as_str method
        let email = Email::new("test@example.com").unwrap();
        assert_eq!(email.as_str(), "test@example.com");
        
        // Test comparison
        let email1 = Email::new("same@example.com").unwrap();
        let email2 = Email::new("same@example.com").unwrap();
        let email3 = Email::new("different@example.com").unwrap();
        
        assert_eq!(email1, email2);
        assert_ne!(email1, email3);
    }
    
    #[test]
    fn test_user_id() {
        // Create a new random ID
        let id1 = UserId::new();
        let id2 = UserId::new();
        
        // Two randomly generated IDs should be different
        assert_ne!(id1, id2);
        
        // Test display format and string conversion
        let _uuid_str = id1.to_string(); // Using the Display implementation
        
        // Test from_uuid
        let uuid = Uuid::new_v4();
        let id_from_uuid = UserId::from_uuid(uuid);
        assert_eq!(*id_from_uuid.as_uuid(), uuid);
    }
    
    #[test]
    fn test_money() {
        // Test creation with IDR currency
        let money1 = Money::idr(1000);
        assert_eq!(money1.amount, 1000);
        assert_eq!(money1.currency, Currency::IDR);
        
        // Test direct creation
        let money2 = Money::new(500, Currency::IDR);
        
        // Check properties
        assert_eq!(money2.amount, 500);
        assert_eq!(money2.currency, Currency::IDR);
    }
}
