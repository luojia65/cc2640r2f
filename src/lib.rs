#![doc = "Peripheral access API for CC2640R2F microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 0] = [];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {}
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Always On (AON) Battery And Temperature MONitor (BATMON) residing in the AON domain Note: This module only supports 32 bit Read/Write access from MCU."]
pub struct AON_BATMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_BATMON {}
impl AON_BATMON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aon_batmon::RegisterBlock {
        1074352128 as *const _
    }
}
impl Deref for AON_BATMON {
    type Target = aon_batmon::RegisterBlock;
    fn deref(&self) -> &aon_batmon::RegisterBlock {
        unsafe { &*AON_BATMON::ptr() }
    }
}
#[doc = "Always On (AON) Battery And Temperature MONitor (BATMON) residing in the AON domain Note: This module only supports 32 bit Read/Write access from MCU."]
pub mod aon_batmon;
#[doc = "This module configures the event fabric located in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub struct AON_EVENT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_EVENT {}
impl AON_EVENT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aon_event::RegisterBlock {
        1074343936 as *const _
    }
}
impl Deref for AON_EVENT {
    type Target = aon_event::RegisterBlock;
    fn deref(&self) -> &aon_event::RegisterBlock {
        unsafe { &*AON_EVENT::ptr() }
    }
}
#[doc = "This module configures the event fabric located in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub mod aon_event;
#[doc = "Always On (AON) IO Controller - controls IO operation when the MCU IO Controller (IOC) is powered off and resides in the AON domain. Note: This module only supports 32 bit Read/Write access from MCU."]
pub struct AON_IOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_IOC {}
impl AON_IOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aon_ioc::RegisterBlock {
        1074348032 as *const _
    }
}
impl Deref for AON_IOC {
    type Target = aon_ioc::RegisterBlock;
    fn deref(&self) -> &aon_ioc::RegisterBlock {
        unsafe { &*AON_IOC::ptr() }
    }
}
#[doc = "Always On (AON) IO Controller - controls IO operation when the MCU IO Controller (IOC) is powered off and resides in the AON domain. Note: This module only supports 32 bit Read/Write access from MCU."]
pub mod aon_ioc;
#[doc = "This component control the Real Time Clock residing in AON Note: This module is only supporting 32 bit ReadWrite access."]
pub struct AON_RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_RTC {}
impl AON_RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aon_rtc::RegisterBlock {
        1074339840 as *const _
    }
}
impl Deref for AON_RTC {
    type Target = aon_rtc::RegisterBlock;
    fn deref(&self) -> &aon_rtc::RegisterBlock {
        unsafe { &*AON_RTC::ptr() }
    }
}
#[doc = "This component control the Real Time Clock residing in AON Note: This module is only supporting 32 bit ReadWrite access."]
pub mod aon_rtc;
#[doc = "This component controls AON_SYSCTL, which is the device's system controller. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub struct AON_SYSCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_SYSCTL {}
impl AON_SYSCTL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aon_sysctl::RegisterBlock {
        1074331648 as *const _
    }
}
impl Deref for AON_SYSCTL {
    type Target = aon_sysctl::RegisterBlock;
    fn deref(&self) -> &aon_sysctl::RegisterBlock {
        unsafe { &*AON_SYSCTL::ptr() }
    }
}
#[doc = "This component controls AON_SYSCTL, which is the device's system controller. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub mod aon_sysctl;
#[doc = "This component control the Wakeup controller residing in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub struct AON_WUC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_WUC {}
impl AON_WUC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aon_wuc::RegisterBlock {
        1074335744 as *const _
    }
}
impl Deref for AON_WUC {
    type Target = aon_wuc::RegisterBlock;
    fn deref(&self) -> &aon_wuc::RegisterBlock {
        unsafe { &*AON_WUC::ptr() }
    }
}
#[doc = "This component control the Wakeup controller residing in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub mod aon_wuc;
#[doc = "Configuration registers controlling analog peripherals of AUX. Registers Fields should be considered static unless otherwise noted (as dynamic)"]
pub struct AUX_ADI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_ADI4 {}
impl AUX_ADI4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aux_adi4::RegisterBlock {
        1074573312 as *const _
    }
}
impl Deref for AUX_ADI4 {
    type Target = aux_adi4::RegisterBlock;
    fn deref(&self) -> &aux_adi4::RegisterBlock {
        unsafe { &*AUX_ADI4::ptr() }
    }
}
#[doc = "Configuration registers controlling analog peripherals of AUX. Registers Fields should be considered static unless otherwise noted (as dynamic)"]
pub mod aux_adi4;
#[doc = "AUX Analog/Digital Input Output Controller"]
pub struct AUX_AIODIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_AIODIO0 {}
impl AUX_AIODIO0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aux_aiodio0::RegisterBlock {
        1074532352 as *const _
    }
}
impl Deref for AUX_AIODIO0 {
    type Target = aux_aiodio0::RegisterBlock;
    fn deref(&self) -> &aux_aiodio0::RegisterBlock {
        unsafe { &*AUX_AIODIO0::ptr() }
    }
}
#[doc = "AUX Analog/Digital Input Output Controller"]
pub mod aux_aiodio0;
#[doc = "AUX Analog/Digital Input Output Controller"]
pub struct AUX_AIODIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_AIODIO1 {}
impl AUX_AIODIO1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aux_aiodio1::RegisterBlock {
        1074536448 as *const _
    }
}
impl Deref for AUX_AIODIO1 {
    type Target = aux_aiodio1::RegisterBlock;
    fn deref(&self) -> &aux_aiodio1::RegisterBlock {
        unsafe { &*AUX_AIODIO1::ptr() }
    }
}
#[doc = "AUX Analog/Digital Input Output Controller"]
pub mod aux_aiodio1;
#[doc = "AUX Analog Peripheral Control Module"]
pub struct AUX_ANAIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_ANAIF {}
impl AUX_ANAIF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aux_anaif::RegisterBlock {
        1074565120 as *const _
    }
}
impl Deref for AUX_ANAIF {
    type Target = aux_anaif::RegisterBlock;
    fn deref(&self) -> &aux_anaif::RegisterBlock {
        unsafe { &*AUX_ANAIF::ptr() }
    }
}
#[doc = "AUX Analog Peripheral Control Module"]
pub mod aux_anaif;
#[doc = "This is the DDI for the digital block that controls all the analog clock oscillators (OSC_DIG) and performs qualification of the clocks generated."]
pub struct AUX_DDI0_OSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_DDI0_OSC {}
impl AUX_DDI0_OSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aux_ddi0_osc::RegisterBlock {
        1074569216 as *const _
    }
}
impl Deref for AUX_DDI0_OSC {
    type Target = aux_ddi0_osc::RegisterBlock;
    fn deref(&self) -> &aux_ddi0_osc::RegisterBlock {
        unsafe { &*AUX_DDI0_OSC::ptr() }
    }
}
#[doc = "This is the DDI for the digital block that controls all the analog clock oscillators (OSC_DIG) and performs qualification of the clocks generated."]
pub mod aux_ddi0_osc;
#[doc = "AUX Event Controller"]
pub struct AUX_EVCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_EVCTL {}
impl AUX_EVCTL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aux_evctl::RegisterBlock {
        1074548736 as *const _
    }
}
impl Deref for AUX_EVCTL {
    type Target = aux_evctl::RegisterBlock;
    fn deref(&self) -> &aux_evctl::RegisterBlock {
        unsafe { &*AUX_EVCTL::ptr() }
    }
}
#[doc = "AUX Event Controller"]
pub mod aux_evctl;
#[doc = "AUX Sensor Control Engine Control Module"]
pub struct AUX_SCE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_SCE {}
impl AUX_SCE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aux_sce::RegisterBlock {
        1074663424 as *const _
    }
}
impl Deref for AUX_SCE {
    type Target = aux_sce::RegisterBlock;
    fn deref(&self) -> &aux_sce::RegisterBlock {
        unsafe { &*AUX_SCE::ptr() }
    }
}
#[doc = "AUX Sensor Control Engine Control Module"]
pub mod aux_sce;
#[doc = "AUX Semaphore Controller"]
pub struct AUX_SMPH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_SMPH {}
impl AUX_SMPH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aux_smph::RegisterBlock {
        1074561024 as *const _
    }
}
impl Deref for AUX_SMPH {
    type Target = aux_smph::RegisterBlock;
    fn deref(&self) -> &aux_smph::RegisterBlock {
        unsafe { &*AUX_SMPH::ptr() }
    }
}
#[doc = "AUX Semaphore Controller"]
pub mod aux_smph;
#[doc = "AUX Time To Digital Converter"]
pub struct AUX_TDCIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_TDCIF {}
impl AUX_TDCIF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aux_tdcif::RegisterBlock {
        1074544640 as *const _
    }
}
impl Deref for AUX_TDCIF {
    type Target = aux_tdcif::RegisterBlock;
    fn deref(&self) -> &aux_tdcif::RegisterBlock {
        unsafe { &*AUX_TDCIF::ptr() }
    }
}
#[doc = "AUX Time To Digital Converter"]
pub mod aux_tdcif;
#[doc = "AUX Timer"]
pub struct AUX_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_TIMER {}
impl AUX_TIMER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aux_timer::RegisterBlock {
        1074556928 as *const _
    }
}
impl Deref for AUX_TIMER {
    type Target = aux_timer::RegisterBlock;
    fn deref(&self) -> &aux_timer::RegisterBlock {
        unsafe { &*AUX_TIMER::ptr() }
    }
}
#[doc = "AUX Timer"]
pub mod aux_timer;
#[doc = "AUX Wake-up controller"]
pub struct AUX_WUC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_WUC {}
impl AUX_WUC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aux_wuc::RegisterBlock {
        1074552832 as *const _
    }
}
impl Deref for AUX_WUC {
    type Target = aux_wuc::RegisterBlock;
    fn deref(&self) -> &aux_wuc::RegisterBlock {
        unsafe { &*AUX_WUC::ptr() }
    }
}
#[doc = "AUX Wake-up controller"]
pub mod aux_wuc;
#[doc = "Customer configuration area (CCFG)"]
pub struct CCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCFG {}
impl CCFG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccfg::RegisterBlock {
        1342189568 as *const _
    }
}
impl Deref for CCFG {
    type Target = ccfg::RegisterBlock;
    fn deref(&self) -> &ccfg::RegisterBlock {
        unsafe { &*CCFG::ptr() }
    }
}
#[doc = "Customer configuration area (CCFG)"]
pub mod ccfg;
#[doc = "Cortex-M's Data watchpoint and Trace (DWT)"]
pub struct CPU_DWT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_DWT {}
impl CPU_DWT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cpu_dwt::RegisterBlock {
        3758100480 as *const _
    }
}
impl Deref for CPU_DWT {
    type Target = cpu_dwt::RegisterBlock;
    fn deref(&self) -> &cpu_dwt::RegisterBlock {
        unsafe { &*CPU_DWT::ptr() }
    }
}
#[doc = "Cortex-M's Data watchpoint and Trace (DWT)"]
pub mod cpu_dwt;
#[doc = "Cortex-M's Flash Patch and Breakpoint (FPB)"]
pub struct CPU_FPB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_FPB {}
impl CPU_FPB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cpu_fpb::RegisterBlock {
        3758104576 as *const _
    }
}
impl Deref for CPU_FPB {
    type Target = cpu_fpb::RegisterBlock;
    fn deref(&self) -> &cpu_fpb::RegisterBlock {
        unsafe { &*CPU_FPB::ptr() }
    }
}
#[doc = "Cortex-M's Flash Patch and Breakpoint (FPB)"]
pub mod cpu_fpb;
#[doc = "Cortex-M's Instrumentation Trace Macrocell (ITM)"]
pub struct CPU_ITM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_ITM {}
impl CPU_ITM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cpu_itm::RegisterBlock {
        3758096384 as *const _
    }
}
impl Deref for CPU_ITM {
    type Target = cpu_itm::RegisterBlock;
    fn deref(&self) -> &cpu_itm::RegisterBlock {
        unsafe { &*CPU_ITM::ptr() }
    }
}
#[doc = "Cortex-M's Instrumentation Trace Macrocell (ITM)"]
pub mod cpu_itm;
#[doc = "Cortex-M's System Control Space (SCS)"]
pub struct CPU_SCS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_SCS {}
impl CPU_SCS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cpu_scs::RegisterBlock {
        3758153728 as *const _
    }
}
impl Deref for CPU_SCS {
    type Target = cpu_scs::RegisterBlock;
    fn deref(&self) -> &cpu_scs::RegisterBlock {
        unsafe { &*CPU_SCS::ptr() }
    }
}
#[doc = "Cortex-M's System Control Space (SCS)"]
pub mod cpu_scs;
#[doc = "Cortex-M's TI proprietary registers"]
pub struct CPU_TIPROP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_TIPROP {}
impl CPU_TIPROP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cpu_tiprop::RegisterBlock {
        3759136768 as *const _
    }
}
impl Deref for CPU_TIPROP {
    type Target = cpu_tiprop::RegisterBlock;
    fn deref(&self) -> &cpu_tiprop::RegisterBlock {
        unsafe { &*CPU_TIPROP::ptr() }
    }
}
#[doc = "Cortex-M's TI proprietary registers"]
pub mod cpu_tiprop;
#[doc = "Cortex-M3's Trace Port Interface Unit (TPIU)"]
pub struct CPU_TPIU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_TPIU {}
impl CPU_TPIU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cpu_tpiu::RegisterBlock {
        3758358528 as *const _
    }
}
impl Deref for CPU_TPIU {
    type Target = cpu_tpiu::RegisterBlock;
    fn deref(&self) -> &cpu_tpiu::RegisterBlock {
        unsafe { &*CPU_TPIU::ptr() }
    }
}
#[doc = "Cortex-M3's Trace Port Interface Unit (TPIU)"]
pub mod cpu_tpiu;
#[doc = "Crypto core with DMA capability and local key storage"]
pub struct CRYPTO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYPTO {}
impl CRYPTO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crypto::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for CRYPTO {
    type Target = crypto::RegisterBlock;
    fn deref(&self) -> &crypto::RegisterBlock {
        unsafe { &*CRYPTO::ptr() }
    }
}
#[doc = "Crypto core with DMA capability and local key storage"]
pub mod crypto;
#[doc = "Event Fabric Component Definition"]
pub struct EVENT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVENT {}
impl EVENT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const event::RegisterBlock {
        1074278400 as *const _
    }
}
impl Deref for EVENT {
    type Target = event::RegisterBlock;
    fn deref(&self) -> &event::RegisterBlock {
        unsafe { &*EVENT::ptr() }
    }
}
#[doc = "Event Fabric Component Definition"]
pub mod event;
#[doc = "Factory configuration area (FCFG1)"]
pub struct FCFG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCFG1 {}
impl FCFG1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fcfg1::RegisterBlock {
        1342181376 as *const _
    }
}
impl Deref for FCFG1 {
    type Target = fcfg1::RegisterBlock;
    fn deref(&self) -> &fcfg1::RegisterBlock {
        unsafe { &*FCFG1::ptr() }
    }
}
#[doc = "Factory configuration area (FCFG1)"]
pub mod fcfg1;
#[doc = "Flash sub-system registers, includes the Flash Memory Controller (FMC), flash read path, and an integrated Efuse controller and EFUSEROM."]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash::RegisterBlock {
        1073938432 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash sub-system registers, includes the Flash Memory Controller (FMC), flash read path, and an integrated Efuse controller and EFUSEROM."]
