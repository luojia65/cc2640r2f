#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub ctl: CTL,
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    pub meascfg: MEASCFG,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - Internal. Only to be used through TI provided API."]
    pub tempp0: TEMPP0,
    #[doc = "0x10 - Internal. Only to be used through TI provided API."]
    pub tempp1: TEMPP1,
    #[doc = "0x14 - Internal. Only to be used through TI provided API."]
    pub tempp2: TEMPP2,
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    pub batmonp0: BATMONP0,
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    pub batmonp1: BATMONP1,
    #[doc = "0x20 - Internal. Only to be used through TI provided API."]
    pub iostrp0: IOSTRP0,
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    pub flashpumpp0: FLASHPUMPP0,
    #[doc = "0x28 - Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1"]
    pub bat: BAT,
    #[doc = "0x2c - Battery Update Indicates BAT Updates"]
    pub batupd: BATUPD,
    #[doc = "0x30 - Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1."]
    pub temp: TEMP,
    #[doc = "0x34 - Temperature Update Indicates TEMP Updates"]
    pub tempupd: TEMPUPD,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ctl;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct MEASCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod meascfg;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct TEMPP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tempp0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct TEMPP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tempp1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct TEMPP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tempp2;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct BATMONP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod batmonp0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct BATMONP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod batmonp1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct IOSTRP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrp0;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct FLASHPUMPP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flashpumpp0;
#[doc = "Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1"]
pub struct BAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1"]
pub mod bat;
#[doc = "Battery Update Indicates BAT Updates"]
pub struct BATUPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Battery Update Indicates BAT Updates"]
pub mod batupd;
#[doc = "Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1."]
pub struct TEMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1."]
pub mod temp;
#[doc = "Temperature Update Indicates TEMP Updates"]
pub struct TEMPUPD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Temperature Update Indicates TEMP Updates"]
pub mod tempupd;
