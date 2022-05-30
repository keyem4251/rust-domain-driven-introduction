fn main() {
    fn_4_1();
    fn_4_2();
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

fn fn_4_2() {
    #[derive(Debug)]
    struct Baggage {
        value: String,
    }

    impl Baggage {
        fn new(value: String) -> Self {
            Baggage { value }
        }
    }

    #[derive(Debug)]
    struct PhysicalDistributionBase {
        name: String,
    }

    impl PhysicalDistributionBase {
        fn new(name: String) -> Self {
            PhysicalDistributionBase { name }
        }

        fn ship(&self, baggage: Baggage) -> Baggage {
            println!("{:?} {:?}", self.name, baggage.value);
            baggage
        }

        fn receive(&self, baggage: Baggage) {
            println!("{:?} {:?}", self.name, baggage.value);
        } 
    }

    struct TransportService {}

    impl TransportService {
        fn new() -> Self {
            TransportService {}
        }

        fn transport(&self, from: &PhysicalDistributionBase, to: &PhysicalDistributionBase, baggage: Baggage) {
            let shipped_baggage = from.ship(baggage);
            to.receive(shipped_baggage);
        }
    }

    let baggage1 = Baggage::new("baggage1".to_string());
    
    let from = PhysicalDistributionBase::new("base1".to_string());
    let to = PhysicalDistributionBase::new("base2".to_string());

    let transport_service = TransportService::new();
    transport_service.transport(&from, &to, baggage1);
}
