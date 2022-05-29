fn main() {
    fn_3_1();
    fn_3_2();
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

fn fn_3_2() {
    #[derive(PartialEq, Debug)]
    struct UserId {
        value: String,
    }

    impl UserId {
        fn new(value: String) -> Self {
            UserId { value }
        }
    }

    #[derive(Debug)]
    struct User {
        name: String,
        id: UserId,
    }

    impl User {
        fn new(id: UserId, name: String) -> Self {
            User { id, name }
        }

        fn change_name(&mut self, name: String) {
            if name.len() < 3 {
                panic!("ユーザー名は3文字以上です");
            }
            self.name = name;
        }
    }

    impl PartialEq for User {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    fn check(lhs: &User, rhs: &User) {
        if  lhs == rhs {
            println!("同一のユーザです")
        } else {
            println!("別のユーザです")
        }
    }

    let user_id = UserId::new("011".to_string());
    let mut user = User::new(user_id, "test".to_string());
    user.change_name("test2".to_string());

    let user_id_2 = UserId::new("012".to_string());
    let user_2 = User::new(user_id_2, "test".to_string());

    check(&user, &user_2);

    let user_id_3 = UserId::new("011".to_string());
    let user_3 = User::new(user_id_3, "test".to_string());
    check(&user, &user_3);
}
