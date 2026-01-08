pub mod pit {
    pub mod envelope {
        pub mod v1 {
            include!("gen/pit.envelope.v1.rs");
        }
    }
    pub mod events {
        pub mod v1 {
            include!("gen/pit.events.v1.rs");
        }
    }
}
