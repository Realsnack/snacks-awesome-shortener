pub use common_derive::ProtoMessage;
pub use common_derive::TypeString;

pub trait TypeString {
    fn type_as_string(&self) -> String;
}

pub trait ProtoMessage {
    type Proto;

    fn to_proto(&self) -> Self::Proto;
}
