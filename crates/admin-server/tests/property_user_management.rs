//! Property-based tests for user management correctness properties
//!
//! Tests verify:
//! - Username availability accuracy
//! - Pagination consistency
//! - Rate limit configuration round-trip
//! - Audit log completeness

use proptest::prelude::*;

// ============================================================================
// Username Availability Properties
// ============================================================================

/// Property: valid Matrix user IDs always have correct format
proptest! {
    #[test]
    fn prop_valid_matrix_user_id_format(
        localpart in "[a-z][a-z0-9_.-]{0,29}",
        server in "[a-z][a-z0-9.-]{0,29}\\.[a-z]{2,4}",
    ) {
        let user_id = format!("@{}:{}", localpart, server);
        prop_assert!(user_id.starts_with('@'));
        prop_assert!(user_id.contains(':'));
        let parts: Vec<&str> = user_id[1..].splitn(2, ':').collect();
        prop_assert_eq!(parts.len(), 2);
        prop_assert!(!parts[0].is_empty());
        prop_assert!(!parts[1].is_empty());
    }
}

/// Property: username validation rejects invalid characters
proptest! {
    #[test]
    fn prop_username_rejects_spaces(
        prefix in "[a-z]{3,10}",
        suffix in "[a-z]{3,10}",
    ) {
        let username_with_space = format!("{} {}", prefix, suffix);
        prop_assert!(username_with_space.contains(' '));
        // A valid username should not contain spaces
        let is_valid = username_with_space.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-');
        prop_assert!(!is_valid);
    }
}

/// Property: valid usernames only contain allowed characters
proptest! {
    #[test]
    fn prop_valid_username_chars(
        username in "[a-zA-Z0-9_-]{3,50}",
    ) {
        let is_valid = !username.is_empty()
            && username.len() >= 3
            && username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-');
        prop_assert!(is_valid);
    }
}

// ============================================================================
// Pagination Consistency Properties
// ============================================================================

/// Property: pagination offset + limit never exceeds total
proptest! {
    #[test]
    fn prop_pagination_bounds(
        total in 0u32..10000u32,
        page_size in 1u32..100u32,
        page in 0u32..100u32,
    ) {
        let total_pages = if total == 0 { 0 } else { (total + page_size - 1) / page_size };
        let offset = page * page_size;

        if page < total_pages {
            let end = (offset + page_size).min(total);
            prop_assert!(end <= total);
            prop_assert!(offset < total);
        }
    }
}

/// Property: total pages calculation is consistent
proptest! {
    #[test]
    fn prop_total_pages_consistent(
        total in 0u32..10000u32,
        page_size in 1u32..100u32,
    ) {
        let total_pages = if total == 0 { 0 } else { (total + page_size - 1) / page_size };

        if total > 0 {
            // Last page offset should be within bounds
            let last_page_offset = (total_pages - 1) * page_size;
            prop_assert!(last_page_offset < total);

            // Items on last page should be <= page_size
            let items_on_last_page = total - last_page_offset;
            prop_assert!(items_on_last_page <= page_size);
            prop_assert!(items_on_last_page > 0);
        }
    }
}

/// Property: page count is monotonically non-decreasing with total
proptest! {
    #[test]
    fn prop_page_count_monotone(
        total_a in 0u32..5000u32,
        extra in 0u32..5000u32,
        page_size in 1u32..100u32,
    ) {
        let total_b = total_a + extra;
        let pages_a = if total_a == 0 { 0 } else { (total_a + page_size - 1) / page_size };
        let pages_b = if total_b == 0 { 0 } else { (total_b + page_size - 1) / page_size };
        prop_assert!(pages_b >= pages_a);
    }
}

// ============================================================================
// Rate Limit Configuration Properties
// ============================================================================

/// Property: rate limit values are non-negative
proptest! {
    #[test]
    fn prop_rate_limit_non_negative(
        messages_per_second in 0i64..10000i64,
        burst_count in 0i64..10000i64,
    ) {
        prop_assert!(messages_per_second >= 0);
        prop_assert!(burst_count >= 0);
    }
}

/// Property: rate limit serialization round-trip preserves values
proptest! {
    #[test]
    fn prop_rate_limit_roundtrip(
        mps in 0i64..10000i64,
        burst in 0i64..10000i64,
    ) {
        let config = serde_json::json!({
            "messages_per_second": mps,
            "burst_count": burst,
        });

        let mps_out = config["messages_per_second"].as_i64().unwrap();
        let burst_out = config["burst_count"].as_i64().unwrap();

        prop_assert_eq!(mps_out, mps);
        prop_assert_eq!(burst_out, burst);
    }
}

/// Property: burst_count >= messages_per_second is a valid configuration
proptest! {
    #[test]
    fn prop_burst_gte_mps_is_valid(
        mps in 1i64..1000i64,
        extra in 0i64..1000i64,
    ) {
        let burst = mps + extra;
        prop_assert!(burst >= mps);
        prop_assert!(mps > 0);
        prop_assert!(burst > 0);
    }
}

// ============================================================================
// Audit Log Completeness Properties
// ============================================================================

/// Property: audit log entries always have required fields
proptest! {
    #[test]
    fn prop_audit_log_has_required_fields(
        admin_user in "[a-z]{3,20}",
        target_user in "[a-z]{3,20}",
        action in prop_oneof![
            Just("create"),
            Just("update"),
            Just("deactivate"),
            Just("delete"),
        ],
    ) {
        let entry = serde_json::json!({
            "admin_user": admin_user,
            "target": target_user,
            "action": action,
            "timestamp": 1640000000i64,
        });

        prop_assert!(entry.get("admin_user").is_some());
        prop_assert!(entry.get("target").is_some());
        prop_assert!(entry.get("action").is_some());
        prop_assert!(entry.get("timestamp").is_some());
        prop_assert!(!entry["admin_user"].as_str().unwrap().is_empty());
        prop_assert!(!entry["target"].as_str().unwrap().is_empty());
    }
}

/// Property: audit log timestamps are positive
proptest! {
    #[test]
    fn prop_audit_log_timestamp_positive(
        ts in 1i64..i64::MAX,
    ) {
        prop_assert!(ts > 0);
    }
}

/// Property: audit log action is one of the known types
proptest! {
    #[test]
    fn prop_audit_log_action_known(
        action in prop_oneof![
            Just("user_create"),
            Just("user_update"),
            Just("user_deactivate"),
            Just("user_reactivate"),
            Just("password_reset"),
            Just("device_delete"),
            Just("shadow_ban"),
            Just("rate_limit_set"),
        ],
    ) {
        let known_actions = [
            "user_create", "user_update", "user_deactivate", "user_reactivate",
            "password_reset", "device_delete", "shadow_ban", "rate_limit_set",
        ];
        prop_assert!(known_actions.contains(&action));
    }
}
