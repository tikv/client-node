const { get } = require("http");
const tikv = require("./tikv_client");

const client = new tikv.TransactionClient("127.0.0.1:2379");
const txn = client.begin(true);
txn.put("k1", "v1");
txn.put("k2", "v2");
txn.put("k3", "v3");
txn.put("k4", "v4");
const v4 = txn.get("k4");
txn.commit();
console.log(v4);
const snapshot = client.snapshot(client.current_timestamp(), true);
const result = snapshot.get("k3");
console.log(result);
const values = snapshot.batch_get(["k1", "k4"]);
values.forEach(element => {
    console.log(element);
});

const result2 = snapshot.scan("k1", "k2", 10, true, true);
result2.forEach(element => {
    console.log(element);
});
