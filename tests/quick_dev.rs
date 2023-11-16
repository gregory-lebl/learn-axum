#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn test_hello() -> Result<()> {
  let http_client = httpc_test::new_client("http://localhost:3000")?;

	http_client.do_get("/hello?name=Greg").await?.print().await?;

  Ok(())
}