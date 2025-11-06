# 🏋️‍♂️ GymBuddy - Fitness Tracker API

## 📘 Overview
**GymBuddy** is a backend API built with **Rust** and **Axum**, allowing users to manage fitness data such as workouts, muscle groups, and exercises.  
Currently, it supports full **CRUD operations for users**, laying the foundation for more complex fitness tracking features.

---

## Current Progress (as of Nov 2025)

### Backend Setup
- Project initialized and version-controlled on GitHub  
- Built with **Rust + Axum** web framework  

- Organized modular structure:
src/
├── api/ → route handlers (Axum)
├── db/ → database connections & queries (SQLx)
├── models/ → data models (User, NewUser)
└── main.rs → app entry point


- Connected to a **SQLite** database via SQLx  
- `.env` configured with `DATABASE_URL`  
- Health check route added at `/health`

---

### User CRUD API

Implemented all REST endpoints for managing users:

| Method | Route | Description |
| `GET` | `/api/users` | Fetch all users |
| `POST` | `/api/users` | Create a new user |
| `PUT` | `/api/users/:id` | Update user by ID |
| `DELETE` | `/api/users/:id` | Delete user by ID |

**Example JSON (for POST/PUT):**
```json
{
    "username": "Kevin Jacob",
    "email": "kevin@example.com"
}

--Testing
API tested successfully using Postman

Verified database operations (insert, update, delete) with live SQLite database

Console logs confirm real-time data retrieval from gymbuddy.db

⚙️ How to Run
1. Clone the repository
2. Create a .env file with:
DATABASE_URL=sqlite://gymbuddy.db
3. Run the server:
cargo run

Visit:
http://127.0.0.1:3000

--Next Steps
Add Workout model and routes (exercises, sets, reps, weight, muscle group)

Link users to workouts via foreign key relationship

Expand README with example workout endpoints

Add structured API documentation (docs/architecture.md)

--Tech Stack
Language: Rust

Framework: Axum

Database: SQLite + SQLx

Async Runtime: Tokio

Serialization: Serde

Environment Management: dotenvy