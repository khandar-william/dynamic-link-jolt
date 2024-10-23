// Demonstrate function returning string
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn arbitrary_string() -> String {
    "Hello, World!".to_string()
}

// Demonstrate function executing guest code
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn execute_guest_code() -> u128 {
    let (prove_fib, _verify_fib) = guest::build_fib();

    let (output, _proof) = prove_fib(50);
    output
}
