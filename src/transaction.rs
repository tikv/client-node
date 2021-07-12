use std::sync::Arc;

use crate::{
    utils::{js_array_to_rust_keys, send_result, to_bound_range, RUNTIME},
    Snapshot, Transaction, TransactionClient,
};
use neon::prelude::*;
use tikv_client::TimestampExt as _;
use tikv_client::TransactionOptions;
use tikv_client::{Key, KvPair};

impl TransactionClient {
    pub fn connect(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let pd_endpoint = cx.argument::<JsString>(0)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let result = tikv_client::TransactionClient::new(vec![pd_endpoint], None);
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let result = result.await;
            send_result(queue, callback, result);
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
            send_result(queue, callback, inner);
        });
        Ok(cx.undefined())
    }

    pub fn snapshot(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<TransactionClient>, _>(&mut cx)?;
        let timestamp = cx.argument::<JsNumber>(0)?.value(&mut cx) as u64;
        let pessimistic = cx.argument::<JsBoolean>(1)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);
        let inner = client.inner.clone();

        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let inner = inner.snapshot(
                tikv_client::Timestamp::from_version(timestamp),
                if pessimistic {
                    TransactionOptions::new_pessimistic()
                } else {
                    TransactionOptions::new_optimistic()
                },
            );
            send_result(queue, callback, Ok(inner));
        });
        Ok(cx.undefined())
    }

    pub fn current_timestamp(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<TransactionClient>, _>(&mut cx)?;
        let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
        let inner = client.inner.clone();

        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let result = inner.current_timestamp().await;
            send_result(queue, callback, result.map(|op| Some(op)));
        });
        Ok(cx.undefined())
    }

    pub fn gc(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<TransactionClient>, _>(&mut cx)?;
        let safepoint = cx.argument::<JsNumber>(0)?.value(&mut cx) as u64;
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let inner = client.inner.clone();

        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let result = inner
                .gc(tikv_client::Timestamp::from_version(safepoint))
                .await;
            send_result(queue, callback, result);
        });
        Ok(cx.undefined())
    }
}

