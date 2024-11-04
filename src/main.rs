mod config;
mod error;
mod progress;
mod command;

use std::thread::sleep;
use std::time::Duration;
use owo_colors::OwoColorize;
use crate::config::Config;
use crate::error::CustomError;
use crate::progress::Progress;

const ERROR_MESSAGE_DISPLAY_DURATION: Duration = Duration::from_secs(30);
const SUCCESS_MESSAGE_DISPLAY_DURATION: Duration = Duration::from_secs(10);
const MINIMUM_NUMBER_OF_TASKS: u8 = 0;

fn main() {
    if let Err(error) = run_script() {
        eprintln!("Error: {}", error.to_string().red());
        sleep(ERROR_MESSAGE_DISPLAY_DURATION);
    }
}

fn run_script() -> Result<(), CustomError> {
    let config = Config::new()?;
    let mut total_number_of_tasks = MINIMUM_NUMBER_OF_TASKS;
    if !config.stop_processes_containing_names.is_empty() { total_number_of_tasks += 1 };
    if !config.start_applications_at_file_paths.is_empty() { total_number_of_tasks += 1 };
    let mut progress = Progress::new(total_number_of_tasks);

    progress.update_progress("Stopping processes...");
    for process_name in config.stop_processes_containing_names.iter() {
        println!("Stopping processes matching '{process_name}'");
        match command::stop_process(process_name) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
    }

    progress.update_progress("Starting applications...");
    for application_file_path in config.start_applications_at_file_paths.iter() {
        println!("Starting application '{application_file_path}'");
        command::start_application(application_file_path)?
    }
    println!("Done! Ending script in {} seconds...", SUCCESS_MESSAGE_DISPLAY_DURATION.as_secs());
    sleep(SUCCESS_MESSAGE_DISPLAY_DURATION);
    Ok(())
}