use crate::{
    utils::{
        bytes_to_js_string, js_array_to_rust_keys, js_array_to_rust_pairs, send_result,
        to_bound_range, RUNTIME,
    },
    RawClient,
};
use neon::prelude::*;
use std::{convert::TryInto, u32};

impl RawClient {
    pub fn connect(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let pd_endpoint = cx.argument::<JsString>(0)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let result = tikv_client::RawClient::new(vec![pd_endpoint]);
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let result = result.await;
            send_result(queue, callback, result).unwrap();
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
            send_result(queue, callback, result).unwrap();
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
                        // let js_array = JsArray::new(&mut cx, content.len() as u32);
                        // for (i, obj) in content.iter().enumerate() {
                        //     let js_string = cx.number(*obj as f64);
                        //     js_array.set(&mut cx, i as u32, js_string).unwrap();
                        // }
                        // vec![cx.undefined().upcast(), js_array.upcast()]

                        vec![cx.null().upcast(), bytes_to_js_string(&mut cx, content)]
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
            send_result(queue, callback, result).unwrap();
        });

        Ok(cx.undefined())
    }

    pub fn batch_get(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;
        let keys = cx.argument::<JsArray>(0)?;
        let keys = js_array_to_rust_keys(&mut cx, keys);
        let cf = cx.argument::<JsString>(1)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);

        let inner = client.inner.with_cf(cf.try_into().unwrap());
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let result = inner.batch_get(keys).await;
            send_result(queue, callback, result).unwrap();
        });

        Ok(cx.undefined())
    }

    pub fn scan(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;
        let start = cx.argument::<JsString>(0)?.value(&mut cx).into_bytes();
        let end = cx.argument::<JsString>(1)?.value(&mut cx).into_bytes();
        let limit = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;
        let include_start = cx.argument::<JsBoolean>(3)?.value(&mut cx);
        let include_end = cx.argument::<JsBoolean>(4)?.value(&mut cx);
        let cf = cx.argument::<JsString>(5)?.value(&mut cx);

        let callback = cx.argument::<JsFunction>(6)?.root(&mut cx);
        let inner = client.inner.with_cf(cf.try_into().unwrap());
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let range = to_bound_range(Some(start), Some(end), include_start, include_end);

            let result = inner.scan(range, limit).await;
            send_result(queue, callback, result).unwrap();
        });

        Ok(cx.undefined())
    }

    pub fn scan_keys(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;
        let start = cx.argument::<JsString>(0)?.value(&mut cx).into_bytes();
        let end = cx.argument::<JsString>(1)?.value(&mut cx).into_bytes();
        let limit = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;
        let include_start = cx.argument::<JsBoolean>(3)?.value(&mut cx);
        let include_end = cx.argument::<JsBoolean>(4)?.value(&mut cx);
        let cf = cx.argument::<JsString>(5)?.value(&mut cx);

        let callback = cx.argument::<JsFunction>(6)?.root(&mut cx);
        let inner = client.inner.with_cf(cf.try_into().unwrap());
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let range = to_bound_range(Some(start), Some(end), include_start, include_end);

            let result = inner.scan_keys(range, limit).await;
            send_result(queue, callback, result).unwrap();
        });

        Ok(cx.undefined())
    }

    pub fn batch_put(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;
        let pairs = cx.argument::<JsArray>(0)?;
        let pairs = js_array_to_rust_pairs(&mut cx, pairs);
        let cf = cx.argument::<JsString>(1)?.value(&mut cx);

        let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);
        let inner = client.inner.with_cf(cf.try_into().unwrap());
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let result = inner.batch_put(pairs).await;
            send_result(queue, callback, result).unwrap();
        });

        Ok(cx.undefined())
    }

    pub fn batch_delete(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;
        let keys = cx.argument::<JsArray>(0)?;
        let keys = js_array_to_rust_keys(&mut cx, keys);
        let cf = cx.argument::<JsString>(1)?.value(&mut cx);

        let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);
        let inner = client.inner.with_cf(cf.try_into().unwrap());
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let result = inner.batch_delete(keys).await;
            send_result(queue, callback, result).unwrap();
        });

        Ok(cx.undefined())
    }

    pub fn delete_range(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<RawClient>, _>(&mut cx)?;
        let queue = cx.queue();
        let start = cx.argument::<JsString>(0)?.value(&mut cx).into_bytes();
        let end = cx.argument::<JsString>(1)?.value(&mut cx).into_bytes();
        let include_start = cx.argument::<JsBoolean>(2)?.value(&mut cx);
        let include_end = cx.argument::<JsBoolean>(3)?.value(&mut cx);
        let cf = cx.argument::<JsString>(4)?.value(&mut cx);
        let inner = client.inner.with_cf(cf.try_into().unwrap());

        let callback = cx.argument::<JsFunction>(5)?.root(&mut cx);

        RUNTIME.spawn(async move {
            let range = to_bound_range(Some(start), Some(end), include_start, include_end);

            let result = inner.delete_range(range).await;
            send_result(queue, callback, result).unwrap();
        });

        Ok(cx.undefined())
    }
}
