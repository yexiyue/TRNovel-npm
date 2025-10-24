use napi::Result;
use napi_derive::napi;

#[napi]
pub async fn run(args: Vec<String>) -> Result<()> {
  trnovel::try_run(args)
    .await
    .map_err(|e| napi::Error::from_reason(e.to_string()))
}
