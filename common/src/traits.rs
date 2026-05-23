pub use common_derive::TypeString;

pub trait TypeString {
    fn type_as_string(&self) -> String;
}
