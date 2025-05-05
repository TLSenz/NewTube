
#[tokio::main]
async fn main() {
    let _ = NewTube::rocket().launch().await;
}
