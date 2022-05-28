use regex::Regex;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::fmt;
use std::ops::Add;

fn main() {
    fn_2_1();
    fn_2_2();
    fn_2_3();
    fn_2_4();
    fn_2_5();
}

fn fn_2_1() {
    #[derive(PartialEq)]
    struct FullName {
        first_name: String,
        last_name: String,
    }

    impl FullName {
        fn new(first_name: String, last_name: String) -> Self {
            FullName {
                first_name,
                last_name,
            }
        }
    }

    // 値オブジェクトを作成
    let full_name = FullName::new("first".to_string(), "last".to_string());
    println!("{} {}", full_name.first_name, full_name.last_name);

    // PartialEqで値が等価な場合にtrueを返し、インスタンスではなく値として扱う
    let full_name2 = FullName::new("first".to_string(), "last".to_string());
    println!("{}", full_name == full_name2);
}

fn fn_2_2() {
    #[derive(PartialEq, Debug)]
    struct Name {
        value: String,
    }

    impl Name {
        fn new(value: String) -> Self {
            let regex = Regex::new("[a-zA-Z]").unwrap();
            if regex.is_match(&value) {
                return Name { value };
            }
            panic!("許可されていないモイjが使われています");
        }
    }

    #[derive(PartialEq, Debug)]
    struct FullName {
        first_name: Name,
        last_name: Name,
    }

    impl FullName {
        fn new(first_name: Name, last_name: Name) -> Self {
            FullName {
                first_name,
                last_name,
            }
        }
    }

    let first_name = Name::new("first".to_string());
    let last_name = Name::new("last".to_string());
    let full_name = FullName::new(first_name, last_name);
    println!("{:?} {:?}", full_name.first_name, full_name.last_name);
}

fn fn_2_3() {
    // ふるまいをもった値オブジェクト
    #[derive(PartialEq, Debug)]
    struct Money {
        amount: Decimal,
        currency: String,
    }

    impl Money {
        fn new(amount: Decimal, currency: String) -> Self {
            Money { amount, currency }
        }

        fn add(&self, money: Money) -> Money {
            if self.currency != money.currency {
                panic!(
                    "通貨単位が異なります this: {}, arg: {}",
                    self.currency, money.currency
                );
            }
            Money::new(money.amount, money.currency)
        }
    }

    let my_money = Money::new(dec!(1000), "JPY".to_string());
    let allowance = Money::new(dec!(3000), "JPY".to_string());
    let result = my_money.add(allowance);
    println!("{:?}", result);
}

fn fn_2_4() {
    // ふるまいをもった値オブジェクト（Addトレイト）
    #[derive(PartialEq, Debug)]
    struct Money {
        amount: Decimal,
        currency: String,
    }

    impl Money {
        fn new(amount: Decimal, currency: String) -> Self {
            Money { amount, currency }
        }
    }

    impl Add for Money {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            if self.currency != rhs.currency {
                panic!(
                    "通貨単位が異なります lhs: {}, rhs: {}",
                    self.currency, rhs.currency
                );
            }
            Self {
                amount: self.amount + rhs.amount,
                currency: self.currency,
            }
        }
    }

    let my_money = Money::new(dec!(1000), "JPY".to_string());
    let allowance = Money::new(dec!(3000), "JPY".to_string());
    let result = my_money + allowance;
    println!("{:?}", result);
}

fn fn_2_5() {
    struct ModelNumber {
        product_code: String,
        branch: String,
        lot: String,
    }

    impl ModelNumber {
        fn new(product_code: String, branch: String, lot: String) -> Self {
            ModelNumber {
                product_code,
                branch,
                lot,
            }
        }
    }

    impl fmt::Display for ModelNumber {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}-{}-{}", self.product_code, self.branch, self.lot)
        }
    }

    let model_number = ModelNumber::new("111".to_string(), "019".to_string(), "193".to_string());
    println!("{}", model_number);

    #[derive(Debug)]
    struct UserName {
        value: String,
    }

    impl UserName {
        fn new(value: String) -> Self {
            if value.len() < 3 {
                panic!("ユーザー名は3文字以上です");
            }
            UserName { value }
        }
    }

    let user_name = UserName::new("test".to_string());
    println!("{:?}", user_name.value);
}
