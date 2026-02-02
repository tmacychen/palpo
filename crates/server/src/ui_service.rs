// 服务器端 UI 静态文件服务

use salvo::prelude::*;
use salvo::http::StatusCode;

/// 开发模式：从文件系统加载
#[cfg(debug_assertions)]
pub fn router() -> Router {
    Router::with_path("_admin")
        .get(serve_index_dev)
        .push(Router::with_path("<**path>").get(serve_static_dev))
}

/// 生产模式：从嵌入资源加载
#[cfg(not(debug_assertions))]
pub fn router() -> Router {
    Router::with_path("_admin")
        .get(serve_index_prod)
        .push(Router::with_path("<**path>").get(serve_static_prod))
}

/// 开发模式：从 Trunk dev server 代理
#[cfg(debug_assertions)]
async fn serve_index_dev(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    // 开发模式下，重定向到 Trunk dev server
    res.render(Redirect::found("http://localhost:8080/"));
}

#[cfg(debug_assertions)]
async fn serve_static_dev(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let path: String = req.param("path").unwrap_or_default();
    // 重定向到 Trunk dev server
    res.render(Redirect::found(format!("http://localhost:8080/{}", path)));
}

/// 生产模式：从嵌入的静态文件加载
#[cfg(not(debug_assertions))]
static INDEX_HTML: &[u8] = include_bytes!("../../ui/dist/index.html");

#[cfg(not(debug_assertions))]
static WASM_APP: &[u8] = include_bytes!("../../ui/dist/palpo-ui_bg.wasm");

#[cfg(not(debug_assertions))]
static JS_APP: &[u8] = include_bytes!("../../ui/dist/palpo-ui.js");

#[cfg(not(debug_assertions))]
async fn serve_index_prod(_req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.status_code(StatusCode::OK)
        .content_type("text/html")
        .render(INDEX_HTML);
}

#[cfg(not(debug_assertions))]
async fn serve_static_prod(req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    let path: String = req.param("path").unwrap_or_default();
    
    match path.as_str() {
        "palpo-ui_bg.wasm" => {
            res.status_code(StatusCode::OK)
                .content_type("application/wasm")
                .render(WASM_APP);
        }
        "palpo-ui.js" => {
            res.status_code(StatusCode::OK)
                .content_type("application/javascript")
                .render(JS_APP);
        }
        _ => {
            // 其他静态文件（CSS 等）
            res.status_code(StatusCode::NOT_FOUND)
                .render("Not Found");
        }
    }
}
