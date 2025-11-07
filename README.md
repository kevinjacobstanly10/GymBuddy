# 🏋️‍♂️ GymBuddy - Fitness Tracker API

## 📘 Overview
**GymBuddy** is a backend API built with **Rust** and **Axum**, allowing users to manage fitness data such as workouts, muscle groups, exercises, and track progress.  
It supports **CRUD operations for users, workouts, exercises, and workout entries**, along with analytics to summarize training performance.

## 🗂 API Endpoints Summary

**Users**
- `GET /api/users` → Fetch all users
- `POST /api/users` → Create a new user
- `GET /api/users/:id` → Get a user by ID
- `PUT /api/users/:id` → Update a user by ID
- `DELETE /api/users/:id` → Delete a user by ID
- `GET /api/users/:id/progress` → Get per-workout progress with volume and top exercises

**Workouts**
- `GET /api/workouts` → List all workouts
- `POST /api/workouts` → Create a new workout
- `GET /api/workouts/:id` → Fetch a workout by ID
- `DELETE /api/workouts/:id` → Delete a workout
- `GET /api/workouts/:id/entries` → Get entries for a workout
- `GET /api/workouts/:id/summary` → Get workout summary

**Exercises**
- `GET /api/exercises` → List all exercises
- `POST /api/exercises` → Add new exercises

**Workout Entries**
- `GET /api/workout_entries` → List all workout entries (detailed)
- `POST /api/workout_entries` → Create a workout entry
- `PUT /api/workout_entries/:id` → Update a workout entry
- `DELETE /api/workout_entries/:id` → Delete a workout entry

## Current Progress (as of Nov 2025)

### Backend Setup
- Project initialized and version-controlled on GitHub  
- Built with **Rust + Axum** web framework  
- Organized modular structure:
src/
├── api/ → route handlers (Axum)
├── db/ → database connections & queries (SQLx)
├── models/ → data models (User, NewUser, Workout, Exercise, WorkoutEntry)
└── main.rs → app entry point

- Connected to a **SQLite** database via SQLx  
- `.env` configured with `DATABASE_URL`  
- Health check route added at `/health`

## User CRUD API
Implemented all REST endpoints for managing users:

| Method | Route | Description |
| ------ | ----- | ----------- |
| `GET` | `/api/users` | Fetch all users |
| `POST` | `/api/users` | Create a new user |
| `PUT` | `/api/users/:id` | Update user by ID |
| `DELETE` | `/api/users/:id` | Delete user by ID |
| `GET` | `/api/users/:id/progress` | Get per-workout progress with volume and top exercises |

**Example JSON (for POST/PUT):**
```json
{
    "username": "Kevin Jacob",
    "email": "kevin@example.com"
}
```
## 🏋️ Workouts, Exercises & Entries
### Workouts
CRUD operations: create, fetch, delete

Each workout is linked to a user via user_id

Example workout:
```json
{
    "user_id": 1,
    "date": "2025-11-07",
    "notes": "Chest day"
}
```
### Exercises
CRUD operations for exercises

Supports multiple muscle groups and descriptions

Example exercises:
```json
[
    {
        "name": "Barbell Bench Press",
        "muscle_group": "Chest",
        "description": "A compound exercise that targets the chest, triceps, and shoulders."
    },
    {
        "name": "Barbell Squat",
        "muscle_group": "Legs",
        "description": "Targets quadriceps, hamstrings, and glutes."
    }
]
```
### Workout Entries
Link workouts to exercises with sets, reps, and weight

Supports detailed queries per workout or all entries

Example entry:
```json
{
    "workout_id": 1,
    "exercise_id": 1,
    "sets": 4,
    "reps": 10,
    "weight": 80.0
}
```
## Progress Analytics
GET/api/users/:id/progress returns per-workout summaries:

total_sets, total_reps, total_volume

muscle_groups distribution (volume per muscle group)

top_exercises by volume

Example Response:
```json
[
    {
        "date": "2025-11-07",
        "muscle_groups": { "Chest": 5360.0 },
        "top_exercises": [
            { "name": "Barbell Bench Press", "volume": 3200.0 },
            { "name": "Incline Dumbbell Press", "volume": 2160.0 }
        ],
        "total_reps": 22,
        "total_sets": 7,
        "total_volume": 5360.0,
        "workout_id": 1
    },
    {
        "date": "2025-11-08",
        "muscle_groups": { "Hamstrings": 2880.0, "Legs": 4000.0 },
        "top_exercises": [
            { "name": "Barbell Squat", "volume": 4000.0 },
            { "name": "Barbell Romanian Deadlift", "volume": 2880.0 }
        ],
        "total_reps": 22,
        "total_sets": 7,
        "total_volume": 6880.0,
        "workout_id": 2
    }
]
```
## How to Run
Clone the repository

Create a .env file with:
DATABASE_URL=sqlite://gymbuddy.db

Run the server:

cargo run
Visit: http://127.0.0.1:3000

## Next Steps
Implement user authentication (JWT-based)

Expand workout analytics (weekly/monthly summaries)

Add unit and integration tests for all endpoints

Build structured API documentation (docs/architecture.md)

## Tech Stack
Language: Rust

Framework: Axum

Database: SQLite + SQLx

Async Runtime: Tokio

Serialization: Serde

Environment Management: dotenvy