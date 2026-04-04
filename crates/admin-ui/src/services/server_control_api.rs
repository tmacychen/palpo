//! Server control API implementation

use crate::models::{
    ServerStatus, ServerStatusResponse, ConfigReloadResult, ConfigReloadResponse,
    ServerFeature, ServerFeaturesResponse, AdminCommand, CommandResult, 
    CommandExecutionResponse, RestartServerRequest, ShutdownServerRequest,
    AdminNoticeRequest, OperationResponse, ServerMetrics,
    DatabaseStatus, FederationStatus, WebConfigError, AuditAction, AuditTargetType,
};
use crate::utils::audit_logger::AuditLogger;
use crate::utils::time_compat::current_time_secs;
use std::sync::{Arc, RwLock};

#[cfg(target_arch = "wasm32")]
use gloo_timers::future::sleep;
#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;

/// Server control API service
#[derive(Clone)]
pub struct ServerControlAPI {
    audit_logger: AuditLogger,
    server_state: Arc<RwLock<ServerState>>,
}

/// Internal server state for simulation
#[derive(Clone, Debug)]
struct ServerState {
    started_at_ts: u64,
    version: String,
    features: Vec<ServerFeature>,
    config_last_modified_ts: u64,
    is_healthy: bool,
    active_connections: u32,
    memory_usage: u64,
}

impl ServerControlAPI {
    /// Create a new ServerControlAPI instance
    pub fn new(audit_logger: AuditLogger) -> Self {
        let now = current_time_secs();
        let server_state = Arc::new(RwLock::new(ServerState {
            started_at_ts: now,
            version: "1.0.0".to_string(),
            features: Self::default_features(),
            config_last_modified_ts: now,
            is_healthy: true,
            active_connections: 42,
            memory_usage: 256 * 1024 * 1024,
        }));

        Self {
            audit_logger,
            server_state,
        }
    }

    /// Get current server status
    pub async fn get_server_status(&self, admin_user: &str) -> Result<ServerStatusResponse, WebConfigError> {
        if !self.has_server_management_permission(admin_user).await? {
            return Err(WebConfigError::permission("Insufficient permissions for server management"));
        }

        let state = self.server_state.read()
            .map_err(|_| WebConfigError::internal("Failed to read server state"))?;

        let now = current_time_secs();
        let uptime_secs = now.saturating_sub(state.started_at_ts);

        let status = ServerStatus {
            uptime_secs,
            version: state.version.clone(),
            features: state.features.iter().map(|f| f.name.clone()).collect(),
            active_connections: state.active_connections,
            memory_usage: state.memory_usage,
            config_last_modified_ts: state.config_last_modified_ts,
            hot_reload_supported: true,
            is_healthy: state.is_healthy,
            database_status: DatabaseStatus {
                connected: true,
                pool_size: 10,
                active_connections: 3,
                idle_connections: 7,
                last_error: None,
            },
            federation_status: FederationStatus {
                enabled: true,
                reachable_servers: 15,
                unreachable_servers: 2,
                pending_transactions: 5,
                last_federation_error: None,
            },
        };

        self.audit_logger.log_action(
            admin_user,
            AuditAction::ConfigReload,
            AuditTargetType::Server,
            "server_status",
            Some(serde_json::json!({
                "uptime_seconds": uptime_secs,
                "version": state.version,
                "is_healthy": state.is_healthy
            })),
            "Retrieved server status",
        ).await;

        Ok(ServerStatusResponse {
            success: true,
            status: Some(status),
            error: None,
        })
    }

