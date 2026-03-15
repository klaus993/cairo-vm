//! Tests for `validate_subsequence.cairo`.

use std::sync::LazyLock;

use crate::error_utils::{expect_hint_assert_not_zero, expect_ok, VmCheck};
use cairo_vm::cairo_args;
use cairo_vm::math_utils::safe_div_usize;
use cairo_vm::types::program::Program;
use cairo_vm::vm::runners::cairo_function_runner::CairoFunctionRunner;
use rstest::{fixture, rstest};

// ===================== Shared constants (LazyLock) =====================

/// The compiled Cairo validate_subsequence program, loaded once and shared across all tests.
static PROGRAM: LazyLock<Program> = LazyLock::new(|| {
    let bytes = include_bytes!("validate_subsequence_compiled.json");
    Program::from_bytes(bytes, None).expect("Failed to load validate_subsequence_compiled.json")
});

// ===================== Fixture =====================

/// Creates a fresh `CairoFunctionRunner` from the shared `PROGRAM`.
#[fixture]
fn runner() -> CairoFunctionRunner {
    CairoFunctionRunner::new(&PROGRAM).unwrap()
}

// ===================== test_validate_subsequence =====================

#[rstest]
// Case: [3, 7, 9, 4] (sub_elm_size=1) is a valid subsequence of
//       [3, 0, 1, 0, 5, 0, 7, 0, 9, 0, 2, 0, 4, 0] (elm_size=2).
// Matching is done by the first field of each elm_size=2 chunk.
// Expected: Success.
#[case::valid_subsequence(
    1,
    vec![3u64, 7, 9, 4],
    2,
    vec![3u64, 0, 1, 0, 5, 0, 7, 0, 9, 0, 2, 0, 4, 0],
    expect_ok,
)]
// Case: [3, 9, 7, 4] (sub_elm_size=1) is NOT a valid subsequence of the same array
//       because 9 appears after 7 in the array, so the order is wrong.
// Expected: AssertNotZero error.
#[case::invalid_subsequence(
    1,
    vec![3u64, 9, 7, 4],
    2,
    vec![3u64, 0, 1, 0, 5, 0, 7, 0, 9, 0, 2, 0, 4, 0],
    expect_hint_assert_not_zero,
)]
fn test_validate_subsequence(
    mut runner: CairoFunctionRunner,
    #[case] sub_elm_size: usize,
    #[case] sub_array: Vec<u64>,
    #[case] elm_size: usize,
    #[case] array: Vec<u64>,
    #[case] check: VmCheck<()>,
) {
    let n_sub_elms = safe_div_usize(sub_array.len(), sub_elm_size)
        .expect("sub_array length must be divisible by sub_elm_size");
    let n_elms =
        safe_div_usize(array.len(), elm_size).expect("array length must be divisible by elm_size");

    let args = cairo_args!(sub_array, n_sub_elms, array, n_elms, sub_elm_size, elm_size);

    let res = runner.run_default_cairo0("validate_subsequence", &args);
    check(&res);
}
