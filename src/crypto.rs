#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Channel 0 Control"]
    pub dmach0ctl: DMACH0CTL,
    #[doc = "0x04 - DMA Channel 0 External Address"]
    pub dmach0extaddr: DMACH0EXTADDR,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - DMA Channel 0 Length"]
    pub dmach0len: DMACH0LEN,
    _reserved1: [u8; 8usize],
    #[doc = "0x18 - DMA Controller Status"]
    pub dmastat: DMASTAT,
    #[doc = "0x1c - DMA Controller Software Reset"]
    pub dmaswreset: DMASWRESET,
    #[doc = "0x20 - DMA Channel 1 Control"]
    pub dmach1ctl: DMACH1CTL,
    #[doc = "0x24 - DMA Channel 1 External Address"]
    pub dmach1extaddr: DMACH1EXTADDR,
    _reserved2: [u8; 4usize],
    #[doc = "0x2c - DMA Channel 1 Length"]
    pub dmach1len: DMACH1LEN,
    _reserved3: [u8; 72usize],
    #[doc = "0x78 - DMA Controller Master Configuration"]
    pub dmabuscfg: DMABUSCFG,
    #[doc = "0x7c - DMA Controller Port Error"]
    pub dmaporterr: DMAPORTERR,
    _reserved4: [u8; 124usize],
    #[doc = "0xfc - DMA Controller Version"]
    pub dmahwver: DMAHWVER,
    _reserved5: [u8; 768usize],
    #[doc = "0x400 - Key Write Area"]
    pub keywritearea: KEYWRITEAREA,
    #[doc = "0x404 - Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error."]
    pub keywrittenarea: KEYWRITTENAREA,
    #[doc = "0x408 - Key Size This register defines the size of the keys that are written with DMA."]
    pub keysize: KEYSIZE,
    #[doc = "0x40c - Key Read Area"]
    pub keyreadarea: KEYREADAREA,
    _reserved6: [u8; 240usize],
    #[doc = "0x500 - Clear AES_KEY2/GHASH Key"]
    pub aeskey2: AESKEY2,
    _reserved7: [u8; 12usize],
    #[doc = "0x510 - Clear AES_KEY3"]
    pub aeskey3: AESKEY3,
    _reserved8: [u8; 44usize],
    #[doc = "0x540 - AES Initialization Vector"]
    pub aesiv: AESIV,
    _reserved9: [u8; 12usize],
    #[doc = "0x550 - AES Input/Output Buffer Control"]
    pub aesctl: AESCTL,
    #[doc = "0x554 - Crypto Data Length LSW"]
    pub aesdatalen0: AESDATALEN0,
    #[doc = "0x558 - Crypto Data Length MSW"]
    pub aesdatalen1: AESDATALEN1,
    #[doc = "0x55c - AES Authentication Length"]
    pub aesauthlen: AESAUTHLEN,
    #[doc = "0x560 - Data Input/Output"]
    pub aesdataout0: AESDATAOUT0,
    #[doc = "0x564 - AES Data Input/Output 3"]
    pub aesdataout1: AESDATAOUT1,
    #[doc = "0x568 - AES Data Input/Output 2"]
    pub aesdataout2: AESDATAOUT2,
    #[doc = "0x56c - AES Data Input/Output 3"]
    pub aesdataout3: AESDATAOUT3,
    #[doc = "0x570 - AES Tag Output"]
    pub aestagout: AESTAGOUT,
    _reserved10: [u8; 396usize],
    #[doc = "0x700 - Master Algorithm Select This register configures the internal destination of the DMA controller."]
    pub algsel: ALGSEL,
    #[doc = "0x704 - Master Protection Control"]
    pub dmaprotctl: DMAPROTCTL,
    _reserved11: [u8; 56usize],
    #[doc = "0x740 - Software Reset"]
    pub swreset: SWRESET,
    _reserved12: [u8; 60usize],
    #[doc = "0x780 - Control Interrupt Configuration"]
    pub irqtype: IRQTYPE,
    #[doc = "0x784 - Interrupt Enable"]
    pub irqen: IRQEN,
    #[doc = "0x788 - Interrupt Clear"]
    pub irqclr: IRQCLR,
    #[doc = "0x78c - Interrupt Set"]
    pub irqset: IRQSET,
    #[doc = "0x790 - Interrupt Status"]
    pub irqstat: IRQSTAT,
    _reserved13: [u8; 104usize],
    #[doc = "0x7fc - CTRL Module Version"]
    pub hwver: HWVER,
}
#[doc = "DMA Channel 0 Control"]
pub struct DMACH0CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel 0 Control"]
pub mod dmach0ctl;
#[doc = "DMA Channel 0 External Address"]
pub struct DMACH0EXTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel 0 External Address"]
pub mod dmach0extaddr;
#[doc = "DMA Channel 0 Length"]
pub struct DMACH0LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel 0 Length"]
pub mod dmach0len;
#[doc = "DMA Controller Status"]
pub struct DMASTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Controller Status"]
pub mod dmastat;
#[doc = "DMA Controller Software Reset"]
pub struct DMASWRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Controller Software Reset"]
pub mod dmaswreset;
#[doc = "DMA Channel 1 Control"]
pub struct DMACH1CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel 1 Control"]
pub mod dmach1ctl;
#[doc = "DMA Channel 1 External Address"]
pub struct DMACH1EXTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel 1 External Address"]
pub mod dmach1extaddr;
#[doc = "DMA Channel 1 Length"]
pub struct DMACH1LEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel 1 Length"]
pub mod dmach1len;
#[doc = "DMA Controller Master Configuration"]
pub struct DMABUSCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Controller Master Configuration"]
pub mod dmabuscfg;
#[doc = "DMA Controller Port Error"]
pub struct DMAPORTERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Controller Port Error"]
pub mod dmaporterr;
#[doc = "DMA Controller Version"]
pub struct DMAHWVER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Controller Version"]
pub mod dmahwver;
#[doc = "Key Write Area"]
pub struct KEYWRITEAREA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Write Area"]
pub mod keywritearea;
#[doc = "Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error."]
pub struct KEYWRITTENAREA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error."]
pub mod keywrittenarea;
#[doc = "Key Size This register defines the size of the keys that are written with DMA."]
pub struct KEYSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Size This register defines the size of the keys that are written with DMA."]
pub mod keysize;
#[doc = "Key Read Area"]
pub struct KEYREADAREA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key Read Area"]
pub mod keyreadarea;
#[doc = "Clear AES_KEY2/GHASH Key"]
pub struct AESKEY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear AES_KEY2/GHASH Key"]
pub mod aeskey2;
#[doc = "Clear AES_KEY3"]
pub struct AESKEY3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear AES_KEY3"]
pub mod aeskey3;
#[doc = "AES Initialization Vector"]
pub struct AESIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Initialization Vector"]
pub mod aesiv;
#[doc = "AES Input/Output Buffer Control"]
pub struct AESCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Input/Output Buffer Control"]
pub mod aesctl;
#[doc = "Crypto Data Length LSW"]
pub struct AESDATALEN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Crypto Data Length LSW"]
pub mod aesdatalen0;
#[doc = "Crypto Data Length MSW"]
pub struct AESDATALEN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Crypto Data Length MSW"]
pub mod aesdatalen1;
#[doc = "AES Authentication Length"]
pub struct AESAUTHLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Authentication Length"]
pub mod aesauthlen;
#[doc = "Data Input/Output"]
pub struct AESDATAOUT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Input/Output"]
pub mod aesdataout0;
#[doc = "AES Data Input/Output 0"]
pub struct AESDATAIN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Data Input/Output 0"]
pub mod aesdatain0;
#[doc = "AES Data Input/Output 3"]
pub struct AESDATAOUT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Data Input/Output 3"]
pub mod aesdataout1;
#[doc = "AES Data Input/Output 1"]
pub struct AESDATAIN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Data Input/Output 1"]
pub mod aesdatain1;
#[doc = "AES Data Input/Output 2"]
pub struct AESDATAOUT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Data Input/Output 2"]
pub mod aesdataout2;
#[doc = "AES Data Input/Output 2"]
pub struct AESDATAIN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Data Input/Output 2"]
pub mod aesdatain2;
#[doc = "AES Data Input/Output 3"]
pub struct AESDATAOUT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Data Input/Output 3"]
pub mod aesdataout3;
#[doc = "Data Input/Output"]
pub struct AESDATAIN3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Input/Output"]
pub mod aesdatain3;
#[doc = "AES Tag Output"]
pub struct AESTAGOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Tag Output"]
pub mod aestagout;
#[doc = "Master Algorithm Select This register configures the internal destination of the DMA controller."]
pub struct ALGSEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Algorithm Select This register configures the internal destination of the DMA controller."]
pub mod algsel;
#[doc = "Master Protection Control"]
pub struct DMAPROTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Protection Control"]
pub mod dmaprotctl;
#[doc = "Software Reset"]
pub struct SWRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Reset"]
pub mod swreset;
#[doc = "Control Interrupt Configuration"]
pub struct IRQTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Interrupt Configuration"]
pub mod irqtype;
#[doc = "Interrupt Enable"]
pub struct IRQEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable"]
pub mod irqen;
#[doc = "Interrupt Clear"]
pub struct IRQCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear"]
pub mod irqclr;
#[doc = "Interrupt Set"]
pub struct IRQSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Set"]
pub mod irqset;
#[doc = "Interrupt Status"]
pub struct IRQSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status"]
pub mod irqstat;
#[doc = "CTRL Module Version"]
pub struct HWVER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTRL Module Version"]
pub mod hwver;
