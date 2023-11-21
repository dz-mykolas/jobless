#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    // hc.do_get("/hello").await?.print().await?;

    // let req_login = hc.do_post(
    //     "/api/login",
    //     json!({
    //         "username": "haha1",
    //         "password": "pass",
    //     })
    // );
    // req_login.await?.print().await?;

    // hc.do_get("/api/companies").await?.print().await?;
    hc.do_get("/api/companies/4").await?.print().await?;

    Ok(())
}
