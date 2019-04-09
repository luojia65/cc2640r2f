#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
    pub adcctl: ADCCTL,
    #[doc = "0x14 - ADC FIFO Status FIFO can hold up to four ADC samples."]
    pub adcfifostat: ADCFIFOSTAT,
    #[doc = "0x18 - ADC FIFO"]
    pub adcfifo: ADCFIFO,
    #[doc = "0x1c - ADC Trigger"]
    pub adctrig: ADCTRIG,
    #[doc = "0x20 - Current Source Control"]
    pub isrcctl: ISRCCTL,
}
#[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
pub struct ADCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
pub mod adcctl;
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples."]
pub struct ADCFIFOSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples."]
pub mod adcfifostat;
#[doc = "ADC FIFO"]
pub struct ADCFIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC FIFO"]
pub mod adcfifo;
#[doc = "ADC Trigger"]
pub struct ADCTRIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Trigger"]
pub mod adctrig;
#[doc = "Current Source Control"]
pub struct ISRCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Source Control"]
pub mod isrcctl;
