fn main() {
    fn_2_1();
}

fn fn_2_1() {
    struct FullName {
        first_name: String,
        last_name: String,
    }

    impl FullName {
        fn new(first_name: String, last_name: String) -> Self {
            FullName {
                first_name,
                last_name,
            }
        }
    }

    let full_name = FullName::new("first".to_string(), "last".to_string());
    println!("{} {}", full_name.first_name, full_name.last_name);
}
