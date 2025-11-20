use lambda_http::{Body, Error, Request, Response};
use std::env;

use aws_config::BehaviorVersion;
use aws_config;
use aws_sdk_s3::Client;

// Main HTTP entrypoint: route between /health and /items.
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Read the path so we can branch on /health vs /items.
    let path = event.uri().path();

    // /health endpoint – simple liveness check.
    if path == "/health" {
        let resp = Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body("ok".into())
            .map_err(Box::new)?;
        return Ok(resp);
    }

    // /items endpoint – list objects from our S3 bucket.
    if path == "/items" {
        let bucket_name = env::var("BUCKET_NAME").map_err(Box::new)?;
        let body_text = list_items(&bucket_name).await?;

        let resp = Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body(body_text.into())
            .map_err(Box::new)?;
        return Ok(resp);
    }

    let resp = Response::builder()
        .status(404)
        .header("content-type", "text/plain")
        .body("not found".into())
        .map_err(Box::new)?;
    Ok(resp)
}

// Helper that talks to S3 and returns one line per object key (or 'no items').
async fn list_items(bucket_name: &str) -> Result<String, Error> {
    // Load AWS SDK config (credentials, region, etc.) using the latest behavior version.
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;
    let client = Client::new(&config);

    // Call S3 ListObjectsV2 on our bucket.
    let resp = client
        .list_objects_v2()
        .bucket(bucket_name)
        .send()
        .await
        .map_err(Box::new)?;

    let mut keys: Vec<String> = Vec::new();
    for obj in resp.contents() {
        if let Some(key) = obj.key() {
            keys.push(key.to_string());
        }
    }

    if keys.is_empty() {
        Ok("no items".to_string())
    } else {
        Ok(keys.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::Request;

    // Smoke test to keep /health behavior stable.
    #[tokio::test]
    async fn test_health_handler_returns_ok() {
        let mut request = Request::default();
        *request.uri_mut() = "/health".parse().unwrap();

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(body_string, "ok");
    }
}
