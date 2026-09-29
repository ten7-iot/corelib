#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkScenario {
    LoraJoinTimeout = 0x01,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkReason {
    Timeout = 0x01,
}