#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL1 {
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
pub struct RESERVED23R {
    bits: u16,
}
impl RESERVED23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSCHFCTRIMFRACTR {
    bits: u8,
}
impl RCOSCHFCTRIMFRACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSCHFCTRIMFRACT_ENR {
    bits: bool,
}
impl RCOSCHFCTRIMFRACT_ENR {
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
pub struct SPARE2R {
    bits: u16,
}
impl SPARE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_HF_FAST_STARTR {
    bits: u8,
}
impl XOSC_HF_FAST_STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED23W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED23W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RCOSCHFCTRIMFRACTW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSCHFCTRIMFRACTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RCOSCHFCTRIMFRACT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSCHFCTRIMFRACT_ENW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPARE2W<'a> {
    w: &'a mut W,
}
impl<'a> _SPARE2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 32767;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_HF_FAST_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_HF_FAST_STARTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 23:31 - 31:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved23(&self) -> RESERVED23R {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED23R { bits }
    }
    #[doc = "Bits 18:22 - 22:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschfctrimfract(&self) -> RCOSCHFCTRIMFRACTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSCHFCTRIMFRACTR { bits }
    }
    #[doc = "Bit 17 - 17:17\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschfctrimfract_en(&self) -> RCOSCHFCTRIMFRACT_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCOSCHFCTRIMFRACT_ENR { bits }
    }
    #[doc = "Bits 2:16 - 16:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare2(&self) -> SPARE2R {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SPARE2R { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_fast_start(&self) -> XOSC_HF_FAST_STARTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSC_HF_FAST_STARTR { bits }
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
    #[doc = "Bits 23:31 - 31:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved23(&mut self) -> _RESERVED23W {
        _RESERVED23W { w: self }
    }
    #[doc = "Bits 18:22 - 22:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschfctrimfract(&mut self) -> _RCOSCHFCTRIMFRACTW {
        _RCOSCHFCTRIMFRACTW { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschfctrimfract_en(&mut self) -> _RCOSCHFCTRIMFRACT_ENW {
        _RCOSCHFCTRIMFRACT_ENW { w: self }
    }
    #[doc = "Bits 2:16 - 16:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare2(&mut self) -> _SPARE2W {
        _SPARE2W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_fast_start(&mut self) -> _XOSC_HF_FAST_STARTW {
        _XOSC_HF_FAST_STARTW { w: self }
    }
}
