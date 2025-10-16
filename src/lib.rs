use plugind_core::{context::{Context, InvokeResult}, plugin};

#[plugin]
pub async fn init(_: Vec<u8>, _: Context) -> InvokeResult {

    Ok(b"Hello World".to_vec())
}