// 用户管理 API 客户端

use serde::{Deserialize, Serialize};
use crate::services::api::{ApiClient, ApiError};
use super::auth::AuthApi;

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,                    // 用户 ID (@user:server.com)
    pub displayname: Option<String>,
    #[serde(rename = "avatar_url")]
    pub avatar_url: Option<String>,
    #[serde(rename = "is_guest")]
    pub is_guest: bool,
    pub admin: bool,
    pub deactivated: bool,
    #[serde(rename = "shadow_banned")]
    pub shadow_banned: bool,
    pub locked: bool,
    #[serde(rename = "creation_ts")]
    pub creation_ts: i64,
    #[serde(rename = "appservice_id")]
    pub appservice_id: Option<String>,
    #[serde(rename = "consent_version")]
    pub consent_version: Option<String>,
    #[serde(rename = "consent_ts")]
    pub consent_ts: Option<i64>,
    #[serde(rename = "consent_server_notice_sent")]
    pub consent_server_notice_sent: Option<i64>,
    #[serde(rename = "user_type")]
    pub user_type: Option<String>,
    pub external_ids: Option<Vec<ExternalId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalId {
    #[serde(rename = "auth_provider")]
    pub auth_provider: String,
    #[serde(rename = "external_id")]
    pub external_id: String,
}

/// 用户列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListResponse {
    pub users: Vec<UserInfo>,
    pub total: i64,
    #[serde(rename = "next_token")]
    pub next_token: Option<String>,
}

/// 用户详情（包含完整信息）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDetail {
    pub name: String,
    pub displayname: Option<String>,
    #[serde(rename = "avatar_url")]
    pub avatar_url: Option<String>,
    pub threepids: Option<Vec<ThreePid>>,
    #[serde(rename = "is_guest")]
    pub is_guest: bool,
    pub admin: bool,
    pub deactivated: bool,
    #[serde(rename = "shadow_banned")]
    pub shadow_banned: bool,
    pub locked: bool,
    #[serde(rename = "creation_ts")]
    pub creation_ts: i64,
    #[serde(rename = "appservice_id")]
    pub appservice_id: Option<String>,
    #[serde(rename = "consent_version")]
    pub consent_version: Option<String>,
    #[serde(rename = "consent_ts")]
    pub consent_ts: Option<i64>,
    #[serde(rename = "consent_server_notice_sent")]
    pub consent_server_notice_sent: Option<i64>,
    #[serde(rename = "user_type")]
    pub user_type: Option<String>,
    pub external_ids: Option<Vec<ExternalId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreePid {
    pub medium: String,  // "email" or "msisdn"
    pub address: String,
    #[serde(rename = "added_at")]
    pub added_at: i64,
    #[serde(rename = "validated_at")]
    pub validated_at: Option<i64>,
}

/// 创建/更新用户请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateUserRequest {
    pub password: Option<String>,
    #[serde(rename = "logout_devices")]
    pub logout_devices: Option<bool>,
    pub displayname: Option<String>,
    #[serde(rename = "avatar_url")]
    pub avatar_url: Option<String>,
    pub threepids: Option<Vec<ThreePid>>,
    #[serde(rename = "external_ids")]
    pub external_ids: Option<Vec<ExternalId>>,
    pub admin: Option<bool>,
    pub deactivated: Option<bool>,
    pub locked: Option<bool>,
    #[serde(rename = "user_type")]
    pub user_type: Option<String>,
}

/// 重置密码请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    #[serde(rename = "new_password")]
    pub new_password: String,
    #[serde(rename = "logout_devices")]
    pub logout_devices: Option<bool>,
}

/// 速率限制配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitOverride {
    #[serde(rename = "messages_per_second")]
    pub messages_per_second: i32,
    #[serde(rename = "burst_count")]
    pub burst_count: i32,
}

/// 用户加入的房间响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinedRoomsResponse {
    #[serde(rename = "joined_rooms")]
    pub joined_rooms: Vec<String>,
    pub total: i32,
}

