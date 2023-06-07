use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8001")?;

    hc.do_get("/hello?name=Jen").await?.print().await?;

    Ok(())
}