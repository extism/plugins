use extism_pdk::*;

#[plugin_fn]
pub fn http_get(Json(input): Json<HttpRequest>) -> FnResult<Memory> {
    let res = http::request::<()>(&input, None)?;
    let res = res.to_memory()?;
    Ok(res)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct HttpRequestWithBody {
    #[serde(flatten)]
    req: HttpRequest,
    data: String,
}

#[plugin_fn]
pub fn http_post(Json(input): Json<HttpRequestWithBody>) -> FnResult<Memory> {
    let res = http::request::<&str>(&input.req, Some(&input.data))?;
    let res = res.into_memory();
    Ok(res)
}
