#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UDMACH14BSEL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVR {
    #[doc = "Always asserted"]
    ALWAYS_ACTIVE,
    #[doc = "CPU halted "]
    CPU_HALTED,
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    AON_RTC_UPD,
    #[doc = "DMA burst request event from AUX, configured by AUX_EVCTL:DMACTL"]
    AUX_DMABREQ,
    #[doc = "DMA single request event from AUX, configured by AUX_EVCTL:DMACTL"]
    AUX_DMASREQ,
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    AUX_SW_DMABREQ,
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    AUX_ADC_IRQ,
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0\n\n"]
    AUX_OBSMUX0,
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL"]
    AUX_ADC_FIFO_ALMOST_FULL,
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE"]
    AUX_ADC_DONE,
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    AUX_SMPH_AUTOTAKE_DONE,
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV"]
    AUX_TIMER1_EV,
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV"]
    AUX_TIMER0_EV,
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    AUX_TDC_DONE,
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV"]
    AUX_AON_WU_EV,
    #[doc = "TRNG Interrupt event, controlled by TRNG:IRQEN.EN"]
    TRNG_IRQ,
    #[doc = "Software event 3, triggered by SWEV.SWEV3"]
    SWEV3,
    #[doc = "Software event 2, triggered by SWEV.SWEV2"]
    SWEV2,
    #[doc = "Software event 1, triggered by SWEV.SWEV1"]
    SWEV1,
    #[doc = "Software event 0, triggered by SWEV.SWEV0"]
    SWEV0,
    #[doc = "Watchdog non maskable interrupt event, controlled by WDT:CTL.INTTYPE"]
    WDT_NMI,
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    CRYPTO_DMA_DONE_IRQ,
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    CRYPTO_RESULT_AVAIL_IRQ,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here."]
    PORT_EVENT7,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here."]
    PORT_EVENT6,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT5,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT4,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    PORT_EVENT3,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    PORT_EVENT2,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT1 wil be routed here."]
    PORT_EVENT1,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT0 wil be routed here. "]
    PORT_EVENT0,
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    GPT3B_DMABREQ,
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    GPT3A_DMABREQ,
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    GPT2B_DMABREQ,
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    GPT2A_DMABREQ,
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    GPT1B_DMABREQ,
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    GPT1A_DMABREQ,
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    GPT0B_DMABREQ,
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    GPT0A_DMABREQ,
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    GPT3B_CMP,
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    GPT3A_CMP,
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    GPT2B_CMP,
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    GPT2A_CMP,
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    GPT1B_CMP,
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    GPT1A_CMP,
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    GPT0B_CMP,
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    GPT0A_CMP,
    #[doc = "UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    UART0_TX_DMASREQ,
    #[doc = "UART0 TX DMA burst request, controlled by UART0:DMACTL.TXDMAE"]
    UART0_TX_DMABREQ,
    #[doc = "UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE"]
    UART0_RX_DMASREQ,
    #[doc = "UART0 RX DMA burst request, controlled by UART0:DMACTL.RXDMAE"]
    UART0_RX_DMABREQ,
    #[doc = "SSI1 TX DMA single request, controlled by SSI0:DMACR.TXDMAE "]
    SSI1_TX_DMASREQ,
    #[doc = "SSI1 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE "]
    SSI1_TX_DMABREQ,
    #[doc = "SSI1 RX DMA single request, controlled by SSI0:DMACR.RXDMAE "]
    SSI1_RX_DMASREQ,
    #[doc = "SSI1 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE "]
    SSI1_RX_DMABREQ,
    #[doc = "SSI0 TX DMA single request, controlled by SSI0:DMACR.TXDMAE "]
    SSI0_TX_DMASREQ,
    #[doc = "SSI0 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE "]
    SSI0_TX_DMABREQ,
    #[doc = "SSI0 RX DMA single request, controlled by SSI0:DMACR.RXDMAE "]
    SSI0_RX_DMASREQ,
    #[doc = "SSI0 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE "]
    SSI0_RX_DMABREQ,
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE "]
    DMA_DONE_COMB,
    #[doc = "DMA bus error, corresponds to UDMA0:ERROR.STATUS"]
    DMA_ERR,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    UART0_COMB,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    SSI1_COMB,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    SSI0_COMB,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    RFC_CPE_1,
    #[doc = "AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event.\nMCU domain wakeup control AON_EVENT:MCUWUSEL\nAUX domain wakeup control AON_EVENT:AUXWUSEL\n\n"]
    AUX_SWEV1,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    RFC_CPE_0,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    RFC_HW_COMB,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    RFC_CMD_ACK,
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    WDT_IRQ,
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    DMA_CH18_DONE,
    #[doc = "FLASH controller error event,  the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    FLASH,
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    DMA_CH0_DONE,
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    GPT1B,
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    GPT1A,
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    GPT0B,
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    GPT0A,
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    GPT3B,
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    GPT3A,
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    GPT2B,
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    GPT2A,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    AUX_COMB,
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    AON_AUX_SWEV0,
    #[doc = "Interrupt event from I2C"]
    I2C_IRQ,
    #[doc = "Interrupt event from I2S"]
    I2S_IRQ,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    AON_RTC_COMB,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and  IOC:IOCFGn.EDGE_DET settings"]
    AON_GPIO_EDGE,
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    AON_PROG2,
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    AON_PROG1,
    #[doc = "AON programmable event 0. Event selected by AON_EVENT  MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV"]
    AON_PROG0,
    #[doc = "Always inactive"]
    NONE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EVR::ALWAYS_ACTIVE => 121,
            EVR::CPU_HALTED => 120,
            EVR::AON_RTC_UPD => 119,
            EVR::AUX_DMABREQ => 118,
            EVR::AUX_DMASREQ => 117,
            EVR::AUX_SW_DMABREQ => 116,
            EVR::AUX_ADC_IRQ => 115,
            EVR::AUX_OBSMUX0 => 114,
            EVR::AUX_ADC_FIFO_ALMOST_FULL => 113,
            EVR::AUX_ADC_DONE => 112,
            EVR::AUX_SMPH_AUTOTAKE_DONE => 111,
            EVR::AUX_TIMER1_EV => 110,
            EVR::AUX_TIMER0_EV => 109,
            EVR::AUX_TDC_DONE => 108,
            EVR::AUX_COMPB => 107,
            EVR::AUX_COMPA => 106,
            EVR::AUX_AON_WU_EV => 105,
            EVR::TRNG_IRQ => 104,
            EVR::SWEV3 => 103,
            EVR::SWEV2 => 102,
            EVR::SWEV1 => 101,
            EVR::SWEV0 => 100,
            EVR::WDT_NMI => 99,
            EVR::CRYPTO_DMA_DONE_IRQ => 94,
            EVR::CRYPTO_RESULT_AVAIL_IRQ => 93,
            EVR::PORT_EVENT7 => 92,
            EVR::PORT_EVENT6 => 91,
            EVR::PORT_EVENT5 => 90,
            EVR::PORT_EVENT4 => 89,
            EVR::PORT_EVENT3 => 88,
            EVR::PORT_EVENT2 => 87,
            EVR::PORT_EVENT1 => 86,
            EVR::PORT_EVENT0 => 85,
            EVR::GPT3B_DMABREQ => 84,
            EVR::GPT3A_DMABREQ => 83,
            EVR::GPT2B_DMABREQ => 82,
            EVR::GPT2A_DMABREQ => 81,
            EVR::GPT1B_DMABREQ => 80,
            EVR::GPT1A_DMABREQ => 79,
            EVR::GPT0B_DMABREQ => 78,
            EVR::GPT0A_DMABREQ => 77,
            EVR::GPT3B_CMP => 68,
            EVR::GPT3A_CMP => 67,
            EVR::GPT2B_CMP => 66,
            EVR::GPT2A_CMP => 65,
            EVR::GPT1B_CMP => 64,
            EVR::GPT1A_CMP => 63,
            EVR::GPT0B_CMP => 62,
            EVR::GPT0A_CMP => 61,
            EVR::UART0_TX_DMASREQ => 51,
            EVR::UART0_TX_DMABREQ => 50,
            EVR::UART0_RX_DMASREQ => 49,
            EVR::UART0_RX_DMABREQ => 48,
            EVR::SSI1_TX_DMASREQ => 47,
            EVR::SSI1_TX_DMABREQ => 46,
            EVR::SSI1_RX_DMASREQ => 45,
            EVR::SSI1_RX_DMABREQ => 44,
            EVR::SSI0_TX_DMASREQ => 43,
            EVR::SSI0_TX_DMABREQ => 42,
            EVR::SSI0_RX_DMASREQ => 41,
            EVR::SSI0_RX_DMABREQ => 40,
            EVR::DMA_DONE_COMB => 39,
            EVR::DMA_ERR => 38,
            EVR::UART0_COMB => 36,
            EVR::SSI1_COMB => 35,
            EVR::SSI0_COMB => 34,
            EVR::RFC_CPE_1 => 30,
            EVR::AUX_SWEV1 => 29,
            EVR::RFC_CPE_0 => 27,
            EVR::RFC_HW_COMB => 26,
            EVR::RFC_CMD_ACK => 25,
            EVR::WDT_IRQ => 24,
            EVR::DMA_CH18_DONE => 22,
            EVR::FLASH => 21,
            EVR::DMA_CH0_DONE => 20,
            EVR::GPT1B => 19,
            EVR::GPT1A => 18,
            EVR::GPT0B => 17,
            EVR::GPT0A => 16,
            EVR::GPT3B => 15,
            EVR::GPT3A => 14,
            EVR::GPT2B => 13,
            EVR::GPT2A => 12,
            EVR::AUX_COMB => 11,
            EVR::AON_AUX_SWEV0 => 10,
            EVR::I2C_IRQ => 9,
            EVR::I2S_IRQ => 8,
            EVR::AON_RTC_COMB => 7,
            EVR::AON_GPIO_EDGE => 4,
            EVR::AON_PROG2 => 3,
            EVR::AON_PROG1 => 2,
            EVR::AON_PROG0 => 1,
            EVR::NONE => 0,
            EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EVR {
        match value {
            121 => EVR::ALWAYS_ACTIVE,
            120 => EVR::CPU_HALTED,
            119 => EVR::AON_RTC_UPD,
            118 => EVR::AUX_DMABREQ,
            117 => EVR::AUX_DMASREQ,
            116 => EVR::AUX_SW_DMABREQ,
            115 => EVR::AUX_ADC_IRQ,
            114 => EVR::AUX_OBSMUX0,
            113 => EVR::AUX_ADC_FIFO_ALMOST_FULL,
            112 => EVR::AUX_ADC_DONE,
            111 => EVR::AUX_SMPH_AUTOTAKE_DONE,
            110 => EVR::AUX_TIMER1_EV,
            109 => EVR::AUX_TIMER0_EV,
            108 => EVR::AUX_TDC_DONE,
            107 => EVR::AUX_COMPB,
            106 => EVR::AUX_COMPA,
            105 => EVR::AUX_AON_WU_EV,
            104 => EVR::TRNG_IRQ,
            103 => EVR::SWEV3,
            102 => EVR::SWEV2,
            101 => EVR::SWEV1,
            100 => EVR::SWEV0,
            99 => EVR::WDT_NMI,
            94 => EVR::CRYPTO_DMA_DONE_IRQ,
            93 => EVR::CRYPTO_RESULT_AVAIL_IRQ,
            92 => EVR::PORT_EVENT7,
            91 => EVR::PORT_EVENT6,
            90 => EVR::PORT_EVENT5,
            89 => EVR::PORT_EVENT4,
            88 => EVR::PORT_EVENT3,
            87 => EVR::PORT_EVENT2,
            86 => EVR::PORT_EVENT1,
            85 => EVR::PORT_EVENT0,
            84 => EVR::GPT3B_DMABREQ,
            83 => EVR::GPT3A_DMABREQ,
            82 => EVR::GPT2B_DMABREQ,
            81 => EVR::GPT2A_DMABREQ,
            80 => EVR::GPT1B_DMABREQ,
            79 => EVR::GPT1A_DMABREQ,
            78 => EVR::GPT0B_DMABREQ,
            77 => EVR::GPT0A_DMABREQ,
            68 => EVR::GPT3B_CMP,
            67 => EVR::GPT3A_CMP,
            66 => EVR::GPT2B_CMP,
            65 => EVR::GPT2A_CMP,
            64 => EVR::GPT1B_CMP,
            63 => EVR::GPT1A_CMP,
            62 => EVR::GPT0B_CMP,
            61 => EVR::GPT0A_CMP,
            51 => EVR::UART0_TX_DMASREQ,
            50 => EVR::UART0_TX_DMABREQ,
            49 => EVR::UART0_RX_DMASREQ,
            48 => EVR::UART0_RX_DMABREQ,
            47 => EVR::SSI1_TX_DMASREQ,
            46 => EVR::SSI1_TX_DMABREQ,
            45 => EVR::SSI1_RX_DMASREQ,
            44 => EVR::SSI1_RX_DMABREQ,
            43 => EVR::SSI0_TX_DMASREQ,
            42 => EVR::SSI0_TX_DMABREQ,
            41 => EVR::SSI0_RX_DMASREQ,
            40 => EVR::SSI0_RX_DMABREQ,
            39 => EVR::DMA_DONE_COMB,
            38 => EVR::DMA_ERR,
            36 => EVR::UART0_COMB,
            35 => EVR::SSI1_COMB,
            34 => EVR::SSI0_COMB,
            30 => EVR::RFC_CPE_1,
            29 => EVR::AUX_SWEV1,
            27 => EVR::RFC_CPE_0,
            26 => EVR::RFC_HW_COMB,
            25 => EVR::RFC_CMD_ACK,
            24 => EVR::WDT_IRQ,
            22 => EVR::DMA_CH18_DONE,
            21 => EVR::FLASH,
            20 => EVR::DMA_CH0_DONE,
            19 => EVR::GPT1B,
            18 => EVR::GPT1A,
            17 => EVR::GPT0B,
            16 => EVR::GPT0A,
            15 => EVR::GPT3B,
            14 => EVR::GPT3A,
            13 => EVR::GPT2B,
            12 => EVR::GPT2A,
            11 => EVR::AUX_COMB,
            10 => EVR::AON_AUX_SWEV0,
            9 => EVR::I2C_IRQ,
            8 => EVR::I2S_IRQ,
            7 => EVR::AON_RTC_COMB,
            4 => EVR::AON_GPIO_EDGE,
            3 => EVR::AON_PROG2,
            2 => EVR::AON_PROG1,
            1 => EVR::AON_PROG0,
            0 => EVR::NONE,
            i => EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline]
    pub fn is_always_active(&self) -> bool {
        *self == EVR::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `CPU_HALTED`"]
    #[inline]
    pub fn is_cpu_halted(&self) -> bool {
        *self == EVR::CPU_HALTED
    }
    #[doc = "Checks if the value of the field is `AON_RTC_UPD`"]
    #[inline]
    pub fn is_aon_rtc_upd(&self) -> bool {
        *self == EVR::AON_RTC_UPD
    }
    #[doc = "Checks if the value of the field is `AUX_DMABREQ`"]
    #[inline]
    pub fn is_aux_dmabreq(&self) -> bool {
        *self == EVR::AUX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `AUX_DMASREQ`"]
    #[inline]
    pub fn is_aux_dmasreq(&self) -> bool {
        *self == EVR::AUX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `AUX_SW_DMABREQ`"]
    #[inline]
    pub fn is_aux_sw_dmabreq(&self) -> bool {
        *self == EVR::AUX_SW_DMABREQ
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_IRQ`"]
    #[inline]
    pub fn is_aux_adc_irq(&self) -> bool {
        *self == EVR::AUX_ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `AUX_OBSMUX0`"]
    #[inline]
    pub fn is_aux_obsmux0(&self) -> bool {
        *self == EVR::AUX_OBSMUX0
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_ALMOST_FULL`"]
    #[inline]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == EVR::AUX_ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_SMPH_AUTOTAKE_DONE`"]
    #[inline]
    pub fn is_aux_smph_autotake_done(&self) -> bool {
        *self == EVR::AUX_SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_AON_WU_EV`"]
    #[inline]
    pub fn is_aux_aon_wu_ev(&self) -> bool {
        *self == EVR::AUX_AON_WU_EV
    }
    #[doc = "Checks if the value of the field is `TRNG_IRQ`"]
    #[inline]
    pub fn is_trng_irq(&self) -> bool {
        *self == EVR::TRNG_IRQ
    }
    #[doc = "Checks if the value of the field is `SWEV3`"]
    #[inline]
    pub fn is_swev3(&self) -> bool {
        *self == EVR::SWEV3
    }
    #[doc = "Checks if the value of the field is `SWEV2`"]
    #[inline]
    pub fn is_swev2(&self) -> bool {
        *self == EVR::SWEV2
    }
    #[doc = "Checks if the value of the field is `SWEV1`"]
    #[inline]
    pub fn is_swev1(&self) -> bool {
        *self == EVR::SWEV1
    }
    #[doc = "Checks if the value of the field is `SWEV0`"]
    #[inline]
    pub fn is_swev0(&self) -> bool {
        *self == EVR::SWEV0
    }
    #[doc = "Checks if the value of the field is `WDT_NMI`"]
    #[inline]
    pub fn is_wdt_nmi(&self) -> bool {
        *self == EVR::WDT_NMI
    }
    #[doc = "Checks if the value of the field is `CRYPTO_DMA_DONE_IRQ`"]
    #[inline]
    pub fn is_crypto_dma_done_irq(&self) -> bool {
        *self == EVR::CRYPTO_DMA_DONE_IRQ
    }
    #[doc = "Checks if the value of the field is `CRYPTO_RESULT_AVAIL_IRQ`"]
    #[inline]
    pub fn is_crypto_result_avail_irq(&self) -> bool {
        *self == EVR::CRYPTO_RESULT_AVAIL_IRQ
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT7`"]
    #[inline]
    pub fn is_port_event7(&self) -> bool {
        *self == EVR::PORT_EVENT7
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT6`"]
    #[inline]
    pub fn is_port_event6(&self) -> bool {
        *self == EVR::PORT_EVENT6
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT5`"]
    #[inline]
    pub fn is_port_event5(&self) -> bool {
        *self == EVR::PORT_EVENT5
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT4`"]
    #[inline]
    pub fn is_port_event4(&self) -> bool {
        *self == EVR::PORT_EVENT4
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT3`"]
    #[inline]
    pub fn is_port_event3(&self) -> bool {
        *self == EVR::PORT_EVENT3
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT2`"]
    #[inline]
    pub fn is_port_event2(&self) -> bool {
        *self == EVR::PORT_EVENT2
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT1`"]
    #[inline]
    pub fn is_port_event1(&self) -> bool {
        *self == EVR::PORT_EVENT1
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT0`"]
    #[inline]
    pub fn is_port_event0(&self) -> bool {
        *self == EVR::PORT_EVENT0
    }
    #[doc = "Checks if the value of the field is `GPT3B_DMABREQ`"]
    #[inline]
    pub fn is_gpt3b_dmabreq(&self) -> bool {
        *self == EVR::GPT3B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT3A_DMABREQ`"]
    #[inline]
    pub fn is_gpt3a_dmabreq(&self) -> bool {
        *self == EVR::GPT3A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT2B_DMABREQ`"]
    #[inline]
    pub fn is_gpt2b_dmabreq(&self) -> bool {
        *self == EVR::GPT2B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT2A_DMABREQ`"]
    #[inline]
    pub fn is_gpt2a_dmabreq(&self) -> bool {
        *self == EVR::GPT2A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT1B_DMABREQ`"]
    #[inline]
    pub fn is_gpt1b_dmabreq(&self) -> bool {
        *self == EVR::GPT1B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT1A_DMABREQ`"]
    #[inline]
    pub fn is_gpt1a_dmabreq(&self) -> bool {
        *self == EVR::GPT1A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT0B_DMABREQ`"]
    #[inline]
    pub fn is_gpt0b_dmabreq(&self) -> bool {
        *self == EVR::GPT0B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT0A_DMABREQ`"]
    #[inline]
    pub fn is_gpt0a_dmabreq(&self) -> bool {
        *self == EVR::GPT0A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT3B_CMP`"]
    #[inline]
    pub fn is_gpt3b_cmp(&self) -> bool {
        *self == EVR::GPT3B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT3A_CMP`"]
    #[inline]
    pub fn is_gpt3a_cmp(&self) -> bool {
        *self == EVR::GPT3A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT2B_CMP`"]
    #[inline]
    pub fn is_gpt2b_cmp(&self) -> bool {
        *self == EVR::GPT2B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT2A_CMP`"]
    #[inline]
    pub fn is_gpt2a_cmp(&self) -> bool {
        *self == EVR::GPT2A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT1B_CMP`"]
    #[inline]
    pub fn is_gpt1b_cmp(&self) -> bool {
        *self == EVR::GPT1B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT1A_CMP`"]
    #[inline]
    pub fn is_gpt1a_cmp(&self) -> bool {
        *self == EVR::GPT1A_CMP
    }
    #[doc = "Checks if the value of the field is `GPT0B_CMP`"]
    #[inline]
    pub fn is_gpt0b_cmp(&self) -> bool {
        *self == EVR::GPT0B_CMP
    }
    #[doc = "Checks if the value of the field is `GPT0A_CMP`"]
    #[inline]
    pub fn is_gpt0a_cmp(&self) -> bool {
        *self == EVR::GPT0A_CMP
    }
    #[doc = "Checks if the value of the field is `UART0_TX_DMASREQ`"]
    #[inline]
    pub fn is_uart0_tx_dmasreq(&self) -> bool {
        *self == EVR::UART0_TX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `UART0_TX_DMABREQ`"]
    #[inline]
    pub fn is_uart0_tx_dmabreq(&self) -> bool {
        *self == EVR::UART0_TX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `UART0_RX_DMASREQ`"]
    #[inline]
    pub fn is_uart0_rx_dmasreq(&self) -> bool {
        *self == EVR::UART0_RX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `UART0_RX_DMABREQ`"]
    #[inline]
    pub fn is_uart0_rx_dmabreq(&self) -> bool {
        *self == EVR::UART0_RX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `SSI1_TX_DMASREQ`"]
    #[inline]
    pub fn is_ssi1_tx_dmasreq(&self) -> bool {
        *self == EVR::SSI1_TX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `SSI1_TX_DMABREQ`"]
    #[inline]
    pub fn is_ssi1_tx_dmabreq(&self) -> bool {
        *self == EVR::SSI1_TX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `SSI1_RX_DMASREQ`"]
    #[inline]
    pub fn is_ssi1_rx_dmasreq(&self) -> bool {
        *self == EVR::SSI1_RX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `SSI1_RX_DMABREQ`"]
    #[inline]
    pub fn is_ssi1_rx_dmabreq(&self) -> bool {
        *self == EVR::SSI1_RX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `SSI0_TX_DMASREQ`"]
    #[inline]
    pub fn is_ssi0_tx_dmasreq(&self) -> bool {
        *self == EVR::SSI0_TX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `SSI0_TX_DMABREQ`"]
    #[inline]
    pub fn is_ssi0_tx_dmabreq(&self) -> bool {
        *self == EVR::SSI0_TX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `SSI0_RX_DMASREQ`"]
    #[inline]
    pub fn is_ssi0_rx_dmasreq(&self) -> bool {
        *self == EVR::SSI0_RX_DMASREQ
    }
    #[doc = "Checks if the value of the field is `SSI0_RX_DMABREQ`"]
    #[inline]
    pub fn is_ssi0_rx_dmabreq(&self) -> bool {
        *self == EVR::SSI0_RX_DMABREQ
    }
    #[doc = "Checks if the value of the field is `DMA_DONE_COMB`"]
    #[inline]
    pub fn is_dma_done_comb(&self) -> bool {
        *self == EVR::DMA_DONE_COMB
    }
    #[doc = "Checks if the value of the field is `DMA_ERR`"]
    #[inline]
    pub fn is_dma_err(&self) -> bool {
        *self == EVR::DMA_ERR
    }
    #[doc = "Checks if the value of the field is `UART0_COMB`"]
    #[inline]
    pub fn is_uart0_comb(&self) -> bool {
        *self == EVR::UART0_COMB
    }
    #[doc = "Checks if the value of the field is `SSI1_COMB`"]
    #[inline]
    pub fn is_ssi1_comb(&self) -> bool {
        *self == EVR::SSI1_COMB
    }
    #[doc = "Checks if the value of the field is `SSI0_COMB`"]
    #[inline]
    pub fn is_ssi0_comb(&self) -> bool {
        *self == EVR::SSI0_COMB
    }
    #[doc = "Checks if the value of the field is `RFC_CPE_1`"]
    #[inline]
    pub fn is_rfc_cpe_1(&self) -> bool {
        *self == EVR::RFC_CPE_1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `RFC_CPE_0`"]
    #[inline]
    pub fn is_rfc_cpe_0(&self) -> bool {
        *self == EVR::RFC_CPE_0
    }
    #[doc = "Checks if the value of the field is `RFC_HW_COMB`"]
    #[inline]
    pub fn is_rfc_hw_comb(&self) -> bool {
        *self == EVR::RFC_HW_COMB
    }
    #[doc = "Checks if the value of the field is `RFC_CMD_ACK`"]
    #[inline]
    pub fn is_rfc_cmd_ack(&self) -> bool {
        *self == EVR::RFC_CMD_ACK
    }
    #[doc = "Checks if the value of the field is `WDT_IRQ`"]
    #[inline]
    pub fn is_wdt_irq(&self) -> bool {
        *self == EVR::WDT_IRQ
    }
    #[doc = "Checks if the value of the field is `DMA_CH18_DONE`"]
    #[inline]
    pub fn is_dma_ch18_done(&self) -> bool {
        *self == EVR::DMA_CH18_DONE
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline]
    pub fn is_flash(&self) -> bool {
        *self == EVR::FLASH
    }
    #[doc = "Checks if the value of the field is `DMA_CH0_DONE`"]
    #[inline]
    pub fn is_dma_ch0_done(&self) -> bool {
        *self == EVR::DMA_CH0_DONE
    }
    #[doc = "Checks if the value of the field is `GPT1B`"]
    #[inline]
    pub fn is_gpt1b(&self) -> bool {
        *self == EVR::GPT1B
    }
    #[doc = "Checks if the value of the field is `GPT1A`"]
    #[inline]
    pub fn is_gpt1a(&self) -> bool {
        *self == EVR::GPT1A
    }
    #[doc = "Checks if the value of the field is `GPT0B`"]
    #[inline]
    pub fn is_gpt0b(&self) -> bool {
        *self == EVR::GPT0B
    }
    #[doc = "Checks if the value of the field is `GPT0A`"]
    #[inline]
    pub fn is_gpt0a(&self) -> bool {
        *self == EVR::GPT0A
    }
    #[doc = "Checks if the value of the field is `GPT3B`"]
    #[inline]
    pub fn is_gpt3b(&self) -> bool {
        *self == EVR::GPT3B
    }
    #[doc = "Checks if the value of the field is `GPT3A`"]
    #[inline]
    pub fn is_gpt3a(&self) -> bool {
        *self == EVR::GPT3A
    }
    #[doc = "Checks if the value of the field is `GPT2B`"]
    #[inline]
    pub fn is_gpt2b(&self) -> bool {
        *self == EVR::GPT2B
    }
    #[doc = "Checks if the value of the field is `GPT2A`"]
    #[inline]
    pub fn is_gpt2a(&self) -> bool {
        *self == EVR::GPT2A
    }
    #[doc = "Checks if the value of the field is `AUX_COMB`"]
    #[inline]
    pub fn is_aux_comb(&self) -> bool {
        *self == EVR::AUX_COMB
    }
    #[doc = "Checks if the value of the field is `AON_AUX_SWEV0`"]
    #[inline]
    pub fn is_aon_aux_swev0(&self) -> bool {
        *self == EVR::AON_AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `I2C_IRQ`"]
    #[inline]
    pub fn is_i2c_irq(&self) -> bool {
        *self == EVR::I2C_IRQ
    }
    #[doc = "Checks if the value of the field is `I2S_IRQ`"]
    #[inline]
    pub fn is_i2s_irq(&self) -> bool {
        *self == EVR::I2S_IRQ
    }
    #[doc = "Checks if the value of the field is `AON_RTC_COMB`"]
    #[inline]
    pub fn is_aon_rtc_comb(&self) -> bool {
        *self == EVR::AON_RTC_COMB
    }
    #[doc = "Checks if the value of the field is `AON_GPIO_EDGE`"]
    #[inline]
    pub fn is_aon_gpio_edge(&self) -> bool {
        *self == EVR::AON_GPIO_EDGE
    }
    #[doc = "Checks if the value of the field is `AON_PROG2`"]
    #[inline]
    pub fn is_aon_prog2(&self) -> bool {
        *self == EVR::AON_PROG2
    }
    #[doc = "Checks if the value of the field is `AON_PROG1`"]
    #[inline]
    pub fn is_aon_prog1(&self) -> bool {
        *self == EVR::AON_PROG1
    }
    #[doc = "Checks if the value of the field is `AON_PROG0`"]
    #[inline]
    pub fn is_aon_prog0(&self) -> bool {
        *self == EVR::AON_PROG0
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == EVR::NONE
    }
}
#[doc = "Values that can be written to the field `EV`"]
pub enum EVW {
    #[doc = "Always asserted"]
    ALWAYS_ACTIVE,
    #[doc = "CPU halted "]
    CPU_HALTED,
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    AON_RTC_UPD,
    #[doc = "DMA burst request event from AUX, configured by AUX_EVCTL:DMACTL"]
    AUX_DMABREQ,
    #[doc = "DMA single request event from AUX, configured by AUX_EVCTL:DMACTL"]
    AUX_DMASREQ,
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    AUX_SW_DMABREQ,
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    AUX_ADC_IRQ,
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0\n\n"]
    AUX_OBSMUX0,
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL"]
    AUX_ADC_FIFO_ALMOST_FULL,
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE"]
    AUX_ADC_DONE,
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    AUX_SMPH_AUTOTAKE_DONE,
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV"]
    AUX_TIMER1_EV,
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV"]
    AUX_TIMER0_EV,
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    AUX_TDC_DONE,
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV"]
    AUX_AON_WU_EV,
    #[doc = "TRNG Interrupt event, controlled by TRNG:IRQEN.EN"]
    TRNG_IRQ,
    #[doc = "Software event 3, triggered by SWEV.SWEV3"]
    SWEV3,
    #[doc = "Software event 2, triggered by SWEV.SWEV2"]
    SWEV2,
    #[doc = "Software event 1, triggered by SWEV.SWEV1"]
    SWEV1,
    #[doc = "Software event 0, triggered by SWEV.SWEV0"]
    SWEV0,
    #[doc = "Watchdog non maskable interrupt event, controlled by WDT:CTL.INTTYPE"]
    WDT_NMI,
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    CRYPTO_DMA_DONE_IRQ,
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    CRYPTO_RESULT_AVAIL_IRQ,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here."]
    PORT_EVENT7,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here."]
    PORT_EVENT6,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT5,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    PORT_EVENT4,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    PORT_EVENT3,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    PORT_EVENT2,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT1 wil be routed here."]
    PORT_EVENT1,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT0 wil be routed here. "]
    PORT_EVENT0,
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    GPT3B_DMABREQ,
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    GPT3A_DMABREQ,
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    GPT2B_DMABREQ,
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    GPT2A_DMABREQ,
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    GPT1B_DMABREQ,
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    GPT1A_DMABREQ,
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    GPT0B_DMABREQ,
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    GPT0A_DMABREQ,
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    GPT3B_CMP,
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    GPT3A_CMP,
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    GPT2B_CMP,
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    GPT2A_CMP,
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    GPT1B_CMP,
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    GPT1A_CMP,
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    GPT0B_CMP,
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    GPT0A_CMP,
    #[doc = "UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    UART0_TX_DMASREQ,
    #[doc = "UART0 TX DMA burst request, controlled by UART0:DMACTL.TXDMAE"]
    UART0_TX_DMABREQ,
    #[doc = "UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE"]
    UART0_RX_DMASREQ,
    #[doc = "UART0 RX DMA burst request, controlled by UART0:DMACTL.RXDMAE"]
    UART0_RX_DMABREQ,
    #[doc = "SSI1 TX DMA single request, controlled by SSI0:DMACR.TXDMAE "]
    SSI1_TX_DMASREQ,
    #[doc = "SSI1 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE "]
    SSI1_TX_DMABREQ,
    #[doc = "SSI1 RX DMA single request, controlled by SSI0:DMACR.RXDMAE "]
    SSI1_RX_DMASREQ,
    #[doc = "SSI1 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE "]
    SSI1_RX_DMABREQ,
    #[doc = "SSI0 TX DMA single request, controlled by SSI0:DMACR.TXDMAE "]
    SSI0_TX_DMASREQ,
    #[doc = "SSI0 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE "]
    SSI0_TX_DMABREQ,
    #[doc = "SSI0 RX DMA single request, controlled by SSI0:DMACR.RXDMAE "]
    SSI0_RX_DMASREQ,
    #[doc = "SSI0 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE "]
    SSI0_RX_DMABREQ,
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE "]
    DMA_DONE_COMB,
    #[doc = "DMA bus error, corresponds to UDMA0:ERROR.STATUS"]
    DMA_ERR,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    UART0_COMB,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    SSI1_COMB,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    SSI0_COMB,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    RFC_CPE_1,
    #[doc = "AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event.\nMCU domain wakeup control AON_EVENT:MCUWUSEL\nAUX domain wakeup control AON_EVENT:AUXWUSEL\n\n"]
    AUX_SWEV1,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    RFC_CPE_0,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    RFC_HW_COMB,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    RFC_CMD_ACK,
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    WDT_IRQ,
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    DMA_CH18_DONE,
    #[doc = "FLASH controller error event,  the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    FLASH,
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    DMA_CH0_DONE,
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    GPT1B,
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    GPT1A,
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    GPT0B,
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    GPT0A,
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    GPT3B,
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    GPT3A,
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    GPT2B,
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    GPT2A,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    AUX_COMB,
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    AON_AUX_SWEV0,
    #[doc = "Interrupt event from I2C"]
    I2C_IRQ,
    #[doc = "Interrupt event from I2S"]
    I2S_IRQ,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    AON_RTC_COMB,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and  IOC:IOCFGn.EDGE_DET settings"]
    AON_GPIO_EDGE,
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    AON_PROG2,
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    AON_PROG1,
    #[doc = "AON programmable event 0. Event selected by AON_EVENT  MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV"]
    AON_PROG0,
    #[doc = "Always inactive"]
    NONE,
}
impl EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EVW::ALWAYS_ACTIVE => 121,
            EVW::CPU_HALTED => 120,
            EVW::AON_RTC_UPD => 119,
            EVW::AUX_DMABREQ => 118,
            EVW::AUX_DMASREQ => 117,
            EVW::AUX_SW_DMABREQ => 116,
            EVW::AUX_ADC_IRQ => 115,
            EVW::AUX_OBSMUX0 => 114,
            EVW::AUX_ADC_FIFO_ALMOST_FULL => 113,
            EVW::AUX_ADC_DONE => 112,
            EVW::AUX_SMPH_AUTOTAKE_DONE => 111,
            EVW::AUX_TIMER1_EV => 110,
            EVW::AUX_TIMER0_EV => 109,
            EVW::AUX_TDC_DONE => 108,
            EVW::AUX_COMPB => 107,
            EVW::AUX_COMPA => 106,
            EVW::AUX_AON_WU_EV => 105,
            EVW::TRNG_IRQ => 104,
            EVW::SWEV3 => 103,
            EVW::SWEV2 => 102,
            EVW::SWEV1 => 101,
            EVW::SWEV0 => 100,
            EVW::WDT_NMI => 99,
            EVW::CRYPTO_DMA_DONE_IRQ => 94,
            EVW::CRYPTO_RESULT_AVAIL_IRQ => 93,
            EVW::PORT_EVENT7 => 92,
            EVW::PORT_EVENT6 => 91,
            EVW::PORT_EVENT5 => 90,
            EVW::PORT_EVENT4 => 89,
            EVW::PORT_EVENT3 => 88,
            EVW::PORT_EVENT2 => 87,
            EVW::PORT_EVENT1 => 86,
            EVW::PORT_EVENT0 => 85,
            EVW::GPT3B_DMABREQ => 84,
            EVW::GPT3A_DMABREQ => 83,
            EVW::GPT2B_DMABREQ => 82,
            EVW::GPT2A_DMABREQ => 81,
            EVW::GPT1B_DMABREQ => 80,
            EVW::GPT1A_DMABREQ => 79,
            EVW::GPT0B_DMABREQ => 78,
            EVW::GPT0A_DMABREQ => 77,
            EVW::GPT3B_CMP => 68,
            EVW::GPT3A_CMP => 67,
            EVW::GPT2B_CMP => 66,
            EVW::GPT2A_CMP => 65,
            EVW::GPT1B_CMP => 64,
            EVW::GPT1A_CMP => 63,
            EVW::GPT0B_CMP => 62,
            EVW::GPT0A_CMP => 61,
            EVW::UART0_TX_DMASREQ => 51,
            EVW::UART0_TX_DMABREQ => 50,
            EVW::UART0_RX_DMASREQ => 49,
            EVW::UART0_RX_DMABREQ => 48,
            EVW::SSI1_TX_DMASREQ => 47,
            EVW::SSI1_TX_DMABREQ => 46,
            EVW::SSI1_RX_DMASREQ => 45,
            EVW::SSI1_RX_DMABREQ => 44,
            EVW::SSI0_TX_DMASREQ => 43,
            EVW::SSI0_TX_DMABREQ => 42,
            EVW::SSI0_RX_DMASREQ => 41,
            EVW::SSI0_RX_DMABREQ => 40,
            EVW::DMA_DONE_COMB => 39,
            EVW::DMA_ERR => 38,
            EVW::UART0_COMB => 36,
            EVW::SSI1_COMB => 35,
            EVW::SSI0_COMB => 34,
            EVW::RFC_CPE_1 => 30,
            EVW::AUX_SWEV1 => 29,
            EVW::RFC_CPE_0 => 27,
            EVW::RFC_HW_COMB => 26,
            EVW::RFC_CMD_ACK => 25,
            EVW::WDT_IRQ => 24,
            EVW::DMA_CH18_DONE => 22,
            EVW::FLASH => 21,
            EVW::DMA_CH0_DONE => 20,
            EVW::GPT1B => 19,
            EVW::GPT1A => 18,
            EVW::GPT0B => 17,
            EVW::GPT0A => 16,
            EVW::GPT3B => 15,
            EVW::GPT3A => 14,
            EVW::GPT2B => 13,
            EVW::GPT2A => 12,
            EVW::AUX_COMB => 11,
            EVW::AON_AUX_SWEV0 => 10,
            EVW::I2C_IRQ => 9,
            EVW::I2S_IRQ => 8,
            EVW::AON_RTC_COMB => 7,
            EVW::AON_GPIO_EDGE => 4,
            EVW::AON_PROG2 => 3,
            EVW::AON_PROG1 => 2,
            EVW::AON_PROG0 => 1,
            EVW::NONE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EVW<'a> {
    w: &'a mut W,
}
impl<'a> _EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Always asserted"]
    #[inline]
    pub fn always_active(self) -> &'a mut W {
        self.variant(EVW::ALWAYS_ACTIVE)
    }
    #[doc = "CPU halted"]
    #[inline]
    pub fn cpu_halted(self) -> &'a mut W {
        self.variant(EVW::CPU_HALTED)
    }
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    #[inline]
    pub fn aon_rtc_upd(self) -> &'a mut W {
        self.variant(EVW::AON_RTC_UPD)
    }
    #[doc = "DMA burst request event from AUX, configured by AUX_EVCTL:DMACTL"]
    #[inline]
    pub fn aux_dmabreq(self) -> &'a mut W {
        self.variant(EVW::AUX_DMABREQ)
    }
    #[doc = "DMA single request event from AUX, configured by AUX_EVCTL:DMACTL"]
    #[inline]
    pub fn aux_dmasreq(self) -> &'a mut W {
        self.variant(EVW::AUX_DMASREQ)
    }
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    #[inline]
    pub fn aux_sw_dmabreq(self) -> &'a mut W {
        self.variant(EVW::AUX_SW_DMABREQ)
    }
    #[doc = "AUX ADC interrupt event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_IRQ. Status flags are found here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline]
    pub fn aux_adc_irq(self) -> &'a mut W {
        self.variant(EVW::AUX_ADC_IRQ)
    }
    #[doc = "Loopback of OBSMUX0 through AUX, corresponds to AUX_EVCTL:EVTOMCUFLAGS.OBSMUX0"]
    #[inline]
    pub fn aux_obsmux0(self) -> &'a mut W {
        self.variant(EVW::AUX_OBSMUX0)
    }
    #[doc = "AUX ADC FIFO watermark event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL"]
    #[inline]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(EVW::AUX_ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "AUX ADC done, corresponds to AUX_EVCTL:EVTOMCUFLAGS.ADC_DONE"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(EVW::AUX_ADC_DONE)
    }
    #[doc = "Autotake event from AUX semaphore, configured by AUX_SMPH:AUTOTAKE"]
    #[inline]
    pub fn aux_smph_autotake_done(self) -> &'a mut W {
        self.variant(EVW::AUX_SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX timer 1 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER1_EV"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(EVW::AUX_TIMER1_EV)
    }
    #[doc = "AUX timer 0 event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.TIMER0_EV"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(EVW::AUX_TIMER0_EV)
    }
    #[doc = "AUX TDC measurement done event, corresponds to the flag AUX_EVCTL:EVTOMCUFLAGS.TDC_DONE and the AUX_TDC status AUX_TDC:STAT.DONE"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(EVW::AUX_TDC_DONE)
    }
    #[doc = "AUX Compare B event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPB"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(EVW::AUX_COMPB)
    }
    #[doc = "AUX Compare A event, corresponds to AUX_EVCTL:EVTOMCUFLAGS.AUX_COMPA"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(EVW::AUX_COMPA)
    }
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV"]
    #[inline]
    pub fn aux_aon_wu_ev(self) -> &'a mut W {
        self.variant(EVW::AUX_AON_WU_EV)
    }
    #[doc = "TRNG Interrupt event, controlled by TRNG:IRQEN.EN"]
    #[inline]
    pub fn trng_irq(self) -> &'a mut W {
        self.variant(EVW::TRNG_IRQ)
    }
    #[doc = "Software event 3, triggered by SWEV.SWEV3"]
    #[inline]
    pub fn swev3(self) -> &'a mut W {
        self.variant(EVW::SWEV3)
    }
    #[doc = "Software event 2, triggered by SWEV.SWEV2"]
    #[inline]
    pub fn swev2(self) -> &'a mut W {
        self.variant(EVW::SWEV2)
    }
    #[doc = "Software event 1, triggered by SWEV.SWEV1"]
    #[inline]
    pub fn swev1(self) -> &'a mut W {
        self.variant(EVW::SWEV1)
    }
    #[doc = "Software event 0, triggered by SWEV.SWEV0"]
    #[inline]
    pub fn swev0(self) -> &'a mut W {
        self.variant(EVW::SWEV0)
    }
    #[doc = "Watchdog non maskable interrupt event, controlled by WDT:CTL.INTTYPE"]
    #[inline]
    pub fn wdt_nmi(self) -> &'a mut W {
        self.variant(EVW::WDT_NMI)
    }
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    #[inline]
    pub fn crypto_dma_done_irq(self) -> &'a mut W {
        self.variant(EVW::CRYPTO_DMA_DONE_IRQ)
    }
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    #[inline]
    pub fn crypto_result_avail_irq(self) -> &'a mut W {
        self.variant(EVW::CRYPTO_RESULT_AVAIL_IRQ)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT7 wil be routed here."]
    #[inline]
    pub fn port_event7(self) -> &'a mut W {
        self.variant(EVW::PORT_EVENT7)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT6 wil be routed here."]
    #[inline]
    pub fn port_event6(self) -> &'a mut W {
        self.variant(EVW::PORT_EVENT6)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    #[inline]
    pub fn port_event5(self) -> &'a mut W {
        self.variant(EVW::PORT_EVENT5)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT4 wil be routed here."]
    #[inline]
    pub fn port_event4(self) -> &'a mut W {
        self.variant(EVW::PORT_EVENT4)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    #[inline]
    pub fn port_event3(self) -> &'a mut W {
        self.variant(EVW::PORT_EVENT3)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    #[inline]
    pub fn port_event2(self) -> &'a mut W {
        self.variant(EVW::PORT_EVENT2)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT1 wil be routed here."]
    #[inline]
    pub fn port_event1(self) -> &'a mut W {
        self.variant(EVW::PORT_EVENT1)
    }
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT0 wil be routed here."]
    #[inline]
    pub fn port_event0(self) -> &'a mut W {
        self.variant(EVW::PORT_EVENT0)
    }
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline]
    pub fn gpt3b_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT3B_DMABREQ)
    }
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline]
    pub fn gpt3a_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT3A_DMABREQ)
    }
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline]
    pub fn gpt2b_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT2B_DMABREQ)
    }
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline]
    pub fn gpt2a_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT2A_DMABREQ)
    }
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline]
    pub fn gpt1b_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT1B_DMABREQ)
    }
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline]
    pub fn gpt1a_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT1A_DMABREQ)
    }
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline]
    pub fn gpt0b_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT0B_DMABREQ)
    }
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline]
    pub fn gpt0a_dmabreq(self) -> &'a mut W {
        self.variant(EVW::GPT0A_DMABREQ)
    }
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    #[inline]
    pub fn gpt3b_cmp(self) -> &'a mut W {
        self.variant(EVW::GPT3B_CMP)
    }
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    #[inline]
    pub fn gpt3a_cmp(self) -> &'a mut W {
        self.variant(EVW::GPT3A_CMP)
    }
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    #[inline]
    pub fn gpt2b_cmp(self) -> &'a mut W {
        self.variant(EVW::GPT2B_CMP)
    }
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    #[inline]
    pub fn gpt2a_cmp(self) -> &'a mut W {
        self.variant(EVW::GPT2A_CMP)
    }
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    #[inline]
    pub fn gpt1b_cmp(self) -> &'a mut W {
        self.variant(EVW::GPT1B_CMP)
    }
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    #[inline]
    pub fn gpt1a_cmp(self) -> &'a mut W {
        self.variant(EVW::GPT1A_CMP)
    }
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    #[inline]
    pub fn gpt0b_cmp(self) -> &'a mut W {
        self.variant(EVW::GPT0B_CMP)
    }
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    #[inline]
    pub fn gpt0a_cmp(self) -> &'a mut W {
        self.variant(EVW::GPT0A_CMP)
    }
    #[doc = "UART0 TX DMA single request, controlled by UART0:DMACTL.TXDMAE"]
    #[inline]
    pub fn uart0_tx_dmasreq(self) -> &'a mut W {
        self.variant(EVW::UART0_TX_DMASREQ)
    }
    #[doc = "UART0 TX DMA burst request, controlled by UART0:DMACTL.TXDMAE"]
    #[inline]
    pub fn uart0_tx_dmabreq(self) -> &'a mut W {
        self.variant(EVW::UART0_TX_DMABREQ)
    }
    #[doc = "UART0 RX DMA single request, controlled by UART0:DMACTL.RXDMAE"]
    #[inline]
    pub fn uart0_rx_dmasreq(self) -> &'a mut W {
        self.variant(EVW::UART0_RX_DMASREQ)
    }
    #[doc = "UART0 RX DMA burst request, controlled by UART0:DMACTL.RXDMAE"]
    #[inline]
    pub fn uart0_rx_dmabreq(self) -> &'a mut W {
        self.variant(EVW::UART0_RX_DMABREQ)
    }
    #[doc = "SSI1 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    #[inline]
    pub fn ssi1_tx_dmasreq(self) -> &'a mut W {
        self.variant(EVW::SSI1_TX_DMASREQ)
    }
    #[doc = "SSI1 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE"]
    #[inline]
    pub fn ssi1_tx_dmabreq(self) -> &'a mut W {
        self.variant(EVW::SSI1_TX_DMABREQ)
    }
    #[doc = "SSI1 RX DMA single request, controlled by SSI0:DMACR.RXDMAE"]
    #[inline]
    pub fn ssi1_rx_dmasreq(self) -> &'a mut W {
        self.variant(EVW::SSI1_RX_DMASREQ)
    }
    #[doc = "SSI1 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE"]
    #[inline]
    pub fn ssi1_rx_dmabreq(self) -> &'a mut W {
        self.variant(EVW::SSI1_RX_DMABREQ)
    }
    #[doc = "SSI0 TX DMA single request, controlled by SSI0:DMACR.TXDMAE"]
    #[inline]
    pub fn ssi0_tx_dmasreq(self) -> &'a mut W {
        self.variant(EVW::SSI0_TX_DMASREQ)
    }
    #[doc = "SSI0 TX DMA burst request , controlled by SSI0:DMACR.TXDMAE"]
    #[inline]
    pub fn ssi0_tx_dmabreq(self) -> &'a mut W {
        self.variant(EVW::SSI0_TX_DMABREQ)
    }
    #[doc = "SSI0 RX DMA single request, controlled by SSI0:DMACR.RXDMAE"]
    #[inline]
    pub fn ssi0_rx_dmasreq(self) -> &'a mut W {
        self.variant(EVW::SSI0_RX_DMASREQ)
    }
    #[doc = "SSI0 RX DMA burst request , controlled by SSI0:DMACR.RXDMAE"]
    #[inline]
    pub fn ssi0_rx_dmabreq(self) -> &'a mut W {
        self.variant(EVW::SSI0_RX_DMABREQ)
    }
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE"]
    #[inline]
    pub fn dma_done_comb(self) -> &'a mut W {
        self.variant(EVW::DMA_DONE_COMB)
    }
    #[doc = "DMA bus error, corresponds to UDMA0:ERROR.STATUS"]
    #[inline]
    pub fn dma_err(self) -> &'a mut W {
        self.variant(EVW::DMA_ERR)
    }
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    #[inline]
    pub fn uart0_comb(self) -> &'a mut W {
        self.variant(EVW::UART0_COMB)
    }
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    #[inline]
    pub fn ssi1_comb(self) -> &'a mut W {
        self.variant(EVW::SSI1_COMB)
    }
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    #[inline]
    pub fn ssi0_comb(self) -> &'a mut W {
        self.variant(EVW::SSI0_COMB)
    }
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    #[inline]
    pub fn rfc_cpe_1(self) -> &'a mut W {
        self.variant(EVW::RFC_CPE_1)
    }
    #[doc = "AUX software event 1, triggered by AUX_EVCTL:SWEVSET.SWEV1, also available as AUX_EVENT2 AON wake up event. MCU domain wakeup control AON_EVENT:MCUWUSEL AUX domain wakeup control AON_EVENT:AUXWUSEL"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(EVW::AUX_SWEV1)
    }
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    #[inline]
    pub fn rfc_cpe_0(self) -> &'a mut W {
        self.variant(EVW::RFC_CPE_0)
    }
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    #[inline]
    pub fn rfc_hw_comb(self) -> &'a mut W {
        self.variant(EVW::RFC_HW_COMB)
    }
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    #[inline]
    pub fn rfc_cmd_ack(self) -> &'a mut W {
        self.variant(EVW::RFC_CMD_ACK)
    }
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    #[inline]
    pub fn wdt_irq(self) -> &'a mut W {
        self.variant(EVW::WDT_IRQ)
    }
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    #[inline]
    pub fn dma_ch18_done(self) -> &'a mut W {
        self.variant(EVW::DMA_CH18_DONE)
    }
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    #[inline]
    pub fn flash(self) -> &'a mut W {
        self.variant(EVW::FLASH)
    }
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    #[inline]
    pub fn dma_ch0_done(self) -> &'a mut W {
        self.variant(EVW::DMA_CH0_DONE)
    }
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    #[inline]
    pub fn gpt1b(self) -> &'a mut W {
        self.variant(EVW::GPT1B)
    }
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    #[inline]
    pub fn gpt1a(self) -> &'a mut W {
        self.variant(EVW::GPT1A)
    }
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    #[inline]
    pub fn gpt0b(self) -> &'a mut W {
        self.variant(EVW::GPT0B)
    }
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    #[inline]
    pub fn gpt0a(self) -> &'a mut W {
        self.variant(EVW::GPT0A)
    }
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    #[inline]
    pub fn gpt3b(self) -> &'a mut W {
        self.variant(EVW::GPT3B)
    }
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    #[inline]
    pub fn gpt3a(self) -> &'a mut W {
        self.variant(EVW::GPT3A)
    }
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    #[inline]
    pub fn gpt2b(self) -> &'a mut W {
        self.variant(EVW::GPT2B)
    }
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    #[inline]
    pub fn gpt2a(self) -> &'a mut W {
        self.variant(EVW::GPT2A)
    }
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline]
    pub fn aux_comb(self) -> &'a mut W {
        self.variant(EVW::AUX_COMB)
    }
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aon_aux_swev0(self) -> &'a mut W {
        self.variant(EVW::AON_AUX_SWEV0)
    }
    #[doc = "Interrupt event from I2C"]
    #[inline]
    pub fn i2c_irq(self) -> &'a mut W {
        self.variant(EVW::I2C_IRQ)
    }
    #[doc = "Interrupt event from I2S"]
    #[inline]
    pub fn i2s_irq(self) -> &'a mut W {
        self.variant(EVW::I2S_IRQ)
    }
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    #[inline]
    pub fn aon_rtc_comb(self) -> &'a mut W {
        self.variant(EVW::AON_RTC_COMB)
    }
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and IOC:IOCFGn.EDGE_DET settings"]
    #[inline]
    pub fn aon_gpio_edge(self) -> &'a mut W {
        self.variant(EVW::AON_GPIO_EDGE)
    }
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    #[inline]
    pub fn aon_prog2(self) -> &'a mut W {
        self.variant(EVW::AON_PROG2)
    }
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    #[inline]
    pub fn aon_prog1(self) -> &'a mut W {
        self.variant(EVW::AON_PROG1)
    }
    #[doc = "AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV"]
    #[inline]
    pub fn aon_prog0(self) -> &'a mut W {
        self.variant(EVW::AON_PROG0)
    }
    #[doc = "Always inactive"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(EVW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - 6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline]
    pub fn ev(&self) -> EVR {
        EVR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - 6:0\\] Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline]
    pub fn ev(&mut self) -> _EVW {
        _EVW { w: self }
    }
}
