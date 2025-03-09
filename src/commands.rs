use crate::models::Task;

pub fn add_task(tasks : &mut Vec<Task> , description : String){
    tasks.push(Task{
        description,
        completed : false,
    });
    println!("Task added!");
}

pub fn list_tasks(tasks : &Vec<Task>){
    if tasks.is_empty(){
        println!("No task found");
    }
    else{
        for (i,task) in tasks.iter().enumerate(){
            println!("{}. [{}] {}" , i+1, if task.completed {"X"} else {" "} , task.description);
        }
    }
}

pub fn complete_task(tasks : &mut Vec<Task> , index : usize){
    if let Some(task) = tasks.get_mut(index){
        task.completed = true;
        println!("The task has been completed");
    }
    else{
        println!("Invalid task index");
    }
}

pub fn delete_task(tasks : &mut Vec<Task> , index : usize ){
    if index < tasks.len(){
        tasks.remove(index);
        println!("Task deleted");
    }
    else{
        println!("Invalid task index");
    }
}