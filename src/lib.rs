use std::{convert::TryInto, sync::Arc};

use lazy_static::lazy_static;
use neon::{handle::Managed, prelude::*};
use tokio::runtime::Runtime;
lazy_static! {
    pub(crate) static ref RUNTIME: Runtime = Runtime::new().unwrap();
}
pub struct RawClient {
    inner: Arc<tikv_client::RawClient>,
}

impl Finalize for RawClient {}

impl RawClient {
    pub fn connect(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let pd_endpoint = cx.argument::<JsString>(0)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let result = tikv_client::RawClient::new(vec![pd_endpoint]);
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let result = result.await;
            queue.send(move |mut cx| {
                let callback = callback.into_inner(&mut cx);
                let this = cx.undefined();
                let args: Vec<Handle<JsValue>> = match result {
                    Ok(client) => vec![
                        cx.null().upcast(),
                        cx.boxed(RawClient {
                            inner: Arc::new(client),
                        })
                        .upcast(),
                    ],
                    Err(err) => vec![cx.error(err.to_string())?.upcast()],
                };
                callback.call(&mut cx, this, args)?;
                Ok(())
            });
        });
        Ok(cx.undefined())
    }

    pub fn put(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;
        let key = cx.argument::<JsString>(0)?.value(&mut cx);
        let value = cx.argument::<JsString>(1)?.value(&mut cx);
        let cf = cx.argument::<JsString>(2)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(3)?.root(&mut cx);

        let inner = client.inner.with_cf(cf.try_into().unwrap());
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let result = inner.put(key, value).await;
            queue.send(move |mut cx| {
                let callback = callback.into_inner(&mut cx);
                let this = cx.undefined();
                let args: Vec<Handle<JsValue>> = vec![
                    match result {
                        Ok(_) => cx.null().upcast(),
                        Err(err) => cx.error(err.to_string())?.upcast(),
                    },
                    cx.undefined().upcast(),
                ];
                callback.call(&mut cx, this, args)?;
                Ok(())
            });
        });

        Ok(cx.undefined())
    }

    pub fn get(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;
        let key = cx.argument::<JsString>(0)?.value(&mut cx);
        let cf = cx.argument::<JsString>(1)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);

        let inner = client.inner.with_cf(cf.try_into().unwrap());
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let value: Option<Vec<u8>> = inner.get(key).await.unwrap();
            queue.send(move |mut cx| {
                let callback = callback.into_inner(&mut cx);
                let this = cx.undefined();
                let args: Vec<Handle<JsValue>> = match value {
                    Some(content) => {
                        let content = std::str::from_utf8(&content).unwrap().to_owned();
                        // let js_array = JsArray::new(&mut cx, content.len() as u32);
                        // for (i, obj) in content.iter().enumerate() {
                        //     let js_string = cx.number(*obj as f64);
                        //     js_array.set(&mut cx, i as u32, js_string).unwrap();
                        // }
                        // vec![cx.undefined().upcast(), js_array.upcast()]
                        vec![cx.undefined().upcast(), cx.string(content).upcast()]
                    }
                    None => vec![cx.null().upcast(), cx.undefined().upcast()],
                };
                callback.call(&mut cx, this, args)?;
                Ok(())
            });
        });

        Ok(cx.undefined())
    }

    pub fn delete(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;
        let key = cx.argument::<JsString>(0)?.value(&mut cx);
        let cf = cx.argument::<JsString>(1)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);

        let inner = client.inner.with_cf(cf.try_into().unwrap());
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let result = inner.delete(key).await;
            queue.send(move |mut cx| {
                let callback = callback.into_inner(&mut cx);
                let this = cx.undefined();
                let args: Vec<Handle<JsValue>> = vec![
                    match result {
                        Ok(_) => cx.null().upcast(),
                        Err(err) => cx.error(err.to_string())?.upcast(),
                    },
                    cx.undefined().upcast(),
                ];
                callback.call(&mut cx, this, args)?;
                Ok(())
            });
        });

        Ok(cx.undefined())
    }

    pub fn batch_get(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;
        let keys = cx.argument::<JsArray>(0)?.to_vec(&mut cx)?;
        let keys = keys
            .into_iter()
            .map(|k| {
                k.downcast::<JsString, _>(&mut cx)
                    .or_throw(&mut cx)
                    .unwrap()
                    .value(&mut cx)
            })
            .collect::<Vec<String>>();
        let cf = cx.argument::<JsString>(1)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);

        let inner = client.inner.with_cf(cf.try_into().unwrap());
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let result = inner.batch_get(keys).await;
            queue.send(move |mut cx| {
                let callback = callback.into_inner(&mut cx);
                let this = cx.undefined();
                let args: Vec<Handle<JsValue>> = match result {
                    Ok(values) => {
                        let js_array = JsArray::new(&mut cx, values.len() as u32);
                        for (i, obj) in values.iter().enumerate() {
                            let pair = JsArray::new(&mut cx, 2 as u32);
                            let v1 = cx.string(
                                std::str::from_utf8(&Vec::from(obj.0.clone()))
                                    .unwrap()
                                    .to_owned(),
                            );
                            let v2 = cx.string(std::str::from_utf8(&obj.1).unwrap().to_owned());
                            pair.set(&mut cx, 0 as u32, v1)?;
                            pair.set(&mut cx, 1 as u32, v2)?;
                            js_array.set(&mut cx, i as u32, pair).unwrap();
                        }
                        vec![cx.null().upcast(), js_array.upcast()]
                    }
                    Err(err) => vec![cx.error(err.to_string())?.upcast(), cx.undefined().upcast()],
                };
                callback.call(&mut cx, this, args)?;
                Ok(())
            });
        });

        Ok(cx.undefined())
    }

    pub fn scan(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;

        todo!()
    }

    pub fn scan_keys(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;

        todo!()
    }

    pub fn batch_put(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;

        todo!()
    }

    pub fn batch_delete(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;

        todo!()
    }

    pub fn delete_range(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;

        todo!()
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("connect", RawClient::connect)?;
    cx.export_function("put", RawClient::put)?;
    cx.export_function("get", RawClient::get)?;
    cx.export_function("delete", RawClient::delete)?;
    cx.export_function("batch_get", RawClient::batch_get)?;
    cx.export_function("scan", RawClient::scan)?;
    cx.export_function("scan_keys", RawClient::scan_keys)?;
    cx.export_function("batch_put", RawClient::batch_put)?;
    cx.export_function("batch_delete", RawClient::batch_delete)?;
    cx.export_function("delete_range", RawClient::delete_range)?;
    Ok(())
}
