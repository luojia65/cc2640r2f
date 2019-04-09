#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub iostrmin: IOSTRMIN,
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    pub iostrmed: IOSTRMED,
    #[doc = "0x08 - Internal. Only to be used through TI provided API."]
    pub iostrmax: IOSTRMAX,
    #[doc = "0x0c - IO Latch Control Controls transparency of all latches holding I/O or configuration state from the MCU IOC"]
    pub ioclatch: IOCLATCH,
    #[doc = "0x10 - SCLK_LF External Output Control"]
    pub clk32kctl: CLK32KCTL,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub struct IOSTRMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmin;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct IOSTRMED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmed;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct IOSTRMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmax;
#[doc = "IO Latch Control Controls transparency of all latches holding I/O or configuration state from the MCU IOC"]
pub struct IOCLATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IO Latch Control Controls transparency of all latches holding I/O or configuration state from the MCU IOC"]
pub mod ioclatch;
#[doc = "SCLK_LF External Output Control"]
pub struct CLK32KCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCLK_LF External Output Control"]
pub mod clk32kctl;
