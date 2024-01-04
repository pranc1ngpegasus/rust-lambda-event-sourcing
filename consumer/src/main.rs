use aws_lambda_events::sqs::SqsEventObj;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use proto::example::v1::Greet;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(exec)).await
}

pub async fn exec(event: LambdaEvent<SqsEventObj<Greet>>) -> Result<(), Error> {
    let (event, _) = event.into_parts();
    let data: Greet = event.records[0].body.to_owned();

    println!("data: {:?}", data);

    Ok(())
}
