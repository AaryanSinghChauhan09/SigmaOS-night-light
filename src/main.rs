//! SigmaOS-night-light — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_night_light_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-night-light: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-night-light: ok");
}
