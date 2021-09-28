// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

"use strict";

// @ts-ignore
const inner = require("../index.node");
import { OperationAfterCommitError, UndertminedError, WriteConflictError, AlreadyExistError, DeadlockError } from "./error";
inner.init(
  OperationAfterCommitError,
  UndertminedError,
  WriteConflictError,
  AlreadyExistError,
  DeadlockError
);

var deasync = require("deasync");
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

export class RawClient {
  boxed: any;
  /**
   * Construct a raw client.
   * @class RawClient
   * @param { string } pd_endpoint - PD endpoint
   * @example new tikv.RawClient("127.0.0.1:2379")
   */
  constructor(pd_endpoint: string) {
    this.boxed = raw_connect_sync(pd_endpoint);
  }

  /**
   * Get a raw key.
   * @param { string } key - raw key
   * @param { string } cf - configuration
   * @example client.get("key", "default")
   */
  get(key: string, cf: string) {
    return get_sync.call(this.boxed, key, cf);
  }

  /**
   * Put a raw key.
   * @param { string } key - raw key
   * @param { string } value - raw value
   * @param { string } cf - configuration
   * @example client.put("key", "value", "default")
   */
  put(key: string, value: string, cf: string) {
    return put_sync.call(this.boxed, key, value, cf);
  }

  /**
   * Delete a raw key.
   * @param { string } key - raw key
   * @param { string } cf - configuration
   * @example client.delete("key", "default")
   * @returns { boolean }
   */
  delete(key: string, cf: string): boolean {
    return delete_sync.call(this.boxed, key, cf);
  }

  /**
   * Batch get raw keys.
   * @param { string[] } keys - raw keys
   * @param { string } cf - configuration
   * @example client.batch_get(["key1", "key2"], "default")
   */
  batch_get(keys: string[], cf: string) {
    return batch_get_sync.call(this.boxed, keys, cf);
  }

  /**
   * Batch put raw keys.
   * @param { string[] } keys - raw keys
   * @param { string[] } values - raw values
   * @param { string } cf - configuration
   * @example client.batch_put(["key1", "key2"], ["value1", "value2"], "default")
   */
  batch_put(keys: string[], cf: string) {
    return batch_put_sync.call(this.boxed, keys, cf);
  }

  /**
   * Batch delete raw keys.
   * @param { string[] } keys - raw keys
   * @param { string } cf - configuration
   * @example client.batch_delete(["key1", "key2"], "default")
   */
  batch_delete(keys: string[], cf: string) {
    return batch_delete_sync.call(this.boxed, keys, cf);
  }

  /**
   * Create a new 'scan' request.
   * @param { string } start - start key
   * @param { string } end - end key
   * @param { number } limit - limit
   * @param { boolean } include_start - include start key
   * @param { boolean } include_end - include end key
   * @param { string } cf - configuration
   * @example client.scan("k1", "k5", 10, true, true, "default");
   */
  scan(start: string, end: string, limit: number, include_start: boolean, include_end: boolean, cf: string) {
    return scan_sync.call(
      this.boxed,
      start,
      end,
      limit,
      include_start,
      include_end,
      cf
    );
  }

  /**
   * Create a new 'scan_keys' request.
   * @param { string } start - start key
   * @param { string } end - end key
   * @param { number } limit - limit
   * @param { boolean } include_start - include start key
   * @param { boolean } include_end - include end key
   * @param { string } cf - configuration
   * @example client.scan_keys("k1", "k5", 10, true, true, "default");
   */
  scan_keys(start: string, end: string, limit: number, include_start: boolean, include_end: boolean, cf: string) {
    return scan_keys_sync.call(
      this.boxed,
      start,
      end,
      limit,
      include_start,
      include_end,
      cf
    );
  }

  /**
   * Create a new 'delete_range' request.
   * @param { string } start - start key
   * @param { string } end - end key
   * @param { boolean } include_start - include start key
   * @param { boolean } include_end - include end key
   * @param { string } cf - configuration
   * @example client.delete_range("k1", "k5", true, true, "default");
   */
  delete_range(start: string, end: string, include_start: boolean, include_end: boolean, cf: string) {
    return delete_range_sync.call(
      this.boxed,
      start,
      end,
      include_start,
      include_end,
      cf
    );
  }
}

