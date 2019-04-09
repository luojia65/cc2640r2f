#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AMPCOMPTH2 {
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
pub struct LPMUPDATE_LTHR {
    bits: u8,
}
impl LPMUPDATE_LTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
pub struct LPMUPDATE_HTHR {
    bits: u8,
}
impl LPMUPDATE_HTHR {
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
pub struct ADC_COMP_AMPTH_LPMR {
    bits: u8,
}
impl ADC_COMP_AMPTH_LPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPARE8R {
    bits: u8,
}
impl SPARE8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADC_COMP_AMPTH_HPMR {
    bits: u8,
}
impl ADC_COMP_AMPTH_HPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPARE0R {
    bits: u8,
}
impl SPARE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LPMUPDATE_LTHW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMUPDATE_LTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPMUPDATE_HTHW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMUPDATE_HTHW<'a> {
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
pub struct _ADC_COMP_AMPTH_LPMW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_COMP_AMPTH_LPMW<'a> {
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
pub struct _SPARE8W<'a> {
    w: &'a mut W,
}
impl<'a> _SPARE8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC_COMP_AMPTH_HPMW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_COMP_AMPTH_HPMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPARE0W<'a> {
    w: &'a mut W,
}
impl<'a> _SPARE0W<'a> {
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
    #[doc = "Bits 26:31 - 31:26\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lpmupdate_lth(&self) -> LPMUPDATE_LTHR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPMUPDATE_LTHR { bits }
    }
    #[doc = "Bits 24:25 - 25:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare24(&self) -> SPARE24R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPARE24R { bits }
    }
    #[doc = "Bits 18:23 - 23:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lpmupdate_hth(&self) -> LPMUPDATE_HTHR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPMUPDATE_HTHR { bits }
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
    pub fn adc_comp_ampth_lpm(&self) -> ADC_COMP_AMPTH_LPMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADC_COMP_AMPTH_LPMR { bits }
    }
    #[doc = "Bits 8:9 - 9:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare8(&self) -> SPARE8R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPARE8R { bits }
    }
    #[doc = "Bits 2:7 - 7:2\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn adc_comp_ampth_hpm(&self) -> ADC_COMP_AMPTH_HPMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADC_COMP_AMPTH_HPMR { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare0(&self) -> SPARE0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SPARE0R { bits }
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
    #[doc = "Bits 26:31 - 31:26\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lpmupdate_lth(&mut self) -> _LPMUPDATE_LTHW {
        _LPMUPDATE_LTHW { w: self }
    }
    #[doc = "Bits 24:25 - 25:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare24(&mut self) -> _SPARE24W {
        _SPARE24W { w: self }
    }
    #[doc = "Bits 18:23 - 23:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lpmupdate_hth(&mut self) -> _LPMUPDATE_HTHW {
        _LPMUPDATE_HTHW { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare16(&mut self) -> _SPARE16W {
        _SPARE16W { w: self }
    }
    #[doc = "Bits 10:15 - 15:10\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn adc_comp_ampth_lpm(&mut self) -> _ADC_COMP_AMPTH_LPMW {
        _ADC_COMP_AMPTH_LPMW { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare8(&mut self) -> _SPARE8W {
        _SPARE8W { w: self }
    }
    #[doc = "Bits 2:7 - 7:2\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn adc_comp_ampth_hpm(&mut self) -> _ADC_COMP_AMPTH_HPMW {
        _ADC_COMP_AMPTH_HPMW { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare0(&mut self) -> _SPARE0W {
        _SPARE0W { w: self }
    }
}
