# Re-export module outputs for environment-specific access

output "vpc_id" {
  description = "VPC ID"
  value       = module.realworld.vpc_id
}

output "public_subnet_ids" {
  description = "Public subnet IDs"
  value       = module.realworld.public_subnet_ids
}

output "private_subnet_ids" {
  description = "Private subnet IDs"
  value       = module.realworld.private_subnet_ids
}

output "alb_dns_name" {
  description = "DNS name of the Application Load Balancer"
  value       = module.realworld.alb_dns_name
}

output "alb_zone_id" {
  description = "Zone ID of the Application Load Balancer"
  value       = module.realworld.alb_zone_id
}

output "alb_url" {
  description = "URL of the Application Load Balancer"
  value       = module.realworld.alb_url
}

output "rds_endpoint" {
  description = "RDS endpoint"
  value       = module.realworld.rds_endpoint
}

output "rds_address" {
  description = "RDS address"
  value       = module.realworld.rds_address
}

output "rds_port" {
  description = "RDS port"
  value       = module.realworld.rds_port
}

output "rds_security_group_id" {
  description = "RDS security group ID"
  value       = module.realworld.rds_security_group_id
}

output "ecs_cluster_name" {
  description = "ECS cluster name"
  value       = module.realworld.ecs_cluster_name
}

output "ecs_service_name" {
  description = "ECS service name"
  value       = module.realworld.ecs_service_name
}

output "cloudwatch_log_group" {
  description = "CloudWatch log group name"
  value       = module.realworld.cloudwatch_log_group
}

output "db_password_secret_arn" {
  description = "ARN of the database password secret in Secrets Manager"
  value       = module.realworld.db_password_secret_arn
  sensitive   = true
}

output "jwt_secret_arn" {
  description = "ARN of the JWT secret in Secrets Manager"
  value       = module.realworld.jwt_secret_arn
  sensitive   = true
}

output "password_pepper_secret_arn" {
  description = "ARN of the password pepper secret in Secrets Manager"
  value       = module.realworld.password_pepper_secret_arn
  sensitive   = true
}

output "bastion_instance_id" {
  description = "Bastion instance ID"
  value       = module.realworld.bastion_instance_id
}

output "bastion_private_ip" {
  description = "Bastion private IP address"
  value       = module.realworld.bastion_private_ip
}

output "eic_endpoint_id" {
  description = "EC2 Instance Connect Endpoint ID"
  value       = module.realworld.eic_endpoint_id
}

output "bastion_connect_command" {
  description = "Command to connect to bastion via EC2 Instance Connect"
  value       = module.realworld.bastion_connect_command
}
