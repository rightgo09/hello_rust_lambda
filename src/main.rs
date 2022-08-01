use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let f = service_fn(my_handler);
    run(f).await?;
    Ok(())
}

async fn my_handler(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    Ok(json!({ "message": "hello, world!" }))
}
