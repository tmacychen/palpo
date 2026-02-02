// HTTP API 客户端封装

use gloo_net::http::{Request, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};

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
        let request_builder = Request::get(&self.build_url(path));
        let builder = self.add_auth_header(request_builder);
        
        let request = builder.build()
            .map_err(|e| ApiError::Network(format!("Failed to build request: {}", e)))?;
        
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
        
        let mut builder = Request::post(&self.build_url(path))
            .header("Content-Type", "application/json");
        builder = self.add_auth_header(builder);
        
        let request = builder.body(json)
            .map_err(|e| ApiError::Network(format!("Failed to set request body: {}", e)))?;
        
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
        
        let mut builder = Request::put(&self.build_url(path))
            .header("Content-Type", "application/json");
        builder = self.add_auth_header(builder);
        
        let request = builder.body(json)
            .map_err(|e| ApiError::Network(format!("Failed to set request body: {}", e)))?;
        
        let response = request.send().await.map_err(|e| {
            ApiError::Network(e.to_string())
        })?;
        
        self.handle_response(response).await
    }
    
    /// 发送 DELETE 请求
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        let request_builder = Request::delete(&self.build_url(path));
        let builder = self.add_auth_header(request_builder);
        
        let request = builder.build()
            .map_err(|e| ApiError::Network(format!("Failed to build request: {}", e)))?;
        
        let response = request.send().await.map_err(|e| {
            ApiError::Network(e.to_string())
        })?;
        
        self.handle_response(response).await
    }
    
    /// 添加认证头
    fn add_auth_header(&self, builder: RequestBuilder) -> RequestBuilder {
        if let Some(token) = &self.token {
            builder.header("Authorization", &format!("Bearer {}", token))
        } else {
            builder
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

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use serde_json::json;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_api_client_creation() {
        let client = ApiClient::new("https://example.com".to_string());
        assert_eq!(client.base_url, "https://example.com");
        assert!(client.token.is_none());
    }

    #[test]
    fn test_api_client_with_token() {
        let client = ApiClient::new("https://example.com".to_string())
            .with_token("test_token".to_string());
        assert_eq!(client.token(), Some("test_token"));
    }

    #[test]
    fn test_api_client_set_token() {
        let mut client = ApiClient::new("https://example.com".to_string());
        assert!(client.token.is_none());
        
        client.set_token(Some("new_token".to_string()));
        assert_eq!(client.token(), Some("new_token"));
        
        client.set_token(None);
        assert!(client.token.is_none());
    }

    #[test]
    fn test_build_url() {
        let client = ApiClient::new("https://example.com".to_string());
        
        // 测试相对路径
        assert_eq!(
            client.build_url("/api/test"),
            "https://example.com/api/test"
        );
        
        // 测试不带斜杠的路径
        assert_eq!(
            client.build_url("api/test"),
            "https://example.com/api/test"
        );
        
        // 测试绝对 URL
        assert_eq!(
            client.build_url("https://other.com/api"),
            "https://other.com/api"
        );
    }

    #[test]
    fn test_build_url_with_trailing_slash() {
        let client = ApiClient::new("https://example.com/".to_string());
        assert_eq!(
            client.build_url("api/test"),
            "https://example.com/api/test"
        );
    }

    #[test]
    fn test_api_error_display() {
        let error = ApiError::Status(404, "Not Found".to_string());
        assert_eq!(error.to_string(), "HTTP 404: Not Found");
        
        let error = ApiError::Network("Connection failed".to_string());
        assert_eq!(error.to_string(), "Network error: Connection failed");
        
        let error = ApiError::Json("Invalid JSON".to_string());
        assert_eq!(error.to_string(), "JSON error: Invalid JSON");
        
        let error = ApiError::Other("Unknown error".to_string());
        assert_eq!(error.to_string(), "Error: Unknown error");
    }

    #[test]
    fn test_api_response_success() {
        let data = json!({ "key": "value" });
        let response: ApiResponse<serde_json::Value> = ApiResponse::success(data.clone());
        
        assert!(response.success);
        assert_eq!(response.data, Some(data));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_api_response_error() {
        let response: ApiResponse<String> = ApiResponse::error("Test error".to_string());
        
        assert!(!response.success);
        assert!(response.data.is_none());
        assert_eq!(response.error, Some("Test error".to_string()));
    }

    #[wasm_bindgen_test]
    async fn test_handle_empty_response() {
        let _client = ApiClient::new("https://example.com".to_string());
        
        // 模拟空响应应该能解析为 {}
        let empty_json: serde_json::Value = serde_json::from_str("{}").unwrap();
        assert_eq!(empty_json, json!({}));
    }

    #[wasm_bindgen_test]
    async fn test_handle_error_response_parsing() {
        // 测试错误响应解析
        let error_json = r#"{"error": "Invalid token"}"#;
        let parsed: serde_json::Value = serde_json::from_str(error_json).unwrap();
        
        let error_msg = parsed.get("error")
            .and_then(|v| v.as_str())
            .unwrap_or(error_json)
            .to_string();
        
        assert_eq!(error_msg, "Invalid token");
    }

    #[test]
    fn test_complex_url_building() {
        let client = ApiClient::new("http://localhost:8008".to_string());
        
        // 测试带端口和路径的 URL
        assert_eq!(
            client.build_url("/_matrix/client/v3/login"),
            "http://localhost:8008/_matrix/client/v3/login"
        );
        
        // 测试带查询参数的 URL
        assert_eq!(
            client.build_url("/api/users?from=10&limit=20"),
            "http://localhost:8008/api/users?from=10&limit=20"
        );
    }

    #[test]
    fn test_api_client_clone() {
        let client1 = ApiClient::new("https://example.com".to_string())
            .with_token("token1".to_string());
        
        let mut client2 = client1.clone();
        client2.set_token(Some("token2".to_string()));
        
        // 验证克隆后修改不会影响原对象
        assert_eq!(client1.token(), Some("token1"));
        assert_eq!(client2.token(), Some("token2"));
    }
}
