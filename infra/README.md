# RealWorld AWS Infrastructure

This directory contains Terraform configuration to deploy the RealWorld application to AWS using ECS Fargate.

## Architecture Overview

The infrastructure includes:

- **VPC** with public and private subnets across 2 availability zones
- **Application Load Balancer** (ALB) in public subnets
- **ECS Fargate** cluster running the application in private subnets
- **RDS PostgreSQL** database in private subnets
- **NAT Gateways** for outbound internet access from private subnets
- Support for container images from GitHub Container Registry or other registries
- **CloudWatch** for logs and monitoring
- **Secrets Manager** for sensitive configuration
- **IAM roles** with least-privilege access

## Prerequisites

1. **AWS Account** with appropriate permissions
2. **AWS CLI** configured with credentials
3. **Terraform** >= 1.0 installed
4. **Docker image** built and pushed do GitHub Container Registry.

## File Structure

- `provider.tf` - AWS provider and Terraform configuration
- `variables.tf` - Input variables
- `terraform.tfvars.example` - Example variable values
- `vpc.tf` - VPC, subnets, NAT gateways, route tables
- `security_groups.tf` - Security groups for ALB, ECS, and RDS
- `rds.tf` - PostgreSQL database
- `iam.tf` - IAM roles and policies
- `secrets.tf` - AWS Secrets Manager secrets
- `alb.tf` - Application Load Balancer
- `cloudwatch.tf` - CloudWatch log groups
- `ecs.tf` - ECS cluster, task definition, and service
- `outputs.tf` - Output values

## Initial Setup

1. **Bootstrap Terraform state backend:**

   The Terraform state is stored in S3 with DynamoDB for locking. Bootstrap it first:

   ```bash
   cd infra
   # Create S3 bucket and DynamoDB table for the environment
   .setup/setup-terraform.sh bootstrap dev
   ```

   This creates:
   - S3 bucket: `realworld-dev-terraform-state`
     - Versioning enabled
     - AES256 encryption
     - Public access blocked
   - DynamoDB table: `realworld-dev-terraform-lock`

   Both resources are tagged with Environment, Name, Project, and ManagedBy tags.

2. **Navigate to environment directory:**
   ```bash
   cd tf/envs/dev  # or cd tf/envs/test for test environment
   ```

3. **Initialize Terraform:**
   ```bash
   terraform init
   ```

## Deployment

1. **Review the plan:**
   ```bash
   terraform plan
   ```

2. **Apply the configuration:**
   ```bash
   terraform apply
   ```

   This will create all resources. The initial deployment takes ~10-15 minutes.

3. **Get the application URL:**
   ```bash
   terraform output alb_url
   ```

## Retrieving Generated Secrets

All secrets are automatically generated and stored in AWS Secrets Manager. To retrieve them:

```bash
# Get database password
aws secretsmanager get-secret-value \
  --secret-id $(terraform output -raw db_password_secret_arn) \
  --query SecretString --output text

# Get JWT secret
aws secretsmanager get-secret-value \
  --secret-id $(terraform output -raw jwt_secret_arn) \
  --query SecretString --output text

# Get password pepper (needed for local development)
aws secretsmanager get-secret-value \
  --secret-id $(terraform output -raw password_pepper_secret_arn) \
  --query SecretString --output text
```

## Monitoring

- **Application Logs:** CloudWatch Logs at `/ecs/realworld-{environment}`
- **ECS Metrics:** CloudWatch Container Insights
- **RDS Metrics:** CloudWatch RDS metrics
- **ALB Metrics:** CloudWatch ALB metrics

View logs:
```bash
aws logs tail /ecs/realworld-dev --follow
```

## Cleanup

To destroy all resources:

```bash
terraform destroy
```

To destroy the state backend (after destroying all infrastructure):
```bash
./setup-terraform.sh teardown dev
```