use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use probe_rs::{flashing::{self, BinOptions}, Permissions, Session};

pub type OrchestrationResult<T> = Result<T, OrchestrationError>;
pub type OrchestrationError = Box<dyn std::error::Error>;

pub fn flash_proj_with_flags(project: &Path, board_str: &String, flags: &[&str]) -> OrchestrationResult<()> {
    let exec = build_board_binary(project, flags)?;
    println!("[i] Flashing board");
    flash_board_with_bianry(&exec, board_str)?;
    println!("[+] Board flashed successfully");
    Ok(())
}

pub fn build_board_binary(project: &Path, flags: &[&str]) -> OrchestrationResult<PathBuf> {
    println!("[i] Getting CWD");
    let cwd = env::current_dir().map_err(|e| {
        OrchestrationError::from(format!("Failed to get current working directory: {}", e))
    })?;

    println!("[i] Moving current working directory to manfiest");
    change_dir(project)?;
    let res = build_in_wd(flags);

    println!("[i] Moving current working directory back to original");
    change_dir(&cwd).expect("Unable to recover to old CWD");
    return res;
}

pub fn flash_board_with_bianry(bin_pth: &Path, board_str: &String) -> OrchestrationResult<()>{
    let mut board = Session::auto_attach(board_str, Permissions::default())?;
    flashing::download_file(&mut board, bin_pth, flashing::Format::Bin(BinOptions {base_address: None, skip: 0x00 }))?;
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
    println!("[i] Building Board, this may take a while please wait ..");

    let log = build_board(flags)?;
    println!("[+] Board built");
    println!("[i] Extracting executable from build logs");

    let exec = extract_bin_path_from_logs(&log)?;
    println!("[+] Found executable");
    println!("[+] Board built successfully");

    Ok(PathBuf::from(exec))
}

fn build_board(flags: &[&str]) -> OrchestrationResult<String> {
    // Do not atempt to use the Cargo create to build the board as it will result in
    // nothing but pain, corte-x has issues with running the cargo build command
    // because it fails to find the correct target board platform
    // Hence we need to change the CWD into the correct location and build from there
    // This forces our hand in using --message-format=json to get the output of the build
    // as we need explicit debug logs to find the executable that was built
    let mut cargo = Command::new("cargo");
    let mut command = cargo
        .arg("build")
        .arg("--release")
        // format with json so we can find the output bin 
        // as this enables "debug" information
        .arg("--message-format=json"); 

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
