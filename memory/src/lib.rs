use extism_pdk::*;
use serde::*;

#[derive(Serialize, Deserialize)]
struct AllocRequest {
    pub bytes: u64,
}

#[plugin_fn]
pub unsafe fn alloc_memory(Json(input): Json<AllocRequest>) -> FnResult<()> {
    let offs = extism::alloc(input.bytes);
    if offs == 0 {
        return Err(WithReturnCode::new(Error::msg("Failed to allocate memory"), 1));
    }

    Ok(())
}

#[plugin_fn]
pub unsafe fn alloc_var(Json(input): Json<AllocRequest>) -> FnResult<()> {
    let buffer = vec![0u8; input.bytes as usize];

    if let Err(e) = var::set("buffer", buffer) {
       return Err(WithReturnCode::from(e));
    }

    Ok(())
}