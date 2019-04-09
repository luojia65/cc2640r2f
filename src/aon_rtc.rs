#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control This register contains various bitfields for configuration of RTC"]
    pub ctl: CTL,
    #[doc = "0x04 - Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield."]
    pub evflags: EVFLAGS,
    #[doc = "0x08 - Second Counter Value, Integer Part"]
    pub sec: SEC,
    #[doc = "0x0c - Second Counter Value, Fractional Part"]
    pub subsec: SUBSEC,
    #[doc = "0x10 - Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle."]
    pub subsecinc: SUBSECINC,
    #[doc = "0x14 - Channel Configuration"]
    pub chctl: CHCTL,
    #[doc = "0x18 - Channel 0 Compare Value"]
    pub ch0cmp: CH0CMP,
    #[doc = "0x1c - Channel 1 Compare Value"]
    pub ch1cmp: CH1CMP,
    #[doc = "0x20 - Channel 2 Compare Value"]
    pub ch2cmp: CH2CMP,
    #[doc = "0x24 - Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\] event."]
    pub ch2cmpinc: CH2CMPINC,
    #[doc = "0x28 - Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL."]
    pub ch1capt: CH1CAPT,
    #[doc = "0x2c - AON Synchronization This register is used for synchronizing between MCU and entire AON domain."]
    pub sync: SYNC,
}
#[doc = "Control This register contains various bitfields for configuration of RTC"]
pub struct CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control This register contains various bitfields for configuration of RTC"]
pub mod ctl;
#[doc = "Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield."]
pub struct EVFLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield."]
pub mod evflags;
#[doc = "Second Counter Value, Integer Part"]
pub struct SEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Second Counter Value, Integer Part"]
pub mod sec;
#[doc = "Second Counter Value, Fractional Part"]
pub struct SUBSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Second Counter Value, Fractional Part"]
pub mod subsec;
#[doc = "Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle."]
pub struct SUBSECINC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle."]
pub mod subsecinc;
#[doc = "Channel Configuration"]
pub struct CHCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Configuration"]
pub mod chctl;
#[doc = "Channel 0 Compare Value"]
pub struct CH0CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 0 Compare Value"]
pub mod ch0cmp;
#[doc = "Channel 1 Compare Value"]
pub struct CH1CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 Compare Value"]
pub mod ch1cmp;
#[doc = "Channel 2 Compare Value"]
pub struct CH2CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 Compare Value"]
pub mod ch2cmp;
#[doc = "Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\] event."]
pub struct CH2CMPINC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\] event."]
pub mod ch2cmpinc;
#[doc = "Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL."]
pub struct CH1CAPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL."]
pub mod ch1capt;
#[doc = "AON Synchronization This register is used for synchronizing between MCU and entire AON domain."]
pub struct SYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AON Synchronization This register is used for synchronizing between MCU and entire AON domain."]
pub mod sync;
