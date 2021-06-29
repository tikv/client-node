use std::ops::Bound;

use neon::{
    context::{Context, TaskContext},
    prelude::Handle,
    result::JsResultExt,
    types::{JsArray, JsString, JsValue, Value},
};
use std::ops::Bound;
use tikv_client::{Key, KvPair};

pub fn bytes_to_js_string<'a>(cx: &mut TaskContext<'a>, bytes: Vec<u8>) -> Handle<'a, JsValue> {
    let content = std::str::from_utf8(&bytes).unwrap().to_owned();
    cx.string(content).upcast()
}

// pub fn bytes_to_js_string<'a>(cx: &'a mut TaskContext, bytes: Vec<u8>) -> Handle<'a, JsValue> {
//     let content = std::str::from_utf8(&bytes).unwrap().to_owned();
//     cx.string(content).upcast()
// }

pub enum CommonTypes {
    Unit(()),
    Keys(Vec<Key>),
    KvPairs(Vec<KvPair>),
}

impl From<()> for CommonTypes {
    fn from(item: ()) -> Self {
        CommonTypes::Unit(())
    }
}

impl From<Vec<Key>> for CommonTypes {
    fn from(item: Vec<Key>) -> Self {
        CommonTypes::Keys(item)
    }
}

impl From<Vec<KvPair>> for CommonTypes {
    fn from(item: Vec<KvPair>) -> Self {
        CommonTypes::KvPairs(item)
    }
}

pub fn result_to_js_array<'a>(
    cx: &mut TaskContext<'a>,
    result: Result<CommonTypes, tikv_client::Error>,
) -> Vec<Handle<'a, JsValue>> {
    match result {
        Ok(values) => match values {
            CommonTypes::Unit(_) => vec![cx.null().upcast(), cx.undefined().upcast()],
            CommonTypes::Keys(keys) => {
                vec![cx.null().upcast(), rust_keys_to_js_array(cx, keys).upcast()]
            }
            CommonTypes::KvPairs(pairs) => vec![
                cx.null().upcast(),
                rust_pairs_to_js_array(cx, pairs).upcast(),
            ],
        },
        Err(err) => vec![
            cx.error(err.to_string()).unwrap().upcast(),
            cx.undefined().upcast(),
        ],
    }
}

pub fn error_to_js_value<'a, T, C: Context<'a>>(
    cx: &mut C,
    err: tikv_client::Error,
) -> Handle<'a, JsValue> {
    cx.error(err.to_string()).unwrap().upcast()
}

pub fn unit_to_js_undefined<'a, T, C: Context<'a>>(cx: &mut C, _unit: T) -> Handle<'a, JsValue> {
    cx.undefined().upcast()
}

pub fn rust_pairs_to_js_array<'a>(
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

pub fn rust_keys_to_js_array<'a>(cx: &mut TaskContext<'a>, keys: Vec<Key>) -> Handle<'a, JsArray> {
    let js_array = JsArray::new(cx, keys.len() as u32);
    for (i, obj) in keys.iter().enumerate() {
        let v1 = cx.string(
            std::str::from_utf8(&Vec::from(obj.clone()))
                .unwrap()
                .to_owned(),
        );
        js_array.set(cx, i as u32, v1).unwrap();
    }
    js_array
}

pub fn js_array_to_rust_keys<'a>(
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

pub fn js_array_to_rust_pairs<'a>(
    cx: &mut FunctionContext<'a>,
    array: Handle<JsArray>,
) -> impl IntoIterator<Item = impl Into<KvPair>> {
    let array = array.to_vec(cx).unwrap(); // TODO: remove unwrap here
    let mut pairs = vec![];
    for k in array.into_iter() {
        let pair_result = k.downcast::<JsArray, _>(cx).or_throw(cx);
        match pair_result {
            Ok(pair) => {
                let pair_0 = pair
                    .get(cx, 0 as u32)
                    .unwrap()
                    .downcast::<JsString, _>(cx)
                    .or_throw(cx)
                    .unwrap()
                    .value(cx); // TODO: remove unwrap here
                let pair_1 = pair
                    .get(cx, 1 as u32)
                    .unwrap()
                    .downcast::<JsString, _>(cx)
                    .or_throw(cx)
                    .unwrap()
                    .value(cx); // TODO: remove unwrap here
                pairs.push(KvPair::new(pair_0, pair_1));
            }
            Err(err) => println!("{}", err.to_string()),
        }
    }
    pairs
}

pub fn to_bound_range(
    start: Option<Vec<u8>>,
    end: Option<Vec<u8>>,
    include_start: bool,
    include_end: bool,
) -> tikv_client::BoundRange {
    let start_bound = if let Some(start) = start {
        if include_start {
            Bound::Included(start)
        } else {
            Bound::Excluded(start)
        }
    } else {
        Bound::Unbounded
    };
    let end_bound = if let Some(end) = end {
        if include_end {
            Bound::Included(end)
        } else {
            Bound::Excluded(end)
        }
    } else {
        Bound::Unbounded
    };
    tikv_client::BoundRange::from((start_bound, end_bound))
}
