terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "5.80"
    }
  }
  backend "s3" {
    bucket = "575737149124-terraform-backend"
    key    = "graph-rs-ecr/tfstate"
    region = "us-east-1"
  }

  required_version = ">= 1.9.2"
}

provider "aws" {
  default_tags {
    tags = {
      app = "graph_rs"
    }
  }
}
