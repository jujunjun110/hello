use reqwest::StatusCode;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://abehiroshi.la.coocan.jp/hoi";
    dbg!(url);
    let res = reqwest::get(url).await?;
    match res.status() {
        StatusCode::OK => {
            let body = res.text().await?;
            println!("response is \n{}", body);
        }
        StatusCode::NOT_FOUND => {
            dbg!("not found");
        }
        _ => {
            dbg!("other errors");
        }
    }
    Ok(())
}
