#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved000: RESERVED000,
    #[doc = "0x04 - Interrupt Control Type Read this register to see the number of interrupt lines that the NVIC supports."]
    pub ictr: ICTR,
    #[doc = "0x08 - Auxiliary Control This register is used to disable certain aspects of functionality within the processor"]
    pub actlr: ACTLR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick."]
    pub stcsr: STCSR,
    #[doc = "0x14 - SysTick Reload Value This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0."]
    pub strvr: STRVR,
    #[doc = "0x18 - SysTick Current Value Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as STCSR.COUNTFLAG)."]
    pub stcvr: STCVR,
    #[doc = "0x1c - SysTick Calibration Value Used to enable software to scale to any required speed using divide and multiply."]
    pub stcr: STCR,
    _reserved1: [u8; 224usize],
    #[doc = "0x100 - Irq 0 to 31 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
    pub nvic_iser0: NVIC_ISER0,
    #[doc = "0x104 - Irq 32 to 63 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
    pub nvic_iser1: NVIC_ISER1,
    #[doc = "0x108 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved0: RESERVED0,
    _reserved2: [u8; 116usize],
    #[doc = "0x180 - Irq 0 to 31 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
    pub nvic_icer0: NVIC_ICER0,
    #[doc = "0x184 - Irq 32 to 63 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
    pub nvic_icer1: NVIC_ICER1,
    #[doc = "0x188 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved1: RESERVED1,
    _reserved3: [u8; 116usize],
    #[doc = "0x200 - Irq 0 to 31 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
    pub nvic_ispr0: NVIC_ISPR0,
    #[doc = "0x204 - Irq 32 to 63 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
    pub nvic_ispr1: NVIC_ISPR1,
    #[doc = "0x208 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved2: RESERVED2,
    _reserved4: [u8; 116usize],
    #[doc = "0x280 - Irq 0 to 31 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
    pub nvic_icpr0: NVIC_ICPR0,
    #[doc = "0x284 - Irq 32 to 63 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
    pub nvic_icpr1: NVIC_ICPR1,
    #[doc = "0x288 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved3: RESERVED3,
    _reserved5: [u8; 116usize],
    #[doc = "0x300 - Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
    pub nvic_iabr0: NVIC_IABR0,
    #[doc = "0x304 - Irq 32 to 63 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
    pub nvic_iabr1: NVIC_IABR1,
    #[doc = "0x308 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved4: RESERVED4,
    _reserved6: [u8; 244usize],
    #[doc = "0x400 - Irq 0 to 3 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr0: NVIC_IPR0,
    #[doc = "0x404 - Irq 4 to 7 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr1: NVIC_IPR1,
    #[doc = "0x408 - Irq 8 to 11 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr2: NVIC_IPR2,
    #[doc = "0x40c - Irq 12 to 15 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr3: NVIC_IPR3,
    #[doc = "0x410 - Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr4: NVIC_IPR4,
    #[doc = "0x414 - Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr5: NVIC_IPR5,
    #[doc = "0x418 - Irq 24 to 27 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr6: NVIC_IPR6,
    #[doc = "0x41c - Irq 28 to 31 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr7: NVIC_IPR7,
    #[doc = "0x420 - Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
    pub nvic_ipr8: NVIC_IPR8,
    #[doc = "0x424 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved5: RESERVED5,
    _reserved7: [u8; 2264usize],
    #[doc = "0xd00 - CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core."]
    pub cpuid: CPUID,
    #[doc = "0xd04 - Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception."]
    pub icsr: ICSR,
    #[doc = "0xd08 - Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF."]
    pub vtor: VTOR,
    #[doc = "0xd0c - Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point)."]
    pub aircr: AIRCR,
    #[doc = "0xd10 - System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states."]
    pub scr: SCR,
    #[doc = "0xd14 - Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode."]
    pub ccr: CCR,
    #[doc = "0xd18 - System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    pub shpr1: SHPR1,
    #[doc = "0xd1c - System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    pub shpr2: SHPR2,
    #[doc = "0xd20 - System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
    pub shpr3: SHPR3,
    #[doc = "0xd24 - System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault."]
    pub shcsr: SHCSR,
    #[doc = "0xd28 - Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A."]
    pub cfsr: CFSR,
    #[doc = "0xd2c - Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit."]
    pub hfsr: HFSR,
    #[doc = "0xd30 - Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored."]
    pub dfsr: DFSR,
    #[doc = "0xd34 - Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault."]
    pub mmfar: MMFAR,
    #[doc = "0xd38 - Bus Fault Address This register is used to read the address of the location that generated a Bus Fault."]
    pub bfar: BFAR,
    #[doc = "0xd3c - Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0."]
    pub afsr: AFSR,
    #[doc = "0xd40 - Processor Feature 0"]
    pub id_pfr0: ID_PFR0,
    #[doc = "0xd44 - Processor Feature 1"]
    pub id_pfr1: ID_PFR1,
    #[doc = "0xd48 - Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself."]
    pub id_dfr0: ID_DFR0,
    #[doc = "0xd4c - Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M."]
    pub id_afr0: ID_AFR0,
    #[doc = "0xd50 - Memory Model Feature 0 General information on the memory model and memory management support."]
    pub id_mmfr0: ID_MMFR0,
    #[doc = "0xd54 - Memory Model Feature 1 General information on the memory model and memory management support."]
    pub id_mmfr1: ID_MMFR1,
    #[doc = "0xd58 - Memory Model Feature 2 General information on the memory model and memory management support."]
    pub id_mmfr2: ID_MMFR2,
    #[doc = "0xd5c - Memory Model Feature 3 General information on the memory model and memory management support."]
    pub id_mmfr3: ID_MMFR3,
    #[doc = "0xd60 - ISA Feature 0 Information on the instruction set attributes register"]
    pub id_isar0: ID_ISAR0,
    #[doc = "0xd64 - ISA Feature 1 Information on the instruction set attributes register"]
    pub id_isar1: ID_ISAR1,
    #[doc = "0xd68 - ISA Feature 2 Information on the instruction set attributes register"]
    pub id_isar2: ID_ISAR2,
    #[doc = "0xd6c - ISA Feature 3 Information on the instruction set attributes register"]
    pub id_isar3: ID_ISAR3,
    #[doc = "0xd70 - ISA Feature 4 Information on the instruction set attributes register"]
    pub id_isar4: ID_ISAR4,
    _reserved8: [u8; 20usize],
    #[doc = "0xd88 - Coprocessor Access Control This register specifies the access privileges for coprocessors."]
    pub cpacr: CPACR,
    _reserved9: [u8; 4usize],
    #[doc = "0xd90 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved6: RESERVED6,
    _reserved10: [u8; 92usize],
    #[doc = "0xdf0 - Debug Halting Control and Status The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit. Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1): C_HALT=0, C_STEP=0, C_MASKINTS=0 Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=0, C_MASKINTS=1 Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=0 Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=1 Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=1, C_STEP=x, C_MASKINTS=x Remain in Debug state"]
    pub dhcsr: DHCSR,
    #[doc = "0xdf4 - Deubg Core Register Selector The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable. Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed."]
    pub dcrsr: DCRSR,
    #[doc = "0xdf8 - Debug Core Register Data"]
    pub dcrdr: DCRDR,
    #[doc = "0xdfc - Debug Exception and Monitor Control The purpose of this register is vector catching and debug monitor control. This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
    pub demcr: DEMCR,
    _reserved11: [u8; 256usize],
    #[doc = "0xf00 - Software Trigger Interrupt"]
    pub stir: STIR,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED000 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved000;
