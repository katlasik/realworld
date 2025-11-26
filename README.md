> ### Rust + Axum codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.

### [Demo](https://demo.realworld.io/)&nbsp;&nbsp;&nbsp;&nbsp;[RealWorld](https://github.com/gothinkster/realworld)

## Tech Stack

- **[Axum](https://github.com/tokio-rs/axum)** - Modern, ergonomic web framework
- **[SQLx](https://github.com/launchbadge/sqlx)** - Compile-time verified SQL queries
- **[PostgreSQL](https://www.postgresql.org/)** - Reliable relational database
- **[Sea Query](https://github.com/SeaQL/sea-query)** - Type-safe query builder
- **[Tokio](https://tokio.rs/)** - Async runtime for high-performance networking
- **[Argon2](https://github.com/RustCrypto/password-hashes)** - Secure password hashing
- **[JWT](https://github.com/Keats/jsonwebtoken)** - JSON Web Token implementation

## Prerequisites

- **Rust** 1.75+ ([install](https://rustup.rs/))
- **PostgreSQL** 15+ ([install](https://www.postgresql.org/download/))
- **Docker** (optional, for running PostgreSQL in container)

## Getting Started

### 1. Set Up Database

```bash
cd docker
docker-compose up -d
cd ..
```

This starts PostgreSQL on `localhost:5432` with:
- Database: `realworld`
- Username: `realworld`
- Password: `realworld`

### 2. Configure Environment

```bash
cp .env.example .env
```

Edit `.env` to match your setup (if using different database credentials):

```env
DATABASE_URL=postgres://realworld:realworld@localhost:5432/realworld
JWT_SECRET=your-secret-key-change-in-production
PASSWORD_PEPPER=your-pepper-change-in-production-and-never-change-after
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

⚠️ **Important**: `PASSWORD_PEPPER` must remain constant in production. Changing it will invalidate all existing passwords.

### 3. Build and Run

```bash
cargo run
```

The server will start at `http://localhost:8080`.

## API Tests (Postman Collection)

The project includes the official RealWorld API test suite:

```bash
# Make sure the server is running first
cargo run

# In another terminal, run the API tests
APIURL=http://localhost:8080/api ./api/run-api-tests.sh
```

Requirements for API tests:
- Node.js and npm installed
- Newman (Postman CLI) will be automatically installed via npx

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Resources

- [RealWorld Spec](https://realworld-docs.netlify.app/)
- [Axum Documentation](https://docs.rs/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)

## Acknowledgments

- [RealWorld](https://github.com/gothinkster/realworld) for the specification
- The Rust community for excellent tools and documentation
- Contributors who help improve this project
