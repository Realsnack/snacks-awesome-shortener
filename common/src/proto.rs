pub mod messaging {
    pub mod v1 {
        pub mod commands {
            include!(concat!(env!("OUT_DIR"), "/messaging.v1.commands.rs"));
        }
        pub mod events {
            include!(concat!(env!("OUT_DIR"), "/messaging.v1.commands.rs"));
        }
    }
}

pub mod common {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/common.v1.rs"));
    }
}
