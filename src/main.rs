use ferris_says::say;
use std::io::{stdout, BufWriter};
use warp::{Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    let health_route = warp::path!("health").and_then(health_handler);
    let routes = health_route.with(warp::cors().allow_any_origin());
    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

async fn health_handler() -> Result<impl Reply> {
    Ok("OK")
}
