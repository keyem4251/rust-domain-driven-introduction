use regex::Regex;

fn main() {
    fn_2_1();
    fn_2_2();
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

    // PartialEqで値が等価な場合にtrueを返し、インスタンスではなく値として扱う
    let full_name2 = FullName::new("first".to_string(), "last".to_string());
    println!("{}", full_name == full_name2);
}

fn fn_2_2() {
    #[derive(PartialEq, Debug)]
    struct Name {
        value: String,
    }

    impl Name {
        fn new(value: String) -> Self {
            let regex = Regex::new("[a-zA-Z]").unwrap();
            if regex.is_match(&value) {
                return Name { value };                
            }
            panic!("許可されていないモイjが使われています");
        }
    }

    #[derive(PartialEq, Debug)]
    struct FullName {
        first_name: Name,
        last_name: Name,
    }

    impl FullName {
        fn new(first_name: Name, last_name: Name) -> Self {
            FullName { first_name, last_name }
        }
    }

    let first_name = Name::new("first".to_string());
    let last_name = Name::new("last".to_string());
    let full_name = FullName::new(first_name, last_name);
    println!("{:?} {:?}", full_name.first_name, full_name.last_name);
}