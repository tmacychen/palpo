// WASM 应用程序入口点

mod app;
mod components;
mod pages;
mod services;
mod types;
mod state;

use app::App;
use leptos::*;

fn main() {
    // 初始化日志
    wasm_logger::init(wasm_logger::Config::default());

    // 设置 panic hook 以在控制台显示错误
    console_error_panic_hook::set_once();

    log::info!("Palpo Admin UI 启动中...");

    // 挂载 Leptos 应用到 #app 容器，包含认证上下文
    mount_to_body(|| view! {
        <crate::state::auth::ProvideAuthContext>
            <App/>
        </crate::state::auth::ProvideAuthContext>
    });
}
