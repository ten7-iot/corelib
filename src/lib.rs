#![no_std]

pub mod networking {
    pub mod byte_manager;
}

pub mod issues {
    pub mod hardware_issue;
    pub mod connectivity_issue;
    pub mod conversion_issue;
    pub mod issue;
}