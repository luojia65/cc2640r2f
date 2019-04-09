#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Output Selection for CPU Interrupt 0"]
    pub cpuirqsel0: CPUIRQSEL0,
    #[doc = "0x04 - Output Selection for CPU Interrupt 1"]
    pub cpuirqsel1: CPUIRQSEL1,
    #[doc = "0x08 - Output Selection for CPU Interrupt 2"]
    pub cpuirqsel2: CPUIRQSEL2,
    #[doc = "0x0c - Output Selection for CPU Interrupt 3"]
    pub cpuirqsel3: CPUIRQSEL3,
    #[doc = "0x10 - Output Selection for CPU Interrupt 4"]
    pub cpuirqsel4: CPUIRQSEL4,
    #[doc = "0x14 - Output Selection for CPU Interrupt 5"]
    pub cpuirqsel5: CPUIRQSEL5,
    #[doc = "0x18 - Output Selection for CPU Interrupt 6"]
    pub cpuirqsel6: CPUIRQSEL6,
    #[doc = "0x1c - Output Selection for CPU Interrupt 7"]
    pub cpuirqsel7: CPUIRQSEL7,
    #[doc = "0x20 - Output Selection for CPU Interrupt 8"]
    pub cpuirqsel8: CPUIRQSEL8,
    #[doc = "0x24 - Output Selection for CPU Interrupt 9"]
    pub cpuirqsel9: CPUIRQSEL9,
    #[doc = "0x28 - Output Selection for CPU Interrupt 10"]
    pub cpuirqsel10: CPUIRQSEL10,
    #[doc = "0x2c - Output Selection for CPU Interrupt 11"]
    pub cpuirqsel11: CPUIRQSEL11,
    #[doc = "0x30 - Output Selection for CPU Interrupt 12"]
    pub cpuirqsel12: CPUIRQSEL12,
    #[doc = "0x34 - Output Selection for CPU Interrupt 13"]
    pub cpuirqsel13: CPUIRQSEL13,
    #[doc = "0x38 - Output Selection for CPU Interrupt 14"]
    pub cpuirqsel14: CPUIRQSEL14,
    #[doc = "0x3c - Output Selection for CPU Interrupt 15"]
    pub cpuirqsel15: CPUIRQSEL15,
    #[doc = "0x40 - Output Selection for CPU Interrupt 16"]
    pub cpuirqsel16: CPUIRQSEL16,
    #[doc = "0x44 - Output Selection for CPU Interrupt 17"]
    pub cpuirqsel17: CPUIRQSEL17,
    #[doc = "0x48 - Output Selection for CPU Interrupt 18"]
    pub cpuirqsel18: CPUIRQSEL18,
    #[doc = "0x4c - Output Selection for CPU Interrupt 19"]
    pub cpuirqsel19: CPUIRQSEL19,
    #[doc = "0x50 - Output Selection for CPU Interrupt 20"]
    pub cpuirqsel20: CPUIRQSEL20,
    #[doc = "0x54 - Output Selection for CPU Interrupt 21"]
    pub cpuirqsel21: CPUIRQSEL21,
    #[doc = "0x58 - Output Selection for CPU Interrupt 22"]
    pub cpuirqsel22: CPUIRQSEL22,
    #[doc = "0x5c - Output Selection for CPU Interrupt 23"]
    pub cpuirqsel23: CPUIRQSEL23,
    #[doc = "0x60 - Output Selection for CPU Interrupt 24"]
    pub cpuirqsel24: CPUIRQSEL24,
    #[doc = "0x64 - Output Selection for CPU Interrupt 25"]
    pub cpuirqsel25: CPUIRQSEL25,
    #[doc = "0x68 - Output Selection for CPU Interrupt 26"]
    pub cpuirqsel26: CPUIRQSEL26,
    #[doc = "0x6c - Output Selection for CPU Interrupt 27"]
    pub cpuirqsel27: CPUIRQSEL27,
    #[doc = "0x70 - Output Selection for CPU Interrupt 28"]
    pub cpuirqsel28: CPUIRQSEL28,
    #[doc = "0x74 - Output Selection for CPU Interrupt 29"]
    pub cpuirqsel29: CPUIRQSEL29,
    #[doc = "0x78 - Output Selection for CPU Interrupt 30"]
    pub cpuirqsel30: CPUIRQSEL30,
    #[doc = "0x7c - Output Selection for CPU Interrupt 31"]
    pub cpuirqsel31: CPUIRQSEL31,
    #[doc = "0x80 - Output Selection for CPU Interrupt 32"]
    pub cpuirqsel32: CPUIRQSEL32,
    #[doc = "0x84 - Output Selection for CPU Interrupt 33"]
    pub cpuirqsel33: CPUIRQSEL33,
    _reserved0: [u8; 120usize],
    #[doc = "0x100 - Output Selection for RFC Event 0"]
    pub rfcsel0: RFCSEL0,
    #[doc = "0x104 - Output Selection for RFC Event 1"]
    pub rfcsel1: RFCSEL1,
    #[doc = "0x108 - Output Selection for RFC Event 2"]
    pub rfcsel2: RFCSEL2,
    #[doc = "0x10c - Output Selection for RFC Event 3"]
    pub rfcsel3: RFCSEL3,
    #[doc = "0x110 - Output Selection for RFC Event 4"]
    pub rfcsel4: RFCSEL4,
    #[doc = "0x114 - Output Selection for RFC Event 5"]
    pub rfcsel5: RFCSEL5,
    #[doc = "0x118 - Output Selection for RFC Event 6"]
    pub rfcsel6: RFCSEL6,
    #[doc = "0x11c - Output Selection for RFC Event 7"]
    pub rfcsel7: RFCSEL7,
    #[doc = "0x120 - Output Selection for RFC Event 8"]
    pub rfcsel8: RFCSEL8,
    #[doc = "0x124 - Output Selection for RFC Event 9"]
    pub rfcsel9: RFCSEL9,
    _reserved1: [u8; 216usize],
    #[doc = "0x200 - Output Selection for GPT0 0"]
    pub gpt0acaptsel: GPT0ACAPTSEL,
    #[doc = "0x204 - Output Selection for GPT0 1"]
    pub gpt0bcaptsel: GPT0BCAPTSEL,
    _reserved2: [u8; 248usize],
    #[doc = "0x300 - Output Selection for GPT1 0"]
    pub gpt1acaptsel: GPT1ACAPTSEL,
    #[doc = "0x304 - Output Selection for GPT1 1"]
    pub gpt1bcaptsel: GPT1BCAPTSEL,
    _reserved3: [u8; 248usize],
    #[doc = "0x400 - Output Selection for GPT2 0"]
    pub gpt2acaptsel: GPT2ACAPTSEL,
    #[doc = "0x404 - Output Selection for GPT2 1"]
    pub gpt2bcaptsel: GPT2BCAPTSEL,
    _reserved4: [u8; 248usize],
    #[doc = "0x500 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach0ssel: UDMACH0SSEL,
    #[doc = "0x504 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach0bsel: UDMACH0BSEL,
    #[doc = "0x508 - Output Selection for DMA Channel 1 SREQ"]
    pub udmach1ssel: UDMACH1SSEL,
    #[doc = "0x50c - Output Selection for DMA Channel 1 REQ"]
    pub udmach1bsel: UDMACH1BSEL,
    #[doc = "0x510 - Output Selection for DMA Channel 2 SREQ"]
    pub udmach2ssel: UDMACH2SSEL,
    #[doc = "0x514 - Output Selection for DMA Channel 2 REQ"]
    pub udmach2bsel: UDMACH2BSEL,
    #[doc = "0x518 - Output Selection for DMA Channel 3 SREQ"]
    pub udmach3ssel: UDMACH3SSEL,
    #[doc = "0x51c - Output Selection for DMA Channel 3 REQ"]
    pub udmach3bsel: UDMACH3BSEL,
    #[doc = "0x520 - Output Selection for DMA Channel 4 SREQ"]
    pub udmach4ssel: UDMACH4SSEL,
    #[doc = "0x524 - Output Selection for DMA Channel 4 REQ"]
    pub udmach4bsel: UDMACH4BSEL,
    #[doc = "0x528 - Output Selection for DMA Channel 5 SREQ"]
    pub udmach5ssel: UDMACH5SSEL,
    #[doc = "0x52c - Output Selection for DMA Channel 5 REQ"]
    pub udmach5bsel: UDMACH5BSEL,
    #[doc = "0x530 - Output Selection for DMA Channel 6 SREQ"]
    pub udmach6ssel: UDMACH6SSEL,
    #[doc = "0x534 - Output Selection for DMA Channel 6 REQ"]
    pub udmach6bsel: UDMACH6BSEL,
    #[doc = "0x538 - Output Selection for DMA Channel 7 SREQ"]
    pub udmach7ssel: UDMACH7SSEL,
    #[doc = "0x53c - Output Selection for DMA Channel 7 REQ"]
    pub udmach7bsel: UDMACH7BSEL,
    #[doc = "0x540 - Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel"]
    pub udmach8ssel: UDMACH8SSEL,
    #[doc = "0x544 - Output Selection for DMA Channel 8 REQ"]
    pub udmach8bsel: UDMACH8BSEL,
    #[doc = "0x548 - Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
    pub udmach9ssel: UDMACH9SSEL,
    #[doc = "0x54c - Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
    pub udmach9bsel: UDMACH9BSEL,
    #[doc = "0x550 - Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
    pub udmach10ssel: UDMACH10SSEL,
    #[doc = "0x554 - Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
    pub udmach10bsel: UDMACH10BSEL,
    #[doc = "0x558 - Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
    pub udmach11ssel: UDMACH11SSEL,
    #[doc = "0x55c - Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
    pub udmach11bsel: UDMACH11BSEL,
    #[doc = "0x560 - Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
    pub udmach12ssel: UDMACH12SSEL,
    #[doc = "0x564 - Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
    pub udmach12bsel: UDMACH12BSEL,
    #[doc = "0x568 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach13ssel: UDMACH13SSEL,
    #[doc = "0x56c - Output Selection for DMA Channel 13 REQ"]
    pub udmach13bsel: UDMACH13BSEL,
    #[doc = "0x570 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach14ssel: UDMACH14SSEL,
    #[doc = "0x574 - Output Selection for DMA Channel 14 REQ"]
    pub udmach14bsel: UDMACH14BSEL,
    #[doc = "0x578 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach15ssel: UDMACH15SSEL,
    #[doc = "0x57c - Output Selection for DMA Channel 15 REQ"]
    pub udmach15bsel: UDMACH15BSEL,
    #[doc = "0x580 - Output Selection for DMA Channel 16 SREQ"]
    pub udmach16ssel: UDMACH16SSEL,
    #[doc = "0x584 - Output Selection for DMA Channel 16 REQ"]
    pub udmach16bsel: UDMACH16BSEL,
    #[doc = "0x588 - Output Selection for DMA Channel 17 SREQ"]
    pub udmach17ssel: UDMACH17SSEL,
    #[doc = "0x58c - Output Selection for DMA Channel 17 REQ"]
    pub udmach17bsel: UDMACH17BSEL,
    #[doc = "0x590 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach18ssel: UDMACH18SSEL,
    #[doc = "0x594 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach18bsel: UDMACH18BSEL,
    #[doc = "0x598 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach19ssel: UDMACH19SSEL,
    #[doc = "0x59c - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach19bsel: UDMACH19BSEL,
    #[doc = "0x5a0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach20ssel: UDMACH20SSEL,
    #[doc = "0x5a4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach20bsel: UDMACH20BSEL,
    #[doc = "0x5a8 - Output Selection for DMA Channel 21 SREQ"]
    pub udmach21ssel: UDMACH21SSEL,
    #[doc = "0x5ac - Output Selection for DMA Channel 21 REQ"]
    pub udmach21bsel: UDMACH21BSEL,
    #[doc = "0x5b0 - Output Selection for DMA Channel 22 SREQ"]
    pub udmach22ssel: UDMACH22SSEL,
    #[doc = "0x5b4 - Output Selection for DMA Channel 22 REQ"]
    pub udmach22bsel: UDMACH22BSEL,
    #[doc = "0x5b8 - Output Selection for DMA Channel 23 SREQ"]
    pub udmach23ssel: UDMACH23SSEL,
    #[doc = "0x5bc - Output Selection for DMA Channel 23 REQ"]
    pub udmach23bsel: UDMACH23BSEL,
    #[doc = "0x5c0 - Output Selection for DMA Channel 24 SREQ"]
    pub udmach24ssel: UDMACH24SSEL,
    #[doc = "0x5c4 - Output Selection for DMA Channel 24 REQ"]
    pub udmach24bsel: UDMACH24BSEL,
    #[doc = "0x5c8 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach25ssel: UDMACH25SSEL,
    #[doc = "0x5cc - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach25bsel: UDMACH25BSEL,
    #[doc = "0x5d0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach26ssel: UDMACH26SSEL,
    #[doc = "0x5d4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach26bsel: UDMACH26BSEL,
    #[doc = "0x5d8 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach27ssel: UDMACH27SSEL,
    #[doc = "0x5dc - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach27bsel: UDMACH27BSEL,
    #[doc = "0x5e0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach28ssel: UDMACH28SSEL,
    #[doc = "0x5e4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach28bsel: UDMACH28BSEL,
    #[doc = "0x5e8 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach29ssel: UDMACH29SSEL,
    #[doc = "0x5ec - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach29bsel: UDMACH29BSEL,
    #[doc = "0x5f0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach30ssel: UDMACH30SSEL,
    #[doc = "0x5f4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach30bsel: UDMACH30BSEL,
    #[doc = "0x5f8 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach31ssel: UDMACH31SSEL,
    #[doc = "0x5fc - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach31bsel: UDMACH31BSEL,
    #[doc = "0x600 - Output Selection for GPT3 0"]
    pub gpt3acaptsel: GPT3ACAPTSEL,
    #[doc = "0x604 - Output Selection for GPT3 1"]
    pub gpt3bcaptsel: GPT3BCAPTSEL,
    _reserved5: [u8; 248usize],
    #[doc = "0x700 - Output Selection for AUX Subscriber 0"]
    pub auxsel0: AUXSEL0,
    _reserved6: [u8; 252usize],
    #[doc = "0x800 - Output Selection for NMI Subscriber 0"]
    pub cm3nmisel0: CM3NMISEL0,
    _reserved7: [u8; 252usize],
    #[doc = "0x900 - Output Selection for I2S Subscriber 0"]
    pub i2sstmpsel0: I2SSTMPSEL0,
    _reserved8: [u8; 252usize],
    #[doc = "0xa00 - Output Selection for FRZ Subscriber The halted debug signal is passed to peripherals such as the General Purpose Timer, Sensor Controller with Digital and Analog Peripherals (AUX), Radio, and RTC. When the system CPU halts, the connected peripherals that have freeze enabled also halt. The programmable output can be set to static values of 0 or 1, and can also be set to pass the halted signal."]
    pub frzsel0: FRZSEL0,
    _reserved9: [u8; 1276usize],
    #[doc = "0xf00 - Set or Clear Software Events"]
    pub swev: SWEV,
}
#[doc = "Output Selection for CPU Interrupt 0"]
pub struct CPUIRQSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 0"]
pub mod cpuirqsel0;
#[doc = "Output Selection for CPU Interrupt 1"]
pub struct CPUIRQSEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 1"]
pub mod cpuirqsel1;
#[doc = "Output Selection for CPU Interrupt 2"]
pub struct CPUIRQSEL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 2"]
pub mod cpuirqsel2;
#[doc = "Output Selection for CPU Interrupt 3"]
pub struct CPUIRQSEL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 3"]
pub mod cpuirqsel3;
#[doc = "Output Selection for CPU Interrupt 4"]
pub struct CPUIRQSEL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 4"]
pub mod cpuirqsel4;
#[doc = "Output Selection for CPU Interrupt 5"]
pub struct CPUIRQSEL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 5"]
pub mod cpuirqsel5;
#[doc = "Output Selection for CPU Interrupt 6"]
pub struct CPUIRQSEL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 6"]
pub mod cpuirqsel6;
#[doc = "Output Selection for CPU Interrupt 7"]
pub struct CPUIRQSEL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 7"]
pub mod cpuirqsel7;
#[doc = "Output Selection for CPU Interrupt 8"]
pub struct CPUIRQSEL8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 8"]
pub mod cpuirqsel8;
#[doc = "Output Selection for CPU Interrupt 9"]
pub struct CPUIRQSEL9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 9"]
pub mod cpuirqsel9;
#[doc = "Output Selection for CPU Interrupt 10"]
pub struct CPUIRQSEL10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 10"]
pub mod cpuirqsel10;
#[doc = "Output Selection for CPU Interrupt 11"]
pub struct CPUIRQSEL11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 11"]
pub mod cpuirqsel11;
#[doc = "Output Selection for CPU Interrupt 12"]
pub struct CPUIRQSEL12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 12"]
pub mod cpuirqsel12;
#[doc = "Output Selection for CPU Interrupt 13"]
pub struct CPUIRQSEL13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 13"]
pub mod cpuirqsel13;
#[doc = "Output Selection for CPU Interrupt 14"]
pub struct CPUIRQSEL14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 14"]
pub mod cpuirqsel14;
#[doc = "Output Selection for CPU Interrupt 15"]
pub struct CPUIRQSEL15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 15"]
pub mod cpuirqsel15;
#[doc = "Output Selection for CPU Interrupt 16"]
pub struct CPUIRQSEL16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 16"]
pub mod cpuirqsel16;
#[doc = "Output Selection for CPU Interrupt 17"]
pub struct CPUIRQSEL17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 17"]
pub mod cpuirqsel17;
#[doc = "Output Selection for CPU Interrupt 18"]
pub struct CPUIRQSEL18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 18"]
pub mod cpuirqsel18;
#[doc = "Output Selection for CPU Interrupt 19"]
pub struct CPUIRQSEL19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 19"]
pub mod cpuirqsel19;
#[doc = "Output Selection for CPU Interrupt 20"]
pub struct CPUIRQSEL20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 20"]
pub mod cpuirqsel20;
#[doc = "Output Selection for CPU Interrupt 21"]
pub struct CPUIRQSEL21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 21"]
pub mod cpuirqsel21;
#[doc = "Output Selection for CPU Interrupt 22"]
pub struct CPUIRQSEL22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 22"]
pub mod cpuirqsel22;
#[doc = "Output Selection for CPU Interrupt 23"]
pub struct CPUIRQSEL23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 23"]
pub mod cpuirqsel23;
#[doc = "Output Selection for CPU Interrupt 24"]
pub struct CPUIRQSEL24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 24"]
pub mod cpuirqsel24;
#[doc = "Output Selection for CPU Interrupt 25"]
pub struct CPUIRQSEL25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 25"]
pub mod cpuirqsel25;
#[doc = "Output Selection for CPU Interrupt 26"]
pub struct CPUIRQSEL26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 26"]
pub mod cpuirqsel26;
#[doc = "Output Selection for CPU Interrupt 27"]
pub struct CPUIRQSEL27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 27"]
pub mod cpuirqsel27;
#[doc = "Output Selection for CPU Interrupt 28"]
pub struct CPUIRQSEL28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 28"]
pub mod cpuirqsel28;
#[doc = "Output Selection for CPU Interrupt 29"]
pub struct CPUIRQSEL29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 29"]
pub mod cpuirqsel29;
#[doc = "Output Selection for CPU Interrupt 30"]
pub struct CPUIRQSEL30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 30"]
pub mod cpuirqsel30;
#[doc = "Output Selection for CPU Interrupt 31"]
pub struct CPUIRQSEL31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 31"]
pub mod cpuirqsel31;
#[doc = "Output Selection for CPU Interrupt 32"]
pub struct CPUIRQSEL32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 32"]
pub mod cpuirqsel32;
#[doc = "Output Selection for CPU Interrupt 33"]
pub struct CPUIRQSEL33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for CPU Interrupt 33"]
pub mod cpuirqsel33;
#[doc = "Output Selection for RFC Event 0"]
pub struct RFCSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for RFC Event 0"]
pub mod rfcsel0;
#[doc = "Output Selection for RFC Event 1"]
pub struct RFCSEL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for RFC Event 1"]
pub mod rfcsel1;
#[doc = "Output Selection for RFC Event 2"]
pub struct RFCSEL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for RFC Event 2"]
pub mod rfcsel2;
#[doc = "Output Selection for RFC Event 3"]
pub struct RFCSEL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for RFC Event 3"]
pub mod rfcsel3;
#[doc = "Output Selection for RFC Event 4"]
pub struct RFCSEL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for RFC Event 4"]
pub mod rfcsel4;
#[doc = "Output Selection for RFC Event 5"]
pub struct RFCSEL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for RFC Event 5"]
pub mod rfcsel5;
#[doc = "Output Selection for RFC Event 6"]
pub struct RFCSEL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for RFC Event 6"]
pub mod rfcsel6;
#[doc = "Output Selection for RFC Event 7"]
pub struct RFCSEL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for RFC Event 7"]
pub mod rfcsel7;
#[doc = "Output Selection for RFC Event 8"]
pub struct RFCSEL8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for RFC Event 8"]
pub mod rfcsel8;
#[doc = "Output Selection for RFC Event 9"]
pub struct RFCSEL9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for RFC Event 9"]
pub mod rfcsel9;
#[doc = "Output Selection for GPT0 0"]
pub struct GPT0ACAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for GPT0 0"]
pub mod gpt0acaptsel;
#[doc = "Output Selection for GPT0 1"]
pub struct GPT0BCAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for GPT0 1"]
pub mod gpt0bcaptsel;
#[doc = "Output Selection for GPT1 0"]
pub struct GPT1ACAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for GPT1 0"]
pub mod gpt1acaptsel;
#[doc = "Output Selection for GPT1 1"]
pub struct GPT1BCAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for GPT1 1"]
pub mod gpt1bcaptsel;
#[doc = "Output Selection for GPT2 0"]
pub struct GPT2ACAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for GPT2 0"]
pub mod gpt2acaptsel;
#[doc = "Output Selection for GPT2 1"]
pub struct GPT2BCAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for GPT2 1"]
pub mod gpt2bcaptsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH0SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach0ssel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH0BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach0bsel;
#[doc = "Output Selection for DMA Channel 1 SREQ"]
pub struct UDMACH1SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 1 SREQ"]
pub mod udmach1ssel;
#[doc = "Output Selection for DMA Channel 1 REQ"]
pub struct UDMACH1BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 1 REQ"]
pub mod udmach1bsel;
#[doc = "Output Selection for DMA Channel 2 SREQ"]
pub struct UDMACH2SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 2 SREQ"]
pub mod udmach2ssel;
#[doc = "Output Selection for DMA Channel 2 REQ"]
pub struct UDMACH2BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 2 REQ"]
pub mod udmach2bsel;
#[doc = "Output Selection for DMA Channel 3 SREQ"]
pub struct UDMACH3SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 3 SREQ"]
pub mod udmach3ssel;
#[doc = "Output Selection for DMA Channel 3 REQ"]
pub struct UDMACH3BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 3 REQ"]
pub mod udmach3bsel;
#[doc = "Output Selection for DMA Channel 4 SREQ"]
pub struct UDMACH4SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 4 SREQ"]
pub mod udmach4ssel;
#[doc = "Output Selection for DMA Channel 4 REQ"]
pub struct UDMACH4BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 4 REQ"]
pub mod udmach4bsel;
#[doc = "Output Selection for DMA Channel 5 SREQ"]
pub struct UDMACH5SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 5 SREQ"]
pub mod udmach5ssel;
#[doc = "Output Selection for DMA Channel 5 REQ"]
pub struct UDMACH5BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 5 REQ"]
pub mod udmach5bsel;
#[doc = "Output Selection for DMA Channel 6 SREQ"]
pub struct UDMACH6SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 6 SREQ"]
pub mod udmach6ssel;
#[doc = "Output Selection for DMA Channel 6 REQ"]
pub struct UDMACH6BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 6 REQ"]
pub mod udmach6bsel;
#[doc = "Output Selection for DMA Channel 7 SREQ"]
pub struct UDMACH7SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 7 SREQ"]
pub mod udmach7ssel;
#[doc = "Output Selection for DMA Channel 7 REQ"]
pub struct UDMACH7BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 7 REQ"]
pub mod udmach7bsel;
#[doc = "Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel"]
pub struct UDMACH8SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel"]
pub mod udmach8ssel;
#[doc = "Output Selection for DMA Channel 8 REQ"]
pub struct UDMACH8BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 8 REQ"]
pub mod udmach8bsel;
#[doc = "Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
pub struct UDMACH9SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
pub mod udmach9ssel;
#[doc = "Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
pub struct UDMACH9BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
pub mod udmach9bsel;
#[doc = "Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
pub struct UDMACH10SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
pub mod udmach10ssel;
#[doc = "Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
pub struct UDMACH10BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
pub mod udmach10bsel;
#[doc = "Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
pub struct UDMACH11SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
pub mod udmach11ssel;
#[doc = "Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
pub struct UDMACH11BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
pub mod udmach11bsel;
#[doc = "Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
pub struct UDMACH12SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
pub mod udmach12ssel;
#[doc = "Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
pub struct UDMACH12BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
pub mod udmach12bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH13SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach13ssel;
#[doc = "Output Selection for DMA Channel 13 REQ"]
pub struct UDMACH13BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 13 REQ"]
pub mod udmach13bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH14SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach14ssel;
#[doc = "Output Selection for DMA Channel 14 REQ"]
pub struct UDMACH14BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 14 REQ"]
pub mod udmach14bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH15SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach15ssel;
#[doc = "Output Selection for DMA Channel 15 REQ"]
pub struct UDMACH15BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 15 REQ"]
pub mod udmach15bsel;
#[doc = "Output Selection for DMA Channel 16 SREQ"]
pub struct UDMACH16SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 16 SREQ"]
pub mod udmach16ssel;
#[doc = "Output Selection for DMA Channel 16 REQ"]
pub struct UDMACH16BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 16 REQ"]
pub mod udmach16bsel;
#[doc = "Output Selection for DMA Channel 17 SREQ"]
pub struct UDMACH17SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 17 SREQ"]
pub mod udmach17ssel;
#[doc = "Output Selection for DMA Channel 17 REQ"]
pub struct UDMACH17BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 17 REQ"]
pub mod udmach17bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH18SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach18ssel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH18BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach18bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH19SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach19ssel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH19BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach19bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH20SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach20ssel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH20BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach20bsel;
#[doc = "Output Selection for DMA Channel 21 SREQ"]
pub struct UDMACH21SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 21 SREQ"]
pub mod udmach21ssel;
#[doc = "Output Selection for DMA Channel 21 REQ"]
pub struct UDMACH21BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 21 REQ"]
pub mod udmach21bsel;
#[doc = "Output Selection for DMA Channel 22 SREQ"]
pub struct UDMACH22SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 22 SREQ"]
pub mod udmach22ssel;
#[doc = "Output Selection for DMA Channel 22 REQ"]
pub struct UDMACH22BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 22 REQ"]
pub mod udmach22bsel;
#[doc = "Output Selection for DMA Channel 23 SREQ"]
pub struct UDMACH23SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 23 SREQ"]
pub mod udmach23ssel;
#[doc = "Output Selection for DMA Channel 23 REQ"]
pub struct UDMACH23BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 23 REQ"]
pub mod udmach23bsel;
#[doc = "Output Selection for DMA Channel 24 SREQ"]
pub struct UDMACH24SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 24 SREQ"]
pub mod udmach24ssel;
#[doc = "Output Selection for DMA Channel 24 REQ"]
pub struct UDMACH24BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for DMA Channel 24 REQ"]
pub mod udmach24bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH25SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach25ssel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH25BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach25bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH26SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach26ssel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH26BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach26bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH27SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach27ssel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH27BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach27bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH28SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach28ssel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH28BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach28bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH29SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach29ssel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH29BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach29bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH30SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach30ssel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH30BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach30bsel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH31SSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach31ssel;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct UDMACH31BSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach31bsel;
#[doc = "Output Selection for GPT3 0"]
pub struct GPT3ACAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for GPT3 0"]
pub mod gpt3acaptsel;
#[doc = "Output Selection for GPT3 1"]
pub struct GPT3BCAPTSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for GPT3 1"]
pub mod gpt3bcaptsel;
#[doc = "Output Selection for AUX Subscriber 0"]
pub struct AUXSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for AUX Subscriber 0"]
pub mod auxsel0;
#[doc = "Output Selection for NMI Subscriber 0"]
pub struct CM3NMISEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for NMI Subscriber 0"]
pub mod cm3nmisel0;
#[doc = "Output Selection for I2S Subscriber 0"]
pub struct I2SSTMPSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for I2S Subscriber 0"]
pub mod i2sstmpsel0;
#[doc = "Output Selection for FRZ Subscriber The halted debug signal is passed to peripherals such as the General Purpose Timer, Sensor Controller with Digital and Analog Peripherals (AUX), Radio, and RTC. When the system CPU halts, the connected peripherals that have freeze enabled also halt. The programmable output can be set to static values of 0 or 1, and can also be set to pass the halted signal."]
pub struct FRZSEL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Output Selection for FRZ Subscriber The halted debug signal is passed to peripherals such as the General Purpose Timer, Sensor Controller with Digital and Analog Peripherals (AUX), Radio, and RTC. When the system CPU halts, the connected peripherals that have freeze enabled also halt. The programmable output can be set to static values of 0 or 1, and can also be set to pass the halted signal."]
pub mod frzsel0;
#[doc = "Set or Clear Software Events"]
pub struct SWEV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set or Clear Software Events"]
pub mod swev;
