#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
    pub gpiodout: GPIODOUT,
    #[doc = "0x04 - Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
    pub iomode: IOMODE,
    #[doc = "0x08 - General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
    pub gpiodin: GPIODIN,
    #[doc = "0x0c - General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
    pub gpiodoutset: GPIODOUTSET,
    #[doc = "0x10 - General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
    pub gpiodoutclr: GPIODOUTCLR,
    #[doc = "0x14 - General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
    pub gpiodouttgl: GPIODOUTTGL,
    #[doc = "0x18 - General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
    pub gpiodie: GPIODIE,
}
#[doc = "General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
pub struct GPIODOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Input Output Data Out The output data register is used to set data on AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
pub mod gpiodout;
#[doc = "Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
pub struct IOMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Input Output Mode This register controls pull-up, pull-down, and output mode for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1"]
pub mod iomode;
#[doc = "General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
pub struct GPIODIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Input Output Data In This register provides synchronized input data for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
pub mod gpiodin;
#[doc = "General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
pub struct GPIODOUTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Input Output Data Out Set Set bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
pub mod gpiodoutset;
#[doc = "General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
pub struct GPIODOUTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Input Output Data Out Clear Clear bits in GPIODOUT instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
pub mod gpiodoutclr;
#[doc = "General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
pub struct GPIODOUTTGL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Input Output Data Out Toggle Toggle bits in GPIODOUT in instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and i = 1 for AUX_AIODIO1."]
pub mod gpiodouttgl;
#[doc = "General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
pub struct GPIODIE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Input Output Digital Input Enable This register controls input buffers for AUXIO that are controlled by instance i of AUX_AIODIO. Hence, in formulas below i = 0 for AUX_AIODIO0 and I = 1 for AUX_AIODIO1."]
pub mod gpiodie;
