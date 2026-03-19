use std::path::Path;

use cairo_vm::types::program::Program;

/// Loads a compiled Cairo program from a `.json` file located next to its `.cairo` source.
///
/// `relative_path` is relative to the `tests_cairo/` directory.
/// Example: `load_cairo_program("math/main_math_test.json")`
///
/// # Panics
/// - If the `.json` file does not exist: run `make tests_cairo_programs` first.
/// - If the `.json` file cannot be parsed as a Cairo `Program`.
pub fn load_cairo_program(relative_path: &str) -> Program {
    // CARGO_MANIFEST_DIR is the `vm/` crate directory.
    // `tests_cairo/` lives one level up (at the workspace root).
    let tests_cairo_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("vm crate should have a parent directory")
        .join("tests_cairo");

    let json_path = tests_cairo_dir.join(relative_path);

    let bytes = std::fs::read(&json_path).unwrap_or_else(|err| {
        panic!(
            "Cairo program not found at {json_path:?}: {err}\n\
             Did you run `make tests_cairo_programs`?"
        )
    });

    Program::from_bytes(&bytes, None)
        .unwrap_or_else(|e| panic!("Failed to parse Cairo program at {json_path:?}: {e}"))
}

/// Asserts that a `MaybeRelocatable` reference equals a value convertible into `MaybeRelocatable`.
#[macro_export]
macro_rules! assert_mr_eq {
    ($left:expr, $right:expr) => {{
        let right_mr = ($right)
            .try_into()
            .unwrap_or_else(|e| panic!("conversion to MaybeRelocatable failed: {e:?}"));
        assert_eq!($left, &right_mr);
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        let right_mr = ($right)
            .try_into()
            .unwrap_or_else(|e| panic!("conversion to MaybeRelocatable failed: {e:?}"));
        assert_eq!($left, &right_mr, $($arg)+);
    }};
}
