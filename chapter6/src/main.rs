fn main() {
    println!("Hello, world!");
}

fn fn_6_1() {
    #[derive(Clone, Debug)]
    struct UserName {
        value: String,
    }

    impl UserName {
        fn new(value: String) -> Self {
            UserName { value }
        }
    }

    #[derive(Clone, Debug)]
    struct User {
        name: UserName,
    }

    impl User {
        fn new(name: UserName) -> Self {
            User { name }
        }
    }

    struct UserService<Repo: UserRepository> {
        user_repository: Repo,
    }

    impl<Repo> UserService<Repo> where Repo: UserRepository {
        fn new(user_repository: Repo) -> Self {
            UserService { user_repository }
        }

        fn exists(&self, user: User) -> bool {
            let found = self.user_repository.find(user.name);
            match found {
                None => false,
                Some(_user) => true,
            }
        }
    }

    trait UserRepository: Copy {
        fn save(&self, user: User);
        fn find(&self, name: UserName) -> Option<User>;
    }

    #[derive(Clone, Copy)]
    struct UserRepositoryImpl {}

    impl UserRepository for UserRepositoryImpl {
        fn save(&self, user: User) {
            println!("save user: {:?}", user);
        }

        fn find(&self, name: UserName) -> Option<User> {
            let rows = User::new(name); // 本来はデータベースで検索
            if rows.name.value == "user" {
                return Some(rows);
            }
            None
        }
    }

    impl UserRepositoryImpl {
        fn new() -> Self {
            UserRepositoryImpl {}
        }
    }

    struct UserApplicationService<Repo: UserRepository> {
        user_repository: Repo,
    }

    impl<Repo> UserApplicationService<Repo> where Repo: UserRepository {
        fn new(user_repository: Repo) -> Self {
            UserApplicationService { user_repository }
        }

        fn register(&self, user_name: String) {
            let user = User::new(UserName::new(user_name));

            let user_service = UserService::new(self.user_repository);
            if user_service.exists(user.clone()) {
                println!("ユーザーはすでに作成されています");
            }
            self.user_repository.save(user);
        }
    }

    let user_repository = UserRepositoryImpl::new();
    let aplication_service = UserApplicationService::new(user_repository);
    aplication_service.register("user name".to_string());
}
