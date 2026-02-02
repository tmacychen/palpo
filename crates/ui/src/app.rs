// 根组件和路由配置

use leptos::*;
use leptos_router::*;

use crate::pages::LoginPage;
use crate::pages::DashboardPage;

/// 应用根组件
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                // 登录页（默认）
                <Route path="/" view=LoginPage/>
                
                // Dashboard（认证后）
                <Route path="/dashboard" view=DashboardPage/>
                
                // 其他路由将在后续任务中添加
                // <Route path="/users" view=UserManagementPage/>
                // <Route path="/rooms" view=RoomManagementPage/>
                // <Route path="/federation" view=FederationManagementPage/>
                // <Route path="/server" view=ServerInfoPage/>
            </Routes>
        </Router>
    }
}

/// 加载状态组件
#[component]
pub fn LoadingSpinner(
    #[prop(default = "加载中...")] message: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center h-64">
            <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-500"></div>
            <p class="mt-4 text-gray-400">{message}</p>
        </div>
    }
}

/// 错误提示组件
#[component]
pub fn ErrorAlert(
    #[prop(into)] message: String,
    #[prop(default = true)] show_icon: bool,
) -> impl IntoView {
    view! {
        <div class="bg-error/20 border border-error/50 text-error rounded-lg p-4 animate-fade-in">
            <div class="flex items-center">
                {if show_icon {
                    Some(view! {
                        <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/>
                        </svg>
                    })
                } else {
                    None
                }}
                <span>{message}</span>
            </div>
        </div>
    }
}

/// 成功提示组件
#[component]
pub fn SuccessAlert(
    #[prop(into)] message: String,
    #[prop(default = true)] show_icon: bool,
) -> impl IntoView {
    view! {
        <div class="bg-success/20 border border-success/50 text-success rounded-lg p-4 animate-fade-in">
            <div class="flex items-center">
                {if show_icon {
                    Some(view! {
                        <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                        </svg>
                    })
                } else {
                    None
                }}
                <span>{message}</span>
            </div>
        </div>
    }
}
