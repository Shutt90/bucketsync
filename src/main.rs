use std::path::Path;

use aws_sdk_s3::model::Bucket;
use aws_sdk_s3::{Client, Credentials};
use aws_sdk_s3::output::PutObjectOutput;
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::error::PutObjectError;
use aws_sdk_s3::types::SdkError;

const bucket_name: &str = "filesync-bucket";
const region: String = "eu-west-2".parse().unwrap();
const credentials: Credentials = Credentials::new(
    "access_key_id",
    "secret_access_key",
    "session_token",
    "expires_after",
    "provider_name"
);

const bucket: Bucket = Bucket::new(bucket_name, region, credentials);

fn main() {

}

pub async fn upload_object(
    client: &Client,
    bucket_name: &str,
    file_name: &str,
    key: &str,
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    let body = ByteStream::from_path(Path::new(file_name)).await;
    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body.unwrap())
        .send()
        .await
}