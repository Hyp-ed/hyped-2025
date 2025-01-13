use std::path::Path;

pub mod orchestrater;

fn main() {
    println!(
        "{:?}",
        orchestrater::flash_proj_with_flags(Path::new("../../boards/stm32h743zi"), &String::from("stm32h743zi"), &[])
    );
}
