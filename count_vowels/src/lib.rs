use extism_pdk::*;
use serde::Serialize;

const VOWELS: &str = "aeiouAEIOU";

#[derive(Serialize)]
struct VowelReport<'a> {
    pub count: u32,
    pub total: u32,
    pub vowels: &'a str,
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

#[plugin_fn]
pub unsafe fn count_vowels<'a>(input: String) -> FnResult<Json<VowelReport<'a>>> {
    let mut count = 0;
    for ch in input.chars() {
        if VOWELS.contains(ch) {
            count += 1;
        }
    }

    let mut total = get_total();
    total += count;
    store_total(total);

    let output = VowelReport {
        count,
        total,
        vowels: VOWELS,
    };

    Ok(Json(output))
}