/// Pusher（推送规则）信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pusher {
    #[serde(rename = "app_display_name")]
    pub app_display_name: String,
    #[serde(rename = "app_id")]
    pub app_id: String,
    #[serde(rename = "data")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "device_display_name")]
    pub device_display_name: Option<String>,
    #[serde(rename = "device_id")]
    pub device_id: Option<String>,
    pub kind: String,
    pub lang: Option<String>,
    pub profile_tag: Option<String>,
    pub pushkey: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushersResponse {
    pub pushers: Vec<Pusher>,
    pub total: i32,
}

/// 账户数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountData {
    pub global: Option<serde_json::Value>,
    pub rooms: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// 用户 API 客户端
pub struct UsersApi {
    client: ApiClient,
}

impl UsersApi {
    /// 创建新的用户 API 客户端
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }
    
    /// 获取用户列表
    pub async fn list_users(&self, params: &UserListParams) -> Result<UserListResponse, ApiError> {
        let mut path = format!("/_synapse/admin/v2/users?limit={}", params.limit);
        
        if let Some(from) = &params.from {
            path.push_str(&format!("&from={}", from));
        }
        if let Some(user_id) = &params.user_id {
            path.push_str(&format!("&user_id={}", user_id));
        }
        if let Some(name) = &params.name {
            path.push_str(&format!("&name={}", name));
        }
        if let Some(guests) = params.guests {
            path.push_str(&format!("&guests={}", guests));
        }
        if let Some(deactivated) = params.deactivated {
            path.push_str(&format!("&deactivated={}", deactivated));
        }
        if let Some(admins) = params.admins {
            path.push_str(&format!("&admins={}", admins));
        }
        if let Some(order_by) = &params.order_by {
            path.push_str(&format!("&order_by={}", order_by));
        }
        if let Some(dir) = &params.dir {
            path.push_str(&format!("&dir={}", dir));
        }
        
        self.client.get(&path).await
    }
    
    /// 获取单个用户信息
    pub async fn get_user(&self, user_id: &str) -> Result<UserDetail, ApiError> {
        let path = format!("/_synapse/admin/v2/users/{}", urlencoding::encode(user_id));
        self.client.get(&path).await
    }
    
    /// 创建或更新用户
    pub async fn create_or_update_user(
        &self,
        user_id: &str,
        request: &CreateUserRequest,
    ) -> Result<UserDetail, ApiError> {
        let path = format!("/_synapse/admin/v2/users/{}", urlencoding::encode(user_id));
        self.client.put(&path, request).await
    }
    
    /// 停用用户
    pub async fn deactivate_user(&self, user_id: &str, erase: bool) -> Result<(), ApiError> {
        let path = format!("/_synapse/admin/v1/deactivate/{}", urlencoding::encode(user_id));
        let body = serde_json::json!({ "erase": erase });
        let _ = self.client.post::<serde_json::Value, _>(&path, &body).await?;
        Ok(())
    }
    
    /// 重置用户密码
    pub async fn reset_password(
        &self,
        user_id: &str,
        new_password: String,
        logout_devices: bool,
    ) -> Result<(), ApiError> {
        let path = format!("/_synapse/admin/v1/reset_password/{}", urlencoding::encode(user_id));
        let request = ResetPasswordRequest {
            new_password,
            logout_devices: Some(logout_devices),
        };
        let _ = self.client.post::<serde_json::Value, _>(&path, &request).await?;
        Ok(())
    }
    
    /// 设置用户管理员状态
    pub async fn set_admin(&self, user_id: &str, admin: bool) -> Result<(), ApiError> {
        let path = format!("/_synapse/admin/v1/users/{}/admin", urlencoding::encode(user_id));
        let body = serde_json::json!({ "admin": admin });
        let _ = self.client.put::<serde_json::Value, _>(&path, &body).await?;
        Ok(())
    }
    
    /// 影子封禁用户
    pub async fn shadow_ban(&self, user_id: &str) -> Result<(), ApiError> {
        let path = format!("/_synapse/admin/v1/users/{}/shadow_ban", urlencoding::encode(user_id));
        let _ = self.client.post::<serde_json::Value, _>(&path, &serde_json::Value::Null).await?;
        Ok(())
    }
    
    /// 解除影子封禁
    pub async fn unshadow_ban(&self, user_id: &str) -> Result<(), ApiError> {
        let path = format!("/_synapse/admin/v1/users/{}/shadow_ban", urlencoding::encode(user_id));
        let _ = self.client.delete::<serde_json::Value>(&path).await?;
        Ok(())
    }
    
    /// 暂停或恢复用户
    pub async fn suspend_user(&self, user_id: &str, suspend: bool) -> Result<(), ApiError> {
        let path = format!("/_synapse/admin/v1/suspend/{}", urlencoding::encode(user_id));
        let body = serde_json::json!({ "suspend": suspend });
        let _ = self.client.put::<serde_json::Value, _>(&path, &body).await?;
        Ok(())
    }
    
    /// 获取用户会话信息
    pub async fn get_whois(&self, user_id: &str) -> Result<WhoisResponse, ApiError> {
        let path = format!("/_synapse/admin/v1/whois/{}", urlencoding::encode(user_id));
        self.client.get(&path).await
    }
    
    /// 获取用户加入的房间
    pub async fn get_joined_rooms(&self, user_id: &str) -> Result<JoinedRoomsResponse, ApiError> {
        let path = format!("/_synapse/admin/v1/users/{}/joined_rooms", urlencoding::encode(user_id));
        self.client.get(&path).await
    }
    
    /// 获取用户的 pushers
    pub async fn get_pushers(&self, user_id: &str) -> Result<PushersResponse, ApiError> {
        let path = format!("/_synapse/admin/v1/users/{}/pushers", urlencoding::encode(user_id));
        self.client.get(&path).await
    }
    
    /// 获取用户账户数据
    pub async fn get_account_data(&self, user_id: &str) -> Result<AccountData, ApiError> {
        let path = format!("/_synapse/admin/v1/users/{}/accountdata", urlencoding::encode(user_id));
        self.client.get(&path).await
    }
    
    /// 获取速率限制配置
    pub async fn get_rate_limit(&self, user_id: &str) -> Result<RateLimitOverride, ApiError> {
        let path = format!("/_synapse/admin/v1/users/{}/override_ratelimit", urlencoding::encode(user_id));
        self.client.get(&path).await
    }
    
    /// 设置速率限制
    pub async fn set_rate_limit(
        &self,
        user_id: &str,
        config: RateLimitOverride,
    ) -> Result<RateLimitOverride, ApiError> {
        let path = format!("/_synapse/admin/v1/users/{}/override_ratelimit", urlencoding::encode(user_id));
        self.client.post(&path, &config).await
    }
    
    /// 删除速率限制覆盖
    pub async fn delete_rate_limit(&self, user_id: &str) -> Result<(), ApiError> {
        let path = format!("/_synapse/admin/v1/users/{}/override_ratelimit", urlencoding::encode(user_id));
        let _ = self.client.delete::<serde_json::Value>(&path).await?;
        Ok(())
    }
}

/// 用户列表查询参数
#[derive(Debug, Clone)]
pub struct UserListParams {
    pub from: Option<i64>,
    pub limit: i64,
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub guests: Option<bool>,
    pub deactivated: Option<bool>,
    pub admins: Option<bool>,
    pub order_by: Option<String>,
    pub dir: Option<String>,
}

impl Default for UserListParams {
    fn default() -> Self {
        Self {
            from: None,
            limit: 100,
            user_id: None,
            name: None,
            guests: Some(true),
            deactivated: Some(false),
            admins: None,
            order_by: None,
            dir: None,
        }
    }
}

/// 用户会话信息（whois 响应）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhoisResponse {
    pub user_id: String,
    pub devices: std::collections::HashMap<String, DeviceSessions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceSessions {
    pub sessions: Vec<SessionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub connections: Vec<ConnectionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub ip: String,
    #[serde(rename = "last_seen")]
    pub last_seen: i64,
    #[serde(rename = "user_agent")]
    pub user_agent: String,
}
