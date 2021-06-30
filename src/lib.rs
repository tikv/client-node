use std::sync::Arc;

use neon::prelude::*;
use tokio::sync::{Mutex, RwLock};

mod raw;
mod transaction;
mod utils;

pub struct RawClient {
    inner: Arc<tikv_client::RawClient>,
}

impl Finalize for RawClient {}

pub struct TransactionClient {
    inner: Arc<tikv_client::TransactionClient>,
}

impl Finalize for TransactionClient {}

pub struct Transaction {
    inner: Arc<Mutex<tikv_client::Transaction>>,
}

impl Finalize for Transaction {}

pub struct Snapshot {
    inner: Arc<Mutex<tikv_client::Snapshot>>,
}

impl Finalize for Snapshot {}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("raw_connect", RawClient::connect)?;
    cx.export_function("raw_put", RawClient::put)?;
    cx.export_function("raw_get", RawClient::get)?;
    cx.export_function("raw_delete", RawClient::delete)?;
    cx.export_function("raw_batch_get", RawClient::batch_get)?;
    cx.export_function("raw_scan", RawClient::scan)?;
    cx.export_function("raw_scan_keys", RawClient::scan_keys)?;
    cx.export_function("raw_batch_put", RawClient::batch_put)?;
    cx.export_function("raw_batch_delete", RawClient::batch_delete)?;
    cx.export_function("raw_delete_range", RawClient::delete_range)?;

    cx.export_function("txn_connect", TransactionClient::connect)?;
    cx.export_function("txn_begin", TransactionClient::begin)?;
    cx.export_function("txn_snapshot", TransactionClient::snapshot)?;
    cx.export_function(
        "txn_current_timestamp",
        TransactionClient::current_timestamp,
    )?;
    cx.export_function("txn_gc", TransactionClient::gc)?;
    cx.export_function("txn_get", Transaction::get)?;
    cx.export_function("txn_get_for_update", Transaction::get_for_update)?;
    cx.export_function("txn_key_exists", Transaction::key_exists)?;
    cx.export_function("txn_batch_get", Transaction::batch_get)?;
    cx.export_function(
        "txn_batch_get_for_update",
        Transaction::batch_get_for_update,
    )?;
    cx.export_function("txn_scan", Transaction::scan)?;
    cx.export_function("txn_scan_keys", Transaction::scan_keys)?;
    cx.export_function("txn_lock_keys", Transaction::lock_keys)?;
    cx.export_function("txn_put", Transaction::put)?;
    cx.export_function("txn_insert", Transaction::insert)?;
    cx.export_function("txn_delete", Transaction::delete)?;
    cx.export_function("txn_commit", Transaction::commit)?;

    cx.export_function("snapshot_get", Snapshot::get)?;
    cx.export_function("snapshot_key_exists", Snapshot::key_exists)?;
    cx.export_function("snapshot_batch_get", Snapshot::batch_get)?;
    cx.export_function("snapshot_scan", Snapshot::scan)?;
    cx.export_function("snapshot_scan_keys", Snapshot::scan_keys)?;
    Ok(())
}
