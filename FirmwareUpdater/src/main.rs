use std::io::Read;
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
                if let Some(ref s) = trg_conf {
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

    let (bin_path, bin_name) = find_target_binary(tmp_path, &trg_toml, &trg_conf); // this will fail if looking for a bin that has file extention
    println!("{}", inf(&format!("Found target bin {} at {}", bin_name, bin_path.to_str().unwrap())));
    
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
    if env::set_current_dir(&cargo_pth).is_err() {
        println!("{}", err("Unable to change working dir to the requestsed directory"));
        exit(4);
    }

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
    let _ = io::stdout().flush();
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

fn find_target_binary(mut build_path: PathBuf, trg_toml: &PathBuf, trg_conf: &Option<PathBuf>) -> (PathBuf, String) { // path and binary name
    let bin_name = parse_toml(trg_toml, "package", "name");
    if let Err(e) = bin_name {
        println!("{} - {} - {}", err("Unable to parse target toml due to the following error"), trg_toml.to_str().unwrap(), e);
        exit(6)
    }
    let bin_name = bin_name.unwrap();
    

    if let Some(conf) = trg_conf {
        let sub_path = parse_toml(conf, "build", "target");
        if let Err(e) = sub_path {
            println!("{} - {} - {}", err("Unable to parse target toml due to the following error"), conf.to_str().unwrap(), e);
            exit(6)
        }
        let sub_path = sub_path.unwrap();
        build_path = build_path.join(sub_path);
    };

    build_path = build_path.join("debug").join(&bin_name);
    if build_path.is_file() {
            return (build_path, bin_name)
        }   
        else {
            println!("{} - {}", err("Error couldnt find build at"), build_path.to_str().unwrap());
            exit(7);
        }
}




fn parse_toml<'a>(targ_file: &PathBuf, target_table: &'a str, target_key: &'a str) -> Result<String, Box<dyn std::error::Error>> {
    let mut toml = fs::File::open(targ_file)?;
    let mut buff = Vec::new();
    toml.read_to_end(&mut buff)?;
    let file = String::from_utf8(buff)?;
    let file = file.lines();
    
    let mut curr_table = String::new(); 
    for line in file {
        let line = line.trim(); // Trim leading and trailing whitespace
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            curr_table = line[1..line.len() - 1].to_owned();
            dbg!(&curr_table);
            continue;
        }
        if curr_table == target_table {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim().trim_matches('"');
                if key == target_key {
                    return Ok(value.to_owned());
                }
            }
        }
    }
    Err(format!("Couldn't find key {} in table {}", target_key, target_table).into())
}

fn load_bin(bin_path: &PathBuf) -> Vec<u8> {
    let f = fs::File::open(bin_path);
    if let Err(e) = f {
        println!("{}", err("Unable to open target binary"));
        exit(8)
    }
    let f = f.unwrap();



    todo!();
}