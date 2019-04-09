#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Random Number Lower Word Readout Value"]
    pub out0: OUT0,
    #[doc = "0x04 - Random Number Upper Word Readout Value"]
    pub out1: OUT1,
    #[doc = "0x08 - Interrupt Status"]
    pub irqflagstat: IRQFLAGSTAT,
    #[doc = "0x0c - Interrupt Mask"]
    pub irqflagmask: IRQFLAGMASK,
    #[doc = "0x10 - Interrupt Flag Clear"]
    pub irqflagclr: IRQFLAGCLR,
    #[doc = "0x14 - Control"]
    pub ctl: CTL,
    #[doc = "0x18 - Configuration 0"]
    pub cfg0: CFG0,
    #[doc = "0x1c - Alarm Control"]
    pub alarmcnt: ALARMCNT,
    #[doc = "0x20 - FRO Enable"]
    pub froen: FROEN,
    #[doc = "0x24 - FRO De-tune Bit"]
    pub frodetune: FRODETUNE,
    #[doc = "0x28 - Alarm Event"]
    pub alarmmask: ALARMMASK,
    #[doc = "0x2c - Alarm Shutdown"]
    pub alarmstop: ALARMSTOP,
    #[doc = "0x30 - LFSR Readout Value"]
    pub lfsr0: LFSR0,
    #[doc = "0x34 - LFSR Readout Value"]
    pub lfsr1: LFSR1,
    #[doc = "0x38 - LFSR Readout Value"]
    pub lfsr2: LFSR2,
    _reserved0: [u8; 60usize],
    #[doc = "0x78 - TRNG Engine Options Information"]
    pub hwopt: HWOPT,
    #[doc = "0x7c - HW Version 0 EIP Number And Core Revision"]
    pub hwver0: HWVER0,
    _reserved1: [u8; 8024usize],
    #[doc = "0x1fd8 - Interrupt Status After Masking"]
    pub irqstatmask: IRQSTATMASK,
    _reserved2: [u8; 4usize],
    #[doc = "0x1fe0 - HW Version 1 TRNG Revision Number"]
    pub hwver1: HWVER1,
    _reserved3: [u8; 8usize],
    #[doc = "0x1fec - Interrupt Set"]
    pub irqset: IRQSET,
    #[doc = "0x1ff0 - SW Reset Control"]
    pub swreset: SWRESET,
    _reserved4: [u8; 4usize],
    #[doc = "0x1ff8 - Interrupt Status"]
    pub irqstat: IRQSTAT,
}
#[doc = "Random Number Lower Word Readout Value"]
pub struct OUT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Random Number Lower Word Readout Value"]
pub mod out0;
#[doc = "Random Number Upper Word Readout Value"]
pub struct OUT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Random Number Upper Word Readout Value"]
pub mod out1;
#[doc = "Interrupt Status"]
pub struct IRQFLAGSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod irqflagstat;
#[doc = "Interrupt Mask"]
pub struct IRQFLAGMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask"]
pub mod irqflagmask;
#[doc = "Interrupt Flag Clear"]
pub struct IRQFLAGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Clear"]
pub mod irqflagclr;
#[doc = "Control"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control"]
pub mod ctl;
#[doc = "Configuration 0"]
pub struct CFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration 0"]
pub mod cfg0;
#[doc = "Alarm Control"]
pub struct ALARMCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm Control"]
pub mod alarmcnt;
#[doc = "FRO Enable"]
pub struct FROEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FRO Enable"]
pub mod froen;
#[doc = "FRO De-tune Bit"]
pub struct FRODETUNE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FRO De-tune Bit"]
pub mod frodetune;
#[doc = "Alarm Event"]
pub struct ALARMMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm Event"]
pub mod alarmmask;
#[doc = "Alarm Shutdown"]
pub struct ALARMSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm Shutdown"]
pub mod alarmstop;
#[doc = "LFSR Readout Value"]
pub struct LFSR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFSR Readout Value"]
pub mod lfsr0;
#[doc = "LFSR Readout Value"]
pub struct LFSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFSR Readout Value"]
pub mod lfsr1;
#[doc = "LFSR Readout Value"]
pub struct LFSR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFSR Readout Value"]
pub mod lfsr2;
#[doc = "TRNG Engine Options Information"]
pub struct HWOPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRNG Engine Options Information"]
pub mod hwopt;
#[doc = "HW Version 0 EIP Number And Core Revision"]
pub struct HWVER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HW Version 0 EIP Number And Core Revision"]
pub mod hwver0;
#[doc = "Interrupt Status After Masking"]
pub struct IRQSTATMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status After Masking"]
pub mod irqstatmask;
#[doc = "HW Version 1 TRNG Revision Number"]
pub struct HWVER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HW Version 1 TRNG Revision Number"]
pub mod hwver1;
#[doc = "Interrupt Set"]
pub struct IRQSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set"]
pub mod irqset;
#[doc = "SW Reset Control"]
pub struct SWRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SW Reset Control"]
pub mod swreset;
#[doc = "Interrupt Status"]
pub struct IRQSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod irqstat;
