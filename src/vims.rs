#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status Displays current VIMS mode and line buffer status"]
    pub stat: STAT,
    #[doc = "0x04 - Control Configure VIMS mode and line buffer settings"]
    pub ctl: CTL,
}
#[doc = "Status Displays current VIMS mode and line buffer status"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Displays current VIMS mode and line buffer status"]
pub mod stat;
#[doc = "Control Configure VIMS mode and line buffer settings"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Configure VIMS mode and line buffer settings"]
pub mod ctl;
