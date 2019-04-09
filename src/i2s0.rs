#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WCLK Source Selection"]
    pub aifwclksrc: AIFWCLKSRC,
    #[doc = "0x04 - DMA Buffer Size Configuration"]
    pub aifdmacfg: AIFDMACFG,
    #[doc = "0x08 - Pin Direction"]
    pub aifdircfg: AIFDIRCFG,
    #[doc = "0x0c - Serial Interface Format Configuration"]
    pub aiffmtcfg: AIFFMTCFG,
    #[doc = "0x10 - Word Selection Bit Mask for Pin 0"]
    pub aifwmask0: AIFWMASK0,
    #[doc = "0x14 - Word Selection Bit Mask for Pin 1"]
    pub aifwmask1: AIFWMASK1,
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    pub aifwmask2: AIFWMASK2,
    #[doc = "0x1c - Audio Interface PWM Debug Value"]
    pub aifpwmvalue: AIFPWMVALUE,
    #[doc = "0x20 - DMA Input Buffer Next Pointer"]
    pub aifinptrnext: AIFINPTRNEXT,
    #[doc = "0x24 - DMA Input Buffer Current Pointer"]
    pub aifinptr: AIFINPTR,
    #[doc = "0x28 - DMA Output Buffer Next Pointer"]
    pub aifoutptrnext: AIFOUTPTRNEXT,
    #[doc = "0x2c - DMA Output Buffer Current Pointer"]
    pub aifoutptr: AIFOUTPTR,
    _reserved0: [u8; 4usize],
    #[doc = "0x34 - Samplestamp Generator Control Register"]
    pub stmpctl: STMPCTL,
    #[doc = "0x38 - Captured XOSC Counter Value, Capture Channel 0"]
    pub stmpxcntcapt0: STMPXCNTCAPT0,
    #[doc = "0x3c - XOSC Period Value"]
    pub stmpxper: STMPXPER,
    #[doc = "0x40 - Captured WCLK Counter Value, Capture Channel 0"]
    pub stmpwcntcapt0: STMPWCNTCAPT0,
    #[doc = "0x44 - WCLK Counter Period Value"]
    pub stmpwper: STMPWPER,
    #[doc = "0x48 - WCLK Counter Trigger Value for Input Pins"]
    pub stmpintrig: STMPINTRIG,
    #[doc = "0x4c - WCLK Counter Trigger Value for Output Pins"]
    pub stmpouttrig: STMPOUTTRIG,
    #[doc = "0x50 - WCLK Counter Set Operation"]
    pub stmpwset: STMPWSET,
    #[doc = "0x54 - WCLK Counter Add Operation"]
    pub stmpwadd: STMPWADD,
    #[doc = "0x58 - XOSC Minimum Period Value Minimum Value of STMPXPER"]
    pub stmpxpermin: STMPXPERMIN,
    #[doc = "0x5c - Current Value of WCNT"]
    pub stmpwcnt: STMPWCNT,
    #[doc = "0x60 - Current Value of XCNT"]
    pub stmpxcnt: STMPXCNT,
    #[doc = "0x64 - Internal. Only to be used through TI provided API."]
    pub stmpxcntcapt1: STMPXCNTCAPT1,
    #[doc = "0x68 - Internal. Only to be used through TI provided API."]
    pub stmpwcntcapt1: STMPWCNTCAPT1,
    _reserved1: [u8; 4usize],
    #[doc = "0x70 - Interrupt Mask Register Selects mask states of the flags in IRQFLAGS that contribute to the I2S_IRQ event."]
    pub irqmask: IRQMASK,
    #[doc = "0x74 - Raw Interrupt Status Register"]
    pub irqflags: IRQFLAGS,
    #[doc = "0x78 - Interrupt Set Register"]
    pub irqset: IRQSET,
    #[doc = "0x7c - Interrupt Clear Register"]
    pub irqclr: IRQCLR,
}
#[doc = "WCLK Source Selection"]
pub struct AIFWCLKSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WCLK Source Selection"]
pub mod aifwclksrc;
#[doc = "DMA Buffer Size Configuration"]
pub struct AIFDMACFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Buffer Size Configuration"]
pub mod aifdmacfg;
#[doc = "Pin Direction"]
pub struct AIFDIRCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Direction"]
pub mod aifdircfg;
#[doc = "Serial Interface Format Configuration"]
pub struct AIFFMTCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Serial Interface Format Configuration"]
pub mod aiffmtcfg;
#[doc = "Word Selection Bit Mask for Pin 0"]
pub struct AIFWMASK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Word Selection Bit Mask for Pin 0"]
pub mod aifwmask0;
#[doc = "Word Selection Bit Mask for Pin 1"]
pub struct AIFWMASK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Word Selection Bit Mask for Pin 1"]
pub mod aifwmask1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct AIFWMASK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod aifwmask2;
#[doc = "Audio Interface PWM Debug Value"]
pub struct AIFPWMVALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Audio Interface PWM Debug Value"]
pub mod aifpwmvalue;
#[doc = "DMA Input Buffer Next Pointer"]
pub struct AIFINPTRNEXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Input Buffer Next Pointer"]
pub mod aifinptrnext;
#[doc = "DMA Input Buffer Current Pointer"]
pub struct AIFINPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Input Buffer Current Pointer"]
pub mod aifinptr;
#[doc = "DMA Output Buffer Next Pointer"]
pub struct AIFOUTPTRNEXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Output Buffer Next Pointer"]
pub mod aifoutptrnext;
#[doc = "DMA Output Buffer Current Pointer"]
pub struct AIFOUTPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Output Buffer Current Pointer"]
pub mod aifoutptr;
#[doc = "Samplestamp Generator Control Register"]
pub struct STMPCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Samplestamp Generator Control Register"]
pub mod stmpctl;
#[doc = "Captured XOSC Counter Value, Capture Channel 0"]
pub struct STMPXCNTCAPT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Captured XOSC Counter Value, Capture Channel 0"]
pub mod stmpxcntcapt0;
#[doc = "XOSC Period Value"]
pub struct STMPXPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XOSC Period Value"]
pub mod stmpxper;
#[doc = "Captured WCLK Counter Value, Capture Channel 0"]
pub struct STMPWCNTCAPT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Captured WCLK Counter Value, Capture Channel 0"]
pub mod stmpwcntcapt0;
#[doc = "WCLK Counter Period Value"]
pub struct STMPWPER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WCLK Counter Period Value"]
pub mod stmpwper;
#[doc = "WCLK Counter Trigger Value for Input Pins"]
pub struct STMPINTRIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WCLK Counter Trigger Value for Input Pins"]
pub mod stmpintrig;
#[doc = "WCLK Counter Trigger Value for Output Pins"]
pub struct STMPOUTTRIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WCLK Counter Trigger Value for Output Pins"]
pub mod stmpouttrig;
#[doc = "WCLK Counter Set Operation"]
pub struct STMPWSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WCLK Counter Set Operation"]
pub mod stmpwset;
#[doc = "WCLK Counter Add Operation"]
pub struct STMPWADD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WCLK Counter Add Operation"]
pub mod stmpwadd;
#[doc = "XOSC Minimum Period Value Minimum Value of STMPXPER"]
pub struct STMPXPERMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XOSC Minimum Period Value Minimum Value of STMPXPER"]
pub mod stmpxpermin;
#[doc = "Current Value of WCNT"]
pub struct STMPWCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Value of WCNT"]
pub mod stmpwcnt;
#[doc = "Current Value of XCNT"]
pub struct STMPXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Value of XCNT"]
pub mod stmpxcnt;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct STMPXCNTCAPT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod stmpxcntcapt1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct STMPWCNTCAPT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod stmpwcntcapt1;
#[doc = "Interrupt Mask Register Selects mask states of the flags in IRQFLAGS that contribute to the I2S_IRQ event."]
pub struct IRQMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register Selects mask states of the flags in IRQFLAGS that contribute to the I2S_IRQ event."]
pub mod irqmask;
#[doc = "Raw Interrupt Status Register"]
pub struct IRQFLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Interrupt Status Register"]
pub mod irqflags;
#[doc = "Interrupt Set Register"]
pub struct IRQSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set Register"]
pub mod irqset;
#[doc = "Interrupt Clear Register"]
pub struct IRQCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear Register"]
pub mod irqclr;
