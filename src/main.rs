mod sql;



#[tokio::main]
async fn main() {
    serve().run(([127, 0, 0, 1], 3030)).await;
}
