pub mod example {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/example.v1.rs"));
    }
}
