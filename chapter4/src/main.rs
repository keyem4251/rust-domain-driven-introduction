fn main() {
    fn_4_1();
}

fn fn_4_1() {
    #[derive(PartialEq)]
    struct UserId {
        value: String,
    }

    impl UserId {
        fn new(value: String) -> Self {
            UserId { value }
        }
    }

    #[derive(PartialEq, Debug)]
    struct UserName {
        value: String,
    }

    impl UserName {
        fn new(value: String) -> Self {
            UserName { value }
        }
    }

    struct User {
        id: UserId,
        name: UserName,
    }

    impl User {
        fn new(id: UserId, name: UserName) -> Self {
            User { id, name }
        }
    }

    struct UserService {}

    impl UserService {
        fn new() -> Self {
            UserService {}
        }

        fn exists(&self, user: &User) -> bool {
            // 重複をデータベース/ファイル/メモリに確認する
            user.id == UserId::new("001".to_string())
        }
    }

    let user_id = UserId::new("001".to_string());
    let user_name = UserName::new("test".to_string());
    let user = User::new(user_id, user_name);
    let user_service = UserService::new();
    println!("{:?}", user.name);
    println!("{}", user_service.exists(&user));
}
