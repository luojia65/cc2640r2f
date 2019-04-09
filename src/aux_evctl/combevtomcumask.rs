#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::COMBEVTOMCUMASK {
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
#[doc = r" Value of the field"]
pub struct RESERVED11R {
    bits: u32,
}
impl RESERVED11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADC_IRQR {
    bits: bool,
}
impl ADC_IRQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct OBSMUX0R {
    bits: bool,
}
impl OBSMUX0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ADC_FIFO_ALMOST_FULLR {
    bits: bool,
}
impl ADC_FIFO_ALMOST_FULLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ADC_DONER {
    bits: bool,
}
impl ADC_DONER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SMPH_AUTOTAKE_DONER {
    bits: bool,
}
impl SMPH_AUTOTAKE_DONER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TIMER1_EVR {
    bits: bool,
}
impl TIMER1_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TIMER0_EVR {
    bits: bool,
}
impl TIMER0_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TDC_DONER {
    bits: bool,
}
impl TDC_DONER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct AUX_COMPBR {
    bits: bool,
}
impl AUX_COMPBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct AUX_COMPAR {
    bits: bool,
}
impl AUX_COMPAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct AON_WU_EVR {
    bits: bool,
}
impl AON_WU_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED11W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 2097151;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IRQW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OBSMUX0W<'a> {
    w: &'a mut W,
}
impl<'a> _OBSMUX0W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC_FIFO_ALMOST_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_FIFO_ALMOST_FULLW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DONEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SMPH_AUTOTAKE_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPH_AUTOTAKE_DONEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMER1_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER1_EVW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMER0_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER0_EVW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TDC_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDC_DONEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUX_COMPBW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_COMPBW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUX_COMPAW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_COMPAW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AON_WU_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _AON_WU_EVW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
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
    #[doc = "Bits 11:31 - 31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved11(&self) -> RESERVED11R {
        let bits = {
            const MASK: u32 = 2097151;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED11R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] EVTOMCUFLAGS.ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn adc_irq(&self) -> ADC_IRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC_IRQR { bits }
    }
    #[doc = "Bit 9 - 9:9\\] EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn obsmux0(&self) -> OBSMUX0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OBSMUX0R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn adc_fifo_almost_full(&self) -> ADC_FIFO_ALMOST_FULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC_FIFO_ALMOST_FULLR { bits }
    }
    #[doc = "Bit 7 - 7:7\\] EVTOMCUFLAGS.ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn adc_done(&self) -> ADC_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC_DONER { bits }
    }
    #[doc = "Bit 6 - 6:6\\] EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn smph_autotake_done(&self) -> SMPH_AUTOTAKE_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SMPH_AUTOTAKE_DONER { bits }
    }
    #[doc = "Bit 5 - 5:5\\] EVTOMCUFLAGS.TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn timer1_ev(&self) -> TIMER1_EVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIMER1_EVR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] EVTOMCUFLAGS.TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn timer0_ev(&self) -> TIMER0_EVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIMER0_EVR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] EVTOMCUFLAGS.TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn tdc_done(&self) -> TDC_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TDC_DONER { bits }
    }
    #[doc = "Bit 2 - 2:2\\] EVTOMCUFLAGS.AUX_COMPB contribution to the AUX_COMB event. 0: Exclude 1: Include."]
    #[inline]
    pub fn aux_compb(&self) -> AUX_COMPBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_COMPBR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] EVTOMCUFLAGS.AUX_COMPA contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn aux_compa(&self) -> AUX_COMPAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_COMPAR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] EVTOMCUFLAGS.AON_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn aon_wu_ev(&self) -> AON_WU_EVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AON_WU_EVR { bits }
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
    #[doc = "Bits 11:31 - 31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved11(&mut self) -> _RESERVED11W {
        _RESERVED11W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] EVTOMCUFLAGS.ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn adc_irq(&mut self) -> _ADC_IRQW {
        _ADC_IRQW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn obsmux0(&mut self) -> _OBSMUX0W {
        _OBSMUX0W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn adc_fifo_almost_full(&mut self) -> _ADC_FIFO_ALMOST_FULLW {
        _ADC_FIFO_ALMOST_FULLW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] EVTOMCUFLAGS.ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn adc_done(&mut self) -> _ADC_DONEW {
        _ADC_DONEW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn smph_autotake_done(&mut self) -> _SMPH_AUTOTAKE_DONEW {
        _SMPH_AUTOTAKE_DONEW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] EVTOMCUFLAGS.TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn timer1_ev(&mut self) -> _TIMER1_EVW {
        _TIMER1_EVW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] EVTOMCUFLAGS.TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn timer0_ev(&mut self) -> _TIMER0_EVW {
        _TIMER0_EVW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] EVTOMCUFLAGS.TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn tdc_done(&mut self) -> _TDC_DONEW {
        _TDC_DONEW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] EVTOMCUFLAGS.AUX_COMPB contribution to the AUX_COMB event. 0: Exclude 1: Include."]
    #[inline]
    pub fn aux_compb(&mut self) -> _AUX_COMPBW {
        _AUX_COMPBW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] EVTOMCUFLAGS.AUX_COMPA contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn aux_compa(&mut self) -> _AUX_COMPAW {
        _AUX_COMPAW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] EVTOMCUFLAGS.AON_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline]
    pub fn aon_wu_ev(&mut self) -> _AON_WU_EVW {
        _AON_WU_EVW { w: self }
    }
}
