// 认证 API 客户端

use serde::{Deserialize, Serialize};
use crate::services::api::{ApiClient, ApiError};

/// 登录请求类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    #[serde(rename = "type")]
    pub login_type: String,
    pub identifier: Identifier,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identifier {
    #[serde(rename = "type")]
    pub id_type: String,
    pub user: String,
}

impl LoginRequest {
    /// 创建密码登录请求
    pub fn password(user: String, password: String) -> Self {
        Self {
            login_type: "m.login.password".to_string(),
            identifier: Identifier {
                id_type: "m.id.user".to_string(),
                user,
            },
            password,
            device_id: None,
        }
    }
}

/// 登录响应类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub access_token: String,
    pub device_id: Option<String>,
    pub well_known: Option<WellKnown>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellKnown {
    #[serde(rename = "m.homeserver")]
    pub homeserver: HomeserverInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeserverInfo {
    pub base_url: String,
}

/// 认证 API 客户端
pub struct AuthApi {
    client: ApiClient,
}

impl AuthApi {
    /// 创建新的认证 API 客户端
    pub fn new(base_url: String) -> Self {
        Self {
            client: ApiClient::new(base_url),
        }
    }
    
    /// 登录
    pub async fn login(&self, request: &LoginRequest) -> Result<LoginResponse, ApiError> {
        self.client.post("/_matrix/client/v3/login", request).await
    }
    
    /// 验证 token 是否有效
    pub async fn validate_token(&self, token: &str) -> Result<bool, ApiError> {
        let mut client = self.client.clone();
        client.set_token(Some(token.to_string()));
        
        // 调用一个需要认证的简单 API 来验证 token
        match client.get::<serde_json::Value>("/_synapse/admin/v1/server_version").await {
            Ok(_) => Ok(true),
            Err(ApiError::Status(401, _)) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_login_request_creation() {
        let req = LoginRequest::password("@user:server.com".to_string(), "password123".to_string());
        assert_eq!(req.login_type, "m.login.password");
        assert_eq!(req.identifier.user, "@user:server.com");
        assert_eq!(req.password, "password123");
    }
}
