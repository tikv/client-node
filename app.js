const RawClient = require('.');


(async () => {
    const client = await RawClient.connect("127.0.0.1:2379");
    await client.put("k1", "v1", "default")
    await client.put("k2", "v2", "default")
    await client.put("k3", "v3", "default")
    await client.put("k4", "v4", "default")
    value = await client.get("k1", "default")
    console.log(value.toString())
    value = await client.get("k2", "default")
    console.log(value.toString())
    value = await client.get("k3", "default")
    console.log(value.toString())
    value = await client.get("k4", "default")
    console.log(value.toString())
})();