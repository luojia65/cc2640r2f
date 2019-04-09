#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPT1BCAPTSEL {
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
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    AON_RTC_UPD,
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
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    PORT_EVENT3,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    PORT_EVENT2,
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
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    UART0_COMB,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    SSI1_COMB,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    SSI0_COMB,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    RFC_CPE_1,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    RFC_CPE_0,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    RFC_HW_COMB,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    RFC_CMD_ACK,
    #[doc = "FLASH controller error event,  the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    FLASH,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    AUX_COMB,
    #[doc = "Interrupt event from I2C"]
    I2C_IRQ,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    AON_RTC_COMB,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and  IOC:IOCFGn.EDGE_DET settings"]
    AON_GPIO_EDGE,
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
            EVR::AON_RTC_UPD => 119,
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
            EVR::PORT_EVENT3 => 88,
            EVR::PORT_EVENT2 => 87,
            EVR::GPT3B_CMP => 68,
            EVR::GPT3A_CMP => 67,
            EVR::GPT2B_CMP => 66,
            EVR::GPT2A_CMP => 65,
            EVR::GPT1B_CMP => 64,
            EVR::GPT1A_CMP => 63,
            EVR::GPT0B_CMP => 62,
            EVR::GPT0A_CMP => 61,
            EVR::UART0_COMB => 36,
            EVR::SSI1_COMB => 35,
            EVR::SSI0_COMB => 34,
            EVR::RFC_CPE_1 => 30,
            EVR::RFC_CPE_0 => 27,
            EVR::RFC_HW_COMB => 26,
            EVR::RFC_CMD_ACK => 25,
            EVR::FLASH => 21,
            EVR::AUX_COMB => 11,
            EVR::I2C_IRQ => 9,
            EVR::AON_RTC_COMB => 7,
            EVR::AON_GPIO_EDGE => 4,
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
            119 => EVR::AON_RTC_UPD,
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
            88 => EVR::PORT_EVENT3,
            87 => EVR::PORT_EVENT2,
            68 => EVR::GPT3B_CMP,
            67 => EVR::GPT3A_CMP,
            66 => EVR::GPT2B_CMP,
            65 => EVR::GPT2A_CMP,
            64 => EVR::GPT1B_CMP,
            63 => EVR::GPT1A_CMP,
            62 => EVR::GPT0B_CMP,
            61 => EVR::GPT0A_CMP,
            36 => EVR::UART0_COMB,
            35 => EVR::SSI1_COMB,
            34 => EVR::SSI0_COMB,
            30 => EVR::RFC_CPE_1,
            27 => EVR::RFC_CPE_0,
            26 => EVR::RFC_HW_COMB,
            25 => EVR::RFC_CMD_ACK,
            21 => EVR::FLASH,
            11 => EVR::AUX_COMB,
            9 => EVR::I2C_IRQ,
            7 => EVR::AON_RTC_COMB,
            4 => EVR::AON_GPIO_EDGE,
            0 => EVR::NONE,
            i => EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline]
    pub fn is_always_active(&self) -> bool {
        *self == EVR::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `AON_RTC_UPD`"]
    #[inline]
    pub fn is_aon_rtc_upd(&self) -> bool {
        *self == EVR::AON_RTC_UPD
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
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline]
    pub fn is_flash(&self) -> bool {
        *self == EVR::FLASH
    }
    #[doc = "Checks if the value of the field is `AUX_COMB`"]
    #[inline]
    pub fn is_aux_comb(&self) -> bool {
        *self == EVR::AUX_COMB
    }
    #[doc = "Checks if the value of the field is `I2C_IRQ`"]
    #[inline]
    pub fn is_i2c_irq(&self) -> bool {
        *self == EVR::I2C_IRQ
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
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    AON_RTC_UPD,
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
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT3 wil be routed here."]
    PORT_EVENT3,
    #[doc = "Port capture event from IOC, configured by IOC:IOCFGn.PORT_ID. Events on ports configured with ENUM PORT_EVENT2 wil be routed here."]
    PORT_EVENT2,
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
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    UART0_COMB,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    SSI1_COMB,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    SSI0_COMB,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE1 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_1 event"]
    RFC_CPE_1,
    #[doc = "Combined Interrupt for CPE Generated events. Corresponding flags are here RFC_DBELL:RFCPEIFG. Only interrupts selected with CPE0 in RFC_DBELL:RFCPEIFG can trigger a RFC_CPE_0 event"]
    RFC_CPE_0,
    #[doc = "Combined RFC hardware interrupt, corresponding flag is here RFC_DBELL:RFHWIFG"]
    RFC_HW_COMB,
    #[doc = "RFC Doorbell Command Acknowledgement Interrupt, equvialent to RFC_DBELL:RFACKIFG.ACKFLAG"]
    RFC_CMD_ACK,
    #[doc = "FLASH controller error event,  the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    FLASH,
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    AUX_COMB,
    #[doc = "Interrupt event from I2C"]
    I2C_IRQ,
    #[doc = "Event from AON_RTC, controlled by the AON_RTC:CTL.COMB_EV_MASK setting"]
    AON_RTC_COMB,
    #[doc = "Edge detect event from IOC. Configureded by the IOC:IOCFGn.EDGE_IRQ_EN and  IOC:IOCFGn.EDGE_DET settings"]
    AON_GPIO_EDGE,
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
            EVW::AON_RTC_UPD => 119,
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
            EVW::PORT_EVENT3 => 88,
            EVW::PORT_EVENT2 => 87,
            EVW::GPT3B_CMP => 68,
            EVW::GPT3A_CMP => 67,
            EVW::GPT2B_CMP => 66,
            EVW::GPT2A_CMP => 65,
            EVW::GPT1B_CMP => 64,
            EVW::GPT1A_CMP => 63,
            EVW::GPT0B_CMP => 62,
            EVW::GPT0A_CMP => 61,
            EVW::UART0_COMB => 36,
            EVW::SSI1_COMB => 35,
            EVW::SSI0_COMB => 34,
            EVW::RFC_CPE_1 => 30,
            EVW::RFC_CPE_0 => 27,
            EVW::RFC_HW_COMB => 26,
            EVW::RFC_CMD_ACK => 25,
            EVW::FLASH => 21,
            EVW::AUX_COMB => 11,
            EVW::I2C_IRQ => 9,
            EVW::AON_RTC_COMB => 7,
            EVW::AON_GPIO_EDGE => 4,
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
    #[doc = "RTC periodic event controlled by AON_RTC:CTL.RTC_UPD_EN"]
    #[inline]
    pub fn aon_rtc_upd(self) -> &'a mut W {
        self.variant(EVW::AON_RTC_UPD)
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
    #[doc = "FLASH controller error event, the status flags are FLASH:FEDACSTAT.FSM_DONE and FLASH:FEDACSTAT.RVF_INT"]
    #[inline]
    pub fn flash(self) -> &'a mut W {
        self.variant(EVW::FLASH)
    }
    #[doc = "AUX combined event, the corresponding flag register is here AUX_EVCTL:EVTOMCUFLAGS"]
    #[inline]
    pub fn aux_comb(self) -> &'a mut W {
        self.variant(EVW::AUX_COMB)
    }
    #[doc = "Interrupt event from I2C"]
    #[inline]
    pub fn i2c_irq(self) -> &'a mut W {
        self.variant(EVW::I2C_IRQ)
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
        W { bits: 88 }
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
