# Rust Backend with Actix Web and PostgreSQL

## Overview
This project is a simple **Rust backend** using **Actix Web** for handling HTTP requests and **PostgreSQL** for storing user data. The focus is on **authentication** and a basic **REST API** to manage users.

## Features
- **User Authentication** (JWT-based login & registration)
- **REST API** (CRUD for users)
- **PostgreSQL Database** (Using SQLx for async queries)
- **Environment Configurations** (dotenv for managing secrets)

## Technologies Used
- [Actix Web](https://actix.rs/) - Fast web framework for Rust
- [SQLx](https://github.com/launchbadge/sqlx) - Async PostgreSQL driver
- [JWT](https://crates.io/crates/jsonwebtoken) - Authentication via JWT
- [dotenv](https://crates.io/crates/dotenv) - Load environment variables
- [serde](https://serde.rs/) - Serialization/Deserialization

## Project Structure
```
my_project/
│── src/
│   ├── main.rs         # Entry point
│   ├── config.rs       # Database connection
│   ├── routes.rs       # API routes
│   ├── handlers/
│   │   ├── auth.rs     # Authentication logic
│   │   ├── users.rs    # User handlers
│   ├── models/
│   │   ├── user.rs     # User model
│   ├── db/
│   │   ├── users.rs    # User DB queries
│── .env                # Environment variables
│── Cargo.toml          # Dependencies
│── migrations/         # SQLx migrations
```

## Setup
### 1. Install Dependencies
Ensure you have **Rust** and **PostgreSQL** installed.
```sh
cargo install sqlx-cli --no-default-features --features postgres
```

### 2. Clone the Repository
```sh
git clone https://github.com/your-username/my_project.git
cd my_project
```

### 3. Configure Environment Variables
Create a `.env` file and set your PostgreSQL connection string:
```
DATABASE_URL=postgres://user:password@localhost:5432/mydatabase
JWT_SECRET=my_super_secret_key
```

### 4. Run Database Migrations
```sh
sqlx migrate run
```

### 5. Start the Server
```sh
cargo run
```

## API Endpoints
### Authentication
| Method | Endpoint      | Description          |
|--------|--------------|----------------------|
| POST   | /register    | Create new user      |
| POST   | /login       | Authenticate user    |

### Users
| Method | Endpoint      | Description           |
|--------|--------------|-----------------------|
| GET    | /users       | List all users        |
| GET    | /users/{id}  | Get user by ID        |
| PUT    | /users/{id}  | Update user           |
| DELETE | /users/{id}  | Delete user           |

## Authentication
This project uses **JWT** for authentication.
- Upon login, users receive a **JWT token**.
- The token must be included in the **Authorization** header for protected routes.

### Example Request (Login)
```sh
curl -X POST http://localhost:8080/login \
 -H "Content-Type: application/json" \
 -d '{"email": "user@example.com", "password": "password123"}'
```

### Example Response
```json
{
  "token": "your_jwt_token_here"
}
```

## Contributing
Pull requests are welcome! If you find any issues, please open an issue.

## License
This project is licensed under the MIT License.

