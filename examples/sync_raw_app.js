const tikv = require("./tikv_client");

const client = new tikv.RawClient("127.0.0.1:2379");

client.put("k1", "v1", "default");
client.put("k2", "v2", "default");
client.put("k3", "v3", "default");

const v1 = client.get("k1", "default");
const v2 = client.get("k2", "default");
const v3 = client.get("k3", "default");

console.log(v1, v2, v3);

client.delete("k1", "default");
client.delete("k2", "default");

const v1_ = client.get("k1", "default");
const v2_ = client.get("k2", "default");

console.log(v1_, v2_, v3);

client.batch_put([["k3", "k4", "k5"], ["v3", "v4", "v5"]], "default");
const vals = client.batch_get(["k3", "k4", "k5"], "default");
for (let i = 0; i < vals.length; i++) {
    console.log(vals[i]);
}

const scan_vals = client.scan("k1", "k5", 10, true, true, "default");
for (let i = 0; i < scan_vals.length; i++) {
    console.log(scan_vals[i]);
}   

