use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::static_embed;

#[derive(RustEmbed)]
#[folder = "web/static"]
struct BmbpStaticAssets;
/// use embed react bootstrap react file in static,the path is `bmbp/ui/lib/<path>`
/// # arco
///
/// ```
/// bmbp/ui/lib/arco/arco-icon.min.js
/// bmbp/ui/lib/arco/arco.min.css
/// bmbp/ui/lib/arco/arco.min.js
/// ```
/// # bmbp style
///
/// ```
/// bmbp/ui/lib/bmbp/bmbp.css
/// ```
///
/// # bootstrap5.3.0
///
/// ```
/// bmbp/ui/lib/bootstrap5/css/bootstrap.css
/// bmbp/ui/lib/bootstrap5/css/bootstrap.min.css
/// bmbp/ui/lib/bootstrap5/js/bootstrap.js
/// bmbp/ui/lib/bootstrap5/js/bootstrap.min.js
/// bmbp/ui/lib/bootstrap5/js/popper.min.js
/// ```
///
/// # Reactjs
///
/// ```
/// bmbp/ui/lib/react/react-dom.production.min.js
/// bmbp/ui/lib/react/react-dom.profiling.min.js
/// bmbp/ui/lib/react/react.production.min.js
/// bmbp/ui/lib/axios.min.js
/// ```
///
/// ## example
/// 
/// `Cargo.toml`
/// ```toml
/// 
/// [dependencies]
/// bmbp_lib_ui = "0.1.1"
/// rust-embed = "8.5.0"
/// salvo = {version="0.69.0", features = ["serve-static"] }
/// tokio = { version = "1.38.0", features = ["full"] }
/// tracing = "0.1.40"
/// tracing-subscriber = { version = "0.3.16", features = ["env-filter"] }
/// ```
/// 
/// #### code
/// ```rust
/// use bmbp_lib_ui::*;
/// use salvo::prelude::*;
/// #[tokio:main]
/// async fn main() {
///   tracing_subscriber::fmt().init();
///   let host = "0.0.0.0:7027";
///   tracing::info!("启动初始化服务,监听地址:{}......", host);
///   let acceptor = TcpListener::new(host).bind().await;
///   let router = build_bmbp_ui_lib_router();
///   Server::new(acceptor).serve(router).await;
/// }
/// 
/// ```
/// 
/// 
pub fn build_bmbp_ui_lib_router() -> Router {
    Router::with_path("bmbp/ui/lib/<**path>").get(static_embed::<BmbpStaticAssets>())
}
