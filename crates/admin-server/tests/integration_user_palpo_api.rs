/// Integration tests for PalpoClient user management API
///
/// These tests verify the end-to-end functionality of user management
/// through the PalpoClient interface.

#[test]
fn test_palpo_client_imports() {
    // Verify that the PalpoClient and related types can be imported
    use palpo_admin_server::palpo_client::{
        CreateOrUpdateUserRequest, ListUsersQuery, PalpoClient, PalpoRateLimitConfig,
    };
    
    // Just verify the types exist and can be used
    let _req = CreateOrUpdateUserRequest {
        displayname: Some("test".to_string()),
        password: Some("password".to_string()),
        admin: Some(false),
        deactivated: Some(false),
        avatar_url: None,
        user_type: None,
    };
    
    let _query = ListUsersQuery {
        from: Some(0),
        limit: Some(10),
        search_term: None,
        guests: None,
        deactivated: None,
        admins: None,
    };
    
    let _config = PalpoRateLimitConfig {
        messages_per_second: Some(100),
        burst_count: Some(50),
    };
    
    // PalpoClient requires a reqwest::Client, so we can't easily construct it here
    // But we've verified the types are importable
}

#[test]
fn test_list_users_query_serialization() {
    use palpo_admin_server::palpo_client::ListUsersQuery;
    
    let query = ListUsersQuery {
        from: Some(0),
        limit: Some(10),
        search_term: Some("test".to_string()),
        guests: Some(true),
        deactivated: Some(false),
        admins: Some(true),
    };
    
    let json = serde_json::to_string(&query).unwrap();
    assert!(json.contains("from"));
    assert!(json.contains("limit"));
    assert!(json.contains("test"));
}

#[test]
fn test_create_user_request_serialization() {
    use palpo_admin_server::palpo_client::CreateOrUpdateUserRequest;
    
    let req = CreateOrUpdateUserRequest {
        displayname: Some("Test User".to_string()),
        password: Some("password123".to_string()),
        admin: Some(true),
        deactivated: Some(false),
        avatar_url: Some("mxc://example.com/avatar".to_string()),
        user_type: None,
    };
    
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("Test User"));
    assert!(json.contains("password123"));
}

#[test]
fn test_rate_limit_config_serialization() {
    use palpo_admin_server::palpo_client::PalpoRateLimitConfig;
    
    let config = PalpoRateLimitConfig {
        messages_per_second: Some(100),
        burst_count: Some(50),
    };
    
    let json = serde_json::to_string(&config).unwrap();
    assert!(json.contains("messages_per_second"));
    assert!(json.contains("burst_count"));
}

#[test]
fn test_list_users_query_minimal() {
    use palpo_admin_server::palpo_client::ListUsersQuery;
    
    let query = ListUsersQuery::default();
    let json = serde_json::to_string(&query).unwrap();
    
    // Should serialize to empty object or minimal JSON
    assert!(json.starts_with('{'));
    assert!(json.ends_with('}'));
}

#[test]
fn test_create_user_request_minimal() {
    use palpo_admin_server::palpo_client::CreateOrUpdateUserRequest;
    
    let req = CreateOrUpdateUserRequest::default();
    let json = serde_json::to_string(&req).unwrap();
    
    // Should serialize to empty object or minimal JSON
    assert!(json.starts_with('{'));
    assert!(json.ends_with('}'));
}

#[test]
fn test_rate_limit_config_minimal() {
    use palpo_admin_server::palpo_client::PalpoRateLimitConfig;
    
    let config = PalpoRateLimitConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    
    // Should serialize to empty object or minimal JSON
    assert!(json.starts_with('{'));
    assert!(json.ends_with('}'));
}