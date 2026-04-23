variable "github_owner" {
  type = string
}

variable "github_token" {
  type      = string
  sensitive = true
}

variable "repository_name" {
  type = string
}
