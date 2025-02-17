#[cfg(test)]
pub use log::{debug, info, trace, warn};

#[cfg(not(test))]
pub use defmt::{debug, info, trace, warn};
