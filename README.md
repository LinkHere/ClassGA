# ClassGA

A scalable web class scheduling system built in **Rust** using a **Genetic Algorithm (GA)**.

## What it schedules
The scheduler uses your core entities:
- Course
- Instructor
- Room
- Subject
- Day

It optimizes schedules by penalizing:
- Room conflicts (same room, same day/period)
- Instructor conflicts (same instructor, same day/period)
- Room capacity violations
- Instructor overload (beyond max sessions/day)

## Architecture (upgrade-friendly)
- **HTTP API layer** (`axum`) for web integration.
- **Domain models** in `models.rs`.
- **GA engine** isolated in `ga.rs`.
- **Store layer** in `store.rs` for in-memory persistence today (easy to replace with Postgres/Redis later).

This separation makes future upgrades straightforward (auth, persistence, multi-campus constraints, custom fitness plugins, etc).

## Run
```bash
cargo run
```

Server starts at `http://localhost:3000`.

## API
### Health
`GET /health`

### Seed data
- `POST /courses`
- `POST /instructors`
- `POST /rooms`
- `POST /subjects`
- `POST /days`

### Generate schedule
`POST /schedule/generate`

You can either:
1. Send all data in the generation payload, or
2. Seed data first and send empty lists in generation payload to reuse stored data.

### Example generation payload
```json
{
  "courses": [
    {
      "id": "c31a70ea-1f14-4ef5-af02-6de64e6d8435",
      "name": "Algorithms 101",
      "subject_id": "56aaf9ef-85e3-4986-93a0-e03e7057c2d1",
      "instructor_id": "1cb7a0dc-72de-4f2d-9e90-94f9e15b5f4f",
      "sessions_per_week": 3,
      "student_count": 30
    }
  ],
  "instructors": [
    {
      "id": "1cb7a0dc-72de-4f2d-9e90-94f9e15b5f4f",
      "name": "Dr. Ada",
      "max_sessions_per_day": 3
    }
  ],
  "rooms": [
    {
      "id": "c24fa11a-e844-4ded-a8ee-bf8d8e8f4b0e",
      "name": "Lab A",
      "capacity": 40
    }
  ],
  "subjects": [
    {
      "id": "56aaf9ef-85e3-4986-93a0-e03e7057c2d1",
      "name": "Computer Science"
    }
  ],
  "days": ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"],
  "periods_per_day": 5,
  "population_size": 50,
  "generations": 200,
  "mutation_rate": 0.15
}
```

## Quick test
```bash
cargo test
```
