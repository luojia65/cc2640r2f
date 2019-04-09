#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPUIRQSEL30 {
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
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV"]
    AUX_AON_WU_EV,
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    CRYPTO_DMA_DONE_IRQ,
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    DMA_CH18_DONE,
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    DMA_CH0_DONE,
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    AON_AUX_SWEV0,
    #[doc = "Interrupt event from I2S"]
    I2S_IRQ,
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    AON_PROG2,
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    AON_PROG1,
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
            EVR::AUX_OBSMUX0 => 114,
            EVR::AUX_ADC_FIFO_ALMOST_FULL => 113,
            EVR::AUX_ADC_DONE => 112,
            EVR::AUX_SMPH_AUTOTAKE_DONE => 111,
            EVR::AUX_TIMER1_EV => 110,
            EVR::AUX_TIMER0_EV => 109,
            EVR::AUX_TDC_DONE => 108,
            EVR::AUX_COMPB => 107,
            EVR::AUX_AON_WU_EV => 105,
            EVR::CRYPTO_DMA_DONE_IRQ => 94,
            EVR::DMA_CH18_DONE => 22,
            EVR::DMA_CH0_DONE => 20,
            EVR::AON_AUX_SWEV0 => 10,
            EVR::I2S_IRQ => 8,
            EVR::AON_PROG2 => 3,
            EVR::AON_PROG1 => 2,
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
            114 => EVR::AUX_OBSMUX0,
            113 => EVR::AUX_ADC_FIFO_ALMOST_FULL,
            112 => EVR::AUX_ADC_DONE,
            111 => EVR::AUX_SMPH_AUTOTAKE_DONE,
            110 => EVR::AUX_TIMER1_EV,
            109 => EVR::AUX_TIMER0_EV,
            108 => EVR::AUX_TDC_DONE,
            107 => EVR::AUX_COMPB,
            105 => EVR::AUX_AON_WU_EV,
            94 => EVR::CRYPTO_DMA_DONE_IRQ,
            22 => EVR::DMA_CH18_DONE,
            20 => EVR::DMA_CH0_DONE,
            10 => EVR::AON_AUX_SWEV0,
            8 => EVR::I2S_IRQ,
            3 => EVR::AON_PROG2,
            2 => EVR::AON_PROG1,
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
    #[doc = "Checks if the value of the field is `AUX_AON_WU_EV`"]
    #[inline]
    pub fn is_aux_aon_wu_ev(&self) -> bool {
        *self == EVR::AUX_AON_WU_EV
    }
    #[doc = "Checks if the value of the field is `CRYPTO_DMA_DONE_IRQ`"]
    #[inline]
    pub fn is_crypto_dma_done_irq(&self) -> bool {
        *self == EVR::CRYPTO_DMA_DONE_IRQ
    }
    #[doc = "Checks if the value of the field is `DMA_CH18_DONE`"]
    #[inline]
    pub fn is_dma_ch18_done(&self) -> bool {
        *self == EVR::DMA_CH18_DONE
    }
    #[doc = "Checks if the value of the field is `DMA_CH0_DONE`"]
    #[inline]
    pub fn is_dma_ch0_done(&self) -> bool {
        *self == EVR::DMA_CH0_DONE
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
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV"]
    AUX_AON_WU_EV,
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    CRYPTO_DMA_DONE_IRQ,
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    DMA_CH18_DONE,
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    DMA_CH0_DONE,
    #[doc = "AUX Software event 0, AUX_EVCTL:SWEVSET.SWEV0"]
    AON_AUX_SWEV0,
    #[doc = "Interrupt event from I2S"]
    I2S_IRQ,
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    AON_PROG2,
    #[doc = "AON programmable event 1. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG1_EV"]
    AON_PROG1,
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
            EVW::AUX_OBSMUX0 => 114,
            EVW::AUX_ADC_FIFO_ALMOST_FULL => 113,
            EVW::AUX_ADC_DONE => 112,
            EVW::AUX_SMPH_AUTOTAKE_DONE => 111,
            EVW::AUX_TIMER1_EV => 110,
            EVW::AUX_TIMER0_EV => 109,
            EVW::AUX_TDC_DONE => 108,
            EVW::AUX_COMPB => 107,
            EVW::AUX_AON_WU_EV => 105,
            EVW::CRYPTO_DMA_DONE_IRQ => 94,
            EVW::DMA_CH18_DONE => 22,
            EVW::DMA_CH0_DONE => 20,
            EVW::AON_AUX_SWEV0 => 10,
            EVW::I2S_IRQ => 8,
            EVW::AON_PROG2 => 3,
            EVW::AON_PROG1 => 2,
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
    #[doc = "AON wakeup event, corresponds flags are here AUX_EVCTL:EVTOMCUFLAGS.AON_WU_EV"]
    #[inline]
    pub fn aux_aon_wu_ev(self) -> &'a mut W {
        self.variant(EVW::AUX_AON_WU_EV)
    }
    #[doc = "CRYPTO DMA input done event, the correspondingg flag is CRYPTO:IRQSTAT.DMA_IN_DONE. Controlled by CRYPTO:IRQEN.DMA_IN_DONE"]
    #[inline]
    pub fn crypto_dma_done_irq(self) -> &'a mut W {
        self.variant(EVW::CRYPTO_DMA_DONE_IRQ)
    }
    #[doc = "DMA done for software tiggered UDMA channel 18, see UDMA0:SOFTREQ"]
    #[inline]
    pub fn dma_ch18_done(self) -> &'a mut W {
        self.variant(EVW::DMA_CH18_DONE)
    }
    #[doc = "DMA done for software tiggered UDMA channel 0, see UDMA0:SOFTREQ"]
    #[inline]
    pub fn dma_ch0_done(self) -> &'a mut W {
        self.variant(EVW::DMA_CH0_DONE)
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
        W { bits: 0 }
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
