#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Radio Timer Counter Value"]
    pub ratcnt: RATCNT,
    _reserved1: [u8; 120usize],
    #[doc = "0x80 - Timer Channel 0 Capture/Compare Register"]
    pub ratch0val: RATCH0VAL,
    #[doc = "0x84 - Timer Channel 1 Capture/Compare Register"]
    pub ratch1val: RATCH1VAL,
    #[doc = "0x88 - Timer Channel 2 Capture/Compare Register"]
    pub ratch2val: RATCH2VAL,
    #[doc = "0x8c - Timer Channel 3 Capture/Compare Register"]
    pub ratch3val: RATCH3VAL,
    #[doc = "0x90 - Timer Channel 4 Capture/Compare Register"]
    pub ratch4val: RATCH4VAL,
    #[doc = "0x94 - Timer Channel 5 Capture/Compare Register"]
    pub ratch5val: RATCH5VAL,
    #[doc = "0x98 - Timer Channel 6 Capture/Compare Register"]
    pub ratch6val: RATCH6VAL,
    #[doc = "0x9c - Timer Channel 7 Capture/Compare Register"]
    pub ratch7val: RATCH7VAL,
}
#[doc = "Radio Timer Counter Value"]
pub struct RATCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio Timer Counter Value"]
pub mod ratcnt;
#[doc = "Timer Channel 0 Capture/Compare Register"]
pub struct RATCH0VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Channel 0 Capture/Compare Register"]
pub mod ratch0val;
#[doc = "Timer Channel 1 Capture/Compare Register"]
pub struct RATCH1VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Channel 1 Capture/Compare Register"]
pub mod ratch1val;
#[doc = "Timer Channel 2 Capture/Compare Register"]
pub struct RATCH2VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Channel 2 Capture/Compare Register"]
pub mod ratch2val;
#[doc = "Timer Channel 3 Capture/Compare Register"]
pub struct RATCH3VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Channel 3 Capture/Compare Register"]
pub mod ratch3val;
#[doc = "Timer Channel 4 Capture/Compare Register"]
pub struct RATCH4VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Channel 4 Capture/Compare Register"]
pub mod ratch4val;
#[doc = "Timer Channel 5 Capture/Compare Register"]
pub struct RATCH5VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Channel 5 Capture/Compare Register"]
pub mod ratch5val;
#[doc = "Timer Channel 6 Capture/Compare Register"]
pub struct RATCH6VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Channel 6 Capture/Compare Register"]
pub mod ratch6val;
#[doc = "Timer Channel 7 Capture/Compare Register"]
pub struct RATCH7VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Channel 7 Capture/Compare Register"]
pub mod ratch7val;
