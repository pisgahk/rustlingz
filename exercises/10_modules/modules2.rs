// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

#[allow(dead_code)]
mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    //use self::fruits::PEAR;
    //use self::veggies::CUCUMBER;

    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}


fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruits::PEAR,
        delicious_snacks::veggies::CUCUMBER,
    );
}
