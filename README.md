# GreatWay

GreatWay is a Rust-based Application Gateway that provides user authentication and role-based access control.

## Features

- User registration and login
- JWT-based authentication
- Role-based access control
- Token expiration handling
- SQLite database for user and role storage

## Prerequisites

- Rust (latest stable version)

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/Jabbslad/greatway.git
   cd greatway
   ```

2. Set up the environment variables:
   Create a `.env` file in the project root and add the following:
   ```
   DATABASE_URL=sqlite:users.db
   JWT_SECRET=your_secret_key_here
   ```

3. Build the project:
   ```
   cargo build
   ```

## Running the Application

To run the application:

```
cargo run
```

The server will start on `localhost:3000` by default.

## API Endpoints

- `POST /register`: Register a new user
  - Body: `{ "username": "user", "password": "pass" }`

- `POST /login`: Login and receive a JWT token
  - Body: `{ "username": "user", "password": "pass" }`

- `GET /protected`: Access a protected route (requires valid JWT token)
  - Header: `Authorization: Bearer <your_token_here>`

## Token Expiration

- Tokens are set to expire after 1 hour
- The `X-Token-Expiry` header in responses indicates the token's expiration time

## Database

User and role information is stored in an SQLite database. The schema includes:

- `users` table: Stores user information
- `user_roles` table: Manages user-role associations

## Security

- Passwords are hashed using bcrypt before storing
- JWTs are used for maintaining user sessions
- Role-based access control is implemented for protected routes

## Error Handling

The application includes custom error handling to provide meaningful error messages for various scenarios, including authentication failures and database errors.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
