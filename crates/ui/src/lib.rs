// UI 库入口点

mod app;
mod components;
pub mod pages;
mod services;
mod types;
mod state;

// 导出根组件
pub use app::App;
pub use state::auth::{ProvideAuthContext, use_auth};
