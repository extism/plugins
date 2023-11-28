use extism_pdk::*;
use serde::Serialize;

const VOWELS: &str = "aeiouAEIOU";

#[derive(Serialize)]
struct VowelReport {
    pub count: u32,
    pub total: u32,
    pub vowels: String,
}

fn get_total() -> u32 {
    match var::get::<String>("total") {
        Ok(Some(v)) => v.parse::<u32>().ok().unwrap(),
        Ok(None) => 0u32,
        Err(_) => panic!("Couldn't access extism variable 'total'"),
    }
}

fn store_total(total: u32) {
    var::set("total", total.to_string()).ok();
}

fn get_vowels() -> String {
    match config::get("vowels") {
        Ok(Some(v)) => v,
        _ => VOWELS.to_string(),
    }
}

#[plugin_fn]
pub fn count_vowels(input: String) -> FnResult<Json<VowelReport>> {
    let mut count = 0;
    let vowels = get_vowels();
    for ch in input.chars() {
        if vowels.contains(ch) {
            count += 1;
        }
    }

    let mut total = get_total();
    total += count;
    store_total(total);

    let output = VowelReport {
        count,
        total,
        vowels,
    };

    Ok(Json(output))
}
