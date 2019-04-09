#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Semaphore 0"]
    pub smph0: SMPH0,
    #[doc = "0x04 - Semaphore 1"]
    pub smph1: SMPH1,
    #[doc = "0x08 - Semaphore 2"]
    pub smph2: SMPH2,
    #[doc = "0x0c - Semaphore 3"]
    pub smph3: SMPH3,
    #[doc = "0x10 - Semaphore 4"]
    pub smph4: SMPH4,
    #[doc = "0x14 - Semaphore 5"]
    pub smph5: SMPH5,
    #[doc = "0x18 - Semaphore 6"]
    pub smph6: SMPH6,
    #[doc = "0x1c - Semaphore 7"]
    pub smph7: SMPH7,
    #[doc = "0x20 - Auto Take Sticky Request for Single Semaphore."]
    pub autotake: AUTOTAKE,
}
#[doc = "Semaphore 0"]
pub struct SMPH0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 0"]
pub mod smph0;
#[doc = "Semaphore 1"]
pub struct SMPH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 1"]
pub mod smph1;
#[doc = "Semaphore 2"]
pub struct SMPH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 2"]
pub mod smph2;
#[doc = "Semaphore 3"]
pub struct SMPH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 3"]
pub mod smph3;
#[doc = "Semaphore 4"]
pub struct SMPH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 4"]
pub mod smph4;
#[doc = "Semaphore 5"]
pub struct SMPH5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 5"]
pub mod smph5;
#[doc = "Semaphore 6"]
pub struct SMPH6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 6"]
pub mod smph6;
#[doc = "Semaphore 7"]
pub struct SMPH7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore 7"]
pub mod smph7;
#[doc = "Auto Take Sticky Request for Single Semaphore."]
pub struct AUTOTAKE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auto Take Sticky Request for Single Semaphore."]
pub mod autotake;
