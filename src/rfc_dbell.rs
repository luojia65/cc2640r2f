#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Doorbell Command Register"]
    pub cmdr: CMDR,
    #[doc = "0x04 - Doorbell Command Status Register"]
    pub cmdsta: CMDSTA,
    #[doc = "0x08 - Interrupt Flags From RF Hardware Modules"]
    pub rfhwifg: RFHWIFG,
    #[doc = "0x0c - Interrupt Enable For RF Hardware Modules"]
    pub rfhwien: RFHWIEN,
    #[doc = "0x10 - Interrupt Flags For Command and Packet Engine Generated Interrupts"]
    pub rfcpeifg: RFCPEIFG,
    #[doc = "0x14 - Interrupt Enable For Command and Packet Engine Generated Interrupts"]
    pub rfcpeien: RFCPEIEN,
    #[doc = "0x18 - Interrupt Vector Selection For Command and Packet Engine Generated Interrupts"]
    pub rfcpeisl: RFCPEISL,
    #[doc = "0x1c - Doorbell Command Acknowledgement Interrupt Flag"]
    pub rfackifg: RFACKIFG,
    #[doc = "0x20 - RF Core General Purpose Output Control"]
    pub sysgpoctl: SYSGPOCTL,
}
#[doc = "Doorbell Command Register"]
pub struct CMDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Doorbell Command Register"]
pub mod cmdr;
#[doc = "Doorbell Command Status Register"]
pub struct CMDSTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Doorbell Command Status Register"]
pub mod cmdsta;
#[doc = "Interrupt Flags From RF Hardware Modules"]
pub struct RFHWIFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flags From RF Hardware Modules"]
pub mod rfhwifg;
#[doc = "Interrupt Enable For RF Hardware Modules"]
pub struct RFHWIEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable For RF Hardware Modules"]
pub mod rfhwien;
#[doc = "Interrupt Flags For Command and Packet Engine Generated Interrupts"]
pub struct RFCPEIFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flags For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeifg;
#[doc = "Interrupt Enable For Command and Packet Engine Generated Interrupts"]
pub struct RFCPEIEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeien;
#[doc = "Interrupt Vector Selection For Command and Packet Engine Generated Interrupts"]
pub struct RFCPEISL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Vector Selection For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeisl;
#[doc = "Doorbell Command Acknowledgement Interrupt Flag"]
pub struct RFACKIFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Doorbell Command Acknowledgement Interrupt Flag"]
pub mod rfackifg;
#[doc = "RF Core General Purpose Output Control"]
pub struct SYSGPOCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RF Core General Purpose Output Control"]
pub mod sysgpoctl;
