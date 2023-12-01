#[macro_export]
macro_rules! main {
    () => {
        fn main() {
            let input = std::io::read_to_string(std::io::stdin())
                .expect("failed to read input from stdin");
            match std::env::args().nth(1).as_deref() {
                Some("1") => println!("{}", part_1(input)),
                Some("2") => println!("{}", part_2(input)),
                _ => eprintln!("Expected AoC part as argument (1 or 2)")
            }
        }
    }
}
