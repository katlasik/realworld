# Docker Setup for RealWorld

This directory contains Docker configuration for running the PostgreSQL database required by the RealWorld application.

## Prerequisites

- Docker and Docker Compose installed on your system

## Quick Start

1. **Start the database:**
   ```bash
   cd docker
   docker-compose up -d
   ```

2. **Check the database is running:**
   ```bash
   docker-compose ps
   ```

3. **View logs:**
   ```bash
   docker-compose logs -f postgres
   ```

4. **Stop the database:**
   ```bash
   docker-compose down
   ```

5. **Stop and remove data (clean slate):**
   ```bash
   docker-compose down -v
   ```

## Database Details

- **Host:** localhost
- **Port:** 5432
- **Database:** realworld
- **Username:** postgres
- **Password:** password

The database credentials match those in `.env.example` in the project root.

## Connecting to the Database

### Using psql
```bash
docker-compose exec postgres psql -U postgres -d realworld
```

### From your application
Make sure your `.env` file contains:
```
DATABASE_URL=postgres://postgres:password@localhost:5432/realworld
```

## Data Persistence

Database data is persisted in a Docker volume named `postgres_data`. This means your data will survive container restarts.

To completely reset the database, remove the volume:
```bash
docker-compose down -v
```
