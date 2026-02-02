// Dashboard 首页

use leptos::*;
use crate::components::MainLayout;

#[component]
pub fn DashboardPage() -> impl IntoView {
    // TODO: 从 API 获取实际数据
    let (user_count, _set_user_count) = create_signal(1234);
    let (room_count, _set_room_count) = create_signal(567);
    let (federation_count, _set_federation_count) = create_signal(42);
    let (server_version, _set_server_version) = create_signal("0.1.0".to_string());
    
    view! {
        <MainLayout>
            <div class="p-6 space-y-6">
                <div class="flex items-center justify-between">
                    <div>
                        <h1 class="text-2xl font-bold text-white">"Dashboard"</h1>
                        <p class="text-gray-400 mt-1">"服务器运行状态概览"</p>
                    </div>
                </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <div class="glass-dark rounded-xl p-6 hover:scale-[1.02] transition-transform duration-200">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-400">"用户总数"</p>
                            <p class="text-2xl font-bold text-white mt-1">{user_count}</p>
                        </div>
                        <div class="w-12 h-12 bg-primary-500/20 rounded-lg flex items-center justify-center">
                            <svg class="w-6 h-6 text-primary-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z"/>
                            </svg>
                        </div>
                    </div>
                    <div class="mt-4">
                        <span class="text-xs text-success">"+12%"</span>
                        <span class="text-xs text-gray-500 ml-1">"较上月"</span>
                    </div>
                </div>
                
                <div class="glass-dark rounded-xl p-6 hover:scale-[1.02] transition-transform duration-200">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-400">"房间总数"</p>
                            <p class="text-2xl font-bold text-white mt-1">{room_count}</p>
                        </div>
                        <div class="w-12 h-12 bg-warning/20 rounded-lg flex items-center justify-center">
                            <svg class="w-6 h-6 text-warning" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/>
                            </svg>
                        </div>
                    </div>
                    <div class="mt-4">
                        <span class="text-xs text-success">"+8%"</span>
                        <span class="text-xs text-gray-500 ml-1">"较上月"</span>
                    </div>
                </div>
                
                <div class="glass-dark rounded-xl p-6 hover:scale-[1.02] transition-transform duration-200">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-400">"联邦服务器"</p>
                            <p class="text-2xl font-bold text-white mt-1">{federation_count}</p>
                        </div>
                        <div class="w-12 h-12 bg-success/20 rounded-lg flex items-center justify-center">
                            <svg class="w-6 h-6 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"/>
                            </svg>
                        </div>
                    </div>
                    <div class="mt-4">
                        <span class="text-xs text-gray-500">"全部在线"</span>
                    </div>
                </div>
                
                <div class="glass-dark rounded-xl p-6 hover:scale-[1.02] transition-transform duration-200">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-400">"服务器版本"</p>
                            <p class="text-xl font-bold text-white mt-1">{server_version}</p>
                        </div>
                        <div class="w-12 h-12 bg-gray-500/20 rounded-lg flex items-center justify-center">
                            <svg class="w-6 h-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"/>
                            </svg>
                        </div>
                    </div>
                    <div class="mt-4">
                        <span class="text-xs text-success">"运行中"</span>
                    </div>
                </div>
            </div>
            
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="glass-dark rounded-xl p-6">
                    <div class="flex items-center justify-between mb-4">
                        <h2 class="text-lg font-semibold text-white">"最近注册用户"</h2>
                        <a href="/users" class="text-sm text-primary-400 hover:text-primary-300">"查看全部"</a>
                    </div>
                    <div class="space-y-3">
                        <div class="flex items-center justify-between py-2 border-b border-gray-700/30">
                            <div class="flex items-center">
                                <div class="w-8 h-8 bg-primary-500/20 rounded-full flex items-center justify-center mr-3">
                                    <span class="text-xs font-bold text-primary-400">A</span>
                                </div>
                                <div>
                                    <p class="text-sm font-medium text-white">@alice:server.com</p>
                                    <p class="text-xs text-gray-400">"2小时前"</p>
                                </div>
                            </div>
                            <span class="text-xs bg-success/20 text-success px-2 py-1 rounded">"活跃"</span>
                        </div>
                        
                        <div class="flex items-center justify-between py-2 border-b border-gray-700/30">
                            <div class="flex items-center">
                                <div class="w-8 h-8 bg-warning/20 rounded-full flex items-center justify-center mr-3">
                                    <span class="text-xs font-bold text-warning">B</span>
                                </div>
                                <div>
                                    <p class="text-sm font-medium text-white">@bob:server.com</p>
                                    <p class="text-xs text-gray-400">"5小时前"</p>
                                </div>
                            </div>
                            <span class="text-xs bg-success/20 text-success px-2 py-1 rounded">"活跃"</span>
                        </div>
                        
                        <div class="flex items-center justify-between py-2">
                            <div class="flex items-center">
                                <div class="w-8 h-8 bg-success/20 rounded-full flex items-center justify-center mr-3">
                                    <span class="text-xs font-bold text-success">C</span>
                                </div>
                                <div>
                                    <p class="text-sm font-medium text-white">@charlie:server.com</p>
                                    <p class="text-xs text-gray-400">"1天前"</p>
                                </div>
                            </div>
                            <span class="text-xs bg-gray-500/20 text-gray-400 px-2 py-1 rounded">"离线"</span>
                        </div>
                    </div>
                </div>
                
                <div class="glass-dark rounded-xl p-6">
                    <h2 class="text-lg font-semibold text-white mb-4">"系统状态"</h2>
                    <div class="space-y-4">
                        <div class="flex items-center justify-between">
                            <span class="text-sm text-gray-400">"数据库连接"</span>
                            <span class="flex items-center text-success">
                                <svg class="w-2 h-2 mr-2" fill="currentColor" viewBox="0 0 8 8">
                                    <circle cx="4" cy="4" r="3"/>
                                </svg>
                                "正常"
                            </span>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-sm text-gray-400">"联邦连接"</span>
                            <span class="flex items-center text-success">
                                <svg class="w-2 h-2 mr-2" fill="currentColor" viewBox="0 0 8 8">
                                    <circle cx="4" cy="4" r="3"/>
                                </svg>
                                "正常"
                            </span>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-sm text-gray-400">"媒体存储"</span>
                            <span class="text-sm text-gray-300">"68% 已使用"</span>
                        </div>
                        
                        <div class="flex items-center justify-between">
                            <span class="text-sm text-gray-400">"运行时间"</span>
                            <span class="text-sm text-gray-300">"15天 8小时"</span>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="glass-dark rounded-xl p-6">
                <h2 class="text-lg font-semibold text-white mb-4">"快捷操作"</h2>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <button class="p-4 bg-gray-800/50 rounded-lg hover:bg-gray-700/50 transition-colors duration-200 text-center group">
                        <div class="w-10 h-10 bg-primary-500/20 rounded-lg flex items-center justify-center mx-auto mb-2 group-hover:bg-primary-500/30 transition-colors">
                            <svg class="w-5 h-5 text-primary-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
                            </svg>
                        </div>
                        <span class="text-sm text-gray-300">"创建用户"</span>
                    </button>
                    
                    <button class="p-4 bg-gray-800/50 rounded-lg hover:bg-gray-700/50 transition-colors duration-200 text-center group">
                        <div class="w-10 h-10 bg-warning/20 rounded-lg flex items-center justify-center mx-auto mb-2 group-hover:bg-warning/30 transition-colors">
                            <svg class="w-5 h-5 text-warning" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/>
                            </svg>
                        </div>
                        <span class="text-sm text-gray-300">"创建房间"</span>
                    </button>
                    
                    <button class="p-4 bg-gray-800/50 rounded-lg hover:bg-gray-700/50 transition-colors duration-200 text-center group">
                        <div class="w-10 h-10 bg-success/20 rounded-lg flex items-center justify-center mx-auto mb-2 group-hover:bg-success/30 transition-colors">
                            <svg class="w-5 h-5 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                            </svg>
                        </div>
                        <span class="text-sm text-gray-300">"查看日志"</span>
                    </button>
                    
                    <button class="p-4 bg-gray-800/50 rounded-lg hover:bg-gray-700/50 transition-colors duration-200 text-center group">
                        <div class="w-10 h-10 bg-error/20 rounded-lg flex items-center justify-center mx-auto mb-2 group-hover:bg-error/30 transition-colors">
                            <svg class="w-5 h-5 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                        </div>
                        <span class="text-sm text-gray-300">"系统设置"</span>
                    </button>
                </div>
            </div>
            </div>
        </MainLayout>
    }
}
