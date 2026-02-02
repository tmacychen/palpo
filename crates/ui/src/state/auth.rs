// 认证状态管理

use leptos::*;
use serde::{Deserialize, Serialize};
use crate::services::auth::{LoginResponse, AuthApi, LoginRequest};
use crate::services::api::ApiError;
use gloo_storage::{LocalStorage, Storage};

/// 认证令牌存储键
const AUTH_TOKEN_KEY: &str = "palpo_admin_token";
const USER_ID_KEY: &str = "palpo_admin_user_id";

/// 认证状态
#[derive(Debug, Clone, PartialEq)]
pub enum AuthState {
    /// 未认证
    Unauthenticated,
    /// 已认证
    Authenticated { token: String, user_id: String },
    /// 认证中
    Loading,
    /// 认证错误
    Error(String),
}

impl AuthState {
    /// 判断是否已认证
    pub fn is_authenticated(&self) -> bool {
        matches!(self, AuthState::Authenticated { .. })
    }
    
    /// 获取 token
    pub fn token(&self) -> Option<&str> {
        match self {
            AuthState::Authenticated { token, .. } => Some(token),
            _ => None,
        }
    }
    
    /// 获取用户 ID
    pub fn user_id(&self) -> Option<&str> {
        match self {
            AuthState::Authenticated { user_id, .. } => Some(user_id),
            _ => None,
        }
    }
}

/// 认证状态上下文
#[derive(Clone)]
pub struct AuthContext {
    /// 当前状态
    pub state: RwSignal<AuthState>,
    /// 服务器地址
    pub server_url: RwSignal<String>,
}

impl AuthContext {
    /// 创建新的认证上下文
    pub fn new() -> Self {
        // 从 localStorage 加载已保存的认证信息
        let (auth_state, server_url) = Self::load_from_storage();
        
        Self {
            state: create_rw_signal(auth_state),
            server_url: create_rw_signal(server_url),
        }
    }
    
    /// 从 localStorage 加载认证信息
    fn load_from_storage() -> (AuthState, String) {
        // 加载服务器地址（默认使用当前域名）
        let server_url = LocalStorage::get::<Result<String, _>>("palpo_admin_server_url")
            .unwrap_or_else(|_| Ok(format!("{}", web_sys::window()
                .and_then(|w| w.location().origin().ok())
                .unwrap_or_else(|| "http://localhost:8008".to_string())))
            .unwrap_or_else(|_| "http://localhost:8008".to_string());
        
        // 加载 token
        match (
            LocalStorage::get::<Result<String, _>>(AUTH_TOKEN_KEY),
            LocalStorage::get::<Result<String, _>>(USER_ID_KEY),
        ) {
            (Ok(Ok(token)), Ok(Ok(user_id))) => {
                if !token.is_empty() && !user_id.is_empty() {
                    (AuthState::Authenticated { token, user_id }, server_url)
                } else {
                    (AuthState::Unauthenticated, server_url)
                }
            }
            _ => (AuthState::Unauthenticated, server_url),
        }
    }
    
    /// 保存认证信息到 localStorage
    fn save_to_storage(token: &str, user_id: &str, server_url: &str) {
        let _ = LocalStorage::set(AUTH_TOKEN_KEY, token);
        let _ = LocalStorage::set(USER_ID_KEY, user_id);
        let _ = LocalStorage::set("palpo_admin_server_url", server_url);
    }
    
    /// 清除 localStorage 中的认证信息
    fn clear_storage() {
        let _ = LocalStorage::delete(AUTH_TOKEN_KEY);
        let _ = LocalStorage::delete(USER_ID_KEY);
    }
    
    /// 登录
    pub async fn login(&self, username: String, password: String) -> Result<(), String> {
        self.state.set(AuthState::Loading);
        
        let api = AuthApi::new(self.server_url.get());
        let request = LoginRequest::password(username, password);
        
        match api.login(&request).await {
            Ok(response) => {
                // 保存认证信息
                Self::save_to_storage(&response.access_token, &response.user_id, &self.server_url.get());
                
                self.state.set(AuthState::Authenticated {
                    token: response.access_token,
                    user_id: response.user_id,
                });
                
                Ok(())
            }
            Err(ApiError::Status(401, msg)) => {
                let error = if msg.to_lowercase().contains("invalid") {
                    "用户名或密码错误".to_string()
                } else {
                    format!("认证失败: {}", msg)
                };
                self.state.set(AuthState::Error(error.clone()));
                Err(error)
            }
            Err(ApiError::Status(code, msg)) => {
                let error = format!("服务器错误 ({}): {}", code, msg);
                self.state.set(AuthState::Error(error.clone()));
                Err(error)
            }
            Err(e) => {
                let error = format!("登录失败: {}", e);
                self.state.set(AuthState::Error(error.clone()));
                Err(error)
            }
        }
    }
    
    /// 登出
    pub fn logout(&self) {
        Self::clear_storage();
        self.state.set(AuthState::Unauthenticated);
    }
    
    /// 验证当前 token 是否有效
    pub async fn validate_current_token(&self) -> bool {
        if let Some(token) = self.state.get().token() {
            let api = AuthApi::new(self.server_url.get());
            match api.validate_token(token).await {
                Ok(valid) => {
                    if !valid {
                        // Token 无效，清除认证状态
                        self.logout();
                    }
                    valid
                }
                Err(_) => {
                    // 验证失败，视为无效
                    self.logout();
                    false
                }
            }
        } else {
            false
        }
    }
}

impl Default for AuthContext {
    fn default() -> Self {
        Self::new()
    }
}

/// 提供认证上下文的组件
#[component]
pub fn ProvideAuthContext(children: Children) -> impl IntoView {
    let context = AuthContext::new();
    
    provide_context(context.clone());
    
    // 验证当前 token（如果已登录）
    spawn(async move {
        if context.state.get().is_authenticated() {
            context.validate_current_token().await;
        }
    });
    
    children()
}

/// 使用认证上下文
pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>().expect("AuthContext not provided")
}
