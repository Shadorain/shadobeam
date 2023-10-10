mod utils;

pub use utils::*;

pub mod common {
    tonic::include_proto!("common");
}
pub mod tasks {
    tonic::include_proto!("tasks");
}
pub mod iface {
    tonic::include_proto!("iface");
}
