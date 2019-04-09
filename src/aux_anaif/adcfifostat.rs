#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADCFIFOSTAT {
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
pub struct RESERVED5R {
    bits: u32,
}
impl RESERVED5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OVERFLOWR {
    bits: bool,
}
impl OVERFLOWR {
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
pub struct UNDERFLOWR {
    bits: bool,
}
impl UNDERFLOWR {
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
pub struct FULLR {
    bits: bool,
}
impl FULLR {
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
pub struct ALMOST_FULLR {
    bits: bool,
}
impl ALMOST_FULLR {
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
pub struct EMPTYR {
    bits: bool,
}
impl EMPTYR {
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
pub struct _RESERVED5W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 134217727;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OVERFLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERFLOWW<'a> {
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
pub struct _UNDERFLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _UNDERFLOWW<'a> {
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
pub struct _FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _FULLW<'a> {
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
pub struct _ALMOST_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _ALMOST_FULLW<'a> {
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
pub struct _EMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _EMPTYW<'a> {
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
    #[doc = "Bits 5:31 - 31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved5(&self) -> RESERVED5R {
        let bits = {
            const MASK: u32 = 134217727;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED5R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] FIFO overflow flag. 0: FIFO has not overflowed. 1: FIFO has overflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO write pointer is static. It is not possible to add more samples to the ADC FIFO. Flush FIFO to clear the flag."]
    #[inline]
    pub fn overflow(&self) -> OVERFLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OVERFLOWR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] FIFO underflow flag. 0: FIFO has not underflowed. 1: FIFO has underflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO read pointer is static. Read returns the previous sample that was read. Flush FIFO to clear the flag."]
    #[inline]
    pub fn underflow(&self) -> UNDERFLOWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNDERFLOWR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] FIFO full flag. 0: FIFO is not full, there is less than 4 samples in the FIFO. 1: FIFO is full, there are 4 samples in the FIFO. When the flag is set, it is not possible to add more samples to the ADC FIFO. An attempt to add samples sets the OVERFLOW flag."]
    #[inline]
    pub fn full(&self) -> FULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FULLR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] FIFO almost full flag. 0: There are less than 3 samples in the FIFO, or the FIFO is full. The FULL flag is also asserted in the latter case. 1: There are 3 samples in the FIFO, there is room for one more sample."]
    #[inline]
    pub fn almost_full(&self) -> ALMOST_FULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALMOST_FULLR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] FIFO empty flag. 0: FIFO contains one or more samples. 1: FIFO is empty. When the flag is set, read returns the previous sample that was read and sets the UNDERFLOW flag."]
    #[inline]
    pub fn empty(&self) -> EMPTYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EMPTYR { bits }
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
    #[doc = "Bits 5:31 - 31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved5(&mut self) -> _RESERVED5W {
        _RESERVED5W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] FIFO overflow flag. 0: FIFO has not overflowed. 1: FIFO has overflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO write pointer is static. It is not possible to add more samples to the ADC FIFO. Flush FIFO to clear the flag."]
    #[inline]
    pub fn overflow(&mut self) -> _OVERFLOWW {
        _OVERFLOWW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] FIFO underflow flag. 0: FIFO has not underflowed. 1: FIFO has underflowed, this flag is sticky until you flush the FIFO. When the flag is set, the ADC FIFO read pointer is static. Read returns the previous sample that was read. Flush FIFO to clear the flag."]
    #[inline]
    pub fn underflow(&mut self) -> _UNDERFLOWW {
        _UNDERFLOWW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] FIFO full flag. 0: FIFO is not full, there is less than 4 samples in the FIFO. 1: FIFO is full, there are 4 samples in the FIFO. When the flag is set, it is not possible to add more samples to the ADC FIFO. An attempt to add samples sets the OVERFLOW flag."]
    #[inline]
    pub fn full(&mut self) -> _FULLW {
        _FULLW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] FIFO almost full flag. 0: There are less than 3 samples in the FIFO, or the FIFO is full. The FULL flag is also asserted in the latter case. 1: There are 3 samples in the FIFO, there is room for one more sample."]
    #[inline]
    pub fn almost_full(&mut self) -> _ALMOST_FULLW {
        _ALMOST_FULLW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] FIFO empty flag. 0: FIFO contains one or more samples. 1: FIFO is empty. When the flag is set, read returns the previous sample that was read and sets the UNDERFLOW flag."]
    #[inline]
    pub fn empty(&mut self) -> _EMPTYW {
        _EMPTYW { w: self }
    }
}
