// WASM 应用程序入口点

use palpo_ui::app::App;

fn main() {
    // 初始化日志
    wasm_logger::init(wasm_logger::Config::default());
    
    // 设置 panic hook 以在控制台显示错误
    console_error_panic_hook::set_once();
    
    log::info!("Palpo Admin UI 启动中...");
    
    // 挂载 Leptos 应用
    leptos::mount_to_body(App);
}
