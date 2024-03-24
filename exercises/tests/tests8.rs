fn main() {
    // Generate a timestamp and print it as an environment variable for the test
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("cargo:TEST_FOO={}", timestamp);

    // Enable the "pass" feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
