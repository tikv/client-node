"use strict";

const inner = require("../index.node");
var deasync = require('deasync');
const raw_connect_sync = deasync(inner.raw_connect);
const get_sync = deasync(inner.raw_get);
const put_sync = deasync(inner.raw_put);
const delete_sync = deasync(inner.raw_delete);
const batch_get_sync = deasync(inner.raw_batch_get);
const batch_put_sync = deasync(inner.raw_batch_put);
const batch_delete_sync = deasync(inner.raw_batch_delete);
const delete_range_sync = deasync(inner.raw_delete_range);
const scan_sync = deasync(inner.raw_scan);
const scan_keys_sync = deasync(inner.raw_scan_keys);
const txn_connect_sync = deasync(inner.txn_connect);
const txn_begin_sync = deasync(inner.txn_begin);
const txn_snapshot_sync = deasync(inner.txn_snapshot);
const txn_current_timestamp_sync = deasync(inner.txn_current_timestamp);
const txn_gc_sync = deasync(inner.txn_gc);
const txn_get_sync = deasync(inner.txn_get);
const txn_get_for_update_sync = deasync(inner.txn_get_for_update);
const txn_key_exists_sync = deasync(inner.txn_key_exists);
const txn_batch_get_sync = deasync(inner.txn_batch_get);
const txn_batch_get_for_update_sync = deasync(inner.txn_batch_get_for_update);
const txn_scan_sync = deasync(inner.txn_scan);
const txn_scan_keys_sync = deasync(inner.txn_scan_keys);
const txn_lock_keys_sync = deasync(inner.txn_lock_keys);
const txn_put_sync = deasync(inner.txn_put);
const txn_insert_sync = deasync(inner.txn_insert);
const txn_delete_sync = deasync(inner.txn_delete);
const txn_commit_sync = deasync(inner.txn_commit);
const snapshot_get_sync = deasync(inner.snapshot_get);
const snapshot_key_exists_sync = deasync(inner.snapshot_key_exists);
const snapshot_batch_get_sync = deasync(inner.snapshot_batch_get);
const snapshot_scan_sync = deasync(inner.snapshot_scan);
const snapshot_scan_keys_sync = deasync(inner.snapshot_scan_keys);

class RawClient {
  constructor(pd_endpoint) {
    this.boxed = raw_connect_sync(pd_endpoint);
  }

  get(key, cf) {
    return get_sync.call(this.boxed, key, cf);
  }

  put(key, value, cf) {
    return put_sync.call(this.boxed, key, value, cf);
  }

  delete(key, cf) {
    return delete_sync.call(this.boxed, key, cf);
  }

  batch_get(keys, cf) {
    return batch_get_sync.call(this.boxed, keys, cf);
  }

  batch_put(kv, cf) {
    return batch_put_sync.call(this.boxed, kv, cf);
  }

  batch_delete(kv, cf) {
    return batch_delete_sync.call(this.boxed, kv, cf);
  }

  scan(start, end, limit, include_start, include_end, cf) {
    return scan_sync.call(this.boxed, start, end, limit, include_start, include_end, cf);
  }

  scan_keys(start, end, limit, include_start, include_end, cf) {
    return scan_keys_sync.call(this.boxed, start, end, limit, include_start, include_end, cf);
  }

  delete_range(start, end, include_start, include_end, cf) {
    return delete_range_sync.call(this.boxed, start, end, include_start, include_end, cf);
  }
}

class Transaction {
  constructor(boxed) {
    this.boxed = boxed;
  }

  get(key) {
    return txn_get_sync.call(this.boxed, key);
  }

  get_for_update(key) {
    return txn_get_for_update_sync.call(this.boxed, key);
  }

  put(key, value) {
    return txn_put_sync.call(this.boxed, key, value);
  }

  insert(key, value) {
    return txn_insert_sync.call(this.boxed, key, value);
  }

  delete(key) {
    return txn_delete_sync.call(this.boxed, key);
  }

  commit() {
    return txn_commit_sync.call(this.boxed);
  }

  key_exists(key) {
    return txn_key_exists_sync.call(this.boxed, key);
  }

  batch_get(kv) {
    return txn_batch_get_sync.call(this.boxed, kv);
  }

  batch_get_for_update(kv) {
    return txn_batch_get_for_update_sync.call(this.boxed, kv);
  }

  scan(start, end, limit, include_start, include_end) {
    return txn_scan_sync.call(this.boxed, start, end, limit, include_start, include_end);
  }

  scan_keys(start, end, limit, include_start, include_end) {
    return txn_scan_keys_sync.call(this.boxed, start, end, limit, include_start, include_end);
  }

  lock_keys(keys) {
    return txn_lock_keys_sync.call(this.boxed, keys);
  }
}

class Snapshot {
  constructor(boxed) {
    this.boxed = boxed;
  }

  get(key) {
    return snapshot_get_sync.call(this.boxed, key);
  }

  key_exists(key) {
    return snapshot_key_exists_sync.call(this.boxed, key);
  }

  batch_get(keys) {
    return snapshot_batch_get_sync.call(this.boxed, keys);
  }

  scan(start, end, limit, include_start, include_end) {
    return snapshot_scan_sync.call(this.boxed, start, end, limit, include_start, include_end);
  }

  scan_keys(start, end, limit, include_start, include_end) {
    return snapshot_scan_keys_sync.call(this.boxed, start, end, limit, include_start, include_end);
  }
}


class TransactionClient {
  constructor(pd_endpoint) {
    this.boxed = txn_connect_sync(pd_endpoint);
  }

  begin(pessimistic) {
    return new Transaction(txn_begin_sync.call(this.boxed, pessimistic));
  }

  snapshot(timestamp, pessimistic) {
    return new Snapshot(txn_snapshot_sync.call(this.boxed, timestamp, pessimistic));
  }

  current_timestamp() {
    return txn_current_timestamp_sync.call(this.boxed);
  }

  gc(safepoint) {
    return txn_gc_sync.call(this.boxed, safepoint);
  }
}

  
module.exports = {
  RawClient: RawClient,
  TransactionClient: TransactionClient,
  Transaction: Transaction, // TODO: #20 let's find out if we need to export these 
  Snapshot: Snapshot
};
