#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EFUSEPROGRAM {
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
pub struct RESERVED31R {
    bits: bool,
}
impl RESERVED31R {
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
pub struct COMPAREDISABLER {
    bits: bool,
}
impl COMPAREDISABLER {
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
pub struct CLOCKSTALLR {
    bits: u16,
}
impl CLOCKSTALLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VPPTOVDDR {
    bits: bool,
}
impl VPPTOVDDR {
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
pub struct ITERATIONSR {
    bits: u8,
}
impl ITERATIONSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WRITECLOCKR {
    bits: u16,
}
impl WRITECLOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED31W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED31W<'a> {
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPAREDISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPAREDISABLEW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLOCKSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _CLOCKSTALLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VPPTOVDDW<'a> {
    w: &'a mut W,
}
impl<'a> _VPPTOVDDW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ITERATIONSW<'a> {
    w: &'a mut W,
}
impl<'a> _ITERATIONSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRITECLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITECLOCKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
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
    #[doc = "Bit 31 - 31:31\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved31(&self) -> RESERVED31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED31R { bits }
    }
    #[doc = "Bit 30 - 30:30\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn comparedisable(&self) -> COMPAREDISABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMPAREDISABLER { bits }
    }
    #[doc = "Bits 14:29 - 29:14\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn clockstall(&self) -> CLOCKSTALLR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        CLOCKSTALLR { bits }
    }
    #[doc = "Bit 13 - 13:13\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vpptovdd(&self) -> VPPTOVDDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VPPTOVDDR { bits }
    }
    #[doc = "Bits 9:12 - 12:9\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn iterations(&self) -> ITERATIONSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ITERATIONSR { bits }
    }
    #[doc = "Bits 0:8 - 8:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn writeclock(&self) -> WRITECLOCKR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WRITECLOCKR { bits }
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
    #[doc = "Bit 31 - 31:31\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved31(&mut self) -> _RESERVED31W {
        _RESERVED31W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn comparedisable(&mut self) -> _COMPAREDISABLEW {
        _COMPAREDISABLEW { w: self }
    }
    #[doc = "Bits 14:29 - 29:14\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn clockstall(&mut self) -> _CLOCKSTALLW {
        _CLOCKSTALLW { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vpptovdd(&mut self) -> _VPPTOVDDW {
        _VPPTOVDDW { w: self }
    }
    #[doc = "Bits 9:12 - 12:9\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn iterations(&mut self) -> _ITERATIONSW {
        _ITERATIONSW { w: self }
    }
    #[doc = "Bits 0:8 - 8:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn writeclock(&mut self) -> _WRITECLOCKW {
        _WRITECLOCKW { w: self }
    }
}
