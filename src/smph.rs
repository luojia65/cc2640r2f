#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU SEMAPHORE 0"]
    pub smph0: SMPH0,
    #[doc = "0x04 - MCU SEMAPHORE 1"]
    pub smph1: SMPH1,
    #[doc = "0x08 - MCU SEMAPHORE 2"]
    pub smph2: SMPH2,
    #[doc = "0x0c - MCU SEMAPHORE 3"]
    pub smph3: SMPH3,
    #[doc = "0x10 - MCU SEMAPHORE 4"]
    pub smph4: SMPH4,
    #[doc = "0x14 - MCU SEMAPHORE 5"]
    pub smph5: SMPH5,
    #[doc = "0x18 - MCU SEMAPHORE 6"]
    pub smph6: SMPH6,
    #[doc = "0x1c - MCU SEMAPHORE 7"]
    pub smph7: SMPH7,
    #[doc = "0x20 - MCU SEMAPHORE 8"]
    pub smph8: SMPH8,
    #[doc = "0x24 - MCU SEMAPHORE 9"]
    pub smph9: SMPH9,
    #[doc = "0x28 - MCU SEMAPHORE 10"]
    pub smph10: SMPH10,
    #[doc = "0x2c - MCU SEMAPHORE 11"]
    pub smph11: SMPH11,
    #[doc = "0x30 - MCU SEMAPHORE 12"]
    pub smph12: SMPH12,
    #[doc = "0x34 - MCU SEMAPHORE 13"]
    pub smph13: SMPH13,
    #[doc = "0x38 - MCU SEMAPHORE 14"]
    pub smph14: SMPH14,
    #[doc = "0x3c - MCU SEMAPHORE 15"]
    pub smph15: SMPH15,
    #[doc = "0x40 - MCU SEMAPHORE 16"]
    pub smph16: SMPH16,
    #[doc = "0x44 - MCU SEMAPHORE 17"]
    pub smph17: SMPH17,
    #[doc = "0x48 - MCU SEMAPHORE 18"]
    pub smph18: SMPH18,
    #[doc = "0x4c - MCU SEMAPHORE 19"]
    pub smph19: SMPH19,
    #[doc = "0x50 - MCU SEMAPHORE 20"]
    pub smph20: SMPH20,
    #[doc = "0x54 - MCU SEMAPHORE 21"]
    pub smph21: SMPH21,
    #[doc = "0x58 - MCU SEMAPHORE 22"]
    pub smph22: SMPH22,
    #[doc = "0x5c - MCU SEMAPHORE 23"]
    pub smph23: SMPH23,
    #[doc = "0x60 - MCU SEMAPHORE 24"]
    pub smph24: SMPH24,
    #[doc = "0x64 - MCU SEMAPHORE 25"]
    pub smph25: SMPH25,
    #[doc = "0x68 - MCU SEMAPHORE 26"]
    pub smph26: SMPH26,
    #[doc = "0x6c - MCU SEMAPHORE 27"]
    pub smph27: SMPH27,
    #[doc = "0x70 - MCU SEMAPHORE 28"]
    pub smph28: SMPH28,
    #[doc = "0x74 - MCU SEMAPHORE 29"]
    pub smph29: SMPH29,
    #[doc = "0x78 - MCU SEMAPHORE 30"]
    pub smph30: SMPH30,
    #[doc = "0x7c - MCU SEMAPHORE 31"]
    pub smph31: SMPH31,
    _reserved0: [u8; 1920usize],
    #[doc = "0x800 - MCU SEMAPHORE 0 ALIAS"]
    pub peek0: PEEK0,
    #[doc = "0x804 - MCU SEMAPHORE 1 ALIAS"]
    pub peek1: PEEK1,
    #[doc = "0x808 - MCU SEMAPHORE 2 ALIAS"]
    pub peek2: PEEK2,
    #[doc = "0x80c - MCU SEMAPHORE 3 ALIAS"]
    pub peek3: PEEK3,
    #[doc = "0x810 - MCU SEMAPHORE 4 ALIAS"]
    pub peek4: PEEK4,
    #[doc = "0x814 - MCU SEMAPHORE 5 ALIAS"]
    pub peek5: PEEK5,
    #[doc = "0x818 - MCU SEMAPHORE 6 ALIAS"]
    pub peek6: PEEK6,
    #[doc = "0x81c - MCU SEMAPHORE 7 ALIAS"]
    pub peek7: PEEK7,
    #[doc = "0x820 - MCU SEMAPHORE 8 ALIAS"]
    pub peek8: PEEK8,
    #[doc = "0x824 - MCU SEMAPHORE 9 ALIAS"]
    pub peek9: PEEK9,
    #[doc = "0x828 - MCU SEMAPHORE 10 ALIAS"]
    pub peek10: PEEK10,
    #[doc = "0x82c - MCU SEMAPHORE 11 ALIAS"]
    pub peek11: PEEK11,
    #[doc = "0x830 - MCU SEMAPHORE 12 ALIAS"]
    pub peek12: PEEK12,
    #[doc = "0x834 - MCU SEMAPHORE 13 ALIAS"]
    pub peek13: PEEK13,
    #[doc = "0x838 - MCU SEMAPHORE 14 ALIAS"]
    pub peek14: PEEK14,
    #[doc = "0x83c - MCU SEMAPHORE 15 ALIAS"]
    pub peek15: PEEK15,
    #[doc = "0x840 - MCU SEMAPHORE 16 ALIAS"]
    pub peek16: PEEK16,
    #[doc = "0x844 - MCU SEMAPHORE 17 ALIAS"]
    pub peek17: PEEK17,
    #[doc = "0x848 - MCU SEMAPHORE 18 ALIAS"]
    pub peek18: PEEK18,
    #[doc = "0x84c - MCU SEMAPHORE 19 ALIAS"]
    pub peek19: PEEK19,
    #[doc = "0x850 - MCU SEMAPHORE 20 ALIAS"]
    pub peek20: PEEK20,
    #[doc = "0x854 - MCU SEMAPHORE 21 ALIAS"]
    pub peek21: PEEK21,
    #[doc = "0x858 - MCU SEMAPHORE 22 ALIAS"]
    pub peek22: PEEK22,
    #[doc = "0x85c - MCU SEMAPHORE 23 ALIAS"]
    pub peek23: PEEK23,
    #[doc = "0x860 - MCU SEMAPHORE 24 ALIAS"]
    pub peek24: PEEK24,
    #[doc = "0x864 - MCU SEMAPHORE 25 ALIAS"]
    pub peek25: PEEK25,
    #[doc = "0x868 - MCU SEMAPHORE 26 ALIAS"]
    pub peek26: PEEK26,
    #[doc = "0x86c - MCU SEMAPHORE 27 ALIAS"]
    pub peek27: PEEK27,
    #[doc = "0x870 - MCU SEMAPHORE 28 ALIAS"]
    pub peek28: PEEK28,
    #[doc = "0x874 - MCU SEMAPHORE 29 ALIAS"]
    pub peek29: PEEK29,
    #[doc = "0x878 - MCU SEMAPHORE 30 ALIAS"]
    pub peek30: PEEK30,
    #[doc = "0x87c - MCU SEMAPHORE 31 ALIAS"]
    pub peek31: PEEK31,
}
#[doc = "MCU SEMAPHORE 0"]
pub struct SMPH0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 0"]
pub mod smph0;
#[doc = "MCU SEMAPHORE 1"]
pub struct SMPH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 1"]
pub mod smph1;
#[doc = "MCU SEMAPHORE 2"]
pub struct SMPH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 2"]
pub mod smph2;
#[doc = "MCU SEMAPHORE 3"]
pub struct SMPH3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 3"]
pub mod smph3;
#[doc = "MCU SEMAPHORE 4"]
pub struct SMPH4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 4"]
pub mod smph4;
#[doc = "MCU SEMAPHORE 5"]
pub struct SMPH5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 5"]
pub mod smph5;
#[doc = "MCU SEMAPHORE 6"]
pub struct SMPH6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 6"]
pub mod smph6;
#[doc = "MCU SEMAPHORE 7"]
pub struct SMPH7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 7"]
pub mod smph7;
#[doc = "MCU SEMAPHORE 8"]
pub struct SMPH8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 8"]
pub mod smph8;
#[doc = "MCU SEMAPHORE 9"]
pub struct SMPH9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 9"]
pub mod smph9;
#[doc = "MCU SEMAPHORE 10"]
pub struct SMPH10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 10"]
pub mod smph10;
#[doc = "MCU SEMAPHORE 11"]
pub struct SMPH11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 11"]
pub mod smph11;
#[doc = "MCU SEMAPHORE 12"]
pub struct SMPH12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 12"]
pub mod smph12;
#[doc = "MCU SEMAPHORE 13"]
pub struct SMPH13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 13"]
pub mod smph13;
#[doc = "MCU SEMAPHORE 14"]
pub struct SMPH14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 14"]
pub mod smph14;
#[doc = "MCU SEMAPHORE 15"]
pub struct SMPH15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 15"]
pub mod smph15;
#[doc = "MCU SEMAPHORE 16"]
pub struct SMPH16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 16"]
pub mod smph16;
#[doc = "MCU SEMAPHORE 17"]
pub struct SMPH17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 17"]
pub mod smph17;
#[doc = "MCU SEMAPHORE 18"]
pub struct SMPH18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 18"]
pub mod smph18;
#[doc = "MCU SEMAPHORE 19"]
pub struct SMPH19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 19"]
pub mod smph19;
#[doc = "MCU SEMAPHORE 20"]
pub struct SMPH20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 20"]
pub mod smph20;
#[doc = "MCU SEMAPHORE 21"]
pub struct SMPH21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 21"]
pub mod smph21;
#[doc = "MCU SEMAPHORE 22"]
pub struct SMPH22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 22"]
pub mod smph22;
#[doc = "MCU SEMAPHORE 23"]
pub struct SMPH23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 23"]
pub mod smph23;
#[doc = "MCU SEMAPHORE 24"]
pub struct SMPH24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 24"]
pub mod smph24;
#[doc = "MCU SEMAPHORE 25"]
pub struct SMPH25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 25"]
pub mod smph25;
#[doc = "MCU SEMAPHORE 26"]
pub struct SMPH26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 26"]
pub mod smph26;
#[doc = "MCU SEMAPHORE 27"]
pub struct SMPH27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 27"]
pub mod smph27;
#[doc = "MCU SEMAPHORE 28"]
pub struct SMPH28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 28"]
pub mod smph28;
#[doc = "MCU SEMAPHORE 29"]
pub struct SMPH29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 29"]
pub mod smph29;
#[doc = "MCU SEMAPHORE 30"]
pub struct SMPH30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 30"]
pub mod smph30;
#[doc = "MCU SEMAPHORE 31"]
pub struct SMPH31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 31"]
pub mod smph31;
#[doc = "MCU SEMAPHORE 0 ALIAS"]
pub struct PEEK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 0 ALIAS"]
pub mod peek0;
#[doc = "MCU SEMAPHORE 1 ALIAS"]
pub struct PEEK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 1 ALIAS"]
pub mod peek1;
#[doc = "MCU SEMAPHORE 2 ALIAS"]
pub struct PEEK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 2 ALIAS"]
pub mod peek2;
#[doc = "MCU SEMAPHORE 3 ALIAS"]
pub struct PEEK3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 3 ALIAS"]
pub mod peek3;
#[doc = "MCU SEMAPHORE 4 ALIAS"]
pub struct PEEK4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 4 ALIAS"]
pub mod peek4;
#[doc = "MCU SEMAPHORE 5 ALIAS"]
pub struct PEEK5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 5 ALIAS"]
pub mod peek5;
#[doc = "MCU SEMAPHORE 6 ALIAS"]
pub struct PEEK6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 6 ALIAS"]
pub mod peek6;
#[doc = "MCU SEMAPHORE 7 ALIAS"]
pub struct PEEK7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 7 ALIAS"]
pub mod peek7;
#[doc = "MCU SEMAPHORE 8 ALIAS"]
pub struct PEEK8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 8 ALIAS"]
pub mod peek8;
#[doc = "MCU SEMAPHORE 9 ALIAS"]
pub struct PEEK9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 9 ALIAS"]
pub mod peek9;
#[doc = "MCU SEMAPHORE 10 ALIAS"]
pub struct PEEK10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 10 ALIAS"]
pub mod peek10;
#[doc = "MCU SEMAPHORE 11 ALIAS"]
pub struct PEEK11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 11 ALIAS"]
pub mod peek11;
#[doc = "MCU SEMAPHORE 12 ALIAS"]
pub struct PEEK12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 12 ALIAS"]
pub mod peek12;
#[doc = "MCU SEMAPHORE 13 ALIAS"]
pub struct PEEK13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 13 ALIAS"]
pub mod peek13;
#[doc = "MCU SEMAPHORE 14 ALIAS"]
pub struct PEEK14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 14 ALIAS"]
pub mod peek14;
#[doc = "MCU SEMAPHORE 15 ALIAS"]
pub struct PEEK15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 15 ALIAS"]
pub mod peek15;
#[doc = "MCU SEMAPHORE 16 ALIAS"]
pub struct PEEK16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 16 ALIAS"]
pub mod peek16;
#[doc = "MCU SEMAPHORE 17 ALIAS"]
pub struct PEEK17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 17 ALIAS"]
pub mod peek17;
#[doc = "MCU SEMAPHORE 18 ALIAS"]
pub struct PEEK18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 18 ALIAS"]
pub mod peek18;
#[doc = "MCU SEMAPHORE 19 ALIAS"]
pub struct PEEK19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 19 ALIAS"]
pub mod peek19;
#[doc = "MCU SEMAPHORE 20 ALIAS"]
pub struct PEEK20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 20 ALIAS"]
pub mod peek20;
#[doc = "MCU SEMAPHORE 21 ALIAS"]
pub struct PEEK21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 21 ALIAS"]
pub mod peek21;
#[doc = "MCU SEMAPHORE 22 ALIAS"]
pub struct PEEK22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 22 ALIAS"]
pub mod peek22;
#[doc = "MCU SEMAPHORE 23 ALIAS"]
pub struct PEEK23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 23 ALIAS"]
pub mod peek23;
#[doc = "MCU SEMAPHORE 24 ALIAS"]
pub struct PEEK24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 24 ALIAS"]
pub mod peek24;
#[doc = "MCU SEMAPHORE 25 ALIAS"]
pub struct PEEK25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 25 ALIAS"]
pub mod peek25;
#[doc = "MCU SEMAPHORE 26 ALIAS"]
pub struct PEEK26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 26 ALIAS"]
pub mod peek26;
#[doc = "MCU SEMAPHORE 27 ALIAS"]
pub struct PEEK27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 27 ALIAS"]
pub mod peek27;
#[doc = "MCU SEMAPHORE 28 ALIAS"]
pub struct PEEK28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 28 ALIAS"]
pub mod peek28;
#[doc = "MCU SEMAPHORE 29 ALIAS"]
pub struct PEEK29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 29 ALIAS"]
pub mod peek29;
#[doc = "MCU SEMAPHORE 30 ALIAS"]
pub struct PEEK30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 30 ALIAS"]
pub mod peek30;
#[doc = "MCU SEMAPHORE 31 ALIAS"]
pub struct PEEK31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU SEMAPHORE 31 ALIAS"]
pub mod peek31;
