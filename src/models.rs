use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub name: String,
    pub subject_id: Uuid,
    pub instructor_id: Uuid,
    pub sessions_per_week: usize,
    pub student_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instructor {
    pub id: Uuid,
    pub name: String,
    pub max_sessions_per_day: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub capacity: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSlot {
    pub day: Day,
    pub period: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub course_id: Uuid,
    pub subject_id: Uuid,
    pub instructor_id: Uuid,
    pub room_id: Uuid,
    pub slot: SessionSlot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub assignments: Vec<Assignment>,
    pub fitness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub courses: Vec<Course>,
    pub instructors: Vec<Instructor>,
    pub rooms: Vec<Room>,
    pub subjects: Vec<Subject>,
    pub days: Vec<Day>,
    pub periods_per_day: usize,
    pub population_size: usize,
    pub generations: usize,
    pub mutation_rate: f64,
}
