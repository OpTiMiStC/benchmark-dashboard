terraform {
  required_version = ">= 1.5.0"

  required_providers {
    github = {
      source  = "integrations/github"
      version = "~> 6.0"
    }
  }
}

provider "github" {
  owner = var.github_owner
  token = var.github_token
}

resource "github_branch_protection" "main" {
  repository_id = var.repository_name
  pattern       = "main"

  allows_deletions    = false
  allows_force_pushes = false

  required_pull_request_reviews {
    required_approving_review_count = 1
  }

  required_status_checks {
    strict   = true
    contexts = ["build"]
  }
}
