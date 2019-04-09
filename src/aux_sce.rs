#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub ctl: CTL,
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    pub fetchstat: FETCHSTAT,
    #[doc = "0x08 - Internal. Only to be used through TI provided API."]
    pub cpustat: CPUSTAT,
    #[doc = "0x0c - Internal. Only to be used through TI provided API."]
    pub wustat: WUSTAT,
    #[doc = "0x10 - Internal. Only to be used through TI provided API."]
    pub reg1_0: REG1_0,
    #[doc = "0x14 - Internal. Only to be used through TI provided API."]
    pub reg3_2: REG3_2,
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    pub reg5_4: REG5_4,
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    pub reg7_6: REG7_6,
    #[doc = "0x20 - Internal. Only to be used through TI provided API."]
    pub loopaddr: LOOPADDR,
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    pub loopcnt: LOOPCNT,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ctl;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FETCHSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fetchstat;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CPUSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cpustat;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct WUSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod wustat;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct REG1_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg1_0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct REG3_2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg3_2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct REG5_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg5_4;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct REG7_6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg7_6;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct LOOPADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod loopaddr;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct LOOPCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod loopcnt;
