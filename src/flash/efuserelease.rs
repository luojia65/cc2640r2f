#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EFUSERELEASE {
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
pub struct ODPYEARR {
    bits: u8,
}
impl ODPYEARR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ODPMONTHR {
    bits: u8,
}
impl ODPMONTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ODPDAYR {
    bits: u8,
}
impl ODPDAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EFUSEYEARR {
    bits: u8,
}
impl EFUSEYEARR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EFUSEMONTHR {
    bits: u8,
}
impl EFUSEMONTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EFUSEDAYR {
    bits: u8,
}
impl EFUSEDAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ODPYEARW<'a> {
    w: &'a mut W,
}
impl<'a> _ODPYEARW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ODPMONTHW<'a> {
    w: &'a mut W,
}
impl<'a> _ODPMONTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ODPDAYW<'a> {
    w: &'a mut W,
}
impl<'a> _ODPDAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EFUSEYEARW<'a> {
    w: &'a mut W,
}
impl<'a> _EFUSEYEARW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EFUSEMONTHW<'a> {
    w: &'a mut W,
}
impl<'a> _EFUSEMONTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EFUSEDAYW<'a> {
    w: &'a mut W,
}
impl<'a> _EFUSEDAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 25:31 - 31:25\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn odpyear(&self) -> ODPYEARR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ODPYEARR { bits }
    }
    #[doc = "Bits 21:24 - 24:21\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn odpmonth(&self) -> ODPMONTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ODPMONTHR { bits }
    }
    #[doc = "Bits 16:20 - 20:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn odpday(&self) -> ODPDAYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ODPDAYR { bits }
    }
    #[doc = "Bits 9:15 - 15:9\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efuseyear(&self) -> EFUSEYEARR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EFUSEYEARR { bits }
    }
    #[doc = "Bits 5:8 - 8:5\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efusemonth(&self) -> EFUSEMONTHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EFUSEMONTHR { bits }
    }
    #[doc = "Bits 0:4 - 4:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efuseday(&self) -> EFUSEDAYR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EFUSEDAYR { bits }
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
    #[doc = "Bits 25:31 - 31:25\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn odpyear(&mut self) -> _ODPYEARW {
        _ODPYEARW { w: self }
    }
    #[doc = "Bits 21:24 - 24:21\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn odpmonth(&mut self) -> _ODPMONTHW {
        _ODPMONTHW { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn odpday(&mut self) -> _ODPDAYW {
        _ODPDAYW { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efuseyear(&mut self) -> _EFUSEYEARW {
        _EFUSEYEARW { w: self }
    }
    #[doc = "Bits 5:8 - 8:5\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efusemonth(&mut self) -> _EFUSEMONTHW {
        _EFUSEMONTHW { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efuseday(&mut self) -> _EFUSEDAYW {
        _EFUSEDAYW { w: self }
    }
}
