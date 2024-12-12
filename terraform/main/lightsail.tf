resource "aws_lightsail_container_service" "container_service" {
  name        = "graph-rs-container-service"
  power       = "nano" # Available options: nano, micro, small, medium, large, xlarge
  scale       = 1      # Number of containers to run
  is_disabled = false

  private_registry_access {
    ecr_image_puller_role {
      is_active = true
    }
  }
}

resource "aws_lightsail_container_service_deployment_version" "deployment" {
  service_name = aws_lightsail_container_service.container_service.name

  container {
    container_name = "main-container"
    image          = "${data.aws_ecr_repository.graph_rs_repo.repository_url}:${var.git_commit_sha}"
    ports = {
      80 = "HTTP"
    }
    command     = []
    environment = {}
  }

  public_endpoint {
    container_name = "main-container"
    container_port = 80 # Exposed port
    health_check {
      healthy_threshold   = 2
      unhealthy_threshold = 2
    }
  }
}

data "aws_iam_policy_document" "aws_ecr_repository_policy" {
  statement {
    effect  = "Allow"
    actions = ["ecr:*"]
    principals {
      type = "AWS"
      identifiers = [
        "arn:aws:iam::575737149124:user/admin",
        "arn:aws:iam::575737149124:role/graph-rs-deploy",
        aws_lightsail_container_service.container_service.private_registry_access[0].ecr_image_puller_role[0].principal_arn
      ]
    }
  }
}

resource "aws_ecr_repository_policy" "policy" {
  repository = data.aws_ecr_repository.graph_rs_repo.name
  policy     = data.aws_iam_policy_document.aws_ecr_repository_policy.json
}
