use lambda_runtime::{run, service_fn, tracing, Error};

pub mod openai;
pub mod llm;
mod event_handler;
use event_handler::function_handler;

pub const DEBUG_PRE: bool = true;
pub const DEBUG_POST: bool = false;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