export class Transaction {
  boxed: any;
  /**
   * @class Transaction
   * @example
   * const client = new tikv.RawClient("127.0.0.1:2379");
   * const txn = client.begin();
   */
  constructor(boxed: any) {
    this.boxed = boxed;
  }

  /**
   * Create a new 'get' request.
   * @param { string } key - key
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * txn.get("key")
   */
  get(key: string) {
    return txn_get_sync.call(this.boxed, key);
  }

  /**
   * Create a `get for update` request.
   * @param { string } key - key
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * txn.get_for_update("key")
   * txn.commit()
   */
  get_for_update(key: string) {
    return txn_get_for_update_sync.call(this.boxed, key);
  }

  /**
   * Create a new 'put' request.
   * @param { string } key - key
   * @param { string } value - value
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * txn.put("key", "value")
   * txn.commit()
   */
  put(key: string, value: string) {
    return txn_put_sync.call(this.boxed, key, value);
  }

  /**
   * Create a new 'insert' request.
   * @param { string } key - key
   * @param { string } value - value
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * txn.insert("key", "value")
   * txn.commit()
   */
  insert(key: string, value: string) {
    return txn_insert_sync.call(this.boxed, key, value);
  }

  /**
   * Create a new 'delete' request.
   * @param { string } key - key
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * txn.delete("key")
   * txn.commit()
   */
  delete(key: string) {
    return txn_delete_sync.call(this.boxed, key);
  }

  /**
   * Create a new 'commit' request.
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * //... Do some actions.
   * txn.commit()
   */
  commit() {
    return txn_commit_sync.call(this.boxed);
  }

  /**
   * Check whether a key exists.
   * @param { string } key - key
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * txn.exists("key")
   * txn.commit()
   */
  key_exists(key: string) {
    return txn_key_exists_sync.call(this.boxed, key);
  }

  /**
   * Create a new 'batch get' request.
   * @param { string[] } keys - keys
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * txn.batch_get(["key1", "key2"])
   * txn.commit()
   * //=> [{key: "key1", value: "value1"}, {key: "key2", value: "value2"}]
   */
  batch_get(keys: string[]) {
    return txn_batch_get_sync.call(this.boxed, keys);
  }

  /**
   * Create a new 'batch get for update' request.
   * @param { string[] } keys - keys
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * txn.batch_get_for_update(["key1", "key2"])
   * txn.commit()
   * //=> [{key: "key1", value: "value1"}, {key: "key2", value: "value2"}]
   */
  batch_get_for_update(keys: string[]) {
    return txn_batch_get_for_update_sync.call(this.boxed, keys);
  }

  /**
   * Create a new 'scan' request.
   * @param { string } start - start key
   * @param { string } end - end key
   * @param { number } limit - limit
   * @param { boolean } include_start - include start key
   * @param { boolean } include_end - include end key
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * txn.scan("start", "end", 10, true, true)
   * txn.commit()
   */
  scan(start: string, end: string, limit: number, include_start: boolean, include_end: boolean) {
    return txn_scan_sync.call(
      this.boxed,
      start,
      end,
      limit,
      include_start,
      include_end
    );
  }

  /**
   * Create a new 'scan keys' request.
   * @param { string } start - start key
   * @param { string } end - end key
   * @param { number } limit - limit
   * @param { boolean } include_start - include start key
   * @param { boolean } include_end - include end key
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * txn.scan_keys("start", "end", 10, true, true)
   * txn.commit()
   * //=> ["key1", "key2"]
   */
  scan_keys(start: string, end: string, limit: number, include_start: boolean, include_end: boolean) {
    return txn_scan_keys_sync.call(
      this.boxed,
      start,
      end,
      limit,
      include_start,
      include_end
    );
  }

  /**
   * Create a new 'lock keys' request.
   * @param { string[] } keys - keys
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * txn.lock_keys(["key1", "key2"])
   * txn.commit()
   */
  lock_keys(keys: string[]) {
    return txn_lock_keys_sync.call(this.boxed, keys);
  }
}

