fn main() {
    fn_5_1();
}

fn fn_5_1() {
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
        // 永続化と再構築に関する振る舞いを定義するので、Update系はRepositoryには定義しない
        // 削除（Delete）はRepository
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

    struct Program<Repo: UserRepository> {
        user_repository: Repo,
    }

    impl<Repo> Program<Repo> where Repo: UserRepository {
        fn new(user_repository: Repo) -> Self {
            Program { user_repository }
        }

        fn create_user(&self, user_name: String) {
            let user = User::new(UserName::new(user_name));

            let user_service = UserService::new(self.user_repository);
            if user_service.exists(user.clone()) {
                println!("ユーザーはすでに作成されています");
            }
            self.user_repository.save(user);
        }
    }

    let user_repository = UserRepositoryImpl::new();
    let program = Program::new(user_repository);
    program.create_user("user name".to_string());

    #[derive(Clone, Copy)]
    struct InMemoryUserRepository {}

    impl UserRepository for InMemoryUserRepository {
        fn save(&self, user: User) {
            println!("inmemory save user: {:?}", user);   
        }

        fn find(&self, name: UserName) -> Option<User> {
            let rows = User::new(name); // テスト用にインメモリで検索
            if rows.name.value == "user" {
                return Some(rows);
            }
            None
        }
    }

    impl InMemoryUserRepository {
        fn new() -> Self {
            InMemoryUserRepository {}
        }
    }

    let inmemory_user_repository = InMemoryUserRepository::new();
    let test_program = Program::new(inmemory_user_repository);
    test_program.create_user("test user name".to_string());
}
