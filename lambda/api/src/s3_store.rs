use lambda_http::Error;
use aws_config::BehaviorVersion;
use aws_config;
use aws_sdk_s3::{primitives::ByteStream, Client};

// Helper that talks to S3 and returns one line per object key (or 'no items').
pub async fn list_items(bucket_name: &str) -> Result<String, Error> {
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
pub async fn put_item(bucket_name: &str, key: &str, body: Vec<u8>) -> Result<(), Error> {
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
pub async fn delete_item(bucket_name: &str, key: &str) -> Result<(), Error> {
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
