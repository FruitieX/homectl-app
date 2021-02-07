// from https://rustwasm.github.io/wasm-bindgen/examples/console-log.html
#[macro_export]
macro_rules! log {
    // no idea why this needs to be wrapped in unsafe {} for rust-analyzer to be
    // happy
    ($($t:tt)*) => (unsafe {
      web_sys::console::log_1(&format_args!($($t)*).to_string().into())
    })
}