#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Out 0 to 3 Alias register for byte access to each bit in DOUT31_0"]
    pub dout3_0: DOUT3_0,
    #[doc = "0x04 - Data Out 4 to 7 Alias register for byte access to each bit in DOUT31_0"]
    pub dout7_4: DOUT7_4,
    #[doc = "0x08 - Data Out 8 to 11 Alias register for byte access to each bit in DOUT31_0"]
    pub dout11_8: DOUT11_8,
    #[doc = "0x0c - Data Out 12 to 15 Alias register for byte access to each bit in DOUT31_0"]
    pub dout15_12: DOUT15_12,
    #[doc = "0x10 - Data Out 16 to 19 Alias register for byte access to each bit in DOUT31_0"]
    pub dout19_16: DOUT19_16,
    #[doc = "0x14 - Data Out 20 to 23 Alias register for byte access to each bit in DOUT31_0"]
    pub dout23_20: DOUT23_20,
    #[doc = "0x18 - Data Out 24 to 27 Alias register for byte access to each bit in DOUT31_0"]
    pub dout27_24: DOUT27_24,
    #[doc = "0x1c - Data Out 28 to 31 Alias register for byte access to each bit in DOUT31_0"]
    pub dout31_28: DOUT31_28,
    _reserved0: [u8; 96usize],
    #[doc = "0x80 - Data Output for DIO 0 to 31"]
    pub dout31_0: DOUT31_0,
    _reserved1: [u8; 12usize],
    #[doc = "0x90 - Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register"]
    pub doutset31_0: DOUTSET31_0,
    _reserved2: [u8; 12usize],
    #[doc = "0xa0 - Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register"]
    pub doutclr31_0: DOUTCLR31_0,
    _reserved3: [u8; 12usize],
    #[doc = "0xb0 - Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
    pub douttgl31_0: DOUTTGL31_0,
    _reserved4: [u8; 12usize],
    #[doc = "0xc0 - Data Input from DIO 0 to 31"]
    pub din31_0: DIN31_0,
    _reserved5: [u8; 12usize],
    #[doc = "0xd0 - Data Output Enable for DIO 0 to 31"]
    pub doe31_0: DOE31_0,
    _reserved6: [u8; 12usize],
    #[doc = "0xe0 - Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
    pub evflags31_0: EVFLAGS31_0,
}
#[doc = "Data Out 0 to 3 Alias register for byte access to each bit in DOUT31_0"]
pub struct DOUT3_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Out 0 to 3 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout3_0;
#[doc = "Data Out 4 to 7 Alias register for byte access to each bit in DOUT31_0"]
pub struct DOUT7_4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Out 4 to 7 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout7_4;
#[doc = "Data Out 8 to 11 Alias register for byte access to each bit in DOUT31_0"]
pub struct DOUT11_8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Out 8 to 11 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout11_8;
#[doc = "Data Out 12 to 15 Alias register for byte access to each bit in DOUT31_0"]
pub struct DOUT15_12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Out 12 to 15 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout15_12;
#[doc = "Data Out 16 to 19 Alias register for byte access to each bit in DOUT31_0"]
pub struct DOUT19_16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Out 16 to 19 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout19_16;
#[doc = "Data Out 20 to 23 Alias register for byte access to each bit in DOUT31_0"]
pub struct DOUT23_20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Out 20 to 23 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout23_20;
#[doc = "Data Out 24 to 27 Alias register for byte access to each bit in DOUT31_0"]
pub struct DOUT27_24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Out 24 to 27 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout27_24;
#[doc = "Data Out 28 to 31 Alias register for byte access to each bit in DOUT31_0"]
pub struct DOUT31_28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Out 28 to 31 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout31_28;
#[doc = "Data Output for DIO 0 to 31"]
pub struct DOUT31_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Output for DIO 0 to 31"]
pub mod dout31_0;
#[doc = "Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register"]
pub struct DOUTSET31_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register"]
pub mod doutset31_0;
#[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register"]
pub struct DOUTCLR31_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register"]
pub mod doutclr31_0;
#[doc = "Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
pub struct DOUTTGL31_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
pub mod douttgl31_0;
#[doc = "Data Input from DIO 0 to 31"]
pub struct DIN31_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Input from DIO 0 to 31"]
pub mod din31_0;
#[doc = "Data Output Enable for DIO 0 to 31"]
pub struct DOE31_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Output Enable for DIO 0 to 31"]
pub mod doe31_0;
#[doc = "Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
pub struct EVFLAGS31_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
pub mod evflags31_0;