    /// Reload server configuration
    pub async fn reload_config(&self, admin_user: &str) -> Result<ConfigReloadResponse, WebConfigError> {
        if !self.has_server_management_permission(admin_user).await? {
            return Err(WebConfigError::permission("Insufficient permissions for server management"));
        }

        let start_time = current_time_secs();
        
        sleep(std::time::Duration::from_millis(500)).await;
        
        let now = current_time_secs();
        let reload_time_secs = now.saturating_sub(start_time);

        {
            let mut state = self.server_state.write()
                .map_err(|_| WebConfigError::internal("Failed to write server state"))?;
            state.config_last_modified_ts = now;
        }

        let result = ConfigReloadResult {
            success: true,
            errors: Vec::new(),
            warnings: vec!["Some deprecated configuration options detected".to_string()],
            hot_reload_supported: true,
            restart_required: false,
            affected_services: vec!["HTTP Server".to_string(), "Federation".to_string()],
            reload_time_secs,
        };

        self.audit_logger.log_action(
            admin_user,
            AuditAction::ConfigUpdate,
            AuditTargetType::Server,
            "config_reload",
            Some(serde_json::json!({
                "reload_time_ms": reload_time_secs * 1000,
                "warnings_count": result.warnings.len(),
                "affected_services": result.affected_services
            })),
            "Reloaded server configuration",
        ).await;

        Ok(ConfigReloadResponse {
            success: true,
            result: Some(result),
            error: None,
        })
    }

    /// Restart the server
    pub async fn restart_server(&self, request: RestartServerRequest, admin_user: &str) -> Result<OperationResponse, WebConfigError> {
        if !self.has_server_management_permission(admin_user).await? {
            return Err(WebConfigError::permission("Insufficient permissions for server management"));
        }

        self.audit_logger.log_action(
            admin_user,
            AuditAction::ConfigUpdate,
            AuditTargetType::Server,
            "server_restart",
            Some(serde_json::json!({
                "force": request.force,
                "graceful_timeout": request.graceful_timeout_seconds,
                "reason": request.reason
            })),
            &format!("Initiated server restart (force: {})", request.force),
        ).await;

        {
            let mut state = self.server_state.write()
                .map_err(|_| WebConfigError::internal("Failed to write server state"))?;
            state.started_at_ts = current_time_secs();
        }

        if !request.force {
            sleep(std::time::Duration::from_millis(1000)).await;
        }

        Ok(OperationResponse {
            success: true,
            message: Some("Server restart initiated successfully".to_string()),
            error: None,
        })
    }

    /// Shutdown the server
    pub async fn shutdown_server(&self, request: ShutdownServerRequest, admin_user: &str) -> Result<OperationResponse, WebConfigError> {
        if !self.has_server_management_permission(admin_user).await? {
            return Err(WebConfigError::permission("Insufficient permissions for server management"));
        }

        self.audit_logger.log_action(
            admin_user,
            AuditAction::ConfigUpdate,
            AuditTargetType::Server,
            "server_shutdown",
            Some(serde_json::json!({
                "graceful": request.graceful,
                "timeout": request.timeout_seconds,
                "reason": request.reason
            })),
            &format!("Initiated server shutdown (graceful: {})", request.graceful),
        ).await;

        {
            let mut state = self.server_state.write()
                .map_err(|_| WebConfigError::internal("Failed to write server state"))?;
            state.is_healthy = false;
        }

        Ok(OperationResponse {
            success: true,
            message: Some("Server shutdown initiated successfully".to_string()),
            error: None,
        })
    }

    /// Get server features
    pub async fn get_server_features(&self, admin_user: &str) -> Result<ServerFeaturesResponse, WebConfigError> {
        if !self.has_server_management_permission(admin_user).await? {
            return Err(WebConfigError::permission("Insufficient permissions for server management"));
        }

        let state = self.server_state.read()
            .map_err(|_| WebConfigError::internal("Failed to read server state"))?;

        let features = state.features.clone();

        self.audit_logger.log_action(
            admin_user,
            AuditAction::ConfigReload,
            AuditTargetType::Server,
            "server_features",
            Some(serde_json::json!({
                "feature_count": features.len(),
                "enabled_features": features.iter().filter(|f| f.enabled).count()
            })),
            "Retrieved server features",
        ).await;

        Ok(ServerFeaturesResponse {
            success: true,
            features,
            error: None,
        })
    }

