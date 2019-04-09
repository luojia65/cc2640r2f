#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration"]
    pub load: LOAD,
    #[doc = "0x04 - Current Count Value"]
    pub value: VALUE,
    #[doc = "0x08 - Control"]
    pub ctl: CTL,
    #[doc = "0x0c - Interrupt Clear"]
    pub icr: ICR,
    #[doc = "0x10 - Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x14 - Masked Interrupt Status"]
    pub mis: MIS,
    _reserved0: [u8; 1024usize],
    #[doc = "0x418 - Test Mode"]
    pub test: TEST,
    #[doc = "0x41c - Interrupt Cause Test Mode"]
    pub int_caus: INT_CAUS,
    _reserved1: [u8; 2016usize],
    #[doc = "0xc00 - Lock"]
    pub lock: LOCK,
}
#[doc = "Configuration"]
pub struct LOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration"]
pub mod load;
#[doc = "Current Count Value"]
pub struct VALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Count Value"]
pub mod value;
#[doc = "Control"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control"]
pub mod ctl;
#[doc = "Interrupt Clear"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear"]
pub mod icr;
#[doc = "Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "Masked Interrupt Status"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked Interrupt Status"]
pub mod mis;
#[doc = "Test Mode"]
pub struct TEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test Mode"]
pub mod test;
#[doc = "Interrupt Cause Test Mode"]
pub struct INT_CAUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Cause Test Mode"]
pub mod int_caus;
#[doc = "Lock"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock"]
pub mod lock;
