const assert = require("assert");
const { OperationAfterCommitError } = require("../tikv_client/error");


// const tikv = require("../tikv_client/asynchronous");

// (async () => {
//     const client = await new tikv.TransactionClient("127.0.0.1:2379");
//     const txn = await client.begin(true);
//     await txn.put("k1", "v1");
//     await txn.commit();
//     txn.get("k1").then(v => console.log(v)).catch(e => console.log(e));
//   })();

const tikv = require("../tikv_client");

const client = new tikv.TransactionClient("127.0.0.1:2379");
const txn = client.begin(true);
txn.put("k1", "v1");
txn.commit();
assert.throws(() => txn.get("k1"), OperationAfterCommitError);