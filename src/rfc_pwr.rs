#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RF Core Power Management and Clock Enable"]
    pub pwmclken: PWMCLKEN,
}
#[doc = "RF Core Power Management and Clock Enable"]
pub struct PWMCLKEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Core Power Management and Clock Enable"]
pub mod pwmclken;
