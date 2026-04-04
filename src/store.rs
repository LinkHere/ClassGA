use std::collections::HashMap;

use parking_lot::RwLock;
use uuid::Uuid;

use crate::models::{Course, Day, Instructor, Room, Subject};

#[derive(Default)]
pub struct AppStore {
    pub courses: RwLock<HashMap<Uuid, Course>>,
    pub instructors: RwLock<HashMap<Uuid, Instructor>>,
    pub rooms: RwLock<HashMap<Uuid, Room>>,
    pub subjects: RwLock<HashMap<Uuid, Subject>>,
    pub days: RwLock<Vec<Day>>,
}

impl AppStore {
    pub fn upsert_course(&self, course: Course) {
        self.courses.write().insert(course.id, course);
    }

    pub fn upsert_instructor(&self, instructor: Instructor) {
        self.instructors.write().insert(instructor.id, instructor);
    }

    pub fn upsert_room(&self, room: Room) {
        self.rooms.write().insert(room.id, room);
    }

    pub fn upsert_subject(&self, subject: Subject) {
        self.subjects.write().insert(subject.id, subject);
    }

    pub fn set_days(&self, days: Vec<Day>) {
        *self.days.write() = days;
    }
}
