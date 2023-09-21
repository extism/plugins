use extism_pdk::*;

#[plugin_fn]
pub fn loop_forever(_: ()) -> FnResult<()> {
    loop {}
    Ok(())
}
