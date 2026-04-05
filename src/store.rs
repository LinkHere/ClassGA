use std::sync::Mutex;

use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::models::{Course, Day, Instructor, Room, Subject};

pub struct AppStore {
    conn: Mutex<Connection>,
}

impl AppStore {
    pub fn new(path: &str) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            r#"
            PRAGMA journal_mode=WAL;

            CREATE TABLE IF NOT EXISTS courses (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                subject_id TEXT NOT NULL,
                instructor_id TEXT NOT NULL,
                sessions_per_week INTEGER NOT NULL,
                student_count INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS instructors (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                max_sessions_per_day INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS rooms (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                capacity INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS subjects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS days (
                day TEXT PRIMARY KEY
            );
            "#,
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn upsert_course(&self, course: Course) -> rusqlite::Result<()> {
        let conn = self.conn.lock().expect("sqlite connection lock poisoned");
        conn.execute(
            "INSERT INTO courses (id, name, subject_id, instructor_id, sessions_per_week, student_count)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(id) DO UPDATE SET
                name=excluded.name,
                subject_id=excluded.subject_id,
                instructor_id=excluded.instructor_id,
                sessions_per_week=excluded.sessions_per_week,
                student_count=excluded.student_count",
            params![
                course.id.to_string(),
                course.name,
                course.subject_id.to_string(),
                course.instructor_id.to_string(),
                course.sessions_per_week as i64,
                course.student_count as i64
            ],
        )?;
        Ok(())
    }

    pub fn upsert_instructor(&self, instructor: Instructor) -> rusqlite::Result<()> {
        let conn = self.conn.lock().expect("sqlite connection lock poisoned");
        conn.execute(
            "INSERT INTO instructors (id, name, max_sessions_per_day)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET
                name=excluded.name,
                max_sessions_per_day=excluded.max_sessions_per_day",
            params![
                instructor.id.to_string(),
                instructor.name,
                instructor.max_sessions_per_day as i64
            ],
        )?;
        Ok(())
    }

    pub fn upsert_room(&self, room: Room) -> rusqlite::Result<()> {
        let conn = self.conn.lock().expect("sqlite connection lock poisoned");
        conn.execute(
            "INSERT INTO rooms (id, name, capacity)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET
                name=excluded.name,
                capacity=excluded.capacity",
            params![room.id.to_string(), room.name, room.capacity as i64],
        )?;
        Ok(())
    }

    pub fn upsert_subject(&self, subject: Subject) -> rusqlite::Result<()> {
        let conn = self.conn.lock().expect("sqlite connection lock poisoned");
        conn.execute(
            "INSERT INTO subjects (id, name)
             VALUES (?1, ?2)
             ON CONFLICT(id) DO UPDATE SET
                name=excluded.name",
            params![subject.id.to_string(), subject.name],
        )?;
        Ok(())
    }

    pub fn set_days(&self, days: Vec<Day>) -> rusqlite::Result<()> {
        let mut conn = self.conn.lock().expect("sqlite connection lock poisoned");
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM days", [])?;
        for day in days {
            tx.execute(
                "INSERT INTO days (day) VALUES (?1)",
                params![day.as_str().to_string()],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn courses(&self) -> rusqlite::Result<Vec<Course>> {
        let conn = self.conn.lock().expect("sqlite connection lock poisoned");
        let mut stmt = conn.prepare(
            "SELECT id, name, subject_id, instructor_id, sessions_per_week, student_count FROM courses",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Course {
                id: parse_uuid(row.get::<_, String>(0)?)?,
                name: row.get(1)?,
                subject_id: parse_uuid(row.get::<_, String>(2)?)?,
                instructor_id: parse_uuid(row.get::<_, String>(3)?)?,
                sessions_per_week: row.get::<_, i64>(4)? as usize,
                student_count: row.get::<_, i64>(5)? as usize,
            })
        })?;

        rows.collect()
    }

    pub fn instructors(&self) -> rusqlite::Result<Vec<Instructor>> {
        let conn = self.conn.lock().expect("sqlite connection lock poisoned");
        let mut stmt = conn.prepare("SELECT id, name, max_sessions_per_day FROM instructors")?;
        let rows = stmt.query_map([], |row| {
            Ok(Instructor {
                id: parse_uuid(row.get::<_, String>(0)?)?,
                name: row.get(1)?,
                max_sessions_per_day: row.get::<_, i64>(2)? as usize,
            })
        })?;

        rows.collect()
    }

    pub fn rooms(&self) -> rusqlite::Result<Vec<Room>> {
        let conn = self.conn.lock().expect("sqlite connection lock poisoned");
        let mut stmt = conn.prepare("SELECT id, name, capacity FROM rooms")?;
        let rows = stmt.query_map([], |row| {
            Ok(Room {
                id: parse_uuid(row.get::<_, String>(0)?)?,
                name: row.get(1)?,
                capacity: row.get::<_, i64>(2)? as usize,
            })
        })?;

        rows.collect()
    }

    pub fn subjects(&self) -> rusqlite::Result<Vec<Subject>> {
        let conn = self.conn.lock().expect("sqlite connection lock poisoned");
        let mut stmt = conn.prepare("SELECT id, name FROM subjects")?;
        let rows = stmt.query_map([], |row| {
            Ok(Subject {
                id: parse_uuid(row.get::<_, String>(0)?)?,
                name: row.get(1)?,
            })
        })?;

        rows.collect()
    }

    pub fn days(&self) -> rusqlite::Result<Vec<Day>> {
        let conn = self.conn.lock().expect("sqlite connection lock poisoned");
        let mut stmt = conn.prepare("SELECT day FROM days")?;
        let rows = stmt.query_map([], |row| {
            let text: String = row.get(0)?;
            Ok(Day::from_str(&text))
        })?;

        rows.collect()
    }
}

fn parse_uuid(id: String) -> rusqlite::Result<Uuid> {
    Uuid::parse_str(&id).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })
}
