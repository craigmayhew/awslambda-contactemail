extern crate lambda_runtime as lambda;
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::{handler_fn, Context, Error};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

#[derive(Deserialize, Clone)]
struct Request {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "phone")]
    phone: String,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "message")]
    message: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    // can be replaced with any other method of initializing `log`
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(e: Request, ctx: Context) -> Result<Response, Error> {
    let mut output_msg = "";
    let name = e.name.as_str();
    if name == "" {
        error!("Empty name in request {}", ctx.request_id);
        output_msg = "Error: name is empty."
    }

    let phone = e.phone.as_str();
    if phone == "" {
        error!("Empty phone number in request {}", ctx.request_id);
        output_msg = "Error: phone is empty."
    }

    let email = e.email.as_str();
    if email == "" {
        error!("Empty E-Mail in request {}", ctx.request_id);
        output_msg = "Error: E-Mail is empty."
    }

    let message = e.message.as_str();
    if message == "" {
        error!("Empty message in request {}", ctx.request_id);
        output_msg = "Error: message is empty."
    }

    Ok(Response {
        req_id: ctx.request_id,
        msg: output_msg.to_string(),
    })
}