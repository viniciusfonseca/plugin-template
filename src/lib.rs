use plugind_core::{context::Context, plugin};

#[plugin]
pub async fn init(_: Vec<u8>, _: Context) -> Vec<u8> {

    b"Hello World".to_vec()
}