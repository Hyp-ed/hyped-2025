use std::path::Path;

pub mod orchestrater;

fn main() {
    println!(
        "{:?}",
        orchestrater::flash_with_flags(Path::new("../../boards/stm32h743zi"), &[])
    );
}
