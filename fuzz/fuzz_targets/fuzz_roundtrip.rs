use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                if let Ok(input) = lexpr::from_str(s) {
                    let string = lexpr::to_string(&input).expect("printing failed");
                    let output = lexpr::from_str(&string).expect("parsing failed");
                    assert_eq!(input, output);
                }
            }
        });
    }
}