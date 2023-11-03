use extism_pdk::*;

#[plugin_fn]
pub unsafe fn http_get(Json(input): Json<HttpRequest>) -> FnResult<Vec<u8>> {
    let res = http::request::<()>(&input, None)?;
    let res = res.body();
    Ok(res)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct HttpRequestWithBody {
    #[serde(flatten)]
    req: HttpRequest,
    data: String,
}

#[plugin_fn]
pub unsafe fn http_post(Json(input): Json<HttpRequestWithBody>) -> FnResult<Vec<u8>> {
    let res = http::request::<&str>(&input.req, Some(&input.data))?;
    let res = res.body();
    Ok(res)
}
