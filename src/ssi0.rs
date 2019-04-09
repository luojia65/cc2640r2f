#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control 0"]
    pub cr0: CR0,
    #[doc = "0x04 - Control 1"]
    pub cr1: CR1,
    #[doc = "0x08 - Data 16-bits wide data register: When read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer. When written, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the TXD output pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer."]
    pub dr: DR,
    #[doc = "0x0c - Status"]
    pub sr: SR,
    #[doc = "0x10 - Clock Prescale"]
    pub cpsr: CPSR,
    #[doc = "0x14 - Interrupt Mask Set and Clear"]
    pub imsc: IMSC,
    #[doc = "0x18 - Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x1c - Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x20 - Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
    pub icr: ICR,
    #[doc = "0x24 - DMA Control"]
    pub dmacr: DMACR,
    #[doc = "0x28 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved1: RESERVED1,
    _reserved0: [u8; 100usize],
    #[doc = "0x90 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved2: RESERVED2,
}
#[doc = "Control 0"]
pub struct CR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control 0"]
pub mod cr0;
#[doc = "Control 1"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control 1"]
pub mod cr1;
#[doc = "Data 16-bits wide data register: When read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer. When written, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the TXD output pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer."]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data 16-bits wide data register: When read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer. When written, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the TXD output pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer."]
pub mod dr;
#[doc = "Status"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status"]
pub mod sr;
#[doc = "Clock Prescale"]
pub struct CPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Prescale"]
pub mod cpsr;
#[doc = "Interrupt Mask Set and Clear"]
pub struct IMSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Set and Clear"]
pub mod imsc;
#[doc = "Raw Interrupt Status"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "Masked Interrupt Status"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked Interrupt Status"]
pub mod mis;
#[doc = "Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
pub mod icr;
#[doc = "DMA Control"]
pub struct DMACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Control"]
pub mod dmacr;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved1;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved2;
