# data "aws_ecr_repository" "graph_rs_repo" {
#   name = var.ecr_repository_name
# }

resource "aws_ecr_repository" "graph_rs_repo" {
  name                 = var.ecr_repository_name
  image_tag_mutability = "IMMUTABLE"
  force_delete         = true
}

import {
  to = aws_ecr_repository.graph_rs_repo
  id = "graph-rs-repository"
}

resource "aws_ecr_lifecycle_policy" "lifecycle_policy" {
  repository = aws_ecr_repository.graph_rs_repo.name
  policy     = file("./lifecycle-policy.json")
}

import {
  to = aws_ecr_lifecycle_policy.lifecycle_policy
  id = "graph-rs-repository"
}
