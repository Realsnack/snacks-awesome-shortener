use tide::Request;

#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(hello);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

async fn hello(mut _req: Request<()>) -> tide::Result<String> {
    println!("hello returned");
    Ok("hello world".into())
}