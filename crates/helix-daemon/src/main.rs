#![forbid(unsafe_code)]

use helix_xlx::XlxBridgeConfig;

fn main() {
    let xlx = XlxBridgeConfig::default();

    println!("Helix Voice Engine research daemon {}", env!("CARGO_PKG_VERSION"));
    println!("XLX integration mode: {:?}", xlx.mode);
    println!("No proprietary codec implementation is included.");
}
