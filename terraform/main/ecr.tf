data "aws_ecr_repository" "graph_rs_repo" {
  name = var.ecr_repository_name
}
