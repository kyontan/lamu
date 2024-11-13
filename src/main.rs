use std::process::Command;
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {

    // execute chmod +x handler to make the file executable
    Command::new("chmod")
        .arg("+x")
        .arg("handler")
        .output()
        .expect("failed to execute process");

    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();

    // execute the handler file, and return the output

    let output = Command::new("./handler")
        .arg(format!("{}", event))
        .output()
        .expect("failed to execute process");

    // try to parse output as json
    let output_str = String::from_utf8_lossy(&output.stdout);
    let output_json: Value = serde_json::from_str(&output_str).unwrap();

    // the encode output as json
    Ok(json!(output_json))
    // Ok(json!(String::from_utf8_lossy(&output.stdout)))

    // let first_name = event["firstName"].as_str().unwrap_or("world");
    // Ok(json!({ "message": format!("Hello, {}!", first_name) }))


}
