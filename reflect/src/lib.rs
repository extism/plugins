use extism_pdk::*;

#[host_fn]
extern "ExtismHost" {
    fn host_reflect(input: String) -> String;
}

#[plugin_fn]
pub unsafe fn reflect(input: String) -> FnResult<String> {
    let input = unsafe { host_reflect(input)? };
    Ok(input)
}
