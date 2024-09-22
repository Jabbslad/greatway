use log::info;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn create_db_pool() -> DbPool {
    let manager = SqliteConnectionManager::file("users.db");
    Pool::new(manager).expect("Failed to create pool.")
}

pub fn initialize_db(pool: &DbPool) -> Result<()> {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL
            )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_roles (
                user_id TEXT,
                role TEXT NOT NULL,
                FOREIGN KEY(user_id) REFERENCES users(id),
                PRIMARY KEY(user_id, role)
            )",
        [],
    )?;

    let users: u32 = conn
        .query_row_and_then("SELECT COUNT(*) FROM users", [], |r| r.get(0))
        .unwrap();

    if users == 0 {
        info!("Creating default admin user");
        create_user(
            &pool,
            &std::env::var("GREATWAY_ADMIN_USERNAME").expect("GREATWAY_ADMIN_USERNAME not set"),
            &std::env::var("GREATWAY_ADMIN_PASSWORD").expect("GATEWAY_ADMIN_PASSWORD not set"),
        )
        .and_then(|user| add_role_to_user(&pool, &user.id, Role::Admin))?;
    }

    Ok(())
}

use bcrypt::{hash, DEFAULT_COST};
use uuid::Uuid;

use crate::models::user::{Role, User};

pub fn create_user(pool: &DbPool, username: &str, password: &str) -> Result<User> {
    let conn = pool.get().expect("Couldn't get db connection from pool");
    let id = Uuid::new_v4().to_string();
    let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");

    conn.execute(
        "INSERT INTO users (id, username, password) VALUES (?, ?, ?)",
        &[&id, username, &hashed_password],
    )?;

    Ok(User {
        id,
        username: username.to_string(),
        password: hashed_password,
    })
}

pub fn get_user_by_username(pool: &DbPool, username: &str) -> Result<Option<User>> {
    let conn = pool.get().expect("Couldn't get db connection from pool");
    let mut stmt = conn.prepare("SELECT id, username, password FROM users WHERE username = ?")?;
    let user_iter = stmt.query_map([username], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
        })
    })?;

    let user = user_iter.map(|u| u.unwrap()).next();
    Ok(user)
}

pub fn add_role_to_user(pool: &DbPool, user_id: &str, role: Role) -> Result<()> {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    conn.execute(
        "INSERT OR IGNORE INTO user_roles (user_id, role) VALUES (?, ?)",
        &[user_id, &serde_json::to_string(&role).unwrap()],
    )?;

    Ok(())
}

pub fn get_user_roles(pool: &DbPool, user_id: &str) -> Result<Vec<Role>> {
    let conn = pool.get().expect("Couldn't get db connection from pool");
    let mut stmt = conn.prepare("SELECT role FROM user_roles WHERE user_id = ?")?;

    let roles = stmt
        .query_map([user_id], |row| {
            let role_str: String = row.get(0)?;
            Ok(serde_json::from_str(&role_str).unwrap())
        })?
        .collect::<Result<Vec<Role>>>()?;

    Ok(roles)
}