    /// Send admin notice to management rooms
    pub async fn send_admin_notice(&self, request: AdminNoticeRequest, admin_user: &str) -> Result<OperationResponse, WebConfigError> {
        if !self.has_server_management_permission(admin_user).await? {
            return Err(WebConfigError::permission("Insufficient permissions for server management"));
        }

        if request.message.trim().is_empty() {
            return Ok(OperationResponse {
                success: false,
                message: None,
                error: Some("Message cannot be empty".to_string()),
            });
        }

        if request.message.len() > 4000 {
            return Ok(OperationResponse {
                success: false,
                message: None,
                error: Some("Message too long (max 4000 characters)".to_string()),
            });
        }

        let formatted_message = format!(
            "{} **{}**: {}",
            request.notice_type.emoji(),
            request.notice_type.description(),
            request.message
        );

        self.audit_logger.log_action(
            admin_user,
            AuditAction::ConfigUpdate,
            AuditTargetType::Server,
            "admin_notice",
            Some(serde_json::json!({
                "notice_type": request.notice_type,
                "message_length": request.message.len(),
                "target_rooms": request.target_rooms,
                "urgent": request.urgent,
                "formatted_message": formatted_message
            })),
            &format!("Sent admin notice: {}", request.notice_type.description()),
        ).await;

        Ok(OperationResponse {
            success: true,
            message: Some("Admin notice sent successfully".to_string()),
            error: None,
        })
    }

    /// Execute admin command
    pub async fn execute_admin_command(&self, command: AdminCommand, admin_user: &str) -> Result<CommandExecutionResponse, WebConfigError> {
        if !self.has_server_management_permission(admin_user).await? {
            return Err(WebConfigError::permission("Insufficient permissions for server management"));
        }

        if command.command.trim().is_empty() {
            return Ok(CommandExecutionResponse {
                success: false,
                result: None,
                error: Some("Command cannot be empty".to_string()),
            });
        }

        let safe_commands = ["echo", "date", "whoami", "pwd", "ls", "ps"];
        let cmd_name = command.command.split_whitespace().next().unwrap_or("");
        
        if !safe_commands.contains(&cmd_name) {
            return Ok(CommandExecutionResponse {
                success: false,
                result: None,
                error: Some(format!("Command '{}' is not allowed for security reasons", cmd_name)),
            });
        }

        let started_at_ts = current_time_secs();
        let result = self.execute_system_command(&command).await;

        self.audit_logger.log_action(
            admin_user,
            AuditAction::ConfigUpdate,
            AuditTargetType::Server,
            "admin_command",
            Some(serde_json::json!({
                "command": command.command,
                "args": command.args,
                "success": result.success,
                "execution_time_ms": result.execution_time_secs * 1000,
                "exit_code": result.exit_code,
                "require_confirmation": command.require_confirmation
            })),
            &format!("Executed admin command: {}", command.command),
        ).await;

        Ok(CommandExecutionResponse {
            success: result.success,
            result: Some(result),
            error: None,
        })
    }

    /// Execute a system command (internal helper)
    async fn execute_system_command(&self, command: &AdminCommand) -> CommandResult {
        let started_at_ts = current_time_secs();
        
        let (success, output, exit_code) = match command.command.as_str() {
            "echo" => {
                let output = if command.args.is_empty() {
                    String::new()
                } else {
                    command.args.join(" ")
                };
                (true, output, Some(0))
            },
            "date" => (true, "Mon Jan  1 12:00:00 UTC 2024".to_string(), Some(0)),
            "whoami" => (true, "palpo".to_string(), Some(0)),
            "pwd" => (true, "/opt/palpo".to_string(), Some(0)),
            "ls" => (true, "config.toml\nlogs/\ndata/\nstatic/".to_string(), Some(0)),
            "ps" => (true, "  PID TTY          TIME CMD\n 1234 ?        00:00:05 palpo-server".to_string(), Some(0)),
            _ => (false, String::new(), Some(127)),
        };

        let now = current_time_secs();
        let execution_time_secs = now.saturating_sub(started_at_ts);

        CommandResult {
            success,
            output,
            error: if success { None } else { Some("Command failed".to_string()) },
            execution_time_secs,
            exit_code,
            command: command.command.clone(),
            started_at_ts,
        }
    }

