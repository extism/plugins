use anyhow::Result;
use extism_pdk::*;
use serde::Serialize;

#[host_fn]
extern "ExtismHost" {
    fn kv_read(key: String) -> Vec<u8>;
    fn kv_write(key: String, value: Vec<u8>);
}

const VOWELS: &str = "aeiouAEIOU";
const KV_REPORT_KEY: &str = "count-vowels";

#[derive(Serialize)]
struct VowelReport {
    pub count: u32,
    pub total: u32,
    pub vowels: String,
}

fn get_total() -> Result<u32> {
    let v = unsafe { kv_read(KV_REPORT_KEY.into()) }?;
    // assume it's the correct size
    let array: [u8; 4] = [v[0], v[1], v[2], v[3]];
    Ok(u32::from_le_bytes(array))
}

fn store_total(total: u32) -> Result<()> {
    unsafe { kv_write(KV_REPORT_KEY.into(), total.to_le_bytes().into())? };
    Ok(())
}

fn get_vowels() -> String {
    match config::get("vowels") {
        Ok(Some(v)) => v,
        _ => VOWELS.to_string(),
    }
}

#[plugin_fn]
pub unsafe fn count_vowels(input: String) -> FnResult<Json<VowelReport>> {
    let mut count = 0;
    let vowels = get_vowels();
    for ch in input.chars() {
        if vowels.contains(ch) {
            count += 1;
        }
    }

    let mut total = get_total()?;
    total += count;
    store_total(total)?;

    let output = VowelReport {
        count,
        total,
        vowels,
    };

    Ok(Json(output))
}
