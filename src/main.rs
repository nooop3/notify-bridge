use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let health = warp::path!("health").map(|| "OK");

    let routes = hello.or(health);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
