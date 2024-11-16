use std::{env, path::{Path, PathBuf}, process::Command};

pub type OrchestrationResult<T> = Result<T, OrchestrationError>;
pub type OrchestrationError = Box<dyn std::error::Error>;

pub fn flash_with_flags(project: &Path, flags: &[&str]) -> OrchestrationResult<()> {
    println!("[i] Getting CWD");
    let cwd = env::current_dir().map_err(|e| {
        OrchestrationError::from(format!("Failed to get current working directory: {}", e))
    })?;

    println!("[i] Moving current working directory to manfiest");
    change_dir(project)?;
    let res = build_in_wd(flags);

    println!("[i] Moving current working directory back to original");
    change_dir(&cwd).expect("Unable to recover to old CWD");    
    Ok(())
}

fn change_dir(pth: &Path) -> OrchestrationResult<()> {
    env::set_current_dir(pth)
        .map_err(|e| OrchestrationError::from(format!("Failed to change directory: {}", e)))?;
    Ok(())
}

fn add_board_as_build_target() -> OrchestrationResult<()> {
    let mut rustup = Command::new("rustup");
    let output = rustup.arg("target").arg("add").arg("thumbv7em-none-eabihf");

    let output = output.output().map_err(|e| {
        OrchestrationError::from(format!(
            "Failed to execute add board as targed because target add: {}",
            e
        ))
    })?;

    if !output.status.success() {
        return Err(OrchestrationError::from(format!(
            "Failed to add board as target: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(())
}

fn build_in_wd(flags: &[&str]) -> OrchestrationResult<PathBuf> {
    println!("[i] Adding board as build target");

    add_board_as_build_target()?;
    println!("[+] Board is now a build target");

    println!("[i] Building Board");
    let log = build_board(flags)?;

    let exec = extract_bin_path_from_logs(&log)?;
    println!("[+] Found executable");

    println!("[+] Board built successfully");

    Ok(PathBuf::from(exec))
}

fn build_board(flags: &[&str]) -> OrchestrationResult<String> { 
    // Do not atempt to use the Cargo create to build the board as it will result in
    // nothing but pain, corte-x has issues with running the cargo build command 
    // because it failes to find the correct target board platform 
    // Hence we need to change ot CWD into the correct location and build from there
    // This forces our hand to use --message-format=json to get the output of the build
    // and then use the archain chants that are in  
    let mut cargo = Command::new("cargo");
    let mut command = cargo
        .arg("build")
        .arg("--release")
        .arg("--message-format=json"); // format with json so we can find the output bin

    if !flags.is_empty() {
        command = command.arg("--features").args(flags);
    }
    // throw error if cargo failed to build
    let output = command
        .output()
        .map_err(|e| OrchestrationError::from(format!("Failed to execute cargo build: {}", e)))?;

    if !output.status.success() {
        return Err(OrchestrationError::from(format!(
            "Failed to build: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    let output = String::from_utf8_lossy(&[output.stdout, output.stderr].concat()).to_string();

    Ok(output)
}

fn extract_bin_path_from_logs(log: &String) -> OrchestrationResult<PathBuf> {
    // look for `"executable":"` because everything that is built should generate an executable tag in the outputted json
    // However all the libaries will generate `"executable":null` hence the only valid should be `"executable":"` for the
    // binary we need to flash to the board
    const EXE_PREFIX: &str = "\"executable\":\"";
    if !log.contains(EXE_PREFIX) {
        return Err(OrchestrationError::from(format!(
            "Failed to find executable in build log: {}",
            log
        )));
    }

    let exec_start = log
        .find(EXE_PREFIX)
        .ok_or_else(|| OrchestrationError::from("Failed to find executable in build log"))?
        + EXE_PREFIX.len();
    let exec_end = log[exec_start..]
        .find('"')
        .ok_or_else(|| OrchestrationError::from("Failed to find end of executable in build log"))?;
    Ok(PathBuf::from(&log[exec_start..exec_start + exec_end]))
}