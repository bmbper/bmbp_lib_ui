use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::static_embed;

#[derive(RustEmbed)]
#[folder = "web/static"]
struct BmbpStaticAssets;
///
/// bmbp/ui/lib/<**path>
pub fn build_bmbp_ui_lib_router() -> Router {
    Router::with_path("bmbp/ui/lib/<**path>").get(static_embed::<BmbpStaticAssets>())
}
