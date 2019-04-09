#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
    pub pwrctl: PWRCTL,
    #[doc = "0x04 - Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets."]
    pub resetctl: RESETCTL,
    #[doc = "0x08 - Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
    pub sleepctl: SLEEPCTL,
}
#[doc = "Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
pub struct PWRCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
pub mod pwrctl;
#[doc = "Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets."]
pub struct RESETCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets."]
pub mod resetctl;
#[doc = "Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
pub struct SLEEPCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
pub mod sleepctl;
