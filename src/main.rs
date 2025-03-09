mod models;
mod commands;
mod storage;

use clap::{Parser,Subcommand};
use commands::{add_task,list_tasks,complete_task,delete_task};
use storage::{load_tasks,save_tasks};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple Todo List CLI Tool" , long_about = None)]
struct Cli{
    #[command(subcommand)]
    command : Commands,
}
 
#[derive(Subcommand)] 
enum Commands{
   Add{description : String},

   List,

   Complete {index : usize},

   Delete {index : usize},
}

fn main()->std::io::Result<()>{

    let cli = Cli::parse();
    let file_path = "tasks.json";
    let mut tasks = load_tasks(file_path)?;

    match cli.command{
        Commands::Add { description } => {
            add_task(&mut tasks, description);
        }
        Commands::List => {
            list_tasks(&tasks);
        }
        Commands::Complete { index } => {
            complete_task(&mut tasks, index-1);
        }
        Commands::Delete { index } => {
            delete_task(&mut tasks, index-1);
        }

    }

    save_tasks(&tasks, file_path)?;
    Ok(())
}
