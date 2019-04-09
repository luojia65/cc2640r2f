#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Clock Management This register contains bitfields related to the MCU clock."]
    pub mcuclk: MCUCLK,
    #[doc = "0x04 - AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
    pub auxclk: AUXCLK,
    #[doc = "0x08 - MCU Configuration This register contains power management related bitfields for the MCU domain."]
    pub mcucfg: MCUCFG,
    #[doc = "0x0c - AUX Configuration This register contains power management related signals for the AUX domain."]
    pub auxcfg: AUXCFG,
    #[doc = "0x10 - AUX Control This register contains events and control signals for the AUX domain."]
    pub auxctl: AUXCTL,
    #[doc = "0x14 - Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up."]
    pub pwrstat: PWRSTAT,
    #[doc = "0x18 - Shutdown Control This register contains bitfields required for entering shutdown mode"]
    pub shutdown: SHUTDOWN,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Control 0 This register contains various chip level control and debug bitfields."]
    pub ctl0: CTL0,
    #[doc = "0x24 - Control 1 This register contains various chip level control and debug bitfields."]
    pub ctl1: CTL1,
    _reserved1: [u8; 8usize],
    #[doc = "0x30 - Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm."]
    pub rechargecfg: RECHARGECFG,
    #[doc = "0x34 - Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
    pub rechargestat: RECHARGESTAT,
    #[doc = "0x38 - Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
    pub osccfg: OSCCFG,
    _reserved2: [u8; 4usize],
    #[doc = "0x40 - JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP."]
    pub jtagcfg: JTAGCFG,
    #[doc = "0x44 - JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
    pub jtagusercode: JTAGUSERCODE,
}
#[doc = "MCU Clock Management This register contains bitfields related to the MCU clock."]
pub struct MCUCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU Clock Management This register contains bitfields related to the MCU clock."]
pub mod mcuclk;
#[doc = "AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
pub struct AUXCLK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
pub mod auxclk;
#[doc = "MCU Configuration This register contains power management related bitfields for the MCU domain."]
pub struct MCUCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU Configuration This register contains power management related bitfields for the MCU domain."]
pub mod mcucfg;
#[doc = "AUX Configuration This register contains power management related signals for the AUX domain."]
pub struct AUXCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUX Configuration This register contains power management related signals for the AUX domain."]
pub mod auxcfg;
#[doc = "AUX Control This register contains events and control signals for the AUX domain."]
pub struct AUXCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AUX Control This register contains events and control signals for the AUX domain."]
pub mod auxctl;
#[doc = "Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up."]
pub struct PWRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up."]
pub mod pwrstat;
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode"]
pub struct SHUTDOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode"]
pub mod shutdown;
#[doc = "Control 0 This register contains various chip level control and debug bitfields."]
pub struct CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control 0 This register contains various chip level control and debug bitfields."]
pub mod ctl0;
#[doc = "Control 1 This register contains various chip level control and debug bitfields."]
pub struct CTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control 1 This register contains various chip level control and debug bitfields."]
pub mod ctl1;
#[doc = "Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm."]
pub struct RECHARGECFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm."]
pub mod rechargecfg;
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
pub struct RECHARGESTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
pub mod rechargestat;
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
pub struct OSCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
pub mod osccfg;
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP."]
pub struct JTAGCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP."]
pub mod jtagcfg;
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
pub struct JTAGUSERCODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
pub mod jtagusercode;
