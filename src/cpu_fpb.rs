#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control This register is used to enable the flash patch block."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators."]
    pub remap: REMAP,
    #[doc = "0x08 - Comparator 0"]
    pub comp0: COMP0,
    #[doc = "0x0c - Comparator 1"]
    pub comp1: COMP1,
    #[doc = "0x10 - Comparator 2"]
    pub comp2: COMP2,
    #[doc = "0x14 - Comparator 3"]
    pub comp3: COMP3,
    #[doc = "0x18 - Comparator 4"]
    pub comp4: COMP4,
    #[doc = "0x1c - Comparator 5"]
    pub comp5: COMP5,
    #[doc = "0x20 - Comparator 6"]
    pub comp6: COMP6,
    #[doc = "0x24 - Comparator 7"]
    pub comp7: COMP7,
}
#[doc = "Control This register is used to enable the flash patch block."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control This register is used to enable the flash patch block."]
pub mod ctrl;
#[doc = "Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators."]
pub struct REMAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators."]
pub mod remap;
#[doc = "Comparator 0"]
pub struct COMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 0"]
pub mod comp0;
#[doc = "Comparator 1"]
pub struct COMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 1"]
pub mod comp1;
#[doc = "Comparator 2"]
pub struct COMP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 2"]
pub mod comp2;
#[doc = "Comparator 3"]
pub struct COMP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 3"]
pub mod comp3;
#[doc = "Comparator 4"]
pub struct COMP4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 4"]
pub mod comp4;
#[doc = "Comparator 5"]
pub struct COMP5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 5"]
pub mod comp5;
#[doc = "Comparator 6"]
pub struct COMP6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 6"]
pub mod comp6;
#[doc = "Comparator 7"]
pub struct COMP7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 7"]
pub mod comp7;
