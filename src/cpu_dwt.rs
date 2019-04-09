#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Use the DWT Control Register to enable the DWT unit."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz)."]
    pub cyccnt: CYCCNT,
    #[doc = "0x08 - CPI Count This register is used to count the total number of instruction cycles beyond the first cycle."]
    pub cpicnt: CPICNT,
    #[doc = "0x0c - Exception Overhead Count This register is used to count the total cycles spent in interrupt processing."]
    pub exccnt: EXCCNT,
    #[doc = "0x10 - Sleep Count This register is used to count the total number of cycles during which the processor is sleeping."]
    pub sleepcnt: SLEEPCNT,
    #[doc = "0x14 - LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle."]
    pub lsucnt: LSUCNT,
    #[doc = "0x18 - Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles."]
    pub foldcnt: FOLDCNT,
    #[doc = "0x1c - Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF."]
    pub pcsr: PCSR,
    #[doc = "0x20 - Comparator 0 This register is used to write the reference value for comparator 0."]
    pub comp0: COMP0,
    #[doc = "0x24 - Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0."]
    pub mask0: MASK0,
    #[doc = "0x28 - Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    pub function0: FUNCTION0,
    _reserved0: [u8; 4usize],
    #[doc = "0x30 - Comparator 1 This register is used to write the reference value for comparator 1."]
    pub comp1: COMP1,
    #[doc = "0x34 - Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1."]
    pub mask1: MASK1,
    #[doc = "0x38 - Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    pub function1: FUNCTION1,
    _reserved1: [u8; 4usize],
    #[doc = "0x40 - Comparator 2 This register is used to write the reference value for comparator 2."]
    pub comp2: COMP2,
    #[doc = "0x44 - Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2."]
    pub mask2: MASK2,
    #[doc = "0x48 - Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    pub function2: FUNCTION2,
    _reserved2: [u8; 4usize],
    #[doc = "0x50 - Comparator 3 This register is used to write the reference value for comparator 3."]
    pub comp3: COMP3,
    #[doc = "0x54 - Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3."]
    pub mask3: MASK3,
    #[doc = "0x58 - Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
    pub function3: FUNCTION3,
}
#[doc = "Control Use the DWT Control Register to enable the DWT unit."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Use the DWT Control Register to enable the DWT unit."]
pub mod ctrl;
#[doc = "Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz)."]
pub struct CYCCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz)."]
pub mod cyccnt;
#[doc = "CPI Count This register is used to count the total number of instruction cycles beyond the first cycle."]
pub struct CPICNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPI Count This register is used to count the total number of instruction cycles beyond the first cycle."]
pub mod cpicnt;
#[doc = "Exception Overhead Count This register is used to count the total cycles spent in interrupt processing."]
pub struct EXCCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Exception Overhead Count This register is used to count the total cycles spent in interrupt processing."]
pub mod exccnt;
#[doc = "Sleep Count This register is used to count the total number of cycles during which the processor is sleeping."]
pub struct SLEEPCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sleep Count This register is used to count the total number of cycles during which the processor is sleeping."]
pub mod sleepcnt;
#[doc = "LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle."]
pub struct LSUCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle."]
pub mod lsucnt;
#[doc = "Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles."]
pub struct FOLDCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fold Count This register is used to count the total number of folded instructions. The counter increments on each instruction which takes 0 cycles."]
pub mod foldcnt;
#[doc = "Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF."]
pub struct PCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF."]
pub mod pcsr;
#[doc = "Comparator 0 This register is used to write the reference value for comparator 0."]
pub struct COMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 0 This register is used to write the reference value for comparator 0."]
pub mod comp0;
#[doc = "Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0."]
pub struct MASK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask 0 Use the DWT Mask Registers 0 to apply a mask to data addresses when matching against COMP0."]
pub mod mask0;
#[doc = "Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub struct FUNCTION0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Function 0 Use the DWT Function Registers 0 to control the operation of the comparator 0. This comparator can: 1. Match against either the PC or the data address. This is controlled by CYCMATCH. This function is only available for comparator 0 (COMP0). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function0;
#[doc = "Comparator 1 This register is used to write the reference value for comparator 1."]
pub struct COMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 1 This register is used to write the reference value for comparator 1."]
pub mod comp1;
#[doc = "Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1."]
pub struct MASK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask 1 Use the DWT Mask Registers 1 to apply a mask to data addresses when matching against COMP1."]
pub mod mask1;
#[doc = "Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub struct FUNCTION1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Function 1 Use the DWT Function Registers 1 to control the operation of the comparator 1. This comparator can: 1. Perform data value comparisons if associated address comparators have performed an address match. This function is only available for comparator 1 (COMP1). 2. Emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function1;
#[doc = "Comparator 2 This register is used to write the reference value for comparator 2."]
pub struct COMP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 2 This register is used to write the reference value for comparator 2."]
pub mod comp2;
#[doc = "Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2."]
pub struct MASK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask 2 Use the DWT Mask Registers 2 to apply a mask to data addresses when matching against COMP2."]
pub mod mask2;
#[doc = "Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub struct FUNCTION2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Function 2 Use the DWT Function Registers 2 to control the operation of the comparator 2. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function2;
#[doc = "Comparator 3 This register is used to write the reference value for comparator 3."]
pub struct COMP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 3 This register is used to write the reference value for comparator 3."]
pub mod comp3;
#[doc = "Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3."]
pub struct MASK3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask 3 Use the DWT Mask Registers 3 to apply a mask to data addresses when matching against COMP3."]
pub mod mask3;
#[doc = "Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub struct FUNCTION3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Function 3 Use the DWT Function Registers 3 to control the operation of the comparator 3. This comparator can emit data or PC couples, trigger the ETM, or generate a watchpoint depending on the operation defined by FUNCTION."]
pub mod function3;
