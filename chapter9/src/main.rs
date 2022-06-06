fn main() {
    println!("Hello, world!");
}

fn fn_9_1() {
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

    trait UserFactory {
        fn create(&self, name: UserName) -> User;
    }

    struct UserFactoryImpl {}

    impl UserFactory for UserFactoryImpl {
        fn create(&self, name: UserName) -> User {
            User { name }
        }
    }

    impl UserFactoryImpl {
        fn new() -> Self {
            UserFactoryImpl {}
        }
    }

    struct UserApplicationService<Repo: UserRepository, Factory: UserFactory> {
        user_repository: Repo,
        user_factory: Factory,
    }

    impl<Repo, Factory> UserApplicationService<Repo, Factory>
    where
        Repo: UserRepository,
        Factory: UserFactory,
    {
        fn new(user_repository: Repo, user_factory: Factory) -> Self {
            UserApplicationService { user_repository, user_factory }
        }

        fn register(&self, user_name: String) {
            let name = UserName::new(user_name);
            let user = self.user_factory.create(name);

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
                    user.change_name(name); // nameがnullかを確認してnullでない場合にchange_nameする
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
    let user_factory = UserFactoryImpl::new();
    let application_service = UserApplicationService::new(user_repository, user_factory);
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
