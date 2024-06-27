use extism_pdk::*;
use serde::*;

#[derive(Serialize, Deserialize)]
struct AllocRequest {
    pub bytes: u64,
}

#[plugin_fn]
pub unsafe fn alloc_memory(Json(input): Json<AllocRequest>) -> FnResult<()> {
    _ = extism::alloc(input.bytes);

    Ok(())
}

#[plugin_fn]
pub unsafe fn alloc_var(Json(input): Json<AllocRequest>) -> FnResult<()> {
    let buffer = vec![0u8; input.bytes as usize];
    _ = var::set("buffer", buffer);

    Ok(())
}