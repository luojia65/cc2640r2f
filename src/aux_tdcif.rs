#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Status"]
    pub stat: STAT,
    #[doc = "0x08 - Result Result of last TDC conversion"]
    pub result: RESULT,
    #[doc = "0x0c - Saturation Configuration"]
    pub satcfg: SATCFG,
    #[doc = "0x10 - Trigger Source Select source and polarity for TDC start and stop events. See the Technical Reference Manual for event timing requirements."]
    pub trigsrc: TRIGSRC,
    #[doc = "0x14 - Trigger Counter Stop-counter control and status."]
    pub trigcnt: TRIGCNT,
    #[doc = "0x18 - Trigger Counter Load Stop-counter load."]
    pub trigcntload: TRIGCNTLOAD,
    #[doc = "0x1c - Trigger Counter Configuration Stop-counter configuration."]
    pub trigcntcfg: TRIGCNTCFG,
    #[doc = "0x20 - Prescaler Control The prescaler can be used to count events that are faster than the AUX clock frequency. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX clock frequency."]
    pub prectl: PRECTL,
    #[doc = "0x24 - Prescaler Counter"]
    pub precnt: PRECNT,
}
#[doc = "Control"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control"]
pub mod ctl;
#[doc = "Status"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status"]
pub mod stat;
#[doc = "Result Result of last TDC conversion"]
pub struct RESULT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Result Result of last TDC conversion"]
pub mod result;
#[doc = "Saturation Configuration"]
pub struct SATCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Saturation Configuration"]
pub mod satcfg;
#[doc = "Trigger Source Select source and polarity for TDC start and stop events. See the Technical Reference Manual for event timing requirements."]
pub struct TRIGSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Source Select source and polarity for TDC start and stop events. See the Technical Reference Manual for event timing requirements."]
pub mod trigsrc;
#[doc = "Trigger Counter Stop-counter control and status."]
pub struct TRIGCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Counter Stop-counter control and status."]
pub mod trigcnt;
#[doc = "Trigger Counter Load Stop-counter load."]
pub struct TRIGCNTLOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Counter Load Stop-counter load."]
pub mod trigcntload;
#[doc = "Trigger Counter Configuration Stop-counter configuration."]
pub struct TRIGCNTCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trigger Counter Configuration Stop-counter configuration."]
pub mod trigcntcfg;
#[doc = "Prescaler Control The prescaler can be used to count events that are faster than the AUX clock frequency. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX clock frequency."]
pub struct PRECTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prescaler Control The prescaler can be used to count events that are faster than the AUX clock frequency. It can be used to: - count pulses on a specified event from the asynchronous event bus. - prescale a specified event from the asynchronous event bus. To use the prescaler output as an event source in TDC measurements you must set both TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to AUX_TDC_PRE. It is recommended to use the prescaler when the signal frequency to measure exceeds 1/10th of the AUX clock frequency."]
pub mod prectl;
#[doc = "Prescaler Counter"]
pub struct PRECNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prescaler Counter"]
pub mod precnt;
