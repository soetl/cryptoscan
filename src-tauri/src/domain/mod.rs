pub(crate) mod settings;
pub(crate) mod coin;

pub trait Entity {}

pub trait Value {
    type ValueType;

    fn value(&self) -> &Self::ValueType;
}

impl Entity for u32 {}
impl Entity for String {}
