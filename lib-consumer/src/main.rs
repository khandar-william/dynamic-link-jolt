const LIBRARY_PATH: &str = "../lib-producer/target/release/liblib_producer.so";

fn main() {
    println!("Entered lib-consumer");

    unsafe {
        // Load dynamic library
        let lib = libloading::Library::new(LIBRARY_PATH).unwrap(); // Error happened here, before executing any function inside

        // Execute arbitrary_string()
        let arbitrary_string_fn: libloading::Symbol<unsafe extern "C" fn() -> String> = lib
            .get(b"arbitrary_string")
            .unwrap();
        let arbitrary_string = arbitrary_string_fn();
        println!("arbitrary_string: {:?}", arbitrary_string);

        // Execute execute_guest_code()
        let execute_guest_fn: libloading::Symbol<unsafe extern "C" fn() -> u128> = lib
            .get(b"execute_guest_code")
            .unwrap();
        let guest_result = execute_guest_fn();
        println!("guest_result: {:?}", guest_result);
    }
}
