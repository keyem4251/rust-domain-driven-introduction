fn main() {
    fn_5_1();
}

fn fn_5_1() {
    struct UserName {
        value: String,
    }

    impl UserName {
        fn new(value: String) -> Self {
            UserName { value }
        }
    }

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
        }
    }
}
