#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture."]
    pub sspsr: SSPSR,
    #[doc = "0x04 - Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit."]
    pub cspsr: CSPSR,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Async Clock Prescaler This register scales the baud rate of the asynchronous output."]
    pub acpr: ACPR,
    _reserved1: [u8; 220usize],
    #[doc = "0xf0 - Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs."]
    pub sppr: SPPR,
    _reserved2: [u8; 524usize],
    #[doc = "0x300 - Formatter and Flush Status"]
    pub ffsr: FFSR,
    #[doc = "0x304 - Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value."]
    pub ffcr: FFCR,
    #[doc = "0x308 - Formatter Synchronization Counter"]
    pub fscr: FSCR,
    _reserved3: [u8; 3220usize],
    #[doc = "0xfa0 - Claim Tag Mask"]
    pub claimmask: CLAIMMASK,
    #[doc = "0xfa4 - Current Claim Tag"]
    pub claimtag: CLAIMTAG,
    _reserved4: [u8; 32usize],
    #[doc = "0xfc8 - Device ID"]
    pub devid: DEVID,
}
#[doc = "Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture."]
pub struct SSPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture."]
pub mod sspsr;
#[doc = "Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit."]
pub struct CSPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit."]
pub mod cspsr;
#[doc = "Async Clock Prescaler This register scales the baud rate of the asynchronous output."]
pub struct ACPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Async Clock Prescaler This register scales the baud rate of the asynchronous output."]
pub mod acpr;
#[doc = "Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs."]
pub struct SPPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs."]
pub mod sppr;
#[doc = "Formatter and Flush Status"]
pub struct FFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Formatter and Flush Status"]
pub mod ffsr;
#[doc = "Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value."]
pub struct FFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value."]
pub mod ffcr;
#[doc = "Formatter Synchronization Counter"]
pub struct FSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Formatter Synchronization Counter"]
pub mod fscr;
#[doc = "Claim Tag Mask"]
pub struct CLAIMMASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Claim Tag Mask"]
pub mod claimmask;
#[doc = "Claim Tag Set"]
pub struct CLAIMSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Claim Tag Set"]
pub mod claimset;
#[doc = "Current Claim Tag"]
pub struct CLAIMTAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Claim Tag"]
pub mod claimtag;
#[doc = "Claim Tag Clear"]
pub struct CLAIMCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Claim Tag Clear"]
pub mod claimclr;
#[doc = "Device ID"]
pub struct DEVID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device ID"]
pub mod devid;