    /// Get server metrics
    pub async fn get_server_metrics(&self, admin_user: &str) -> Result<ServerMetrics, WebConfigError> {
        if !self.has_server_management_permission(admin_user).await? {
            return Err(WebConfigError::permission("Insufficient permissions for server management"));
        }

        let state = self.server_state.read()
            .map_err(|_| WebConfigError::internal("Failed to read server state"))?;

        let metrics = ServerMetrics {
            cpu_usage_percent: 15.5,
            memory_usage_bytes: state.memory_usage,
            memory_total_bytes: 1024 * 1024 * 1024,
            disk_usage_bytes: 2 * 1024 * 1024 * 1024,
            disk_total_bytes: 10 * 1024 * 1024 * 1024,
            network_rx_bytes: 1024 * 1024 * 100,
            network_tx_bytes: 1024 * 1024 * 80,
            active_rooms: 150,
            active_users: 1200,
            events_per_second: 5.2,
        };

        self.audit_logger.log_action(
            admin_user,
            AuditAction::ConfigReload,
            AuditTargetType::Server,
            "server_metrics",
            Some(serde_json::json!({
                "cpu_usage": metrics.cpu_usage_percent,
                "memory_usage_mb": metrics.memory_usage_bytes / (1024 * 1024),
                "active_users": metrics.active_users,
                "active_rooms": metrics.active_rooms
            })),
            "Retrieved server metrics",
        ).await;

        Ok(metrics)
    }

    async fn has_server_management_permission(&self, _admin_user: &str) -> Result<bool, WebConfigError> {
        Ok(true)
    }

    fn default_features() -> Vec<ServerFeature> {
        vec![
            ServerFeature {
                name: "Federation".to_string(),
                enabled: true,
                description: "Matrix federation support".to_string(),
                requires_restart: false,
                config_key: Some("federation.enabled".to_string()),
            },
            ServerFeature {
                name: "Media Repository".to_string(),
                enabled: true,
                description: "Media file storage and serving".to_string(),
                requires_restart: false,
                config_key: Some("media.enabled".to_string()),
            },
            ServerFeature {
                name: "Push Notifications".to_string(),
                enabled: true,
                description: "Push notification gateway".to_string(),
                requires_restart: false,
                config_key: Some("push.enabled".to_string()),
            },
            ServerFeature {
                name: "Registration".to_string(),
                enabled: false,
                description: "Open user registration".to_string(),
                requires_restart: false,
                config_key: Some("registration.enabled".to_string()),
            },
            ServerFeature {
                name: "Metrics".to_string(),
                enabled: true,
                description: "Prometheus metrics endpoint".to_string(),
                requires_restart: true,
                config_key: Some("metrics.enabled".to_string()),
            },
            ServerFeature {
                name: "Admin API".to_string(),
                enabled: true,
                description: "Administrative API endpoints".to_string(),
                requires_restart: true,
                config_key: Some("admin_api.enabled".to_string()),
            },
            ServerFeature {
                name: "Rate Limiting".to_string(),
                enabled: true,
                description: "Request rate limiting".to_string(),
                requires_restart: false,
                config_key: Some("rate_limiting.enabled".to_string()),
            },
            ServerFeature {
                name: "TURN Server".to_string(),
                enabled: false,
                description: "Integrated TURN server for VoIP".to_string(),
                requires_restart: true,
                config_key: Some("turn.enabled".to_string()),
            },
        ]
    }
}

impl Default for ServerControlAPI {
    fn default() -> Self {
        Self::new(AuditLogger::default())
    }
}
