use std::ops::Bound;
use std::{sync::Arc, u32};

use neon::prelude::*;
use neon::{
    context::{Context, TaskContext},
    prelude::Handle,
    result::JsResultExt,
    types::{JsArray, JsString, JsValue},
};
use tikv_client::{Key, KvPair};

use tikv_client::TimestampExt;

use crate::{
    error::CustomError, error::TRANSACTION_ERROR, RawClient, Snapshot, Transaction,
    TransactionClient,
};
use lazy_static::lazy_static;
use tokio::{runtime::Runtime, sync::Mutex};

lazy_static! {
    pub(crate) static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

pub fn bytes_to_js_string<'a>(cx: &mut TaskContext<'a>, bytes: Vec<u8>) -> Handle<'a, JsValue> {
    let content = std::str::from_utf8(&bytes).unwrap().to_owned();
    cx.string(content).upcast()
}

// pub fn bytes_to_js_string<'a>(cx: &mut TaskContext<'a>, bytes: Vec<u8>) -> Handle<'a, JsValue> {
//     let content = std::str::from_utf8(&bytes).unwrap().to_owned();
//     cx.string(content).upcast()
// }

pub trait ToJS: 'static + Send {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue>;
}

impl ToJS for () {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue> {
        cx.undefined().upcast()
    }
}

impl ToJS for Vec<Key> {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue> {
        rust_keys_to_js_array(cx, self).upcast()
    }
}

impl ToJS for Key {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue> {
        bytes_to_js_string(cx, self.into())
    }
}

impl ToJS for tikv_client::Value {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue> {
        bytes_to_js_string(cx, self.into())
    }
}

impl ToJS for Vec<KvPair> {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue> {
        rust_pairs_to_js_array(cx, self).upcast()
    }
}

impl ToJS for tikv_client::RawClient {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue> {
        cx.boxed(RawClient {
            inner: Arc::new(self),
        })
        .upcast()
    }
}

impl ToJS for tikv_client::TransactionClient {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue> {
        cx.boxed(TransactionClient {
            inner: Arc::new(self),
        })
        .upcast()
    }
}

impl ToJS for tikv_client::Transaction {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue> {
        cx.boxed(Transaction {
            inner: Arc::new(Mutex::new(self)),
        })
        .upcast()
    }
}

impl ToJS for tikv_client::Snapshot {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue> {
        cx.boxed(Snapshot {
            inner: Arc::new(Mutex::new(self)),
        })
        .upcast()
    }
}

impl<T: ToJS> ToJS for Option<T> {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue> {
        match self {
            None => cx.undefined().upcast(),
            Some(t) => t.to_js_value(cx),
        }
    }
}

impl ToJS for tikv_client::Timestamp {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue> {
        cx.number(self.version() as f64).upcast()
    }
}

impl ToJS for bool {
    fn to_js_value<'a>(self, cx: &mut TaskContext<'a>) -> Handle<'a, JsValue> {
        cx.boolean(self).upcast()
    }
}

pub fn rust_pairs_to_js_array<'a>(
    cx: &mut TaskContext<'a>,
    values: Vec<KvPair>,
) -> Handle<'a, JsArray> {
    let js_array = JsArray::new(cx, values.len() as u32);
    for (i, obj) in values.iter().enumerate() {
        let pair = JsArray::new(cx, 2);
        let v1 = cx.string(
            std::str::from_utf8(&Vec::from(obj.0.clone()))
                .unwrap()
                .to_owned(),
        );
        let v2 = cx.string(std::str::from_utf8(&obj.1).unwrap().to_owned());
        pair.set(cx, 0, v1).unwrap();
        pair.set(cx, 1, v2).unwrap();
        js_array.set(cx, i as u32, pair).unwrap();
    }
    js_array
}

pub fn rust_keys_to_js_array<'a>(cx: &mut TaskContext<'a>, keys: Vec<Key>) -> Handle<'a, JsArray> {
    let js_array = JsArray::new(cx, keys.len() as u32);
    for (i, obj) in keys.into_iter().enumerate() {
        let v1 = obj.to_js_value(cx);
        js_array.set(cx, i as u32, v1).unwrap();
    }
    js_array
}

pub fn js_array_to_rust_keys<'a>(
    cx: &mut FunctionContext<'a>,
    array: Handle<JsArray>,
) -> Vec<String> {
    let array = array.to_vec(cx).unwrap(); // TODO: #21 remove unwrap here
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
    let array = array.to_vec(cx).unwrap(); // TODO: #21 remove unwrap here
    let mut pairs = vec![];
    for k in array.into_iter() {
        let pair_result = k.downcast::<JsArray, _>(cx).or_throw(cx);
        match pair_result {
            Ok(pair) => {
                let args: Vec<String> = vec![0_u32, 1_u32]
                    .into_iter()
                    .map(|i| {
                        pair.get(cx, i as u32)
                            .unwrap()
                            .downcast::<JsString, _>(cx)
                            .or_throw(cx)
                            .unwrap() // TODO: #21 remove unwrap here
                            .value(cx)
                    })
                    .collect();
                pairs.push(KvPair::new(
                    args.get(0).unwrap().to_owned(),
                    args.get(1).unwrap().to_owned(),
                ));
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

pub fn send_result<T: ToJS>(
    // TODO: #18 do I have to use static lifetime here?
    queue: EventQueue,
    callback: Root<JsFunction>,
    result: Result<T, tikv_client::Error>,
) {
    queue.send(move |mut cx| {
        let result = result.map(|op| op.to_js_value(&mut cx));
        let callback = callback.into_inner(&mut cx);
        let this = cx.undefined();
        let args: Vec<Handle<JsValue>> = match result {
            Ok(values) => vec![cx.null().upcast(), values],
            Err(err) => match err {
                err @ tikv_client::Error::OperationAfterCommitError => vec![
                    TRANSACTION_ERROR
                        .throw(&mut cx, vec![err.to_string()])
                        .unwrap()
                        .upcast(),
                    cx.undefined().upcast(),
                ],
                tikv_client::Error::UndeterminedError(e) => vec![
                    TRANSACTION_ERROR
                        .throw(&mut cx, vec![format!("WriteConlict: {:?}", &e.to_string())])
                        .unwrap()
                        .upcast(),
                    cx.undefined().upcast(),
                ],
                tikv_client::Error::KeyError(e) => {
                    if let Some(conflict) = e.conflict {
                        vec![
                            TRANSACTION_ERROR
                                .throw(&mut cx, vec![format!("WriteConlict: {:?}", conflict)])
                                .unwrap()
                                .upcast(),
                            cx.undefined().upcast(),
                        ]
                    } else if let Some(already_exist) = e.already_exist {
                        vec![
                            TRANSACTION_ERROR
                                .throw(&mut cx, vec![format!("AlreadyExist: {:?}", already_exist)])
                                .unwrap()
                                .upcast(),
                            cx.undefined().upcast(),
                        ]
                    } else if let Some(deadlock) = e.deadlock {
                        vec![
                            TRANSACTION_ERROR
                                .throw(&mut cx, vec![format!("Daedlock: {:?}", deadlock)])
                                .unwrap()
                                .upcast(),
                            cx.undefined().upcast(),
                        ]
                    } else {
                        vec![
                            cx.error(format!("KeyError: {:?}", e)).unwrap().upcast(),
                            cx.undefined().upcast(),
                        ]
                    }
                }
                _ => vec![
                    cx.error(err.to_string()).unwrap().upcast(),
                    cx.undefined().upcast(),
                ],
            },
        };
        callback.call(&mut cx, this, args)?;
        Ok(())
    });
}
