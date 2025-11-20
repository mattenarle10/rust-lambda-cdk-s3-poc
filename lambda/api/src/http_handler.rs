use lambda_http::{Body, Error, Request, RequestExt, Response};
use std::env;

use aws_config::BehaviorVersion;
use aws_config;
use aws_sdk_s3::{primitives::ByteStream, Client};

// Main HTTP entrypoint: route between /health and /items.
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Read the path and HTTP method for simple routing.
    let path = event.uri().path();
    let method = event.method().as_str();

    // /health endpoint – simple liveness check.
    if path == "/health" && method == "GET" {
        let resp = Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body("ok".into())
            .map_err(Box::new)?;
        return Ok(resp);
    }

    // /items endpoints – simple CRUD-style operations on S3.
    if path == "/items" {
        let bucket_name = env::var("BUCKET_NAME").map_err(Box::new)?;

        match method {
            // GET /items -> list all keys in the bucket
            "GET" => {
                let body_text = list_items(&bucket_name).await?;

                let resp = Response::builder()
                    .status(200)
                    .header("content-type", "text/plain")
                    .body(body_text.into())
                    .map_err(Box::new)?;
                return Ok(resp);
            }
            // POST /items?key=... -> create/overwrite an object with the request body as content
            "POST" => {
                let key_param = event
                    .query_string_parameters_ref()
                    .and_then(|params| params.first("key"));

                let key = match key_param {
                    Some(k) => k,
                    None => {
                        let resp = Response::builder()
                            .status(400)
                            .header("content-type", "text/plain")
                            .body("missing ?key=... for POST /items".into())
                            .map_err(Box::new)?;
                        return Ok(resp);
                    }
                };

                let body_bytes = event.body().to_vec();
                put_item(&bucket_name, key, body_bytes).await?;

                let message = format!("created: {key}");
                let resp = Response::builder()
                    .status(201)
                    .header("content-type", "text/plain")
                    .body(message.into())
                    .map_err(Box::new)?;
                return Ok(resp);
            }
            // DELETE /items?key=... -> delete a single object by key
            "DELETE" => {
                let key_param = event
                    .query_string_parameters_ref()
                    .and_then(|params| params.first("key"));

                let key = match key_param {
                    Some(k) => k,
                    None => {
                        let resp = Response::builder()
                            .status(400)
                            .header("content-type", "text/plain")
                            .body("missing ?key=... for DELETE /items".into())
                            .map_err(Box::new)?;
                        return Ok(resp);
                    }
                };

                delete_item(&bucket_name, key).await?;

                let message = format!("deleted: {key}");
                let resp = Response::builder()
                    .status(200)
                    .header("content-type", "text/plain")
                    .body(message.into())
                    .map_err(Box::new)?;
                return Ok(resp);
            }
            // Any other method on /items is not allowed.
            _ => {
                let resp = Response::builder()
                    .status(405)
                    .header("content-type", "text/plain")
                    .body("method not allowed".into())
                    .map_err(Box::new)?;
                return Ok(resp);
            }
        }
    }

    // Fallback 404 for unknown paths.
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

// Helper that writes a new object (or overwrites an existing one) in S3.
async fn put_item(bucket_name: &str, key: &str, body: Vec<u8>) -> Result<(), Error> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;
    let client = Client::new(&config);

    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(ByteStream::from(body))
        .send()
        .await
        .map_err(Box::new)?;

    Ok(())
}

// Helper that deletes a single object from S3.
async fn delete_item(bucket_name: &str, key: &str) -> Result<(), Error> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .load()
        .await;
    let client = Client::new(&config);

    client
        .delete_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await
        .map_err(Box::new)?;

    Ok(())
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
