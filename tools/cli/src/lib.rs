pub mod catalog;
pub mod device;
pub mod environment;
pub mod platform;
pub mod project;
pub mod util;
pub const CONTRACT: u64 = 4;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod skills;

pub mod firmware;

pub mod maintenance_ci;
#[path = "../../../platforms/schema.rs"]
pub mod platform_schema;

pub mod maintenance_device;
pub mod maintenance_evidence;
pub mod maintenance_generate;

pub mod maintenance_checks;

pub mod maintenance_contracts;
pub mod maintenance_skills;

pub mod maintenance_reference;
