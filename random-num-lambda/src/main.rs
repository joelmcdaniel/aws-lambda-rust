use lambda_runtime::{service_fn, Error, LambdaEvent};
use rand::distributions::{Bernoulli, Normal, Uniform};
use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Deserialize)]
#[serde(tag = "distribution", content = "parameters", rename_all = "lowercase")]
enum RngRequest {
    Uniform {
        #[serde(flatten)]
        range: Range<i32>,
    },
    Normal {
        mean: f64,
        std_dev: f64,
    },
    Bernoulli {
        p: f64,
    },
}

#[derive(Serialize)]
struct RngResponse {
    value: f64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(rng_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn rng_handler(event: LambdaEvent<RngRequest>) -> Result<RngResponse, Error> {
    let (event, _context) = event.into_parts();
    let mut rng = rand::thread_rng();
    let value = {
        match event {
            RngRequest::Uniform { range } => {
                rng.sample(Uniform::from(range)) as f64
            },
            RngRequest::Normal { mean, std_dev } => {
                rng.sample(Normal::new(mean, std_dev)) as f64
            },
            RngRequest::Bernoulli { p } => {
                rng.sample(Bernoulli::new(p)) as i8 as f64
            },
        }
    };
    Ok(RngResponse { value })
}
