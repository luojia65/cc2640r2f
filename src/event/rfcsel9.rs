#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RFCSEL9 {
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
    #[doc = "Software event 1, triggered by SWEV.SWEV1"]
    SWEV1,
    #[doc = "Software event 0, triggered by SWEV.SWEV0"]
    SWEV0,
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    CRYPTO_RESULT_AVAIL_IRQ,
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE "]
    DMA_DONE_COMB,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    UART0_COMB,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    SSI1_COMB,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    SSI0_COMB,
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    WDT_IRQ,
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    AON_AUX_SWEV0,
    #[doc = "Interrupt event from I2S"]
    I2S_IRQ,
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
            EVR::SWEV1 => 101,
            EVR::SWEV0 => 100,
            EVR::CRYPTO_RESULT_AVAIL_IRQ => 93,
            EVR::DMA_DONE_COMB => 39,
            EVR::UART0_COMB => 36,
            EVR::SSI1_COMB => 35,
            EVR::SSI0_COMB => 34,
            EVR::WDT_IRQ => 24,
            EVR::AON_AUX_SWEV0 => 10,
            EVR::I2S_IRQ => 8,
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
            101 => EVR::SWEV1,
            100 => EVR::SWEV0,
            93 => EVR::CRYPTO_RESULT_AVAIL_IRQ,
            39 => EVR::DMA_DONE_COMB,
            36 => EVR::UART0_COMB,
            35 => EVR::SSI1_COMB,
            34 => EVR::SSI0_COMB,
            24 => EVR::WDT_IRQ,
            10 => EVR::AON_AUX_SWEV0,
            8 => EVR::I2S_IRQ,
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
    #[doc = "Checks if the value of the field is `CRYPTO_RESULT_AVAIL_IRQ`"]
    #[inline]
    pub fn is_crypto_result_avail_irq(&self) -> bool {
        *self == EVR::CRYPTO_RESULT_AVAIL_IRQ
    }
    #[doc = "Checks if the value of the field is `DMA_DONE_COMB`"]
    #[inline]
    pub fn is_dma_done_comb(&self) -> bool {
        *self == EVR::DMA_DONE_COMB
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
    #[doc = "Checks if the value of the field is `WDT_IRQ`"]
    #[inline]
    pub fn is_wdt_irq(&self) -> bool {
        *self == EVR::WDT_IRQ
    }
    #[doc = "Checks if the value of the field is `AON_AUX_SWEV0`"]
    #[inline]
    pub fn is_aon_aux_swev0(&self) -> bool {
        *self == EVR::AON_AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `I2S_IRQ`"]
    #[inline]
    pub fn is_i2s_irq(&self) -> bool {
        *self == EVR::I2S_IRQ
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
    #[doc = "Software event 1, triggered by SWEV.SWEV1"]
    SWEV1,
    #[doc = "Software event 0, triggered by SWEV.SWEV0"]
    SWEV0,
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    CRYPTO_RESULT_AVAIL_IRQ,
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE "]
    DMA_DONE_COMB,
    #[doc = "UART0 combined interrupt, interrupt flags are found here UART0:MIS"]
    UART0_COMB,
    #[doc = "SSI1 combined interrupt, interrupt flags are found here SSI1:MIS"]
    SSI1_COMB,
    #[doc = "SSI0 combined interrupt, interrupt flags are found here SSI0:MIS"]
    SSI0_COMB,
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    WDT_IRQ,
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    AON_AUX_SWEV0,
    #[doc = "Interrupt event from I2S"]
    I2S_IRQ,
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
            EVW::SWEV1 => 101,
            EVW::SWEV0 => 100,
            EVW::CRYPTO_RESULT_AVAIL_IRQ => 93,
            EVW::DMA_DONE_COMB => 39,
            EVW::UART0_COMB => 36,
            EVW::SSI1_COMB => 35,
            EVW::SSI0_COMB => 34,
            EVW::WDT_IRQ => 24,
            EVW::AON_AUX_SWEV0 => 10,
            EVW::I2S_IRQ => 8,
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
    #[doc = "CRYPTO result available interupt event, the corresponding flag is found here CRYPTO:IRQSTAT.RESULT_AVAIL. Controlled by CRYPTO:IRQSTAT.RESULT_AVAIL"]
    #[inline]
    pub fn crypto_result_avail_irq(self) -> &'a mut W {
        self.variant(EVW::CRYPTO_RESULT_AVAIL_IRQ)
    }
    #[doc = "Combined DMA done, corresponding flags are here UDMA0:REQDONE"]
    #[inline]
    pub fn dma_done_comb(self) -> &'a mut W {
        self.variant(EVW::DMA_DONE_COMB)
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
    #[doc = "Watchdog interrupt event, controlled by WDT:CTL.INTEN"]
    #[inline]
    pub fn wdt_irq(self) -> &'a mut W {
        self.variant(EVW::WDT_IRQ)
    }
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aon_aux_swev0(self) -> &'a mut W {
        self.variant(EVW::AON_AUX_SWEV0)
    }
    #[doc = "Interrupt event from I2S"]
    #[inline]
    pub fn i2s_irq(self) -> &'a mut W {
        self.variant(EVW::I2S_IRQ)
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
        W { bits: 2 }
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
