use std::time::Duration;
use extism_pdk::*;
use serde::Deserialize;

#[link(wasm_import_module = "extism:host/user")]
extern "C" {
    fn get_now_ms() -> u64;
    fn notify() -> ();
}

#[derive(Deserialize)]
struct SleepRequest {
    duration_ms: u64,
}

#[plugin_fn]
pub fn sleep(Json(input): Json<SleepRequest>) -> FnResult<()> {
    let duration = std::time::Duration::from_millis(input.duration_ms);
    let then = get_now();

    while then + duration > get_now() {
        // do nothing
    }
    _ = unsafe { notify() };

    Ok(())
}

fn get_now() -> Duration {
    let now_ms = unsafe { get_now_ms() };
    std::time::Duration::from_millis(now_ms)
}
