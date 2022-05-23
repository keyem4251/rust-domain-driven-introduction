fn main() {
    fn_2_1();
}

fn fn_2_1() {
    #[derive(PartialEq)]
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

    // 値オブジェクトを作成
    let full_name = FullName::new("first".to_string(), "last".to_string());
    println!("{} {}", full_name.first_name, full_name.last_name);

    // PartialEqで値が等価な場合にtrueを返す
    let full_name2 = FullName::new("first".to_string(), "last".to_string());
    println!("{}", full_name == full_name2);
}