export class Snapshot {
  boxed: any;
  /**
   * @class Snapshot
   */
  constructor(boxed: any) {
    this.boxed = boxed;
  }

  /**
   * Create a new 'get' request.
   * @param { string } key - key
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const snapshot = client.snapshot(client.current_timestamp(), true);
   * snapshot.get("key")
   * //=> {key: "key", value: "value"}
   */
  get(key: string) {
    return snapshot_get_sync.call(this.boxed, key);
  }

  /**
   * Create a new 'key exists' request.
   * @param { string } key - key
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const snapshot = client.snapshot(client.current_timestamp(), true);
   * snapshot.key_exists("key")
   * //=> true
   */
  key_exists(key: string) {
    return snapshot_key_exists_sync.call(this.boxed, key);
  }

  /**
   * Create a new 'batch get' request.
   * @param { string[] } keys - keys
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const snapshot = client.snapshot(client.current_timestamp(), true);
   * snapshot.batch_get(["key1", "key2"])
   * //=> [{key: "key1", value: "value1"}, {key: "key2", value: "value2"}]
   */
  batch_get(keys: any) {
    return snapshot_batch_get_sync.call(this.boxed, keys);
  }

  /**
   * Create a new 'scan' request.
   * @param { string } start - start key
   * @param { string } end - end key
   * @param { number } limit - limit
   * @param { boolean } include_start - include start key
   * @param { boolean } include_end - include end key
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const snapshot = client.snapshot(client.current_timestamp(), true);
   * snapshot.scan("start", "end", 10, true, true)
   * //=> [{key: "key1", value: "value1"}, {key: "key2", value: "value2"}]
   */
  scan(start: string, end: string, limit: number, include_start: boolean, include_end: boolean) {
    return snapshot_scan_sync.call(
      this.boxed,
      start,
      end,
      limit,
      include_start,
      include_end
    );
  }

  /**
   * Create a new 'scan keys' request.
   * @param { string } start - start key
   * @param { string } end - end key
   * @param { number } limit - limit
   * @param { boolean } include_start - include start key
   * @param { boolean } include_end - include end key
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const snapshot = client.snapshot(client.current_timestamp(), true);
   * snapshot.scan_keys("start", "end", 10, true, true)
   * //=> ["key1", "key2"]
   */
  scan_keys(start: string, end: string, limit: number, include_start: boolean, include_end: boolean) {
    return snapshot_scan_keys_sync.call(
      this.boxed,
      start,
      end,
      limit,
      include_start,
      include_end
    );
  }
}

export class TransactionClient {
  boxed: any;
  /**
   * @class TransactionClient
   * @param { string } pd_endpoint - PD endpoint
   * @example const client = new tikv.TransactionClient("127.0.0.1:2379");
   */
  constructor(pd_endpoint: string) {
    this.boxed = txn_connect_sync(pd_endpoint);
  }

  /**
   * Create a new 'begin' request.
   * @param { boolean } pessimistic - pessimistic
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const txn = client.begin(true);
   * txn.commit()
   */
  begin(pessimistic: boolean) {
    return new Transaction(txn_begin_sync.call(this.boxed, pessimistic));
  }

  /**
   * Create a new Snapshot
   * @param { number } timestamp - timestamp
   * @param { boolean } pessimistic - pessimistic
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const snapshot = client.snapshot(client.current_timestamp(), true);
   */
  snapshot(timestamp: number, pessimistic: boolean) {
    return new Snapshot(
      txn_snapshot_sync.call(this.boxed, timestamp, pessimistic)
    );
  }

  /**
   * Retrieve the current [`Timestamp`].
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * const timestamp = client.current_timestamp();
   * //=> 1588888888
   */
  current_timestamp() {
    return txn_current_timestamp_sync.call(this.boxed);
  }

  /**
   * Request garbage collection (GC) of the TiKV cluster.
   * @param { number } safepoint - safe point
   * @example
   * const client = new tikv.TransactionClient("127.0.0.1:2379");
   * client.gc(1588888888);
   * //=> true
   */
  gc(safepoint: number) {
    return txn_gc_sync.call(this.boxed, safepoint);
  }
}
