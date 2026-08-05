pub mod messaging {
    pub mod v1 {
        pub mod commands {
            include!(concat!(env!("OUT_DIR"), "/messaging.v1.commands.rs"));
        }
        pub mod events {
            include!(concat!(env!("OUT_DIR"), "/messaging.v1.events.rs"));
        }
    }
}

pub mod common {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/common.v1.rs"));
    }
}

pub mod proto_conversion_error {
    use std::{error, fmt};

    #[derive(Debug, Clone)]
    pub struct ConversionFromNone;

    impl fmt::Display for ConversionFromNone {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "it's not possible to convert None from proto")
        }
    }

    impl error::Error for ConversionFromNone {}
}
