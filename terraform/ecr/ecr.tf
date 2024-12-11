resource "aws_ecr_repository" "graph_rs_repository" {
  name                 = "graph-rs-repository"
  image_tag_mutability = "IMMUTABLE"
  force_delete         = true
}

resource "aws_ecr_repository_policy" "policy" {
  repository = aws_ecr_repository.graph_rs_repository.name
  policy     = file("./repository-policy.json")
}

resource "aws_ecr_lifecycle_policy" "lifecycle_policy" {
  repository = aws_ecr_repository.graph_rs_repository.name
  policy     = file("./lifecycle-policy.json")
}
