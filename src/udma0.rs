#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status"]
    pub status: STATUS,
    #[doc = "0x04 - Configuration"]
    pub cfg: CFG,
    #[doc = "0x08 - Channel Control Data Base Pointer"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Channel Alternate Control Data Base Pointer"]
    pub altctrl: ALTCTRL,
    #[doc = "0x10 - Channel Wait On Request Status"]
    pub waitonreq: WAITONREQ,
    #[doc = "0x14 - Channel Software Request"]
    pub softreq: SOFTREQ,
    #[doc = "0x18 - Channel Set UseBurst"]
    pub setburst: SETBURST,
    #[doc = "0x1c - Channel Clear UseBurst"]
    pub clearburst: CLEARBURST,
    #[doc = "0x20 - Channel Set Request Mask"]
    pub setreqmask: SETREQMASK,
    #[doc = "0x24 - Clear Channel Request Mask"]
    pub clearreqmask: CLEARREQMASK,
    #[doc = "0x28 - Set Channel Enable"]
    pub setchannelen: SETCHANNELEN,
    #[doc = "0x2c - Clear Channel Enable"]
    pub clearchannelen: CLEARCHANNELEN,
    #[doc = "0x30 - Channel Set Primary-Alternate"]
    pub setchnlprialt: SETCHNLPRIALT,
    #[doc = "0x34 - Channel Clear Primary-Alternate"]
    pub clearchnlprialt: CLEARCHNLPRIALT,
    #[doc = "0x38 - Set Channel Priority"]
    pub setchnlpriority: SETCHNLPRIORITY,
    #[doc = "0x3c - Clear Channel Priority"]
    pub clearchnlpriority: CLEARCHNLPRIORITY,
    _reserved0: [u8; 12usize],
    #[doc = "0x4c - Error Status and Clear"]
    pub error: ERROR,
    _reserved1: [u8; 1204usize],
    #[doc = "0x504 - Channel Request Done"]
    pub reqdone: REQDONE,
    _reserved2: [u8; 24usize],
    #[doc = "0x520 - Channel Request Done Mask"]
    pub donemask: DONEMASK,
}
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Configuration"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration"]
pub mod cfg;
#[doc = "Channel Control Data Base Pointer"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Control Data Base Pointer"]
pub mod ctrl;
#[doc = "Channel Alternate Control Data Base Pointer"]
pub struct ALTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Alternate Control Data Base Pointer"]
pub mod altctrl;
#[doc = "Channel Wait On Request Status"]
pub struct WAITONREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Wait On Request Status"]
pub mod waitonreq;
#[doc = "Channel Software Request"]
pub struct SOFTREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Software Request"]
pub mod softreq;
#[doc = "Channel Set UseBurst"]
pub struct SETBURST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Set UseBurst"]
pub mod setburst;
#[doc = "Channel Clear UseBurst"]
pub struct CLEARBURST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Clear UseBurst"]
pub mod clearburst;
#[doc = "Channel Set Request Mask"]
pub struct SETREQMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Set Request Mask"]
pub mod setreqmask;
#[doc = "Clear Channel Request Mask"]
pub struct CLEARREQMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Channel Request Mask"]
pub mod clearreqmask;
#[doc = "Set Channel Enable"]
pub struct SETCHANNELEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set Channel Enable"]
pub mod setchannelen;
#[doc = "Clear Channel Enable"]
pub struct CLEARCHANNELEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Channel Enable"]
pub mod clearchannelen;
#[doc = "Channel Set Primary-Alternate"]
pub struct SETCHNLPRIALT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Set Primary-Alternate"]
pub mod setchnlprialt;
#[doc = "Channel Clear Primary-Alternate"]
pub struct CLEARCHNLPRIALT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Clear Primary-Alternate"]
pub mod clearchnlprialt;
#[doc = "Set Channel Priority"]
pub struct SETCHNLPRIORITY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set Channel Priority"]
pub mod setchnlpriority;
#[doc = "Clear Channel Priority"]
pub struct CLEARCHNLPRIORITY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Channel Priority"]
pub mod clearchnlpriority;
#[doc = "Error Status and Clear"]
pub struct ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Status and Clear"]
pub mod error;
#[doc = "Channel Request Done"]
pub struct REQDONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Request Done"]
pub mod reqdone;
#[doc = "Channel Request Done Mask"]
pub struct DONEMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Request Done Mask"]
pub mod donemask;
