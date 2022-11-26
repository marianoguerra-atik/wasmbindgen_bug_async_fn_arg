use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_js(s: &JsValue);
}

#[wasm_bindgen]
pub async fn call_fn_async(fun: &js_sys::Function) -> Result<String, JsValue> {
    let this = JsValue::null();
    log_js(fun);
    let info = JsValue::from("hello async");
    match fun.call1(&this, &info) {
        Ok(_) => {}
        Err(err) => {
            log_js(&err);
        }
    }
    Ok("result".into())
}

#[wasm_bindgen]
pub fn call_fn_sync(fun: &js_sys::Function) -> Result<String, JsValue> {
    let this = JsValue::null();
    log_js(fun);
    let info = JsValue::from("hello sync");
    match fun.call1(&this, &info) {
        Ok(_) => {}
        Err(err) => {
            log_js(&err);
        }
    }
    Ok("result".into())
}
