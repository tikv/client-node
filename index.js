"use strict"

const { promisify } = require("util");

const {connect, get, put} = require('./index.node');

const connect_async = promisify(connect);
const get_async = promisify(get);
const put_async = promisify(put);

class RawClient {
    constructor(boxed) {
        this.boxed = boxed
    }

    get(key, cf) {
        return get_async.call(this.boxed, key, cf)
    }

    put(key, value, cf) {
        return put_async.call(this.boxed, key, value, cf)
    }
}

RawClient.connect = async function(pd_endpoint) {
    this.boxed = await connect_async(pd_endpoint);
    return new RawClient(this.boxed)
}


module.exports = RawClient;