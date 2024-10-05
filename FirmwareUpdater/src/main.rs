use std::process::Command;
use std::time::SystemTime;
use std::{
    env, fs,
    io::{self, Write},
    path::{self, PathBuf},
    process::exit,
};

fn inf<'a>(s: &'a str) -> String {
    format!("\x1b[34;49m[I] {s} \x1b[0m")
}
fn que<'a>(s: &'a str) -> String {
    format!("\x1b[35;49m[Q] {s} \x1b[0m")
}
fn err<'a>(s: &'a str) -> String {
    format!("\x1b[31;49m[E] {s} \x1b[0m")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let (cargo_pth, trg_toml, trg_conf) = get_paths(&args);
    println!(
        // evil print statment that works :)
        "{}",
        inf(&format!(
            "Got location of target toml as {} {}",
            trg_toml.to_str().unwrap(),
            format!(
                "{}",
                if let Some(s) = trg_conf {
                    format!("and descovered config file in {}", s.to_str().unwrap())
                } else {
                    Default::default()
                }
            )
        ))
    );

    let tmp_path = create_build_path(&cargo_pth);
    if let Err(_) = build_toml(&tmp_path) {
        exit(5);
    }

    
}

fn get_input_line(s: String) -> String {
    print!("{}", s);
    let _ = io::stdout().flush();
    let mut buf: String = Default::default();
    let _ = io::stdin().read_line(&mut buf);
    while buf.ends_with("\n") || buf.ends_with("\r") {
        buf.pop();
    }
    buf
}

#[inline] // I know this only affects external packages, however it makes it clear it doenst need to be seprate flow of logic
fn get_paths(args: &Vec<String>) -> (PathBuf, PathBuf, Option<PathBuf>) {
    let cargo_pth = path::absolute(if args.len() > 1 {
        // first arg is bin path
        args[1].to_owned()
    } else {
        get_input_line(que(
            "Please enter the path to the folder that contains the target Cargo.toml: ",
        ))
        .to_owned()
    });

    if let Err(e) = cargo_pth {
        println!("{}", err(&format!("Unable to resolve path {e}")));
        exit(1)
    };
    let cargo_pth = cargo_pth.unwrap();

    let trg_toml = cargo_pth.join("Cargo.toml");

    let car_p_meta = fs::metadata(&trg_toml); // if this is valid then cargo_pth must be
    if car_p_meta.is_err() {
        println!(
            "{}",
            err(&format!(
                "target Cargo.toml file or its parent dir doesn't exist at {}",
                trg_toml.to_str().unwrap()
            ))
        );
        exit(2)
    };
    let car_p_meta = car_p_meta.unwrap();

    if !car_p_meta.is_file() {
        println!("{}", err("Target file or dir is invalid format"));
        exit(2)
    }

    let trg_conf: Option<PathBuf> =
        if fs::exists(cargo_pth.join(".cargo\\config.toml")).unwrap_or(false) {
            Some(cargo_pth.join(".cargo\\config.toml"))
        } else {
            None
        };

    println!(
        "{}",
        inf("Moving current working directory to be provided cargo path")
    );

    return (cargo_pth, trg_toml, trg_conf);
}

#[inline]
fn create_build_path(cargo_pth: &PathBuf) -> PathBuf {
    let tmp_dir = cargo_pth.join(format!(
        ".tmp_build_{}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap() //unwarp as you shouldn't be flashing boards as a time traveler
            .as_secs()
            .to_string()
    ));
    if fs::exists(&tmp_dir).unwrap_or(true) {
        println!(
            "{}",
            inf("Unable to create tmp dirm, please remove old build dir's and try again")
        );
        exit(3)
    }

    if let Err(e) = fs::create_dir(&tmp_dir) {
        println!("{}", err(&format!("Unable to create tmp dir due to {}", e)));
        exit(3)
    }

    println!(
        "{}",
        inf(&format!("Created tmp dir {}", tmp_dir.to_str().unwrap()))
    );

    tmp_dir
}

#[inline]
fn build_toml(build_path: &PathBuf) -> Result<(), io::Error> {
    // should have already changed CWD to be the target's dir
    print!("{}", inf("Running Cargo Build..."));

    let output = Command::new("cargo")
        .arg("build")
        .arg("--target-dir")
        .arg(build_path.to_str().unwrap())
        .output();
    if let Err(e) = output {
        println!("ERR\n{}", err(&format!("Cargo build failed due to {}", e)));
        return Err(e);
    };
    println!("SUCCESS\n");
    Ok(())
}