impl Snapshot {
    pub fn get(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx.this().downcast_or_throw::<JsBox<Snapshot>, _>(&mut cx)?;
        let key = cx.argument::<JsString>(0)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let value = inner.lock().await.get(key).await.unwrap();
            send_result(queue, callback, Ok(value));
        });

        Ok(cx.undefined())
    }

    pub fn key_exists(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx.this().downcast_or_throw::<JsBox<Snapshot>, _>(&mut cx)?;
        let key = cx.argument::<JsString>(0)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let value = inner.lock().await.key_exists(key).await.unwrap();
            send_result(queue, callback, Ok(value));
        });

        Ok(cx.undefined())
    }

    pub fn batch_get(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx.this().downcast_or_throw::<JsBox<Snapshot>, _>(&mut cx)?;
        let keys = cx.argument::<JsArray>(0)?;
        let keys: Vec<String> = js_array_to_rust_keys(&mut cx, keys);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let result = inner
                .lock()
                .await
                .batch_get(keys)
                .await
                .map(|kvpairs| kvpairs.collect::<Vec<KvPair>>());
            send_result(queue, callback, result);
        });

        Ok(cx.undefined())
    }

    pub fn scan(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx.this().downcast_or_throw::<JsBox<Snapshot>, _>(&mut cx)?;
        let start = cx.argument_opt(0).map(|start| {
            start
                .downcast::<JsString, _>(&mut cx)
                .or_throw(&mut cx)
                .unwrap()
                .value(&mut cx)
                .into_bytes()
        });
        let end = cx.argument_opt(1).map(|end| {
            end.downcast::<JsString, _>(&mut cx)
                .or_throw(&mut cx)
                .unwrap()
                .value(&mut cx)
                .into_bytes()
        });
        let limit = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;
        let include_start = cx.argument::<JsBoolean>(3)?.value(&mut cx);
        let include_end = cx.argument::<JsBoolean>(4)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(5)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let range = to_bound_range(start, end, include_start, include_end);

            let result = inner
                .lock()
                .await
                .scan(range, limit)
                .await
                .map(|kvpairs| kvpairs.collect::<Vec<KvPair>>());
            send_result(queue, callback, result);
        });

        Ok(cx.undefined())
    }

    pub fn scan_keys(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx.this().downcast_or_throw::<JsBox<Snapshot>, _>(&mut cx)?;
        let start = cx.argument::<JsString>(0)?.value(&mut cx).into_bytes();
        let end = cx.argument::<JsString>(1)?.value(&mut cx).into_bytes();
        let limit = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;
        let include_start = cx.argument::<JsBoolean>(3)?.value(&mut cx);
        let include_end = cx.argument::<JsBoolean>(4)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(5)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let range = to_bound_range(Some(start), Some(end), include_start, include_end);

            let result = inner
                .lock()
                .await
                .scan_keys(range, limit)
                .await
                .map(|kvpairs| kvpairs.collect::<Vec<Key>>());
            send_result(queue, callback, result);
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
            let value = inner.lock().await.get(key).await.unwrap();
            send_result(queue, callback, Ok(value));
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
            let value = inner.lock().await.get_for_update(key).await.unwrap();
            send_result(queue, callback, Ok(value));
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
            let value = inner.lock().await.key_exists(key).await.unwrap();
            send_result(queue, callback, Ok(value));
        });

        Ok(cx.undefined())
    }

    pub fn batch_get(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        let keys = cx.argument::<JsArray>(0)?;
        let keys: Vec<String> = js_array_to_rust_keys(&mut cx, keys);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let result = inner
                .lock()
                .await
                .batch_get(keys)
                .await
                .map(|kvpairs| kvpairs.collect::<Vec<KvPair>>());
            send_result(queue, callback, result);
        });

        Ok(cx.undefined())
    }

    pub fn batch_get_for_update(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        let keys = cx.argument::<JsArray>(0)?;
        let keys: Vec<String> = js_array_to_rust_keys(&mut cx, keys);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let result = inner.lock().await.batch_get_for_update(keys).await;
            send_result(queue, callback, result);
        });

        Ok(cx.undefined())
    }

    pub fn scan(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        let start = cx.argument::<JsString>(0)?.value(&mut cx).into_bytes();
        let end = cx.argument::<JsString>(1)?.value(&mut cx).into_bytes();
        let limit = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;
        let include_start = cx.argument::<JsBoolean>(3)?.value(&mut cx);
        let include_end = cx.argument::<JsBoolean>(4)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(5)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let range = to_bound_range(Some(start), Some(end), include_start, include_end);

            let result = inner
                .lock()
                .await
                .scan(range, limit)
                .await
                .map(|kvpairs| kvpairs.collect::<Vec<KvPair>>());
            send_result(queue, callback, result);
        });

        Ok(cx.undefined())
    }

    pub fn scan_keys(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        let start = cx.argument::<JsString>(0)?.value(&mut cx).into_bytes();
        let end = cx.argument::<JsString>(1)?.value(&mut cx).into_bytes();
        let limit = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;
        let include_start = cx.argument::<JsBoolean>(3)?.value(&mut cx);
        let include_end = cx.argument::<JsBoolean>(4)?.value(&mut cx);
        let callback = cx.argument::<JsFunction>(5)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();
        RUNTIME.spawn(async move {
            let range = to_bound_range(Some(start), Some(end), include_start, include_end);

            let result = inner
                .lock()
                .await
                .scan_keys(range, limit)
                .await
                .map(|kvpairs| kvpairs.collect::<Vec<Key>>());
            send_result(queue, callback, result);
        });

        Ok(cx.undefined())
    }

    pub fn lock_keys(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let client = cx
            .this()
            .downcast_or_throw::<JsBox<Transaction>, _>(&mut cx)?;
        let keys = cx.argument::<JsArray>(0)?;
        let keys: Vec<String> = js_array_to_rust_keys(&mut cx, keys);
        let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);

        let inner = client.inner.clone();
        let queue = cx.queue();

        RUNTIME.spawn(async move {
            let result = inner.lock().await.lock_keys(keys).await;
            send_result(queue, callback, result);
        });

        Ok(cx.undefined())
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
            send_result(queue, callback, result);
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
            send_result(queue, callback, result);
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
            send_result(queue, callback, result);
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
            send_result(queue, callback, result);
        });

        Ok(cx.undefined())
    }
}
