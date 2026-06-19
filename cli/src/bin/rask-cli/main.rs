mod args;

use anyhow::{Context, Result};
use args::*;
use clap::Parser;
use rask::project::*;
use rask::task::*;
use rask::user::*;
use rask::Rask;

fn print_json<T: serde::Serialize>(data: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(data)?);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    Rask::init(args.url, args.api_key);

    match args.target {
        Target::Task(action) => match action {
            TaskAction::Create(args) => {
                let new_task = TaskRequest::new(
                    args.title,
                    args.state,
                    args.assigner_name,
                    args.project_name,
                    args.due_at,
                    args.description,
                )
                .context("Failed to create new task")?;
                Task::save(new_task).context("Failed to save new task")?;
                println!("Success to add new task");
            }
            TaskAction::List(args) => {
                let tasks = Task::list().context("Failed to get Task list")?;
                if args.json {
                    print_json(&tasks)?;
                } else {
                    for task in &tasks {
                        println!("{:?}", task);
                    }
                }
            }
        },
        Target::User(action) => match action {
            UserAction::List(args) => {
                let users = User::list().context("Failed to get User list")?;
                if args.json {
                    print_json(&users)?;
                } else {
                    for user in users {
                        println!("{:?}", user);
                    }
                }
            }
        },
        Target::Project(action) => match action {
            ProjectAction::List(args) => {
                let projects = Project::list().context("Failed to get Project list")?;
                if args.json {
                    print_json(&projects)?;
                } else {
                    for project in projects {
                        println!("{:?}", project);
                    }
                }
            }
        },
    }

    Ok(())
}
