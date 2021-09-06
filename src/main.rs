#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

use lambda::error::HandlerError;

use std::error::Error;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "phone")]
    phone: String,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "message")]
    message: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.name == "" {
        error!("Empty name in request {}", c.aws_request_id);
        return Err(c.new_error("Empty name"));
    }

    if e.phone == "" {
        error!("Empty phone number in request {}", c.aws_request_id);
        return Err(c.new_error("Empty phone number"));
    }

    if e.email == "" {
        error!("Empty E-Mail in request {}", c.aws_request_id);
        return Err(c.new_error("Empty E-Mail"));
    }

    if e.message == "" {
        error!("Empty message in request {}", c.aws_request_id);
        return Err(c.new_error("Empty Message"));
    }

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.name),
    })
}