const tikv = require("./tikv_client/asynchronous");

(async () => {
    const client = await new tikv.TransactionClient("127.0.0.1:2379");
    const txn = await client.begin(true);
    await txn.put("k1", "v1");
    await txn.put("k2", "v2");
    await txn.put("k3", "v3");
    await txn.put("k4", "v4");
    const val = await txn.get("k4");
    await txn.commit();
    console.log(val)
    const snapshot = await client.snapshot(await client.current_timestamp(), true);
    const result = await snapshot.get("k3");
    console.log(result)
    const values = await snapshot.batch_get(["k1", "k4"]);
    values.forEach(element => {
      console.log(element);
    })
  
    const result2 = await snapshot.scan("k1", "k2", 10, false, false)
    result2.forEach(element => {
      console.log(element);
    });
  })();