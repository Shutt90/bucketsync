terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

provider "aws" {
  region = "eu-west-2"
}

resource "aws_s3_bucket" "filesync" {
  bucket = "filesync-bucket"

  tags = {
    Name = "Filesync Bucket"
  }
}

resource "aws_s3_bucket_acl" "filesync_bucket_acl" {
  bucket = aws_s3_bucket.filesync.id

  access_control_policy {
    grant {
      grantee {
        id   = 926837967238
        type = "CanonicalUser"
      }
      permission = "FULL_CONTROL"
    }
  }
}