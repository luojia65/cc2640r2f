#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control 0 Controls clock source selects"]
    pub ctl0: CTL0,
    #[doc = "0x04 - Control 1 This register contains OSC_DIG configuration"]
    pub ctl1: CTL1,
    #[doc = "0x08 - RADC External Configuration"]
    pub radcextcfg: RADCEXTCFG,
    #[doc = "0x0c - Amplitude Compensation Control"]
    pub ampcompctl: AMPCOMPCTL,
    #[doc = "0x10 - Amplitude Compensation Threshold 1 This register contains threshold values for amplitude compensation algorithm"]
    pub ampcompth1: AMPCOMPTH1,
    #[doc = "0x14 - Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm."]
    pub ampcompth2: AMPCOMPTH2,
    #[doc = "0x18 - Analog Bypass Values 1"]
    pub anabypassval1: ANABYPASSVAL1,
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    pub anabypassval2: ANABYPASSVAL2,
    #[doc = "0x20 - Analog Test Control"]
    pub atestctl: ATESTCTL,
    #[doc = "0x24 - ADC Doubler Nanoamp Control"]
    pub adcdoublernanoampctl: ADCDOUBLERNANOAMPCTL,
    #[doc = "0x28 - XOSCHF Control"]
    pub xoschfctl: XOSCHFCTL,
    #[doc = "0x2c - Low Frequency Oscillator Control"]
    pub lfoscctl: LFOSCCTL,
    #[doc = "0x30 - RCOSCHF Control"]
    pub rcoschfctl: RCOSCHFCTL,
    #[doc = "0x34 - Status 0 This register contains status signals from OSC_DIG"]
    pub stat0: STAT0,
    #[doc = "0x38 - Status 1 This register contains status signals from OSC_DIG"]
    pub stat1: STAT1,
    #[doc = "0x3c - Status 2 This register contains status signals from AMPCOMP FSM"]
    pub stat2: STAT2,
}
#[doc = "Control 0 Controls clock source selects"]
pub struct CTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control 0 Controls clock source selects"]
pub mod ctl0;
#[doc = "Control 1 This register contains OSC_DIG configuration"]
pub struct CTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control 1 This register contains OSC_DIG configuration"]
pub mod ctl1;
#[doc = "RADC External Configuration"]
pub struct RADCEXTCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RADC External Configuration"]
pub mod radcextcfg;
#[doc = "Amplitude Compensation Control"]
pub struct AMPCOMPCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Amplitude Compensation Control"]
pub mod ampcompctl;
#[doc = "Amplitude Compensation Threshold 1 This register contains threshold values for amplitude compensation algorithm"]
pub struct AMPCOMPTH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Amplitude Compensation Threshold 1 This register contains threshold values for amplitude compensation algorithm"]
pub mod ampcompth1;
#[doc = "Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm."]
pub struct AMPCOMPTH2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm."]
pub mod ampcompth2;
#[doc = "Analog Bypass Values 1"]
pub struct ANABYPASSVAL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Bypass Values 1"]
pub mod anabypassval1;
#[doc = "Internal. Only to be used through TI provided API."]
pub struct ANABYPASSVAL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod anabypassval2;
#[doc = "Analog Test Control"]
pub struct ATESTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Test Control"]
pub mod atestctl;
#[doc = "ADC Doubler Nanoamp Control"]
pub struct ADCDOUBLERNANOAMPCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Doubler Nanoamp Control"]
pub mod adcdoublernanoampctl;
#[doc = "XOSCHF Control"]
pub struct XOSCHFCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "XOSCHF Control"]
pub mod xoschfctl;
#[doc = "Low Frequency Oscillator Control"]
pub struct LFOSCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Frequency Oscillator Control"]
pub mod lfoscctl;
#[doc = "RCOSCHF Control"]
pub struct RCOSCHFCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCOSCHF Control"]
pub mod rcoschfctl;
#[doc = "Status 0 This register contains status signals from OSC_DIG"]
pub struct STAT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status 0 This register contains status signals from OSC_DIG"]
pub mod stat0;
#[doc = "Status 1 This register contains status signals from OSC_DIG"]
pub struct STAT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status 1 This register contains status signals from OSC_DIG"]
pub mod stat1;
#[doc = "Status 2 This register contains status signals from AMPCOMP FSM"]
pub struct STAT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status 2 This register contains status signals from AMPCOMP FSM"]
pub mod stat2;
