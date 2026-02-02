// HTTP API 客户端封装

use gloo_net::http::{Request, Response};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;

/// API 错误类型
#[derive(Debug, Clone)]
pub enum ApiError {
    /// HTTP 错误状态码
    Status(u16, String),
    /// 网络错误
    Network(String),
    /// JSON 解析错误
    Json(String),
    /// 其他错误
    Other(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Status(code, msg) => write!(f, "HTTP {}: {}", code, msg),
            ApiError::Network(msg) => write!(f, "Network error: {}", msg),
            ApiError::Json(msg) => write!(f, "JSON error: {}", msg),
            ApiError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

/// API 客户端配置
#[derive(Clone, Debug)]
pub struct ApiClient {
    /// API 基础 URL
    base_url: String,
    /// 认证 Token
    token: Option<String>,
}

impl ApiClient {
    /// 创建新的 API 客户端
    pub fn new(base_url: String) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            token: None,
        }
    }
    
    /// 设置认证 Token
    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }
    
    /// 更新 Token
    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token;
    }
    
    /// 获取 Token
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
    
    /// 构建完整的 API URL
    fn build_url(&self, path: &str) -> String {
        if path.starts_with("http") {
            path.to_string()
        } else {
            let path = path.trim_start_matches('/');
            format!("{}/{}", self.base_url, path)
        }
    }
    
    /// 发送 GET 请求
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let mut request = Request::get(&self.build_url(path));
        request = self.add_auth_header(request);
        
        let response = request.send().await.map_err(|e| {
            ApiError::Network(e.to_string())
        })?;
        
        self.handle_response(response).await
    }
    
    /// 发送 POST 请求
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        let json = serde_json::to_string(body).map_err(|e| {
            ApiError::Json(e.to_string())
        })?;
        
        let mut request = Request::post(&self.build_url(path))
            .header("Content-Type", "application/json")
            .body(json);
        request = self.add_auth_header(request);
        
        let response = request.send().await.map_err(|e| {
            ApiError::Network(e.to_string())
        })?;
        
        self.handle_response(response).await
    }
    
    /// 发送 PUT 请求
    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        let json = serde_json::to_string(body).map_err(|e| {
            ApiError::Json(e.to_string())
        })?;
        
        let mut request = Request::put(&self.build_url(path))
            .header("Content-Type", "application/json")
            .body(json);
        request = self.add_auth_header(request);
        
        let response = request.send().await.map_err(|e| {
            ApiError::Network(e.to_string())
        })?;
        
        self.handle_response(response).await
    }
    
    /// 发送 DELETE 请求
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let mut request = Request::delete(&self.build_url(path));
        request = self.add_auth_header(request);
        
        let response = request.send().await.map_err(|e| {
            ApiError::Network(e.to_string())
        })?;
        
        self.handle_response(response).await
    }
    
    /// 添加认证头
    fn add_auth_header(&self, request: Request) -> Request {
        if let Some(token) = &self.token {
            request.header("Authorization", &format!("Bearer {}", token))
        } else {
            request
        }
    }
    
    /// 处理响应
    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, ApiError> {
        let status = response.status();
        let text = response.text().await.map_err(|e| {
            ApiError::Network(e.to_string())
        })?;
        
        if status >= 200 && status < 300 {
            if text.is_empty() {
                // 对于空响应，返回空 JSON 对象
                serde_json::from_str("{}").map_err(|e| {
                    ApiError::Json(e.to_string())
                })
            } else {
                serde_json::from_str(&text).map_err(|e| {
                    ApiError::Json(format!("Failed to parse JSON: {}. Response: {}", e, text))
                })
            }
        } else {
            // 尝试解析错误响应
            let error_msg = if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&text) {
                error_json.get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&text)
                    .to_string()
            } else {
                text
            };
            
            Err(ApiError::Status(status, error_msg))
        }
    }
}

/// API 响应包装类型
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    /// 创建错误响应
    pub fn error(msg: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg),
        }
    }
}
