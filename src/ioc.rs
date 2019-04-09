#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration of DIO0"]
    pub iocfg0: IOCFG0,
    #[doc = "0x04 - Configuration of DIO1"]
    pub iocfg1: IOCFG1,
    #[doc = "0x08 - Configuration of DIO2"]
    pub iocfg2: IOCFG2,
    #[doc = "0x0c - Configuration of DIO3"]
    pub iocfg3: IOCFG3,
    #[doc = "0x10 - Configuration of DIO4"]
    pub iocfg4: IOCFG4,
    #[doc = "0x14 - Configuration of DIO5"]
    pub iocfg5: IOCFG5,
    #[doc = "0x18 - Configuration of DIO6"]
    pub iocfg6: IOCFG6,
    #[doc = "0x1c - Configuration of DIO7"]
    pub iocfg7: IOCFG7,
    #[doc = "0x20 - Configuration of DIO8"]
    pub iocfg8: IOCFG8,
    #[doc = "0x24 - Configuration of DIO9"]
    pub iocfg9: IOCFG9,
    #[doc = "0x28 - Configuration of DIO10"]
    pub iocfg10: IOCFG10,
    #[doc = "0x2c - Configuration of DIO11"]
    pub iocfg11: IOCFG11,
    #[doc = "0x30 - Configuration of DIO12"]
    pub iocfg12: IOCFG12,
    #[doc = "0x34 - Configuration of DIO13"]
    pub iocfg13: IOCFG13,
    #[doc = "0x38 - Configuration of DIO14"]
    pub iocfg14: IOCFG14,
    #[doc = "0x3c - Configuration of DIO15"]
    pub iocfg15: IOCFG15,
    #[doc = "0x40 - Configuration of DIO16"]
    pub iocfg16: IOCFG16,
    #[doc = "0x44 - Configuration of DIO17"]
    pub iocfg17: IOCFG17,
    #[doc = "0x48 - Configuration of DIO18"]
    pub iocfg18: IOCFG18,
    #[doc = "0x4c - Configuration of DIO19"]
    pub iocfg19: IOCFG19,
    #[doc = "0x50 - Configuration of DIO20"]
    pub iocfg20: IOCFG20,
    #[doc = "0x54 - Configuration of DIO21"]
    pub iocfg21: IOCFG21,
    #[doc = "0x58 - Configuration of DIO22"]
    pub iocfg22: IOCFG22,
    #[doc = "0x5c - Configuration of DIO23"]
    pub iocfg23: IOCFG23,
    #[doc = "0x60 - Configuration of DIO24"]
    pub iocfg24: IOCFG24,
    #[doc = "0x64 - Configuration of DIO25"]
    pub iocfg25: IOCFG25,
    #[doc = "0x68 - Configuration of DIO26"]
    pub iocfg26: IOCFG26,
    #[doc = "0x6c - Configuration of DIO27"]
    pub iocfg27: IOCFG27,
    #[doc = "0x70 - Configuration of DIO28"]
    pub iocfg28: IOCFG28,
    #[doc = "0x74 - Configuration of DIO29"]
    pub iocfg29: IOCFG29,
    #[doc = "0x78 - Configuration of DIO30"]
    pub iocfg30: IOCFG30,
    #[doc = "0x7c - Configuration of DIO31"]
    pub iocfg31: IOCFG31,
}
#[doc = "Configuration of DIO0"]
pub struct IOCFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO0"]
pub mod iocfg0;
#[doc = "Configuration of DIO1"]
pub struct IOCFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO1"]
pub mod iocfg1;
#[doc = "Configuration of DIO2"]
pub struct IOCFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO2"]
pub mod iocfg2;
#[doc = "Configuration of DIO3"]
pub struct IOCFG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO3"]
pub mod iocfg3;
#[doc = "Configuration of DIO4"]
pub struct IOCFG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO4"]
pub mod iocfg4;
#[doc = "Configuration of DIO5"]
pub struct IOCFG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO5"]
pub mod iocfg5;
#[doc = "Configuration of DIO6"]
pub struct IOCFG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO6"]
pub mod iocfg6;
#[doc = "Configuration of DIO7"]
pub struct IOCFG7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO7"]
pub mod iocfg7;
#[doc = "Configuration of DIO8"]
pub struct IOCFG8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO8"]
pub mod iocfg8;
#[doc = "Configuration of DIO9"]
pub struct IOCFG9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO9"]
pub mod iocfg9;
#[doc = "Configuration of DIO10"]
pub struct IOCFG10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO10"]
pub mod iocfg10;
#[doc = "Configuration of DIO11"]
pub struct IOCFG11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO11"]
pub mod iocfg11;
#[doc = "Configuration of DIO12"]
pub struct IOCFG12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO12"]
pub mod iocfg12;
#[doc = "Configuration of DIO13"]
pub struct IOCFG13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO13"]
pub mod iocfg13;
#[doc = "Configuration of DIO14"]
pub struct IOCFG14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO14"]
pub mod iocfg14;
#[doc = "Configuration of DIO15"]
pub struct IOCFG15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO15"]
pub mod iocfg15;
#[doc = "Configuration of DIO16"]
pub struct IOCFG16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO16"]
pub mod iocfg16;
#[doc = "Configuration of DIO17"]
pub struct IOCFG17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO17"]
pub mod iocfg17;
#[doc = "Configuration of DIO18"]
pub struct IOCFG18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO18"]
pub mod iocfg18;
#[doc = "Configuration of DIO19"]
pub struct IOCFG19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO19"]
pub mod iocfg19;
#[doc = "Configuration of DIO20"]
pub struct IOCFG20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO20"]
pub mod iocfg20;
#[doc = "Configuration of DIO21"]
pub struct IOCFG21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO21"]
pub mod iocfg21;
#[doc = "Configuration of DIO22"]
pub struct IOCFG22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO22"]
pub mod iocfg22;
#[doc = "Configuration of DIO23"]
pub struct IOCFG23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO23"]
pub mod iocfg23;
#[doc = "Configuration of DIO24"]
pub struct IOCFG24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO24"]
pub mod iocfg24;
#[doc = "Configuration of DIO25"]
pub struct IOCFG25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO25"]
pub mod iocfg25;
#[doc = "Configuration of DIO26"]
pub struct IOCFG26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO26"]
pub mod iocfg26;
#[doc = "Configuration of DIO27"]
pub struct IOCFG27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO27"]
pub mod iocfg27;
#[doc = "Configuration of DIO28"]
pub struct IOCFG28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO28"]
pub mod iocfg28;
#[doc = "Configuration of DIO29"]
pub struct IOCFG29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO29"]
pub mod iocfg29;
#[doc = "Configuration of DIO30"]
pub struct IOCFG30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO30"]
pub mod iocfg30;
#[doc = "Configuration of DIO31"]
pub struct IOCFG31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of DIO31"]
pub mod iocfg31;
