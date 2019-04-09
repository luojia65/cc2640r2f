#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data For words to be transmitted: - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO). The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted. For received words: - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register."]
    pub dr: DR,
    #[doc = "0x04 - Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs."]
    pub rsr: RSR,
    #[doc = "0x08 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved0: RESERVED0,
    _reserved0: [u8; 12usize],
    #[doc = "0x18 - Flag Reads from this register return the UART flags."]
    pub fr: FR,
    #[doc = "0x1c - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved2: RESERVED2,
    _reserved1: [u8; 4usize],
    #[doc = "0x24 - Integer Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
    pub ibrd: IBRD,
    #[doc = "0x28 - Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
    pub fbrd: FBRD,
    #[doc = "0x2c - Line Control"]
    pub lcrh: LCRH,
    #[doc = "0x30 - Control"]
    pub ctl: CTL,
    #[doc = "0x34 - Interrupt FIFO Level Select"]
    pub ifls: IFLS,
    #[doc = "0x38 - Interrupt Mask Set/Clear"]
    pub imsc: IMSC,
    #[doc = "0x3c - Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x40 - Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x44 - Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
    pub icr: ICR,
    #[doc = "0x48 - DMA Control"]
    pub dmactl: DMACTL,
    #[doc = "0x4c - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved1: RESERVED1,
    _reserved2: [u8; 64usize],
    #[doc = "0x90 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved3: RESERVED3,
    _reserved3: [u8; 3900usize],
    #[doc = "0xfd0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved4: RESERVED4,
}
#[doc = "Data For words to be transmitted: - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO). The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted. For received words: - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register."]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data For words to be transmitted: - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO). The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted. For received words: - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register."]
pub mod dr;
#[doc = "Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs."]
pub struct RSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs."]
pub mod rsr;
#[doc = "Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors)."]
pub struct ECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors)."]
pub mod ecr;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved0;
#[doc = "Flag Reads from this register return the UART flags."]
pub struct FR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flag Reads from this register return the UART flags."]
pub mod fr;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved2;
#[doc = "Integer Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
pub struct IBRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Integer Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
pub mod ibrd;
#[doc = "Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
pub struct FBRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
pub mod fbrd;
#[doc = "Line Control"]
pub struct LCRH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Control"]
pub mod lcrh;
#[doc = "Control"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control"]
pub mod ctl;
#[doc = "Interrupt FIFO Level Select"]
pub struct IFLS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt FIFO Level Select"]
pub mod ifls;
#[doc = "Interrupt Mask Set/Clear"]
pub struct IMSC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Set/Clear"]
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
pub struct DMACTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Control"]
pub mod dmactl;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved1;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved3;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved4;
