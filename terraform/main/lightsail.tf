resource "aws_lightsail_container_service" "container_service" {
  name        = "graph-rs-container-service"
  power       = "nano" # Available options: nano, micro, small, medium, large, xlarge
  scale       = 1      # Number of containers to run
  is_disabled = false
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
