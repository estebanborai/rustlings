use crate::exercise::{ExerciseList, State};
use std::fs;

pub fn print_progress() {
  let p = Progress::new();

  for e in p.done.exercises.iter() {
    println!("{}", e.name);
  }

  for e in p.pending.exercises.iter() {
    println!("{}", e.name);
  }
}

struct Progress {
  done: ExerciseList,
  pending: ExerciseList
}

impl Progress {
  pub fn new() -> Self {
    let toml_str = &fs::read_to_string("info.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;
    let mut done: ExerciseList = ExerciseList {
      exercises: Vec::new()
    };

    let mut pending: ExerciseList = ExerciseList {
      exercises: Vec::new()
    };

    for ex in exercises.into_iter() {
      match ex.state() {
        State::Done => {
          done.exercises.push(ex);
        },
        State::Pending(_) => {
          pending.exercises.push(ex);
        }
      }
    }
    
    Progress {
      done,
      pending
    }
  }
}