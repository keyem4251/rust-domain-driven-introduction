fn main() {
    println!("Hello, world!");
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

    struct UserService {
        user_repository: Box<dyn UserRepository>,
    }

    impl UserService {
        fn new(user_repository: Box<dyn UserRepository>) -> Self {
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

    trait UserRepository {
        fn save(&self, user: User);
        fn find(&self, name: UserName) -> Option<User>;
    }
}
