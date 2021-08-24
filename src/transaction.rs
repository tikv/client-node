use std::iter::FromIterator;

use crate::{
    utils::{
        bytes_to_js_string, js_array_to_rust_keys, js_array_to_rust_pairs, result_to_js_array,
        send_result, to_bound_range, CommonTypes, RUNTIME,
    },
    Transaction, TransactionClient,
};
use neon::prelude::*;
use tikv_client::{KvPair, TimestampExt};

impl TransactionClient {
    pub fn connect(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let pd_endpoint = cx.argument::<JsString>(0)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let result = tikv_client::TransactionClient::new(vec![pd_endpoint]);
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let result = result.await;
            send_result(queue, callback, result).unwrap();
        });
        Ok(cx.undefined())
    }

    pub fn begin(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<TransactionClient>, _>(&mut cx)?;
        let pessimistic = cx.argument::<JsBoolean>(0)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let inner = client.inner.clone();

        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let inner = if pessimistic {
                inner.begin_pessimistic().await
            } else {
                inner.begin_optimistic().await
            };
            send_result(queue, callback, inner).unwrap();
        });
        Ok(cx.undefined())
    }
}

impl Transaction {
    pub fn get(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        let key = cx.argument::<JsString>(0)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let value = inner.lock().await.get(key).await.unwrap(); //TODO: this is a wrong implementation
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

    pub fn get_for_update(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        let key = cx.argument::<JsString>(0)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let value = inner.lock().await.get_for_update(key).await.unwrap(); //TODO: this is a wrong implementation
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

    pub fn key_exists(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        let key = cx.argument::<JsString>(0)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let value = inner.lock().await.key_exists(key).await.unwrap(); //TODO: this is a wrong implementation
            queue.send(move |mut cx| {
                let callback = callback.into_inner(&mut cx);
                let this = cx.undefined();
                let args: Vec<Handle<JsValue>> =
                    vec![cx.null().upcast(), cx.boolean(value).upcast()];
                callback.call(&mut cx, this, args)?;
                Ok(())
            });
        });

        Ok(cx.undefined())
    }

    pub fn batch_get(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        // let client = cx
        //     .this()
        //     .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        // let keys = cx.argument::<JsArray>(0)?;
        // let keys = js_array_to_rust_keys(&mut cx, keys);
        // let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);

        // let inner = client.inner.clone();
        // let queue = cx.queue();

        // RUNTIME.spawn(async move {
        //     let result = inner.lock().await.batch_get(keys).await;
        // });

        // Ok(cx.undefined())
        todo!()
    }

    pub fn batch_get_for_update(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        todo!()
    }

    pub fn scan(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        todo!()
    }

    pub fn scan_keys(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        todo!()
    }

    pub fn lock_keys(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        todo!()
    }

    pub fn put(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        let key = cx.argument::<JsString>(0)?.value(&mut cx);
        let value = cx.argument::<JsString>(1)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);
        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let result = inner.lock().await.put(key, value).await;
            send_result(queue, callback, result).unwrap();
        });

        Ok(cx.undefined())
    }

    pub fn insert(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        let key = cx.argument::<JsString>(0)?.value(&mut cx);
        let value = cx.argument::<JsString>(1)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);
        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let result = inner.lock().await.insert(key, value).await;
            send_result(queue, callback, result).unwrap();
        });

        Ok(cx.undefined())
    }

    pub fn delete(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        let key = cx.argument::<JsString>(0)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let result = inner.lock().await.delete(key).await;
            send_result(queue, callback, result).unwrap();
        });

        Ok(cx.undefined())
    }

    pub fn commit(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let result = inner.lock().await.commit().await;
            send_result(queue, callback, result).unwrap();
        });

        Ok(cx.undefined())
    }
}