pub mod flash;
#[doc = "MCU GPIO - I/F for controlling and reading IO status and IO event status"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "MCU GPIO - I/F for controlling and reading IO status and IO event status"]
pub mod gpio;
#[doc = "General Purpose Timer."]
pub struct GPT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT0 {}
impl GPT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpt0::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for GPT0 {
    type Target = gpt0::RegisterBlock;
    fn deref(&self) -> &gpt0::RegisterBlock {
        unsafe { &*GPT0::ptr() }
    }
}
#[doc = "General Purpose Timer."]
pub mod gpt0;
#[doc = "General Purpose Timer."]
pub struct GPT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT1 {}
impl GPT1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpt1::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for GPT1 {
    type Target = gpt1::RegisterBlock;
    fn deref(&self) -> &gpt1::RegisterBlock {
        unsafe { &*GPT1::ptr() }
    }
}
#[doc = "General Purpose Timer."]
pub mod gpt1;
#[doc = "General Purpose Timer."]
pub struct GPT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT2 {}
impl GPT2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpt2::RegisterBlock {
        1073815552 as *const _
    }
}
impl Deref for GPT2 {
    type Target = gpt2::RegisterBlock;
    fn deref(&self) -> &gpt2::RegisterBlock {
        unsafe { &*GPT2::ptr() }
    }
}
#[doc = "General Purpose Timer."]
pub mod gpt2;
#[doc = "General Purpose Timer."]
pub struct GPT3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT3 {}
impl GPT3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpt3::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for GPT3 {
    type Target = gpt3::RegisterBlock;
    fn deref(&self) -> &gpt3::RegisterBlock {
        unsafe { &*GPT3::ptr() }
    }
}
#[doc = "General Purpose Timer."]
pub mod gpt3;
#[doc = "I2CMaster/Slave Serial Controler"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c0::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &i2c0::RegisterBlock {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2CMaster/Slave Serial Controler"]
pub mod i2c0;
#[doc = "I2S Audio DMA module supporting formats I2S, LJF, RJF and DSP"]
pub struct I2S0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0 {}
impl I2S0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s0::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for I2S0 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &i2s0::RegisterBlock {
        unsafe { &*I2S0::ptr() }
    }
}
#[doc = "I2S Audio DMA module supporting formats I2S, LJF, RJF and DSP"]
pub mod i2s0;
#[doc = "IO Controller (IOC) - configures all the DIOs and resides in the MCU domain."]
pub struct IOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOC {}
impl IOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ioc::RegisterBlock {
        1074270208 as *const _
    }
}
impl Deref for IOC {
    type Target = ioc::RegisterBlock;
    fn deref(&self) -> &ioc::RegisterBlock {
        unsafe { &*IOC::ptr() }
    }
}
#[doc = "IO Controller (IOC) - configures all the DIOs and resides in the MCU domain."]
pub mod ioc;
#[doc = "Power, Reset and Clock Management"]
pub struct PRCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRCM {}
impl PRCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const prcm::RegisterBlock {
        1074274304 as *const _
    }
}
impl Deref for PRCM {
    type Target = prcm::RegisterBlock;
    fn deref(&self) -> &prcm::RegisterBlock {
        unsafe { &*PRCM::ptr() }
    }
}
#[doc = "Power, Reset and Clock Management"]
pub mod prcm;
#[doc = "RF Core Doorbell"]
pub struct RFC_DBELL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_DBELL {}
impl RFC_DBELL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rfc_dbell::RegisterBlock {
        1074008064 as *const _
    }
}
impl Deref for RFC_DBELL {
    type Target = rfc_dbell::RegisterBlock;
    fn deref(&self) -> &rfc_dbell::RegisterBlock {
        unsafe { &*RFC_DBELL::ptr() }
    }
}
#[doc = "RF Core Doorbell"]
pub mod rfc_dbell;
#[doc = "RF Core Power Management"]
pub struct RFC_PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_PWR {}
impl RFC_PWR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rfc_pwr::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for RFC_PWR {
    type Target = rfc_pwr::RegisterBlock;
    fn deref(&self) -> &rfc_pwr::RegisterBlock {
        unsafe { &*RFC_PWR::ptr() }
    }
}
#[doc = "RF Core Power Management"]
pub mod rfc_pwr;
#[doc = "RF Core Radio Timer"]
pub struct RFC_RAT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_RAT {}
impl RFC_RAT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rfc_rat::RegisterBlock {
        1074016256 as *const _
    }
}
impl Deref for RFC_RAT {
    type Target = rfc_rat::RegisterBlock;
    fn deref(&self) -> &rfc_rat::RegisterBlock {
        unsafe { &*RFC_RAT::ptr() }
    }
}
#[doc = "RF Core Radio Timer"]
pub mod rfc_rat;
#[doc = "MCU Semaphore Module This module provides 32 binary semaphores. The state of a binary semaphore is either taken or available. A semaphore does not implement any ownership attribute. Still, a semaphore can be used to handle mutual exclusion scenarios."]
pub struct SMPH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMPH {}
impl SMPH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const smph::RegisterBlock {
        1074282496 as *const _
    }
}
impl Deref for SMPH {
    type Target = smph::RegisterBlock;
    fn deref(&self) -> &smph::RegisterBlock {
        unsafe { &*SMPH::ptr() }
    }
}
#[doc = "MCU Semaphore Module This module provides 32 binary semaphores. The state of a binary semaphore is either taken or available. A semaphore does not implement any ownership attribute. Still, a semaphore can be used to handle mutual exclusion scenarios."]
pub mod smph;
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub struct SSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI0 {}
impl SSI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssi0::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for SSI0 {
    type Target = ssi0::RegisterBlock;
    fn deref(&self) -> &ssi0::RegisterBlock {
        unsafe { &*SSI0::ptr() }
    }
}
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub mod ssi0;
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub struct SSI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI1 {}
impl SSI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssi1::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for SSI1 {
    type Target = ssi1::RegisterBlock;
    fn deref(&self) -> &ssi1::RegisterBlock {
        unsafe { &*SSI1::ptr() }
    }
}
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub mod ssi1;
#[doc = "True Random Number Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const trng::RegisterBlock {
        1073905664 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    fn deref(&self) -> &trng::RegisterBlock {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Number Generator"]
pub mod trng;
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
pub mod uart0;
#[doc = "ARM Micro Direct Memory Access Controller"]
pub struct UDMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UDMA0 {}
impl UDMA0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const udma0::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for UDMA0 {
    type Target = udma0::RegisterBlock;
    fn deref(&self) -> &udma0::RegisterBlock {
        unsafe { &*UDMA0::ptr() }
    }
}
#[doc = "ARM Micro Direct Memory Access Controller"]
pub mod udma0;
#[doc = "Versatile Instruction Memory System Controls memory access to the Flash and encapsulates the following instruction memories: - Boot ROM - Cache / GPRAM"]
pub struct VIMS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VIMS {}
impl VIMS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vims::RegisterBlock {
        1073954816 as *const _
    }
}
impl Deref for VIMS {
    type Target = vims::RegisterBlock;
    fn deref(&self) -> &vims::RegisterBlock {
        unsafe { &*VIMS::ptr() }
    }
}
#[doc = "Versatile Instruction Memory System Controls memory access to the Flash and encapsulates the following instruction memories: - Boot ROM - Cache / GPRAM"]
pub mod vims;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1074266112 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AON_BATMON"]
    pub AON_BATMON: AON_BATMON,
    #[doc = "AON_EVENT"]
    pub AON_EVENT: AON_EVENT,
    #[doc = "AON_IOC"]
    pub AON_IOC: AON_IOC,
    #[doc = "AON_RTC"]
    pub AON_RTC: AON_RTC,
    #[doc = "AON_SYSCTL"]
    pub AON_SYSCTL: AON_SYSCTL,
    #[doc = "AON_WUC"]
    pub AON_WUC: AON_WUC,
    #[doc = "AUX_ADI4"]
    pub AUX_ADI4: AUX_ADI4,
    #[doc = "AUX_AIODIO0"]
    pub AUX_AIODIO0: AUX_AIODIO0,
    #[doc = "AUX_AIODIO1"]
    pub AUX_AIODIO1: AUX_AIODIO1,
    #[doc = "AUX_ANAIF"]
    pub AUX_ANAIF: AUX_ANAIF,
    #[doc = "AUX_DDI0_OSC"]
    pub AUX_DDI0_OSC: AUX_DDI0_OSC,
    #[doc = "AUX_EVCTL"]
    pub AUX_EVCTL: AUX_EVCTL,
    #[doc = "AUX_SCE"]
    pub AUX_SCE: AUX_SCE,
    #[doc = "AUX_SMPH"]
    pub AUX_SMPH: AUX_SMPH,
    #[doc = "AUX_TDCIF"]
    pub AUX_TDCIF: AUX_TDCIF,
    #[doc = "AUX_TIMER"]
    pub AUX_TIMER: AUX_TIMER,
    #[doc = "AUX_WUC"]
    pub AUX_WUC: AUX_WUC,
    #[doc = "CCFG"]
    pub CCFG: CCFG,
    #[doc = "CPU_DWT"]
    pub CPU_DWT: CPU_DWT,
    #[doc = "CPU_FPB"]
    pub CPU_FPB: CPU_FPB,
    #[doc = "CPU_ITM"]
    pub CPU_ITM: CPU_ITM,
    #[doc = "CPU_SCS"]
    pub CPU_SCS: CPU_SCS,
    #[doc = "CPU_TIPROP"]
    pub CPU_TIPROP: CPU_TIPROP,
    #[doc = "CPU_TPIU"]
    pub CPU_TPIU: CPU_TPIU,
    #[doc = "CRYPTO"]
    pub CRYPTO: CRYPTO,
    #[doc = "EVENT"]
    pub EVENT: EVENT,
    #[doc = "FCFG1"]
    pub FCFG1: FCFG1,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "GPT0"]
    pub GPT0: GPT0,
    #[doc = "GPT1"]
    pub GPT1: GPT1,
    #[doc = "GPT2"]
    pub GPT2: GPT2,
    #[doc = "GPT3"]
    pub GPT3: GPT3,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2S0"]
    pub I2S0: I2S0,
    #[doc = "IOC"]
    pub IOC: IOC,
    #[doc = "PRCM"]
    pub PRCM: PRCM,
    #[doc = "RFC_DBELL"]
    pub RFC_DBELL: RFC_DBELL,
    #[doc = "RFC_PWR"]
    pub RFC_PWR: RFC_PWR,
    #[doc = "RFC_RAT"]
    pub RFC_RAT: RFC_RAT,
    #[doc = "SMPH"]
    pub SMPH: SMPH,
    #[doc = "SSI0"]
    pub SSI0: SSI0,
    #[doc = "SSI1"]
    pub SSI1: SSI1,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UDMA0"]
    pub UDMA0: UDMA0,
    #[doc = "VIMS"]
    pub VIMS: VIMS,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AON_BATMON: AON_BATMON {
                _marker: PhantomData,
            },
            AON_EVENT: AON_EVENT {
                _marker: PhantomData,
            },
            AON_IOC: AON_IOC {
                _marker: PhantomData,
            },
            AON_RTC: AON_RTC {
                _marker: PhantomData,
            },
            AON_SYSCTL: AON_SYSCTL {
                _marker: PhantomData,
            },
            AON_WUC: AON_WUC {
                _marker: PhantomData,
            },
            AUX_ADI4: AUX_ADI4 {
                _marker: PhantomData,
            },
            AUX_AIODIO0: AUX_AIODIO0 {
                _marker: PhantomData,
            },
            AUX_AIODIO1: AUX_AIODIO1 {
                _marker: PhantomData,
            },
            AUX_ANAIF: AUX_ANAIF {
                _marker: PhantomData,
            },
            AUX_DDI0_OSC: AUX_DDI0_OSC {
                _marker: PhantomData,
            },
            AUX_EVCTL: AUX_EVCTL {
                _marker: PhantomData,
            },
            AUX_SCE: AUX_SCE {
                _marker: PhantomData,
            },
            AUX_SMPH: AUX_SMPH {
                _marker: PhantomData,
            },
            AUX_TDCIF: AUX_TDCIF {
                _marker: PhantomData,
            },
            AUX_TIMER: AUX_TIMER {
                _marker: PhantomData,
            },
            AUX_WUC: AUX_WUC {
                _marker: PhantomData,
            },
            CCFG: CCFG {
                _marker: PhantomData,
            },
            CPU_DWT: CPU_DWT {
                _marker: PhantomData,
            },
            CPU_FPB: CPU_FPB {
                _marker: PhantomData,
            },
            CPU_ITM: CPU_ITM {
                _marker: PhantomData,
            },
            CPU_SCS: CPU_SCS {
                _marker: PhantomData,
            },
            CPU_TIPROP: CPU_TIPROP {
                _marker: PhantomData,
            },
            CPU_TPIU: CPU_TPIU {
                _marker: PhantomData,
            },
            CRYPTO: CRYPTO {
                _marker: PhantomData,
            },
            EVENT: EVENT {
                _marker: PhantomData,
            },
            FCFG1: FCFG1 {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            GPT0: GPT0 {
                _marker: PhantomData,
            },
            GPT1: GPT1 {
                _marker: PhantomData,
            },
            GPT2: GPT2 {
                _marker: PhantomData,
            },
            GPT3: GPT3 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2S0: I2S0 {
                _marker: PhantomData,
            },
            IOC: IOC {
                _marker: PhantomData,
            },
            PRCM: PRCM {
                _marker: PhantomData,
            },
            RFC_DBELL: RFC_DBELL {
                _marker: PhantomData,
            },
            RFC_PWR: RFC_PWR {
                _marker: PhantomData,
            },
            RFC_RAT: RFC_RAT {
                _marker: PhantomData,
            },
            SMPH: SMPH {
                _marker: PhantomData,
            },
            SSI0: SSI0 {
                _marker: PhantomData,
            },
            SSI1: SSI1 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UDMA0: UDMA0 {
                _marker: PhantomData,
            },
            VIMS: VIMS {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
