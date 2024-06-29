use std::default;

use chrono::{DateTime, Utc};

/// This file contains the aggregation of the main types
/// used in this application:
/// * Task
/// * Habit
/// * Todo
/// * Daily
/// * Event

#[derive(Debug, Clone, Default)]
pub enum TaskType {
    Task = 1,
    Habit = 2,
    #[default]
    Todo = 3,
    Daily = 4,
    Event = 5,
}

#[derive(Debug, Clone, Default)]
pub enum Status {
    #[default]
    Backlog = 1,
    InProgress = 2,
    Done = 3,
}

#[derive(Debug, Clone)]
pub struct Task {
    title: String,
    task_type: TaskType,
    status: Status,
    description: Option<String>,
    due_date: Option<DateTime<Utc>>,
}

impl Task {
    pub fn builder(title: String, task_type: Option<TaskType>) -> TaskBuilder {
        let builder_task_type = if let Some(task_t) = task_type {
            task_t
        } else {
            // The default type to create when none is specified
            TaskType::Todo
        };

        TaskBuilder {
            title,
            task_type: builder_task_type,
            status: Status::Backlog,
            description: None,
            due_date: None,
        }
    }
}

pub struct TaskBuilder {
    title: String,
    task_type: TaskType,
    status: Status,
    description: Option<String>,
    due_date: Option<DateTime<Utc>>,
}

impl TaskBuilder {
    pub fn with_status(self, status: Status) -> Self {
        TaskBuilder { status, ..self }
    }

    pub fn with_description(self, description: String) -> Self {
        TaskBuilder {
            description: Some(description),
            ..self
        }
    }

    pub fn with_due_data(self, due_date: DateTime<Utc>) -> Self {
        TaskBuilder {
            due_date: Some(due_date),
            ..self
        }
    }

    pub fn build(self) -> Task {
        Task {
            title: self.title,
            task_type: self.task_type,
            status: self.status,
            description: self.description,
            due_date: self.due_date,
        }
    }
}
