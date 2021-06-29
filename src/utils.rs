use std::u32;

use neon::{
    context::{CallContext, Context, FunctionContext, TaskContext},
    object::Object,
    prelude::Handle,
    result::JsResultExt,
    types::{JsArray, JsString, JsValue},
};

use tikv_client::{Error, Key, KvPair};

pub fn bytes_to_js_string<'a>(cx: &mut TaskContext<'a>, bytes: Vec<u8>) -> Handle<'a, JsValue> {
    let content = std::str::from_utf8(&bytes).unwrap().to_owned();
    cx.string(content).upcast()
}

// pub fn bytes_to_js_string<'a>(cx: &'a mut TaskContext, bytes: Vec<u8>) -> Handle<'a, JsValue> {
//     let content = std::str::from_utf8(&bytes).unwrap().to_owned();
//     cx.string(content).upcast()
// }

pub fn error_to_js_value<'a, T, C: Context<'a>>(
    cx: &mut C,
    err: tikv_client::Error,
) -> Handle<'a, JsValue> {
    cx.error(err.to_string()).unwrap().upcast()
}

pub fn unit_to_js_undefined<'a, T, C: Context<'a>>(cx: &mut C, _unit: T) -> Handle<'a, JsValue> {
    cx.undefined().upcast()
}

pub fn kv_pairs_to_js_array<'a>(
    cx: &mut TaskContext<'a>,
    values: Vec<KvPair>,
) -> Handle<'a, JsArray> {
    let js_array = JsArray::new(cx, values.len() as u32);
    for (i, obj) in values.iter().enumerate() {
        let pair = JsArray::new(cx, 2 as u32);
        let v1 = cx.string(
            std::str::from_utf8(&Vec::from(obj.0.clone()))
                .unwrap()
                .to_owned(),
        );
        let v2 = cx.string(std::str::from_utf8(&obj.1).unwrap().to_owned());
        pair.set(cx, 0 as u32, v1).unwrap();
        pair.set(cx, 1 as u32, v2).unwrap();
        js_array.set(cx, i as u32, pair).unwrap();
    }
    js_array
}

pub fn js_array_to_rust_iterator<'a>(
    cx: &mut FunctionContext<'a>,
    array: Handle<JsArray>,
) -> impl IntoIterator<Item = impl Into<Key>> {
    let array = array.to_vec(cx).unwrap(); // TODO: remove unwrap here
    array
        .into_iter()
        .map(|k| {
            k.downcast::<JsString, _>(cx)
                .or_throw(cx)
                .unwrap()
                .value(cx)
        })
        .collect::<Vec<String>>()
}
