fn main() {
    fn_3_1();
}

fn fn_3_1() {
    #[derive(Debug)]
    struct User {
        name: String,
    }

    impl User {
        fn new(name: String) -> Self {
            User { name }
        }

        fn change_name(&mut self, name: String) {
            if name.len() < 3 {
                panic!("ユーザー名は3文字以上です");
            }
            self.name = name;
        }
    }

    let mut user = User::new("test".to_string());
    user.change_name("test2".to_string());
    println!("{:?}", user);
}
