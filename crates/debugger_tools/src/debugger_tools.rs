mod dap_log;
pub use dap_log::*;

use gpui_runtime::App;

pub fn init(cx: &mut App) {
    dap_log::init(cx);
}
