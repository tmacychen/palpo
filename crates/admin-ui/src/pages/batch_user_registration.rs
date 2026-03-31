//! Batch user registration page - CSV upload and bulk import

use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::app::Route;
use crate::models::AuthState;
use crate::services::user_admin_api::UserAdminAPI;
use crate::utils::audit_logger::AuditLogger;
use crate::services::api_client::ApiClient;
use crate::components::loading::Spinner;

/// A parsed user row from CSV
#[derive(Clone, Debug, PartialEq)]
struct CsvUserRow {
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
    pub is_admin: bool,
    pub error: Option<String>,
}

/// Import result for a single user
#[derive(Clone, Debug)]
struct ImportResult {
    pub username: String,
    pub success: bool,
    pub message: String,
}

/// Batch user registration page
#[component]
pub fn BatchUserRegistrationPage() -> Element {
    let mut csv_text = use_signal(|| String::new());
    let mut parsed_rows = use_signal(|| Vec::<CsvUserRow>::new());
    let mut parse_error = use_signal(|| None::<String>);
    let mut import_results = use_signal(|| Vec::<ImportResult>::new());
    let mut importing = use_signal(|| false);
    let mut import_done = use_signal(|| false);
    let mut progress = use_signal(|| 0usize);
    let auth_state = use_context::<Signal<AuthState>>();
    let navigator = use_navigator();

    let mut parse_csv = move |text: String| {
        parse_error.set(None);
        parsed_rows.set(Vec::new());

        if text.trim().is_empty() {
            return;
        }

        let mut rows = Vec::new();
        for (i, line) in text.lines().enumerate() {
            // Skip header line
            if i == 0 && line.to_lowercase().contains("username") {
                continue;
            }
            let line = line.trim();
            if line.is_empty() { continue; }

            let cols: Vec<&str> = line.splitn(4, ',').collect();
            if cols.len() < 2 {
                rows.push(CsvUserRow {
                    username: line.to_string(),
                    password: String::new(),
                    display_name: None,
                    is_admin: false,
                    error: Some("格式错误：至少需要 username,password 两列".to_string()),
                });
                continue;
            }

            let username = cols[0].trim().to_string();
            let password = cols[1].trim().to_string();
            let display_name = cols.get(2).map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
            let is_admin = cols.get(3).map(|s| s.trim().to_lowercase() == "true" || s.trim() == "1").unwrap_or(false);

            let error = if username.is_empty() {
                Some("用户名不能为空".to_string())
            } else if password.len() < 8 {
                Some("密码至少需要 8 个字符".to_string())
            } else {
                None
            };

            rows.push(CsvUserRow { username, password, display_name, is_admin, error });
        }

        if rows.is_empty() {
            parse_error.set(Some("未找到有效的用户数据".to_string()));
        } else {
            parsed_rows.set(rows);
        }
    };

    let valid_count = parsed_rows().iter().filter(|r| r.error.is_none()).count();
    let error_count = parsed_rows().iter().filter(|r| r.error.is_some()).count();
    let success_count = import_results().iter().filter(|r| r.success).count();
    let fail_count = import_results().iter().filter(|r| !r.success).count();

    rsx! {
        div { class: "p-4 sm:p-6 space-y-6",
            // Header
            div { class: "flex items-center gap-4",
                button {
                    class: "inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50",
                    onclick: move |_| { navigator.push(Route::Users {}); },
                    "← 返回用户列表"
                }
                div {
                    h2 { class: "text-2xl font-bold text-gray-900", "批量用户注册" }
                    p { class: "mt-1 text-sm text-gray-500", "通过 CSV 格式批量创建用户账户" }
                }
            }

            // CSV format guide
            div { class: "bg-blue-50 border border-blue-200 rounded-lg p-4",
                h3 { class: "text-sm font-medium text-blue-900 mb-2", "CSV 格式说明" }
                p { class: "text-sm text-blue-800 mb-2", "每行一个用户，字段用逗号分隔：" }
                pre { class: "text-xs bg-blue-100 rounded p-2 font-mono text-blue-900",
                    "username,password,display_name,is_admin\nalice,SecurePass123,Alice Wang,false\nbob,AnotherPass456,Bob Li,true"
                }
                p { class: "text-xs text-blue-700 mt-2", "• display_name 和 is_admin 为可选字段" }
                p { class: "text-xs text-blue-700", "• 密码至少 8 个字符" }
            }

            // CSV input
            div { class: "bg-white shadow rounded-lg p-6 space-y-4",
                h3 { class: "text-lg font-medium text-gray-900", "输入 CSV 数据" }
                textarea {
                    class: "w-full h-48 px-3 py-2 border border-gray-300 rounded-md font-mono text-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500",
                    placeholder: "username,password,display_name,is_admin\nalice,SecurePass123,Alice Wang,false",
                    value: "{csv_text}",
                    oninput: move |evt| {
                        let val = evt.value();
                        csv_text.set(val.clone());
                        parse_csv(val);
                    }
                }

                if let Some(err) = parse_error() {
                    p { class: "text-sm text-red-600", "⚠️ {err}" }
                }

                if !parsed_rows().is_empty() {
                    div { class: "flex gap-4 text-sm",
                        span { class: "text-green-700", "✓ {valid_count} 个有效用户" }
                        if error_count > 0 {
                            span { class: "text-red-600", "✗ {error_count} 个错误" }
                        }
                    }
                }
            }

            // Preview table
            if !parsed_rows().is_empty() {
                div { class: "bg-white shadow rounded-lg overflow-hidden",
                    div { class: "px-6 py-4 border-b border-gray-200",
                        h3 { class: "text-lg font-medium text-gray-900", "预览 ({parsed_rows().len()} 行)" }
                    }
                    div { class: "overflow-x-auto",
                        table { class: "min-w-full divide-y divide-gray-200",
                            thead { class: "bg-gray-50",
                                tr {
                                    th { class: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase", "用户名" }
                                    th { class: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase", "显示名" }
                                    th { class: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase", "管理员" }
                                    th { class: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase", "状态" }
                                }
                            }
                            tbody { class: "bg-white divide-y divide-gray-200",
                                for row in parsed_rows() {
                                    tr { class: if row.error.is_some() { "bg-red-50" } else { "hover:bg-gray-50" },
                                        td { class: "px-4 py-3 text-sm font-mono text-gray-900", "{row.username}" }
                                        td { class: "px-4 py-3 text-sm text-gray-700",
                                            "{row.display_name.as_ref().unwrap_or(&\"-\".to_string())}"
                                        }
                                        td { class: "px-4 py-3 text-sm",
                                            if row.is_admin {
                                                span { class: "text-purple-700", "✓ 管理员" }
                                            } else {
                                                span { class: "text-gray-500", "普通用户" }
                                            }
                                        }
                                        td { class: "px-4 py-3 text-sm",
                                            if let Some(err) = &row.error {
                                                span { class: "text-red-600", "✗ {err}" }
                                            } else {
                                                span { class: "text-green-600", "✓ 有效" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Import button
                if valid_count > 0 && !import_done() {
                    div { class: "flex justify-end",
                        button {
                            class: "px-6 py-2 bg-blue-600 text-white rounded-md text-sm font-medium hover:bg-blue-700 disabled:opacity-50",
                            disabled: importing(),
                            onclick: {
                                let admin_user = match &*auth_state.read() {
                                    AuthState::Authenticated(u) => u.username.clone(),
                                    _ => "admin".to_string(),
                                };
                                move |_| {
                                    let rows = parsed_rows().into_iter().filter(|r| r.error.is_none()).collect::<Vec<_>>();
                                    let admin_user = admin_user.clone();
                                    importing.set(true);
                                    import_results.set(Vec::new());
                                    progress.set(0);

                                    spawn_local(async move {
                                        let audit_logger = AuditLogger::new(1000);
                                        let api_client = ApiClient::new("http://localhost:8081");
                                        let api = UserAdminAPI::new(audit_logger, api_client);
                                        let total = rows.len();
                                        let mut results = Vec::new();

                                        for (i, row) in rows.iter().enumerate() {
                                            progress.set(i + 1);
                                            let req = crate::models::user::CreateUserRequest {
                                                user_id: format!("@{}:localhost", row.username),
                                                displayname: row.display_name.clone(),
                                                avatar_url: None,
                                                is_admin: row.is_admin,
                                                is_guest: false,
                                                user_type: None,
                                                appservice_id: None,
                                            };
                                            // After creating, reset password if provided
                                            match api.create_user(req, &admin_user).await {
                                                Ok(r) if r.success => {
                                                    // Set the password
                                                    let pwd_req = crate::models::user::ResetPasswordRequest {
                                                        user_id: format!("@{}:localhost", row.username),
                                                        new_password: Some(row.password.clone()),
                                                        logout_devices: false,
                                                    };
                                                    let _ = api.reset_password(pwd_req, &admin_user).await;
                                                    results.push(ImportResult {
                                                        username: row.username.clone(),
                                                        success: true,
                                                        message: "创建成功".to_string(),
                                                    });
                                                }
                                                Ok(r) => {
                                                    results.push(ImportResult {
                                                        username: row.username.clone(),
                                                        success: false,
                                                        message: r.error.unwrap_or("创建失败".to_string()),
                                                    });
                                                }
                                                Err(e) => {
                                                    results.push(ImportResult {
                                                        username: row.username.clone(),
                                                        success: false,
                                                        message: e.to_string(),
                                                    });
                                                }
                                            }
                                        }

                                        import_results.set(results);
                                        importing.set(false);
                                        import_done.set(true);
                                    });
                                }
                            },
                            if importing() {
                                "导入中 ({progress()}/{valid_count})..."
                            } else {
                                "🚀 开始导入 {valid_count} 个用户"
                            }
                        }
                    }
                }
            }

            // Import progress
            if importing() {
                div { class: "bg-white shadow rounded-lg p-6",
                    Spinner { size: "medium".to_string(), message: Some(format!("正在导入... ({}/{})", progress(), valid_count)) }
                    div { class: "mt-4 bg-gray-200 rounded-full h-2",
                        {
                            let pct = if valid_count > 0 { progress() * 100 / valid_count } else { 0 };
                            rsx! {
                                div {
                                    class: "bg-blue-600 h-2 rounded-full transition-all",
                                    style: "width: {pct}%"
                                }
                            }
                        }
                    }
                }
            }

            // Import results
            if import_done() && !import_results().is_empty() {
                div { class: "bg-white shadow rounded-lg overflow-hidden",
                    div { class: "px-6 py-4 border-b border-gray-200 flex items-center justify-between",
                        h3 { class: "text-lg font-medium text-gray-900", "导入结果" }
                        div { class: "flex gap-4 text-sm",
                            span { class: "text-green-700", "✓ 成功: {success_count}" }
                            if fail_count > 0 {
                                span { class: "text-red-600", "✗ 失败: {fail_count}" }
                            }
                        }
                    }
                    div { class: "overflow-x-auto",
                        table { class: "min-w-full divide-y divide-gray-200",
                            thead { class: "bg-gray-50",
                                tr {
                                    th { class: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase", "用户名" }
                                    th { class: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase", "结果" }
                                    th { class: "px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase", "消息" }
                                }
                            }
                            tbody { class: "bg-white divide-y divide-gray-200",
                                for result in import_results() {
                                    tr { class: if result.success { "hover:bg-gray-50" } else { "bg-red-50" },
                                        td { class: "px-4 py-3 text-sm font-mono text-gray-900", "{result.username}" }
                                        td { class: "px-4 py-3 text-sm",
                                            if result.success {
                                                span { class: "text-green-600", "✓ 成功" }
                                            } else {
                                                span { class: "text-red-600", "✗ 失败" }
                                            }
                                        }
                                        td { class: "px-4 py-3 text-sm text-gray-700", "{result.message}" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "px-6 py-4 border-t border-gray-200 flex justify-end",
                        button {
                            class: "px-4 py-2 bg-blue-600 text-white rounded-md text-sm",
                            onclick: move |_| { navigator.push(Route::Users {}); },
                            "返回用户列表"
                        }
                    }
                }
            }
        }
    }
}
