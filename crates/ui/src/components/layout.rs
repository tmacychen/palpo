// 布局组件：侧边栏 + 顶部栏

use leptos::*;
use leptos_router::*;
use crate::state::use_auth;

/// 导航菜单项
#[derive(Clone, Debug)]
struct NavItem {
    path: &'static str,
    label: &'static str,
    icon: Icon,
}

#[derive(Clone, Debug)]
enum Icon {
    Dashboard,
    Users,
    Rooms,
    Federation,
    Server,
    Logout,
}

impl Icon {
    fn view(&self) -> impl IntoView {
        match self {
            Icon::Dashboard => view! {
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
                </svg>
            },
            Icon::Users => view! {
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z"/>
                </svg>
            },
            Icon::Rooms => view! {
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/>
                </svg>
            },
            Icon::Federation => view! {
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"/>
                </svg>
            },
            Icon::Server => view! {
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"/>
                </svg>
            },
            Icon::Logout => view! {
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"/>
                </svg>
            },
        }
    }
}

/// 主布局组件（侧边栏 + 顶部栏 + 内容区）
#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    
    // 克隆 navigate 用于不同闭包
    let navigate_effect = navigate.clone();
    
    // 检查是否已认证
    create_effect(move |_| {
        if !auth.state.get().is_authenticated() {
            navigate_effect("/", Default::default());
        }
    });
    
    // 导航菜单项
    let nav_items = vec![
        NavItem {
            path: "/dashboard",
            label: "Dashboard",
            icon: Icon::Dashboard,
        },
        NavItem {
            path: "/users",
            label: "用户管理",
            icon: Icon::Users,
        },
        NavItem {
            path: "/rooms",
            label: "房间管理",
            icon: Icon::Rooms,
        },
        NavItem {
            path: "/federation",
            label: "联邦管理",
            icon: Icon::Federation,
        },
        NavItem {
            path: "/server",
            label: "服务器信息",
            icon: Icon::Server,
        },
    ];
    
    // 获取当前用户
    let current_user = move || {
        auth.state.get().user_id().unwrap_or("未知用户").to_string()
    };
    
    let on_logout = move |_| {
        auth.logout();
        navigate("/", Default::default());
    };
    
    view! {
        <div class="min-h-screen bg-gray-900 text-gray-100 flex">
            // 侧边栏
            <div class="w-64 bg-gray-900/95 backdrop-blur-sm border-r border-gray-700/50 flex flex-col">
                // Logo区域
                <div class="h-16 flex items-center justify-center border-b border-gray-700/30">
                    <div class="flex items-center space-x-3">
                        <div class="w-8 h-8 bg-gradient-to-br from-primary-500 to-primary-700 rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M12 2L2 7v10c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V7l-10-5z"/>
                            </svg>
                        </div>
                        <span class="text-lg font-bold text-white">Palpo Admin</span>
                    </div>
                </div>
                
                // 导航菜单
                <nav class="flex-1 py-4 overflow-y-auto">
                    <ul class="space-y-1 px-3">
                        {nav_items.into_iter()
                            .map(|item| {
                                let path = item.path;
                                let icon = item.icon;
                                let label = item.label;
                                
                                view! {
                                    <li>
                                        <A
                                            href=path
                                            class="flex items-center space-x-3 px-4 py-3 rounded-lg
                                                   text-gray-300 hover:text-white hover:bg-gray-800/50
                                                   transition-all duration-200 group"
                                            active_class="bg-primary-500/20 text-primary-400"
                                        >
                                            {move || icon.view()}
                                            <span class="text-sm font-medium">{label}</span>
                                        </A>
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()
                        }
                    </ul>
                </nav>
                
                // 用户信息
                <div class="p-4 border-t border-gray-700/30">
                    <div class="flex items-center space-x-3">
                        <div class="w-8 h-8 bg-gradient-to-br from-primary-500 to-primary-700 rounded-full
                                    flex items-center justify-center text-white text-sm font-bold">
                            {move || current_user().chars().next().unwrap_or('U').to_string().to_uppercase()}
                        </div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-white truncate">
                                {current_user}
                            </p>
                            <p class="text-xs text-gray-400">管理员</p>
                        </div>
                        <button
                            on:click=on_logout
                            class="p-2 text-gray-400 hover:text-white hover:bg-gray-800/50
                                   rounded-lg transition-colors duration-200"
                            title="退出登录"
                        >
                            {Icon::Logout.view()}
                        </button>
                    </div>
                </div>
            </div>
            
            // 主内容区
            <div class="flex-1 flex flex-col">
                // 顶部栏
                <header class="h-16 bg-gray-900/95 backdrop-blur-sm border-b border-gray-700/50 flex items-center px-6">
                    <div class="flex items-center justify-between w-full">
                        // 面包屑导航
                        <nav class="flex items-center space-x-2">
                            <Breadcrumb/>
                        </nav>
                        
                        // 右侧操作
                        <div class="flex items-center space-x-4">
                            // 刷新按钮
                            <button
                                class="p-2 text-gray-400 hover:text-white hover:bg-gray-800/50
                                       rounded-lg transition-colors duration-200"
                                title="刷新数据"
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                                </svg>
                            </button>
                            
                            // 通知按钮
                            <button
                                class="p-2 text-gray-400 hover:text-white hover:bg-gray-800/50
                                       rounded-lg transition-colors duration-200 relative"
                                title="通知"
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-5 5v-5zM10.5 3.75a6 6 0 00-6 6v2.25l-2.25 2.25v2.25h16.5v-2.25L16.5 12V9.75a6 6 0 00-6-6h-3z"/>
                                </svg>
                                <span class="absolute -top-1 -right-1 w-2 h-2 bg-error rounded-full"></span>
                            </button>
                            
                            // 用户头像
                            <div class="w-8 h-8 bg-gradient-to-br from-primary-500 to-primary-700 rounded-full
                                        flex items-center justify-center text-white text-sm font-bold">
                                {move || current_user().chars().next().unwrap_or('U').to_string().to_uppercase()}
                            </div>
                        </div>
                    </div>
                </header>
                
                // 页面内容
                <main class="flex-1 overflow-y-auto p-6">
                    {children()}
                </main>
            </div>
        </div>
    }
}

/// 面包屑导航组件
#[component]
pub fn Breadcrumb() -> impl IntoView {
    let location = use_location();
    
    // 从当前路径生成分段
    let segments = move || {
        let pathname = location.pathname.get();
        let mut segments = Vec::new();
        
        // 处理根路径
        if pathname == "/" || pathname.is_empty() {
            segments.push(("/".to_string(), "首页".to_string()));
            return segments;
        }
        
        // 分割路径
        let parts: Vec<&str> = pathname.split('/').filter(|s| !s.is_empty()).collect();
        
        // 构建分段
        for (i, part) in parts.iter().enumerate() {
            let path = format!("/ {}", parts[..=i].join("/"));
            let name = match *part {
                "dashboard" => "Dashboard",
                "users" => "用户管理",
                "rooms" => "房间管理",
                "federation" => "联邦管理",
                "server" => "服务器信息",
                _ => part,
            }.to_string();
            
            segments.push((path, name));
        }
        
        // 添加首页
        segments.insert(0, ("/".to_string(), "首页".to_string()));
        
        segments
    };
    
    view! {
        <ol class="flex items-center space-x-2">
            {move || segments().into_iter().enumerate().map(|(i, (path, name))| {
                view! {
                    <li class="flex items-center">
                        {if i > 0 {
                            Some(view! {
                                <svg class="w-4 h-4 text-gray-500 mx-2" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"/>
                                </svg>
                            })
                        } else {
                            None
                        }}
                        <A
                            href=path
                            class=move || format!(
                                "text-sm {}",
                                if i == segments().len() - 1 {
                                    "text-gray-300 font-medium"
                                } else {
                                    "text-gray-500 hover:text-gray-300"
                                }
                            )
                        >
                            {name}
                        </A>
                    </li>
                }
            }).collect::<Vec<_>>()}
        </ol>
    }
}

/// 页面卡片容器
#[component]
pub fn PageCard(
    #[prop(into)] title: String,
    #[prop(optional)] subtitle: Option<String>,
    #[prop(optional)] actions: Option<View>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="glass-dark rounded-xl p-6">
            <div class="flex items-center justify-between mb-6">
                <div>
                    <h1 class="text-2xl font-bold text-white">{title}</h1>
                    {subtitle.map(|s| view! {
                        <p class="text-gray-400 mt-1">{s}</p>
                    })}
                </div>
                {actions}
            </div>
            {children()}
        </div>
    }
}
