// The value of `dylib = "..."` should be the library containing the hot-reloadable functions
// It should normally be the crate name of your sub-crate.
#[hot_lib_reloader::hot_module(dylib = "engine", lib_dir = concat!(env!("CARGO_MANIFEST_DIR"), "\\..\\", "target\\debug"))]
mod hot_lib {
    // Reads public no_mangle functions from lib.rs and  generates hot-reloadable
    // wrapper functions with the same signature inside this module.
    hot_functions_from_file!("engine/src/lib.rs");

    // Because we generate functions with the exact same signatures,
    // we need to import types used
    pub use engine::State;
}
fn main() {
    let mut state = hot_lib::State { counter: 0 };
    // Running in a loop so you can modify the code and see the effects
    loop {
        hot_lib::do_stuff(&mut state);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}