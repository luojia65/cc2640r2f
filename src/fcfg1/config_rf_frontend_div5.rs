#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG_RF_FRONTEND_DIV5 {
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
pub struct IFAMP_IBR {
    bits: u8,
}
impl IFAMP_IBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNA_IBR {
    bits: u8,
}
impl LNA_IBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IFAMP_TRIMR {
    bits: u8,
}
impl IFAMP_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CTL_PA0_TRIMR {
    bits: u8,
}
impl CTL_PA0_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RFLDO_TRIM_OUTPUTR {
    bits: u8,
}
impl RFLDO_TRIM_OUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _IFAMP_IBW<'a> {
    w: &'a mut W,
}
impl<'a> _IFAMP_IBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LNA_IBW<'a> {
    w: &'a mut W,
}
impl<'a> _LNA_IBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IFAMP_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _IFAMP_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTL_PA0_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _CTL_PA0_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFLDO_TRIM_OUTPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _RFLDO_TRIM_OUTPUTW<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ifamp_ib(&self) -> IFAMP_IBR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IFAMP_IBR { bits }
    }
    #[doc = "Bits 24:27 - 27:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lna_ib(&self) -> LNA_IBR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LNA_IBR { bits }
    }
    #[doc = "Bits 19:23 - 23:19\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ifamp_trim(&self) -> IFAMP_TRIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IFAMP_TRIMR { bits }
    }
    #[doc = "Bits 14:18 - 18:14\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ctl_pa0_trim(&self) -> CTL_PA0_TRIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CTL_PA0_TRIMR { bits }
    }
    #[doc = "Bits 0:6 - 6:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rfldo_trim_output(&self) -> RFLDO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RFLDO_TRIM_OUTPUTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ifamp_ib(&mut self) -> _IFAMP_IBW {
        _IFAMP_IBW { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lna_ib(&mut self) -> _LNA_IBW {
        _LNA_IBW { w: self }
    }
    #[doc = "Bits 19:23 - 23:19\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ifamp_trim(&mut self) -> _IFAMP_TRIMW {
        _IFAMP_TRIMW { w: self }
    }
    #[doc = "Bits 14:18 - 18:14\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ctl_pa0_trim(&mut self) -> _CTL_PA0_TRIMW {
        _CTL_PA0_TRIMW { w: self }
    }
    #[doc = "Bits 0:6 - 6:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rfldo_trim_output(&mut self) -> _RFLDO_TRIM_OUTPUTW {
        _RFLDO_TRIM_OUTPUTW { w: self }
    }
}
