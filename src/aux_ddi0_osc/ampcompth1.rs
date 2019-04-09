#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AMPCOMPTH1 {
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
pub struct SPARE24R {
    bits: u8,
}
impl SPARE24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPMRAMP3_LTHR {
    bits: u8,
}
impl HPMRAMP3_LTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPARE16R {
    bits: u8,
}
impl SPARE16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPMRAMP3_HTHR {
    bits: u8,
}
impl HPMRAMP3_HTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IBIASCAP_LPTOHP_OL_CNTR {
    bits: u8,
}
impl IBIASCAP_LPTOHP_OL_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPMRAMP1_THR {
    bits: u8,
}
impl HPMRAMP1_THR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SPARE24W<'a> {
    w: &'a mut W,
}
impl<'a> _SPARE24W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPMRAMP3_LTHW<'a> {
    w: &'a mut W,
}
impl<'a> _HPMRAMP3_LTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPARE16W<'a> {
    w: &'a mut W,
}
impl<'a> _SPARE16W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPMRAMP3_HTHW<'a> {
    w: &'a mut W,
}
impl<'a> _HPMRAMP3_HTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IBIASCAP_LPTOHP_OL_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _IBIASCAP_LPTOHP_OL_CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPMRAMP1_THW<'a> {
    w: &'a mut W,
}
impl<'a> _HPMRAMP1_THW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bits 24:31 - 31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare24(&self) -> SPARE24R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPARE24R { bits }
    }
    #[doc = "Bits 18:23 - 23:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpmramp3_lth(&self) -> HPMRAMP3_LTHR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPMRAMP3_LTHR { bits }
    }
    #[doc = "Bits 16:17 - 17:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare16(&self) -> SPARE16R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPARE16R { bits }
    }
    #[doc = "Bits 10:15 - 15:10\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpmramp3_hth(&self) -> HPMRAMP3_HTHR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPMRAMP3_HTHR { bits }
    }
    #[doc = "Bits 6:9 - 9:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ibiascap_lptohp_ol_cnt(&self) -> IBIASCAP_LPTOHP_OL_CNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IBIASCAP_LPTOHP_OL_CNTR { bits }
    }
    #[doc = "Bits 0:5 - 5:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpmramp1_th(&self) -> HPMRAMP1_THR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPMRAMP1_THR { bits }
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
    #[doc = "Bits 24:31 - 31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare24(&mut self) -> _SPARE24W {
        _SPARE24W { w: self }
    }
    #[doc = "Bits 18:23 - 23:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpmramp3_lth(&mut self) -> _HPMRAMP3_LTHW {
        _HPMRAMP3_LTHW { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare16(&mut self) -> _SPARE16W {
        _SPARE16W { w: self }
    }
    #[doc = "Bits 10:15 - 15:10\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpmramp3_hth(&mut self) -> _HPMRAMP3_HTHW {
        _HPMRAMP3_HTHW { w: self }
    }
    #[doc = "Bits 6:9 - 9:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ibiascap_lptohp_ol_cnt(&mut self) -> _IBIASCAP_LPTOHP_OL_CNTW {
        _IBIASCAP_LPTOHP_OL_CNTW { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpmramp1_th(&mut self) -> _HPMRAMP1_THW {
        _HPMRAMP1_THW { w: self }
    }
}
