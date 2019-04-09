#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Stimulus Port 0"]
    pub stim0: STIM0,
    #[doc = "0x04 - Stimulus Port 1"]
    pub stim1: STIM1,
    #[doc = "0x08 - Stimulus Port 2"]
    pub stim2: STIM2,
    #[doc = "0x0c - Stimulus Port 3"]
    pub stim3: STIM3,
    #[doc = "0x10 - Stimulus Port 4"]
    pub stim4: STIM4,
    #[doc = "0x14 - Stimulus Port 5"]
    pub stim5: STIM5,
    #[doc = "0x18 - Stimulus Port 6"]
    pub stim6: STIM6,
    #[doc = "0x1c - Stimulus Port 7"]
    pub stim7: STIM7,
    #[doc = "0x20 - Stimulus Port 8"]
    pub stim8: STIM8,
    #[doc = "0x24 - Stimulus Port 9"]
    pub stim9: STIM9,
    #[doc = "0x28 - Stimulus Port 10"]
    pub stim10: STIM10,
    #[doc = "0x2c - Stimulus Port 11"]
    pub stim11: STIM11,
    #[doc = "0x30 - Stimulus Port 12"]
    pub stim12: STIM12,
    #[doc = "0x34 - Stimulus Port 13"]
    pub stim13: STIM13,
    #[doc = "0x38 - Stimulus Port 14"]
    pub stim14: STIM14,
    #[doc = "0x3c - Stimulus Port 15"]
    pub stim15: STIM15,
    #[doc = "0x40 - Stimulus Port 16"]
    pub stim16: STIM16,
    #[doc = "0x44 - Stimulus Port 17"]
    pub stim17: STIM17,
    #[doc = "0x48 - Stimulus Port 18"]
    pub stim18: STIM18,
    #[doc = "0x4c - Stimulus Port 19"]
    pub stim19: STIM19,
    #[doc = "0x50 - Stimulus Port 20"]
    pub stim20: STIM20,
    #[doc = "0x54 - Stimulus Port 21"]
    pub stim21: STIM21,
    #[doc = "0x58 - Stimulus Port 22"]
    pub stim22: STIM22,
    #[doc = "0x5c - Stimulus Port 23"]
    pub stim23: STIM23,
    #[doc = "0x60 - Stimulus Port 24"]
    pub stim24: STIM24,
    #[doc = "0x64 - Stimulus Port 25"]
    pub stim25: STIM25,
    #[doc = "0x68 - Stimulus Port 26"]
    pub stim26: STIM26,
    #[doc = "0x6c - Stimulus Port 27"]
    pub stim27: STIM27,
    #[doc = "0x70 - Stimulus Port 28"]
    pub stim28: STIM28,
    #[doc = "0x74 - Stimulus Port 29"]
    pub stim29: STIM29,
    #[doc = "0x78 - Stimulus Port 30"]
    pub stim30: STIM30,
    #[doc = "0x7c - Stimulus Port 31"]
    pub stim31: STIM31,
    _reserved0: [u8; 3456usize],
    #[doc = "0xe00 - Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required."]
    pub ter: TER,
    _reserved1: [u8; 60usize],
    #[doc = "0xe40 - Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode."]
    pub tpr: TPR,
    _reserved2: [u8; 60usize],
    #[doc = "0xe80 - Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set."]
    pub tcr: TCR,
    _reserved3: [u8; 300usize],
    #[doc = "0xfb0 - Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR."]
    pub lar: LAR,
    #[doc = "0xfb4 - Lock Status Use this register to enable write accesses to the Control Register."]
    pub lsr: LSR,
}
#[doc = "Stimulus Port 0"]
pub struct STIM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 0"]
pub mod stim0;
#[doc = "Stimulus Port 1"]
pub struct STIM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 1"]
pub mod stim1;
#[doc = "Stimulus Port 2"]
pub struct STIM2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 2"]
pub mod stim2;
#[doc = "Stimulus Port 3"]
pub struct STIM3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 3"]
pub mod stim3;
#[doc = "Stimulus Port 4"]
pub struct STIM4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 4"]
pub mod stim4;
#[doc = "Stimulus Port 5"]
pub struct STIM5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 5"]
pub mod stim5;
#[doc = "Stimulus Port 6"]
pub struct STIM6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 6"]
pub mod stim6;
#[doc = "Stimulus Port 7"]
pub struct STIM7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 7"]
pub mod stim7;
#[doc = "Stimulus Port 8"]
pub struct STIM8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 8"]
pub mod stim8;
#[doc = "Stimulus Port 9"]
pub struct STIM9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 9"]
pub mod stim9;
#[doc = "Stimulus Port 10"]
pub struct STIM10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 10"]
pub mod stim10;
#[doc = "Stimulus Port 11"]
pub struct STIM11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 11"]
pub mod stim11;
#[doc = "Stimulus Port 12"]
pub struct STIM12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 12"]
pub mod stim12;
#[doc = "Stimulus Port 13"]
pub struct STIM13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 13"]
pub mod stim13;
#[doc = "Stimulus Port 14"]
pub struct STIM14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 14"]
pub mod stim14;
#[doc = "Stimulus Port 15"]
pub struct STIM15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 15"]
pub mod stim15;
#[doc = "Stimulus Port 16"]
pub struct STIM16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 16"]
pub mod stim16;
#[doc = "Stimulus Port 17"]
pub struct STIM17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 17"]
pub mod stim17;
#[doc = "Stimulus Port 18"]
pub struct STIM18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 18"]
pub mod stim18;
#[doc = "Stimulus Port 19"]
pub struct STIM19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 19"]
pub mod stim19;
#[doc = "Stimulus Port 20"]
pub struct STIM20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 20"]
pub mod stim20;
#[doc = "Stimulus Port 21"]
pub struct STIM21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 21"]
pub mod stim21;
#[doc = "Stimulus Port 22"]
pub struct STIM22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 22"]
pub mod stim22;
#[doc = "Stimulus Port 23"]
pub struct STIM23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 23"]
pub mod stim23;
#[doc = "Stimulus Port 24"]
pub struct STIM24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 24"]
pub mod stim24;
#[doc = "Stimulus Port 25"]
pub struct STIM25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 25"]
pub mod stim25;
#[doc = "Stimulus Port 26"]
pub struct STIM26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 26"]
pub mod stim26;
#[doc = "Stimulus Port 27"]
pub struct STIM27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 27"]
pub mod stim27;
#[doc = "Stimulus Port 28"]
pub struct STIM28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 28"]
pub mod stim28;
#[doc = "Stimulus Port 29"]
pub struct STIM29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 29"]
pub mod stim29;
#[doc = "Stimulus Port 30"]
pub struct STIM30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 30"]
pub mod stim30;
#[doc = "Stimulus Port 31"]
pub struct STIM31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stimulus Port 31"]
pub mod stim31;
#[doc = "Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required."]
pub struct TER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required."]
pub mod ter;
#[doc = "Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode."]
pub struct TPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode."]
pub mod tpr;
#[doc = "Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set."]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set."]
pub mod tcr;
#[doc = "Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR."]
pub struct LAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR."]
pub mod lar;
#[doc = "Lock Status Use this register to enable write accesses to the Control Register."]
pub struct LSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock Status Use this register to enable write accesses to the Control Register."]
pub mod lsr;
