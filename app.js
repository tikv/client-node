const { Transaction } = require(".");
const tikv = require(".");

(async () => {
  const client = await new tikv.RawClient("127.0.0.1:2379");
  await client.put("k1", "v1", "default");
  await client.put("k2", "v2", "default");
  await client.put("k3", "v3", "default");
  await client.put("k4", "v4", "default");
  value = await client.get("k1", "default");
  console.log(value.toString());
  value = await client.get("k2", "default");
  console.log(value.toString());
  value = await client.get("k3", "default");
  console.log(value.toString());
  await client.delete("k4", "default");
  await client.get("k4", "default");

  await client.batch_put(
    [
      ["k5", "v5"],
      ["k6", "v6"],
    ],
    "default"
  );
  await client.batch_delete(["k1", "k2", "k5"], "default");
  values = await client.batch_get(
    ["k1", "k2", "k3", "k4", "k5", "k6"],
    "default"
  );

  console.log(values);
})();

(async () => {
  const client = await new tikv.TransactionClient("127.0.0.1:2379");
  const txn = await client.begin(true);
  await txn.put("k1", "v1");
  await txn.put("k2", "v2");
  await txn.put("k3", "v3");
  await txn.put("k4", "v4");
  await txn.commit();

  const snapshot = await client.snapshot(await client.current_timestamp(), true);
  await snapshot.get("k3");
  await snapshot.batch_get(["k1", "k4"]);

  const result = await snapshot.scan("k1", "k10", 10, false, false)
  result.forEach(element => {
    console.log(element);
  });
})();