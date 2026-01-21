# 🏋️‍♂️ GymBuddy — Fitness Tracker Backend API

##  Overview

**GymBuddy** is a backend REST API built using **Rust** and **Axum** that allows users to log workouts, track training progress, analyze muscle group distribution and view fitness analytics over time.

The system is designed as a **high-performance and scalable backend** intended to serve a future mobile or web fitness application. It focuses on structured data modeling, secure authentication and efficient computation of workout analytics.

---

## Project Motivation

GymBuddy was developed to gain experience in:

- Backend API development
- Secure authentication (JWT)
- Relational data modeling
- Analytics computation over user-generated data
- Observability using structured logging
- Writing unit and integration tests

Coming from a **game development background (Unity)**, this project bridges familiar concepts such as state management and performance optimization into a backend systems context. As a fitness enthusiast myself, GymBuddy also solves a real-world problem of accurately tracking workout history and progress beyond simple note-taking apps.

---

## User Stories (Implemented)

- As a user, I can **register** and **log in securely**
- As a user, I can browse **exercises categorized by muscle groups**
- As a user, I can **log workouts** with exercises, sets, reps and weight
- As a user, I can view **weekly training analytics**
- As a user, I can view **lifetime workout history and progress summaries**

---

## Tech Stack

- **Language:** Rust  
- **Framework:** Axum  
- **Database:** SQLite  
- **ORM / Queries:** SQLx  
- **Authentication:** JWT  
- **Password Hashing:** Argon2  
- **Async Runtime:** Tokio  
- **Logging:** tracing  
- **Serialization:** Serde  
- **Environment Config:** dotenvy  
- **Testing:** Cargo (integration tests)

---

## Project Structure
```text
src/
├── api/            # Route handlers
├── auth.rs         # Password hashing & verification
├── jwt.rs          # JWT handling
├── middleware/     # Auth middleware
├── db/             # Database logic
├── models/         # Data models
├── main.rs         # Application entry point
└── lib.rs

tests/
├── api_health.rs
└── auth_register.rs

```

## Authentication

- JWT-based authentication
- Passwords hashed using **Argon2**
- Protected routes require:

Authorization header format:
```http
Authorization: Bearer <token>
```
- Authentication enforced via a custom Axum extractor

---

## API Endpoints

### Public Endpoints

| Method | Route | Description |
|------|------|-------------|
| POST | `/api/register` | Register a new user |
| POST | `/api/login` | Login and receive JWT |
| GET | `/health` | Health check |

---

### Protected Endpoints (JWT Required)

#### Users
| Method | Route | Description |
|------|------|-------------|
| GET | `/api/users` | Fetch all users |
| GET | `/api/users/:id` | Fetch user by ID |
| GET | `/api/users/:id/progress` | User workout analytics |

#### Workouts
| Method | Route | Description |
|------|------|-------------|
| GET | `/api/workouts` | List workouts |
| POST | `/api/workouts` | Create workout |
| GET | `/api/workouts/:id` | Fetch workout |
| PUT | `/api/workouts/:id` | Update workout |
| DELETE | `/api/workouts/:id` | Delete workout |
| GET | `/api/workouts/:id/entries` | Workout entries |
| GET | `/api/workouts/:id/summary` | Workout summary |

#### Exercises
| Method | Route | Description |
|------|------|-------------|
| GET | `/api/exercises` | List exercises |
| POST | `/api/exercises` | Add exercises |

#### Workout Entries
| Method | Route | Description |
|------|------|-------------|
| GET | `/api/workout_entries` | List entries |
| POST | `/api/workout_entries` | Create entry |
| PUT | `/api/workout_entries/:id` | Update entry |
| DELETE | `/api/workout_entries/:id` | Delete entry |

#### Analytics
| Method | Route | Description |
|------|------|-------------|
| GET | `/api/analytics/weekly` | Weekly analytics |

---

## Analytics

### Weekly Analytics
- Total training volume
- Most trained muscle group (last 7 days)

### Progress Analytics
- Per-workout summaries
- Total volume
- Muscle group distribution
- Top exercises by volume

---

## Logging & Observability

This project uses the **tracing** crate for structured logging:

- User registration & login
- Workout creation
- Analytics queries
- Error handling

Example logs:
```text
INFO Registering new user: user@example.com
INFO Login successful for user@example.com
INFO Fetching weekly analytics for user 1
``` 
---

## Testing

Included integration tests for core endpoints:

- Health check endpoint
- User registration flow

Run tests:
```bash
cargo test 
```

Running the Project

1. Clone the repository
git clone <https://github.com/kevinjacobstanly10/GymBuddy.git>
cd GymBuddy

2. Configure environment
Create a .env file:
DATABASE_URL=sqlite://gymbuddy.db

3. Run the server
cargo run
Server runs at:
http://127.0.0.1:3000

## Project Status

### Completed
- RESTful API built with Rust and Axum
- SQLite database integration using SQLx
- Data models for users, workouts, exercises and workout entries
- JWT-based authentication and authorization
- Secure password hashing with Argon2
- Workout analytics (weekly volume and muscle group distribution)
- Structured logging using the tracing crate
- Integration tests for core API endpoints
- Full project documentation and setup guide

### Planned Enhancements
- Monthly and yearly analytics summaries
- Gamified XP and leveling system
- Exportable progress reports
- Web or mobile client application