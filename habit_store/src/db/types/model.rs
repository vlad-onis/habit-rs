use chrono::{DateTime, Utc};
use sqlx::{
    postgres::PgPool,
    prelude::{FromRow, Type},
};
use thiserror::Error;

/// This file contains the aggregation of the main types
/// used in this application:
/// * Task
/// * Habit
/// * Todo
/// * Daily
/// * Event

const TASKS_TABLE: &str = "tasks";

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to insert task because: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Task Type {0} does not exist")]
    UnknownTaskType(i32),

    #[error("Status {0} does not exist")]
    UnknownStatus(i32),
}

#[derive(Debug, Clone, Copy, Default, Type)]
#[repr(i32)]
pub enum TaskType {
    Task = 1,
    Habit = 2,
    #[default]
    Todo = 3,
    Daily = 4,
    Event = 5,
}

impl From<TaskType> for i32 {
    fn from(value: TaskType) -> Self {
        match value {
            TaskType::Task => 1,
            TaskType::Habit => 2,
            TaskType::Todo => 3,
            TaskType::Daily => 4,
            TaskType::Event => 5,
        }
    }
}

impl TryFrom<i32> for TaskType {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TaskType::Task),
            2 => Ok(TaskType::Habit),
            3 => Ok(TaskType::Todo),
            4 => Ok(TaskType::Daily),
            5 => Ok(TaskType::Event),
            _ => Err(Error::UnknownTaskType(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Type)]
#[repr(i32)]
pub enum Status {
    #[default]
    Backlog = 1,
    InProgress = 2,
    Done = 3,
}

impl From<Status> for i32 {
    fn from(value: Status) -> Self {
        match value {
            Status::Backlog => 1,
            Status::InProgress => 2,
            Status::Done => 3,
        }
    }
}

impl TryFrom<i32> for Status {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Status::Backlog),
            2 => Ok(Status::InProgress),
            3 => Ok(Self::Done),
            _ => Err(Error::UnknownStatus(value)),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
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

    pub async fn insert(&self, pool: &PgPool) -> Result<(), Error> {
        let query = format!(
            "INSERT INTO {} (title, task_type, status, description, due_date)
                 VALUES ($1, $2, $3, $4, $5)",
            TASKS_TABLE
        );

        sqlx::query(&query)
            .bind(self.title.clone())
            .bind(self.task_type as i32)
            .bind(self.status as i32)
            .bind(self.description.as_ref())
            .bind(self.due_date)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get_by_title(self, pool: &PgPool, title: &str) -> Result<(), Error> {
        let query = format!("SELECT * FROM {TASKS_TABLE} WHERE title=$1");

        let t = sqlx::query_as::<_, Task>(&query)
            .bind(title)
            .fetch_all(pool)
            .await?;

        println!("{:?}", t);

        Ok(())
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn basic_test(pool: PgPool) -> sqlx::Result<()> {
        let task = Task::builder("test".into(), None).build();
        let r = task.insert(&pool).await;

        println!("{:?}", r);

        let t = task.get_by_title(&pool, "test").await;

        println!("{:?}", t);

        Ok(())
    }
}
