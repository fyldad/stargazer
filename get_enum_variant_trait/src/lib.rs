use rand::prelude::ThreadRng;

pub trait GetEnumVariant {
    fn get_enum_variant(rng: ThreadRng) -> Self;
}
