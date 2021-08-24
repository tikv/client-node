"use strict";

const { promisify } = require("util");

const inner = require("./index.node");

const connect_async = promisify(inner.connect);
const get_async = promisify(inner.get);
const put_async = promisify(inner.put);
const delete_async = promisify(inner.delete);
const batch_get_async = promisify(inner.batch_get);
const batch_put_async = promisify(inner.batch_put);

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
}

module.exports = RawClient;
