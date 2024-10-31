// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

#[allow(dead_code)]
mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }
    pub use self::fruits::PEAR as fruit;
    // /pub use self::fruits::APPLE;
    


    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
    pub use self::veggies::CARROT as veggie;
}

// use delicious_snacks::fruits as fruit


fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}