use std::process::{Command, Stdio};
use crate::error::CustomError;

pub(crate) fn stop_process(process_name: &str) -> Result<(), CustomError> {
    let regex = &format!("*{process_name}*");
    let result = Command::new("powershell")
        .arg("-Command")
        .args(&["Stop-Process", "-Force", "-Name", regex])
        .stdout(Stdio::null())
        .status();
    match result {
        Ok(exit_status) if exit_status.success() => Ok(()),
        _ => Err(CustomError::StoppingProcessFailedError(format!("Failed to stop all processes containing the name '{process_name}'")))
    }
}
pub(crate) fn start_application(application_file_path: &str) -> Result<(), CustomError> {
    let result = Command::new("powershell")
        .arg("-Command")
        .arg(format!("Start-Process \"{application_file_path}\""))
        .spawn();
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(CustomError::StartingApplicationFailedError(
            format!("Failed to start application '{application_file_path}'"))
        )
    }
}