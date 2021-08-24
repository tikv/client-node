"use strict";

const { promisify } = require("util");

const inner = require("./index.node");

const connect_async = promisify(inner.raw_connect);
const get_async = promisify(inner.raw_get);
const put_async = promisify(inner.raw_put);
const delete_async = promisify(inner.raw_delete);
const batch_get_async = promisify(inner.raw_batch_get);
const batch_put_async = promisify(inner.raw_batch_put);
const batch_delete_async = promisify(inner.raw_batch_delete);
const delete_range_async = promisify(inner.raw_delete_range);
const scan_async = promisify(inner.raw_scan);
const scan_keys_async = promisify(inner.raw_scan_keys);

const txn_connect_async = promisify(inner.txn_connect);
const txn_begin_async = promisify(inner.txn_begin);
const txn_get_async = promisify(inner.txn_get);
const txn_get_for_update_async = promisify(inner.txn_get_for_update);
const txn_key_exists_async = promisify(inner.txn_key_exists);
const txn_batch_get_async = promisify(inner.txn_batch_get);
const txn_batch_get_for_update_async = promisify(inner.txn_batch_get_for_update);
const txn_scan_async = promisify(inner.txn_scan);
const txn_scan_keys_async = promisify(inner.txn_scan_keys);
const txn_lock_keys_async = promisify(inner.txn_lock_keys);
const txn_put_async = promisify(inner.txn_put);
const txn_insert_async = promisify(inner.txn_insert);
const txn_delete_async = promisify(inner.txn_delete);
const txn_commit_async = promisify(inner.txn_commit);

class RawClient {
  constructor(pd_endpoint) {
    return (async () => {
      // TODO: error out if the parameter `boxed` is not what we want.
      this.boxed = await connect_async(pd_endpoint);
      return this;
    })();
  }

  get(key, cf) {
    return get_async.call(this.boxed, key, cf);
  }

  put(key, value, cf) {
    return put_async.call(this.boxed, key, value, cf);
  }

  delete(key, cf) {
    return delete_async.call(this.boxed, key, cf);
  }

  batch_get(keys, cf) {
    return batch_get_async.call(this.boxed, keys, cf);
  }

  batch_put(kv_pairs, cf) {
    return batch_put_async.call(this.boxed, kv_pairs, cf);
  }

  batch_delete(keys, cf) {
    return batch_delete_async.call(this.boxed, keys, cf);
  }

  scan(start, end, limit, include_start, include_end, cf) {
    return scan_async.call(
      this.boxed,
      start,
      end,
      limit,
      include_start,
      include_end,
      cf
    );
  }

  scan_keys(start, end, limit, include_start, include_end, cf) {
    return scan_keys_async.call(
      this.boxed,
      start,
      end,
      limit,
      include_start,
      include_end,
      cf
    );
  }

  delete_range(start, end, include_start, include_end, cf) {
    return delete_range_async.call(
      this.boxed,
      start,
      end,
      include_start,
      include_end,
      cf
    );
  }
}

class Transaction {
  constructor(boxed) {
    this.boxed = boxed
  }

  get(key) { 
    return txn_get_async.call(this.boxed, key)
  }

  get_for_update(key) { 
    return txn_get_for_update_async.call(this.boxed, key)
  }

  put(key, value) { 
    return txn_put_async.call(this.boxed, key, value)
  }

  insert(key, value) { 
    return txn_insert_async.call(this.boxed, key, value)
  }

  delete(key) { 
    return txn_delete_async.call(this.boxed, key)
  }

  commit() { 
    return txn_commit_async.call(this.boxed)
  }

}

class TransactionClient {
  constructor(pd_endpoint) {
    return (async () => {
      // TODO: error out if the parameter `boxed` is not what we want.
      this.boxed = await txn_connect_async(pd_endpoint);
      return this;
    })();
  }

  begin(pessimistic) {
    return (async () => {
      const boxed = await txn_begin_async.call(this.boxed, pessimistic);
      return new Transaction(boxed);
    })();
  }
}
module.exports = {
  RawClient: RawClient,
  TransactionClient: TransactionClient,
  Transaction: Transaction
};
