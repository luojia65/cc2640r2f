#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved000: RESERVED000,
    _reserved0: [u8; 4084usize],
    #[doc = "0xff8 - Internal. Only to be used through TI provided API."]
    pub traceclkmux: TRACECLKMUX,
    #[doc = "0xffc - Internal. Only to be used through TI provided API."]
    pub dyn_cg: DYN_CG,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED000 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved000;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct TRACECLKMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod traceclkmux;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct DYN_CG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dyn_cg;
