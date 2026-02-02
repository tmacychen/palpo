// 登录页面（完整实现）

use leptos::*;
use leptos_router::*;
use crate::state::{use_auth, AuthState};

// 辅助函数：从 localStorage 获取值
fn get_local_storage(key: &str) -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item(key).ok()?
}

// 辅助函数：设置 localStorage 值
fn set_local_storage(key: &str, value: &str) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok()?) {
        let _ = storage.set_item(key, value);
    }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_auth();
    let _navigate = use_navigate();
    
    let (username, set_username) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (remember_me, set_remember_me) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(false);
    let (server_url, set_server_url) = create_signal(String::new());
    
    // 初始化服务器地址
    create_effect(move |_| {
        if server_url.get().is_empty() {
            // 尝试从 localStorage 加载
            if let Some(saved_url) = get_local_storage("palpo_admin_server_url") {
                set_server_url.set(saved_url);
            } else {
                // 使用当前域名
                let url = web_sys::window()
                    .and_then(|w| w.location().origin().ok())
                    .unwrap_or_else(|| "http://localhost:8008".to_string());
                set_server_url.set(url);
            }
        }
    });
    
    let navigate = use_navigate();
    
    // 监听认证状态变化
    create_effect(move |_| {
        match auth.state.get() {
            AuthState::Authenticated { .. } => {
                // 已认证，跳转到 dashboard
                navigate("/dashboard", Default::default());
            }
            AuthState::Error(msg) => {
                set_error.set(Some(msg));
            }
            _ => {}
        }
    });
    
    // 如果已认证，显示加载状态
    let auth_clone_for_effect = auth.clone();
    let navigate2 = use_navigate();
    create_effect(move |_| {
        if auth_clone_for_effect.state.get().is_authenticated() {
            navigate2("/dashboard", Default::default());
        }
    });
    
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        
        let user = username.get();
        let pass = password.get();
        let url = server_url.get();
        
        if user.is_empty() || pass.is_empty() {
            set_error.set(Some("请输入用户名和密码".to_string()));
            return;
        }
        
        if url.is_empty() {
            set_error.set(Some("请输入服务器地址".to_string()));
            return;
        }
        
        // 保存服务器地址
        set_local_storage("palpo_admin_server_url", &url);
        auth.server_url.set(url);

        set_loading.set(true);
        set_error.set(None);

        // 执行登录
        let auth_clone = auth.clone();
        leptos::spawn_local(async move {
            match auth_clone.login(user, pass).await {
                Ok(()) => {
                    // 登录成功，导航到 dashboard
                    let navigate = use_navigate();
                    navigate("/dashboard", Default::default());
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    };
    
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900">
            <div class="glass-dark rounded-2xl shadow-2xl w-full max-w-md p-8 animate-fade-in">
                <div class="text-center mb-8">
                    <div class="w-16 h-16 bg-gradient-to-br from-primary-500 to-primary-700 rounded-2xl mx-auto mb-4 flex items-center justify-center">
                        <svg class="w-8 h-8 text-white" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M12 2L2 7v10c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V7l-10-5z"/>
                        </svg>
                    </div>
                    <h1 class="text-2xl font-bold text-white mb-2">"Palpo Admin"</h1>
                    <p class="text-gray-400 text-sm">"Matrix Homeserver 管理系统"</p>
                </div>
                
                <div class="mb-6">
                    <label for="server_url" class="block text-sm font-medium text-gray-300 mb-2">
                        "服务器地址"
                    </label>
                    <input
                        id="server_url"
                        type="text"
                        class="w-full px-4 py-3 bg-gray-800/50 border border-gray-700 rounded-lg
                               text-white placeholder-gray-500 text-sm
                               focus:outline-none focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20
                               transition-all duration-200"
                        placeholder="http://localhost:8008"
                        prop:value=server_url
                        on:input=move |ev| set_server_url.set(event_target_value(&ev))
                        disabled=loading
                    />
                </div>
                
                {move || error.get().map(|msg| view! {
                    <div class="mb-4">
                        <crate::app::ErrorAlert message=msg/>
                    </div>
                })}
                
                <form on:submit=on_submit>
                    <div class="space-y-5">
                        <div>
                            <label for="username" class="block text-sm font-medium text-gray-300 mb-2">
                                "用户名"
                            </label>
                            <input
                                id="username"
                                type="text"
                                class="w-full px-4 py-3 bg-gray-800/50 border border-gray-700 rounded-lg
                                       text-white placeholder-gray-500
                                       focus:outline-none focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20
                                       transition-all duration-200"
                                placeholder="输入用户名（如 @admin:server.com）"
                                prop:value=username
                                on:input=move |ev| set_username.set(event_target_value(&ev))
                                disabled=loading
                                autofocus
                            />
                        </div>
                        
                        <div>
                            <label for="password" class="block text-sm font-medium text-gray-300 mb-2">
                                "密码"
                            </label>
                            <input
                                id="password"
                                type="password"
                                class="w-full px-4 py-3 bg-gray-800/50 border border-gray-700 rounded-lg
                                       text-white placeholder-gray-500
                                       focus:outline-none focus:border-primary-500 focus:ring-2 focus:ring-primary-500/20
                                       transition-all duration-200"
                                placeholder="输入密码"
                                prop:value=password
                                on:input=move |ev| set_password.set(event_target_value(&ev))
                                disabled=loading
                            />
                        </div>
                        
                        <div class="flex items-center">
                            <input 
                                id="remember" 
                                type="checkbox" 
                                class="w-4 h-4 text-primary-600 bg-gray-800 border-gray-600 rounded focus:ring-primary-500"
                                prop:checked=move || remember_me.get()
                                on:input=move |ev| set_remember_me.set(event_target_checked(&ev))
                                disabled=loading
                            />
                            <label for="remember" class="ml-2 text-sm text-gray-400">
                                "记住登录状态"
                            </label>
                        </div>
                        
                        <button
                            type="submit"
                            class="w-full py-3 px-4 bg-gradient-to-r from-primary-500 to-primary-700
                                   text-white font-medium rounded-lg
                                   hover:from-primary-600 hover:to-primary-800
                                   focus:outline-none focus:ring-2 focus:ring-primary-500/50
                                   transition-all duration-200 transform hover:scale-[1.02]
                                   disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
                            disabled=loading
                        >
                            {move || if loading.get() {
                                view! {
                                    <span class="flex items-center justify-center">
                                        <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                        </svg>
                                        "登录中..."
                                    </span>
                                }
                            } else {
                                view! { <span>"登录"</span> }
                            }}
                        </button>
                    </div>
                </form>
                
                <div class="mt-6 text-center">
                    <p class="text-xs text-gray-500">
                        "© 2024 Palpo Matrix Homeserver"
                    </p>
                </div>
            </div>
        </div>
    }
}

/// 辅助函数：获取事件目标值
fn event_target_value(ev: &web_sys::Event) -> String {
    use wasm_bindgen::JsCast;
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.value())
        .unwrap_or_default()
}

/// 辅助函数：获取 checkbox 状态
fn event_target_checked(ev: &web_sys::Event) -> bool {
    use wasm_bindgen::JsCast;
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|input| input.checked())
        .unwrap_or_default()
}
