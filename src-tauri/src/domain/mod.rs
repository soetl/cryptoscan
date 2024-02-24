pub(crate) mod coin;
pub(crate) mod create_coin;
pub(crate) mod delete_coin;
pub(crate) mod find_coin;
pub(crate) mod update_coin;
pub(crate) mod get_all_coins;

pub trait Entity {}

pub trait Value {
    type ValueType;

    fn value(&self) -> &Self::ValueType;
}

impl Entity for u32 {}
