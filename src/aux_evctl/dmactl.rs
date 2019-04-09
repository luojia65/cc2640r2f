#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMACTL {
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
pub struct RESERVED3R {
    bits: u32,
}
impl RESERVED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `REQ_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQ_MODER {
    #[doc = "Single requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    SINGLE,
    #[doc = "Burst requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    BURST,
}
impl REQ_MODER {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            REQ_MODER::SINGLE => true,
            REQ_MODER::BURST => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REQ_MODER {
        match value {
            true => REQ_MODER::SINGLE,
            false => REQ_MODER::BURST,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == REQ_MODER::SINGLE
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline]
    pub fn is_burst(&self) -> bool {
        *self == REQ_MODER::BURST
    }
}
#[doc = r" Value of the field"]
pub struct ENR {
    bits: bool,
}
impl ENR {
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
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELR {
    #[doc = "UDMA0 trigger event will be generated when the ADC FIFO is almost full (3/4 full)."]
    FIFO_ALMOST_FULL,
    #[doc = "UDMA0 trigger event will be generated when there are samples in the ADC FIFO."]
    FIFO_NOT_EMPTY,
}
impl SELR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SELR::FIFO_ALMOST_FULL => true,
            SELR::FIFO_NOT_EMPTY => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELR {
        match value {
            true => SELR::FIFO_ALMOST_FULL,
            false => SELR::FIFO_NOT_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `FIFO_ALMOST_FULL`"]
    #[inline]
    pub fn is_fifo_almost_full(&self) -> bool {
        *self == SELR::FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `FIFO_NOT_EMPTY`"]
    #[inline]
    pub fn is_fifo_not_empty(&self) -> bool {
        *self == SELR::FIFO_NOT_EMPTY
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REQ_MODE`"]
pub enum REQ_MODEW {
    #[doc = "Single requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    SINGLE,
    #[doc = "Burst requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    BURST,
}
impl REQ_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REQ_MODEW::SINGLE => true,
            REQ_MODEW::BURST => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REQ_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _REQ_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REQ_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Single requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(REQ_MODEW::SINGLE)
    }
    #[doc = "Burst requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    #[inline]
    pub fn burst(self) -> &'a mut W {
        self.variant(REQ_MODEW::BURST)
    }
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
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
#[doc = "Values that can be written to the field `SEL`"]
pub enum SELW {
    #[doc = "UDMA0 trigger event will be generated when the ADC FIFO is almost full (3/4 full)."]
    FIFO_ALMOST_FULL,
    #[doc = "UDMA0 trigger event will be generated when there are samples in the ADC FIFO."]
    FIFO_NOT_EMPTY,
}
impl SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SELW::FIFO_ALMOST_FULL => true,
            SELW::FIFO_NOT_EMPTY => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "UDMA0 trigger event will be generated when the ADC FIFO is almost full (3/4 full)."]
    #[inline]
    pub fn fifo_almost_full(self) -> &'a mut W {
        self.variant(SELW::FIFO_ALMOST_FULL)
    }
    #[doc = "UDMA0 trigger event will be generated when there are samples in the ADC FIFO."]
    #[inline]
    pub fn fifo_not_empty(self) -> &'a mut W {
        self.variant(SELW::FIFO_NOT_EMPTY)
    }
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] UDMA0 Request mode"]
    #[inline]
    pub fn req_mode(&self) -> REQ_MODER {
        REQ_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
    #[inline]
    pub fn sel(&self) -> SELR {
        SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] UDMA0 Request mode"]
    #[inline]
    pub fn req_mode(&mut self) -> _REQ_MODEW {
        _REQ_MODEW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
    #[inline]
    pub fn sel(&mut self) -> _SELW {
        _SELW { w: self }
    }
}
