use std::sync::Arc;

use neon::prelude::*;

mod raw;
mod utils;
pub struct RawClient {
    inner: Arc<tikv_client::RawClient>,
}
impl Finalize for RawClient {}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("raw_connect", RawClient::connect)?;
    cx.export_function("raw_put", RawClient::put)?;
    cx.export_function("raw_get", RawClient::get)?;
    cx.export_function("raw_delete", RawClient::delete)?;
    cx.export_function("raw_batch_get", RawClient::batch_get)?;
    cx.export_function("raw_scan", RawClient::scan)?;
    cx.export_function("raw_scan_keys", RawClient::scan_keys)?;
    cx.export_function("raw_batch_put", RawClient::batch_put)?;
    cx.export_function("raw_batch_delete", RawClient::batch_delete)?;
    cx.export_function("raw_delete_range", RawClient::delete_range)?;
    Ok(())
}
