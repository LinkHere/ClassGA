use std::collections::{HashMap, HashSet};

use rand::prelude::*;

use crate::models::{
    Assignment, Course, Day, GenerationRequest, Instructor, Room, Schedule, SessionSlot,
};

#[derive(Clone)]
struct Genome {
    assignments: Vec<Assignment>,
    fitness: f64,
}

pub fn generate_schedule(request: &GenerationRequest) -> Schedule {
    let mut rng = rand::thread_rng();
    let mut population: Vec<Genome> = (0..request.population_size.max(4))
        .map(|_| random_genome(request, &mut rng))
        .collect();

    for _ in 0..request.generations.max(1) {
        population.sort_by(|a, b| b.fitness.total_cmp(&a.fitness));
        let elites = population
            .iter()
            .take(population.len() / 4)
            .cloned()
            .collect::<Vec<_>>();
        let mut next_generation = elites.clone();

        while next_generation.len() < population.len() {
            let parent_a = tournament(&population, 4, &mut rng);
            let parent_b = tournament(&population, 4, &mut rng);
            let mut child = crossover(parent_a, parent_b, &mut rng);
            mutate(&mut child, request, &mut rng);
            child.fitness = fitness(&child.assignments, request);
            next_generation.push(child);
        }

        population = next_generation;
    }

    population.sort_by(|a, b| b.fitness.total_cmp(&a.fitness));
    let best = population.first().expect("population must not be empty");

    Schedule {
        assignments: best.assignments.clone(),
        fitness: best.fitness,
    }
}

fn random_genome(request: &GenerationRequest, rng: &mut ThreadRng) -> Genome {
    let mut assignments = Vec::new();

    for course in &request.courses {
        for _ in 0..course.sessions_per_week.max(1) {
            let day = request.days.choose(rng).cloned().unwrap_or(Day::Monday);
            let slot = SessionSlot {
                day,
                period: rng.gen_range(0..request.periods_per_day.max(1)),
            };
            let room = request
                .rooms
                .choose(rng)
                .expect("at least one room is required");

            assignments.push(Assignment {
                course_id: course.id,
                subject_id: course.subject_id,
                instructor_id: course.instructor_id,
                room_id: room.id,
                slot,
            });
        }
    }

    let fit = fitness(&assignments, request);
    Genome {
        assignments,
        fitness: fit,
    }
}

fn tournament<'a>(population: &'a [Genome], size: usize, rng: &mut ThreadRng) -> &'a Genome {
    let mut best = population
        .choose(rng)
        .expect("population must not be empty");
    for _ in 1..size {
        let candidate = population
            .choose(rng)
            .expect("population must not be empty");
        if candidate.fitness > best.fitness {
            best = candidate;
        }
    }
    best
}

fn crossover(parent_a: &Genome, parent_b: &Genome, rng: &mut ThreadRng) -> Genome {
    let mut assignments = Vec::with_capacity(parent_a.assignments.len());
    for (a, b) in parent_a.assignments.iter().zip(parent_b.assignments.iter()) {
        assignments.push(if rng.gen_bool(0.5) {
            a.clone()
        } else {
            b.clone()
        });
    }

    Genome {
        fitness: 0.0,
        assignments,
    }
}

fn mutate(child: &mut Genome, request: &GenerationRequest, rng: &mut ThreadRng) {
    for assignment in &mut child.assignments {
        if rng.gen_bool(request.mutation_rate.clamp(0.0, 1.0)) {
            if let Some(day) = request.days.choose(rng) {
                assignment.slot.day = day.clone();
            }
            assignment.slot.period = rng.gen_range(0..request.periods_per_day.max(1));
            if let Some(room) = request.rooms.choose(rng) {
                assignment.room_id = room.id;
            }
        }
    }
}

fn fitness(assignments: &[Assignment], request: &GenerationRequest) -> f64 {
    let room_map: HashMap<_, &Room> = request.rooms.iter().map(|r| (r.id, r)).collect();
    let course_map: HashMap<_, &Course> = request.courses.iter().map(|c| (c.id, c)).collect();
    let instructor_map: HashMap<_, &Instructor> =
        request.instructors.iter().map(|i| (i.id, i)).collect();

    let mut score = 10_000.0;
    let mut room_conflicts = 0;
    let mut instructor_conflicts = 0;
    let mut capacity_violations = 0;
    let mut daily_loads: HashMap<(uuid::Uuid, Day), usize> = HashMap::new();
    let mut occupied_rooms: HashSet<(uuid::Uuid, Day, usize)> = HashSet::new();
    let mut occupied_instructors: HashSet<(uuid::Uuid, Day, usize)> = HashSet::new();

    for assignment in assignments {
        let room_slot = (
            assignment.room_id,
            assignment.slot.day.clone(),
            assignment.slot.period,
        );
        if !occupied_rooms.insert(room_slot) {
            room_conflicts += 1;
        }

        let instructor_slot = (
            assignment.instructor_id,
            assignment.slot.day.clone(),
            assignment.slot.period,
        );
        if !occupied_instructors.insert(instructor_slot) {
            instructor_conflicts += 1;
        }

        if let (Some(course), Some(room)) = (
            course_map.get(&assignment.course_id),
            room_map.get(&assignment.room_id),
        ) {
            if room.capacity < course.student_count {
                capacity_violations += 1;
            }
        }

        *daily_loads
            .entry((assignment.instructor_id, assignment.slot.day.clone()))
            .or_insert(0) += 1;
    }

    for ((instructor_id, _), load) in daily_loads {
        if let Some(instructor) = instructor_map.get(&instructor_id) {
            if load > instructor.max_sessions_per_day {
                score -= ((load - instructor.max_sessions_per_day) * 200) as f64;
            }
        }
    }

    score -= (room_conflicts * 500) as f64;
    score -= (instructor_conflicts * 500) as f64;
    score -= (capacity_violations * 300) as f64;

    score.max(0.0)
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::generate_schedule;
    use crate::models::{Course, Day, GenerationRequest, Instructor, Room, Subject};

    #[test]
    fn generates_non_empty_schedule() {
        let subject_id = Uuid::new_v4();
        let instructor_id = Uuid::new_v4();
        let room_id = Uuid::new_v4();

        let request = GenerationRequest {
            courses: vec![Course {
                id: Uuid::new_v4(),
                name: "Algorithms 101".into(),
                subject_id,
                instructor_id,
                sessions_per_week: 3,
                student_count: 30,
            }],
            instructors: vec![Instructor {
                id: instructor_id,
                name: "Dr. Ada".into(),
                max_sessions_per_day: 3,
            }],
            rooms: vec![Room {
                id: room_id,
                name: "Lab A".into(),
                capacity: 40,
            }],
            subjects: vec![Subject {
                id: subject_id,
                name: "Computer Science".into(),
            }],
            days: vec![Day::Monday, Day::Tuesday, Day::Wednesday],
            periods_per_day: 5,
            population_size: 20,
            generations: 30,
            mutation_rate: 0.2,
        };

        let schedule = generate_schedule(&request);
        assert_eq!(schedule.assignments.len(), 3);
    }
}
