resource "aws_lightsail_container_service" "container_service" {
  name        = "graph-rs-container-service"
  power       = "nano" # Available options: nano, micro, small, medium, large, xlarge
  scale       = 1
  is_disabled = false

  private_registry_access {
    ecr_image_puller_role {
      is_active = true
    }
  }
}

output "lightsail_role" {
  value = aws_lightsail_container_service.container_service.private_registry_access[0].ecr_image_puller_role[0].principal_arn
}

resource "aws_lightsail_container_service_deployment_version" "deployment" {
  service_name = aws_lightsail_container_service.container_service.name

  container {
    container_name = "main-container"
    image          = "${aws_ecr_repository.graph_rs_repo.repository_url}:${var.git_commit_sha}"
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
