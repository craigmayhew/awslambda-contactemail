extern crate lambda_runtime as lambda;
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda_runtime::{service_fn, LambdaEvent, Error};
use log::LevelFilter;
use serde_json::{json,Value};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    // can be replaced with any other method of initializing `log`
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn my_handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (e, ctx) = event.into_parts();
    let mut output_msg = "";
    let name = e["name"].as_str().unwrap_or("Empty name in request");
    if name == "" {
        error!("Empty name in request {}", ctx.request_id);
        output_msg = "Error: name is empty."
    }

    let phone = e["phone"].as_str().unwrap_or("Empty phone number in request");
    if phone == "" {
        error!("Empty phone number in request {}", ctx.request_id);
        output_msg = "Error: phone is empty."
    }

    let email = e["email"].as_str().unwrap_or("Empty E-Mail in request");
    if email == "" {
        error!("Empty E-Mail in request {}", ctx.request_id);
        output_msg = "Error: E-Mail is empty."
    }

    let message = e["message"].as_str().unwrap_or("Empty message in request");
    if message == "" {
        error!("Empty message in request {}", ctx.request_id);
        output_msg = "Error: message is empty."
    }

    Ok(json!({ "message": output_msg.to_string() }))
}