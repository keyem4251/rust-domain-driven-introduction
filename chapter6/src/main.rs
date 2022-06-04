fn main() {
    fn_6_1();
    fn_6_2();
}

fn fn_6_1() {
    // ユースケースを記述するアプリケーションサービスを作成
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

    impl<Repo> UserService<Repo>
    where
        Repo: UserRepository,
    {
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

    impl<Repo> UserApplicationService<Repo>
    where
        Repo: UserRepository,
    {
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

        fn get(&self, user_name: String) -> User {
            let target_name = UserName::new(user_name);
            let user = self.user_repository.find(target_name);
            match user {
                Some(user) => user,
                None => panic!("ユーザーが存在しません"),
            }
        }
    }

    let user_repository = UserRepositoryImpl::new();
    let application_service = UserApplicationService::new(user_repository);
    application_service.register("user name".to_string());
    let user = application_service.get("user".to_string());
    println!("{:?}", user);
}

fn fn_6_2() {
    // アプリケーションサービスからドメインオブジェクトを返さないようにDTOを作成
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

        fn change_name(&mut self, name: UserName) {
            self.name = name;
        }
    }

    struct UserService<Repo: UserRepository> {
        user_repository: Repo,
    }

    impl<Repo> UserService<Repo>
    where
        Repo: UserRepository,
    {
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

    impl<Repo> UserApplicationService<Repo>
    where
        Repo: UserRepository,
    {
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

        fn get(&self, user_name: String) -> UserData {
            let target_name = UserName::new(user_name);
            let user = self.user_repository.find(target_name);
            match user {
                Some(user) => UserData::new(&user),
                None => panic!("ユーザーが存在しません"),
            }
        }

        fn update(&self, command: UserUpdateCommand) {
            // 更新対象が名前か確認
            let name = match command.name {
                Some(name) => UserName::new(name),
                None => panic!("名前ではない"),
            };

            let found = self.user_repository.find(name.clone());
            match found {
                Some(mut user) => {
                    user.change_name(name);
                    let user_service = UserService::new(self.user_repository);
                    if user_service.exists(user.clone()) {
                        println!("ユーザーはすでに作成されています");
                    }
                    self.user_repository.save(user);
                },
                None => panic!("ユーザーが存在しません"),
            }
        }
    }

    struct UserData {
        name: String,
    }

    impl UserData {
        fn new(user: &User) -> Self {
            UserData { name: user.name.clone().value }
        }
    }

    let user_repository = UserRepositoryImpl::new();
    let application_service = UserApplicationService::new(user_repository);
    application_service.register("user name".to_string());
    let user_data = application_service.get("user".to_string());
    println!("{:?}", user_data.name);

    #[derive(Clone)]
    struct UserUpdateCommand {
        name: Option<String>,
    }

    impl UserUpdateCommand {
        fn new(name: String) -> Self {
            let target_name = if name == "test".to_string() {
                None
            } else {
                Some(name)
            };
            UserUpdateCommand { name: target_name }
        }
    }

    let command = UserUpdateCommand::new("test".to_string());
    application_service.update(command);
}