#[doc = "Interrupt Control Type Read this register to see the number of interrupt lines that the NVIC supports."]
pub struct ICTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Control Type Read this register to see the number of interrupt lines that the NVIC supports."]
pub mod ictr;
#[doc = "Auxiliary Control This register is used to disable certain aspects of functionality within the processor"]
pub struct ACTLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Control This register is used to disable certain aspects of functionality within the processor"]
pub mod actlr;
#[doc = "SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick."]
pub struct STCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Control and Status This register enables the SysTick features and returns status flags related to SysTick."]
pub mod stcsr;
#[doc = "SysTick Reload Value This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0."]
pub struct STRVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Reload Value This register is used to specify the start value to load into the current value register STCVR.CURRENT when the counter reaches 0. It can be any value between 1 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and STCSR.COUNTFLAG are activated when counting from 1 to 0."]
pub mod strvr;
#[doc = "SysTick Current Value Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as STCSR.COUNTFLAG)."]
pub struct STCVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Current Value Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as STCSR.COUNTFLAG)."]
pub mod stcvr;
#[doc = "SysTick Calibration Value Used to enable software to scale to any required speed using divide and multiply."]
pub struct STCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick Calibration Value Used to enable software to scale to any required speed using divide and multiply."]
pub mod stcr;
#[doc = "Irq 0 to 31 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
pub struct NVIC_ISER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 0 to 31 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_iser0;
#[doc = "Irq 32 to 63 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
pub struct NVIC_ISER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 32 to 63 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_iser1;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved0;
#[doc = "Irq 0 to 31 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
pub struct NVIC_ICER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 0 to 31 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_icer0;
#[doc = "Irq 32 to 63 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
pub struct NVIC_ICER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 32 to 63 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled."]
pub mod nvic_icer1;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved1;
#[doc = "Irq 0 to 31 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
pub struct NVIC_ISPR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 0 to 31 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
pub mod nvic_ispr0;
#[doc = "Irq 32 to 63 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
pub struct NVIC_ISPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 32 to 63 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending."]
pub mod nvic_ispr1;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved2;
#[doc = "Irq 0 to 31 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
pub struct NVIC_ICPR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 0 to 31 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
pub mod nvic_icpr0;
#[doc = "Irq 32 to 63 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
pub struct NVIC_ICPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 32 to 63 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending."]
pub mod nvic_icpr1;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved3;
#[doc = "Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
pub struct NVIC_IABR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
pub mod nvic_iabr0;
#[doc = "Irq 32 to 63 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
pub struct NVIC_IABR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 32 to 63 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt."]
pub mod nvic_iabr1;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved4;
#[doc = "Irq 0 to 3 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub struct NVIC_IPR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 0 to 3 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr0;
#[doc = "Irq 4 to 7 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub struct NVIC_IPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 4 to 7 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr1;
#[doc = "Irq 8 to 11 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub struct NVIC_IPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 8 to 11 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr2;
#[doc = "Irq 12 to 15 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub struct NVIC_IPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 12 to 15 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr3;
#[doc = "Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub struct NVIC_IPR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr4;
#[doc = "Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub struct NVIC_IPR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr5;
#[doc = "Irq 24 to 27 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub struct NVIC_IPR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 24 to 27 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr6;
#[doc = "Irq 28 to 31 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub struct NVIC_IPR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 28 to 31 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr7;
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub struct NVIC_IPR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP."]
pub mod nvic_ipr8;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved5;
#[doc = "CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core."]
pub struct CPUID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core."]
pub mod cpuid;
#[doc = "Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception."]
pub struct ICSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Control State This register is used to set a pending Non-Maskable Interrupt (NMI), set or clear a pending SVC, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, and check the vector number of the active exception."]
pub mod icsr;
#[doc = "Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF."]
pub struct VTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Vector Table Offset This register is used to relocated the vector table base address. The vector table base offset determines the offset from the bottom of the memory map. The two most significant bits and the seven least significant bits of the vector table base offset must be 0. The portion of vector table base offset that is allowed to change is TBLOFF."]
pub mod vtor;
#[doc = "Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point)."]
pub struct AIRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Application Interrupt/Reset Control This register is used to determine data endianness, clear all active state information for debug or to recover from a hard failure, execute a system reset, alter the priority grouping position (binary point)."]
pub mod aircr;
#[doc = "System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states."]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Control This register is used for power-management functions, i.e., signaling to the system when the processor can enter a low power state, controlling how the processor enters and exits low power states."]
pub mod scr;
#[doc = "Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode."]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Control This register is used to enable NMI, HardFault and FAULTMASK to ignore bus fault, trap divide by zero and unaligned accesses, enable user access to the Software Trigger Interrupt Register (STIR), control entry to Thread Mode."]
pub mod ccr;
#[doc = "System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub struct SHPR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handlers 4-7 Priority This register is used to prioritize the following system handlers: Memory manage, Bus fault, and Usage fault. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub mod shpr1;
#[doc = "System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub struct SHPR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handlers 8-11 Priority This register is used to prioritize the SVC handler. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub mod shpr2;
#[doc = "System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub struct SHPR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handlers 12-15 Priority This register is used to prioritize the following system handlers: SysTick, PendSV and Debug Monitor. System Handlers are a special class of exception handler that can have their priority set to any of the priority levels. Most can be masked on (enabled) or off (disabled). When disabled, the fault is always treated as a Hard Fault."]
pub mod shpr3;
#[doc = "System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault."]
pub struct SHCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Handler Control and State This register is used to enable or disable the system handlers, determine the pending status of bus fault, mem manage fault, and SVC, determine the active status of the system handlers. If a fault condition occurs while its fault handler is disabled, the fault escalates to a Hard Fault."]
pub mod shcsr;
#[doc = "Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A."]
pub struct CFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configurable Fault Status This register is used to obtain information about local faults. These registers include three subsections: The first byte is Memory Manage Fault Status Register (MMFSR). The second byte is Bus Fault Status Register (BFSR). The higher half-word is Usage Fault Status Register (UFSR). The flags in these registers indicate the causes of local faults. Multiple flags can be set if more than one fault occurs. These register are read/write-clear. This means that they can be read normally, but writing a 1 to any bit clears that bit. The CFSR is byte accessible. CFSR or its subregisters can be accessed as follows: The following accesses are possible to the CFSR register: - access the complete register with a word access to 0xE000ED28. - access the MMFSR with a byte access to 0xE000ED28 - access the MMFSR and BFSR with a halfword access to 0xE000ED28 - access the BFSR with a byte access to 0xE000ED29 - access the UFSR with a halfword access to 0xE000ED2A."]
pub mod cfsr;
#[doc = "Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit."]
pub struct HFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hard Fault Status This register is used to obtain information about events that activate the Hard Fault handler. This register is a write-clear register. This means that writing a 1 to a bit clears that bit."]
pub mod hfsr;
#[doc = "Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored."]
pub struct DFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Fault Status This register is used to monitor external debug requests, vector catches, data watchpoint match, BKPT instruction execution, halt requests. Multiple flags in the Debug Fault Status Register can be set when multiple fault conditions occur. The register is read/write clear. This means that it can be read normally. Writing a 1 to a bit clears that bit. Note that these bits are not set unless the event is caught. This means that it causes a stop of some sort. If halting debug is enabled, these events stop the processor into debug. If debug is disabled and the debug monitor is enabled, then this becomes a debug monitor handler call, if priority permits. If debug and the monitor are both disabled, some of these events are Hard Faults, and some are ignored."]
pub mod dfsr;
#[doc = "Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault."]
pub struct MMFAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault."]
pub mod mmfar;
#[doc = "Bus Fault Address This register is used to read the address of the location that generated a Bus Fault."]
pub struct BFAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bus Fault Address This register is used to read the address of the location that generated a Bus Fault."]
pub mod bfar;
#[doc = "Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0."]
pub struct AFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Fault Status This register is used to determine additional system fault information to software. Single-cycle high level on an auxiliary faults is latched as one. The bit can only be cleared by writing a one to the corresponding bit. Auxiliary fault inputs to the CPU are tied to 0."]
pub mod afsr;
#[doc = "Processor Feature 0"]
pub struct ID_PFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor Feature 0"]
pub mod id_pfr0;
#[doc = "Processor Feature 1"]
pub struct ID_PFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Processor Feature 1"]
pub mod id_pfr1;
#[doc = "Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself."]
pub struct ID_DFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself."]
pub mod id_dfr0;
#[doc = "Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M."]
pub struct ID_AFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M."]
pub mod id_afr0;
#[doc = "Memory Model Feature 0 General information on the memory model and memory management support."]
pub struct ID_MMFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Model Feature 0 General information on the memory model and memory management support."]
pub mod id_mmfr0;
#[doc = "Memory Model Feature 1 General information on the memory model and memory management support."]
pub struct ID_MMFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Model Feature 1 General information on the memory model and memory management support."]
pub mod id_mmfr1;
#[doc = "Memory Model Feature 2 General information on the memory model and memory management support."]
pub struct ID_MMFR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Model Feature 2 General information on the memory model and memory management support."]
pub mod id_mmfr2;
#[doc = "Memory Model Feature 3 General information on the memory model and memory management support."]
pub struct ID_MMFR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Model Feature 3 General information on the memory model and memory management support."]
pub mod id_mmfr3;
#[doc = "ISA Feature 0 Information on the instruction set attributes register"]
pub struct ID_ISAR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISA Feature 0 Information on the instruction set attributes register"]
pub mod id_isar0;
#[doc = "ISA Feature 1 Information on the instruction set attributes register"]
pub struct ID_ISAR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISA Feature 1 Information on the instruction set attributes register"]
pub mod id_isar1;
#[doc = "ISA Feature 2 Information on the instruction set attributes register"]
pub struct ID_ISAR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISA Feature 2 Information on the instruction set attributes register"]
pub mod id_isar2;
#[doc = "ISA Feature 3 Information on the instruction set attributes register"]
pub struct ID_ISAR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISA Feature 3 Information on the instruction set attributes register"]
pub mod id_isar3;
#[doc = "ISA Feature 4 Information on the instruction set attributes register"]
pub struct ID_ISAR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISA Feature 4 Information on the instruction set attributes register"]
pub mod id_isar4;
#[doc = "Coprocessor Access Control This register specifies the access privileges for coprocessors."]
pub struct CPACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Coprocessor Access Control This register specifies the access privileges for coprocessors."]
pub mod cpacr;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved6;
#[doc = "Debug Halting Control and Status The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit. Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1): C_HALT=0, C_STEP=0, C_MASKINTS=0 Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=0, C_MASKINTS=1 Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=0 Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=1 Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=1, C_STEP=x, C_MASKINTS=x Remain in Debug state"]
pub struct DHCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Halting Control and Status The purpose of this register is to provide status information about the state of the processor, enable core debug, halt and step the processor. For writes, 0xA05F must be written to higher half-word of this register, otherwise the write operation is ignored and no bits are written into the register. If not enabled for Halting mode, C_DEBUGEN = 1, all other fields are disabled. This register is not reset on a core reset. It is reset by a power-on reset. However, C_HALT always clears on a core reset. To halt on a reset, the following bits must be enabled: DEMCR.VC_CORERESET and C_DEBUGEN. Note that writes to this register in any size other than word are unpredictable. It is acceptable to read in any size, and it can be used to avoid or intentionally change a sticky bit. Behavior of the system when writing to this register while CPU is halted (i.e. C_DEBUGEN = 1 and S_HALT= 1): C_HALT=0, C_STEP=0, C_MASKINTS=0 Exit Debug state and start instruction execution. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=0, C_MASKINTS=1 Exit Debug state and start instruction execution. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=0 Exit Debug state, step an instruction and halt. Exceptions activate according to the exception configuration rules. C_HALT=0, C_STEP=1, C_MASKINTS=1 Exit Debug state, step an instruction and halt. PendSV, SysTick and external configurable interrupts are disabled, otherwise exceptions activate according to standard configuration rules. C_HALT=1, C_STEP=x, C_MASKINTS=x Remain in Debug state"]
pub mod dhcsr;
#[doc = "Deubg Core Register Selector The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable. Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed."]
pub struct DCRSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deubg Core Register Selector The purpose of this register is to select the processor register to transfer data to or from. This write-only register generates a handshake to the core to transfer data to or from Debug Core Register Data Register and the selected register. Until this core transaction is complete, DHCSR.S_REGRDY is 0. Note that writes to this register in any size but word are Unpredictable. Note that PSR registers are fully accessible this way, whereas some read as 0 when using MRS instructions. Note that all bits can be written, but some combinations cause a fault when execution is resumed."]
pub mod dcrsr;
#[doc = "Debug Core Register Data"]
pub struct DCRDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Core Register Data"]
pub mod dcrdr;
#[doc = "Debug Exception and Monitor Control The purpose of this register is vector catching and debug monitor control. This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
pub struct DEMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Exception and Monitor Control The purpose of this register is vector catching and debug monitor control. This register manages exception behavior under debug. Vector catching is only available to halting debug. The upper halfword is for monitor controls and the lower halfword is for halting exception support. This register is not reset on a system reset. This register is reset by a power-on reset. The fields MON_EN, MON_PEND, MON_STEP and MON_REQ are always cleared on a core reset. The debug monitor is enabled by software in the reset handler or later, or by the **AHB-AP** port. Vector catching is semi-synchronous. When a matching event is seen, a Halt is requested. Because the processor can only halt on an instruction boundary, it must wait until the next instruction boundary. As a result, it stops on the first instruction of the exception handler. However, two special cases exist when a vector catch has triggered: 1. If a fault is taken during a vector read or stack push error the halt occurs on the corresponding fault handler for the vector error or stack push. 2. If a late arriving interrupt detected during a vector read or stack push error it is not taken. That is, an implementation that supports the late arrival optimization must suppress it in this case."]
pub mod demcr;
#[doc = "Software Trigger Interrupt"]
pub struct STIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software Trigger Interrupt"]
pub mod stir;
