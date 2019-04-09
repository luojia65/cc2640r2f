#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer 0 Configuration"]
    pub t0cfg: T0CFG,
    #[doc = "0x04 - Timer 1 Configuration"]
    pub t1cfg: T1CFG,
    #[doc = "0x08 - Timer 0 Control"]
    pub t0ctl: T0CTL,
    #[doc = "0x0c - Timer 0 Target"]
    pub t0target: T0TARGET,
    #[doc = "0x10 - Timer 1 Target Timer 1 counter target value"]
    pub t1target: T1TARGET,
    #[doc = "0x14 - Timer 1 Control"]
    pub t1ctl: T1CTL,
}
#[doc = "Timer 0 Configuration"]
pub struct T0CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 0 Configuration"]
pub mod t0cfg;
#[doc = "Timer 1 Configuration"]
pub struct T1CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 1 Configuration"]
pub mod t1cfg;
#[doc = "Timer 0 Control"]
pub struct T0CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 0 Control"]
pub mod t0ctl;
#[doc = "Timer 0 Target"]
pub struct T0TARGET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 0 Target"]
pub mod t0target;
#[doc = "Timer 1 Target Timer 1 counter target value"]
pub struct T1TARGET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 1 Target Timer 1 counter target value"]
pub mod t1target;
#[doc = "Timer 1 Control"]
pub struct T1CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer 1 Control"]
pub mod t1ctl;
