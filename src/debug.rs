use wasm_bindgen::prelude::*;

#[allow(dead_code)]
pub fn log_1<T>(x: T)
where
    JsValue: From<T>,
{
    web_sys::console::log_1(&JsValue::from(x));
}

#[allow(dead_code)]
pub fn log_2<T, U>(x1: T, x2: U)
where
    JsValue: From<T>,
    JsValue: From<U>,
{
    web_sys::console::log_2(&JsValue::from(x1), &JsValue::from(x2));
}

#[allow(dead_code)]
pub fn log_3<T, U, V>(x1: T, x2: U, x3: V)
where
    JsValue: From<T>,
    JsValue: From<U>,
    JsValue: From<V>,
{
    web_sys::console::log_3(&JsValue::from(x1), &JsValue::from(x2), &JsValue::from(x3));
}

#[allow(dead_code)]
pub fn log_4<T, U, V, W>(x1: T, x2: U, x3: V, x4: W)
where
    JsValue: From<T>,
    JsValue: From<U>,
    JsValue: From<V>,
    JsValue: From<W>,
{
    web_sys::console::log_4(
        &JsValue::from(x1),
        &JsValue::from(x2),
        &JsValue::from(x3),
        &JsValue::from(x4),
    );
}
