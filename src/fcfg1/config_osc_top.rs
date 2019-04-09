#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG_OSC_TOP {
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
pub struct XOSC_HF_ROW_Q12R {
    bits: u8,
}
impl XOSC_HF_ROW_Q12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_HF_COLUMN_Q12R {
    bits: u16,
}
impl XOSC_HF_COLUMN_Q12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSCLF_CTUNE_TRIMR {
    bits: u8,
}
impl RCOSCLF_CTUNE_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSCLF_RTUNE_TRIMR {
    bits: u8,
}
impl RCOSCLF_RTUNE_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_HF_ROW_Q12W<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_HF_ROW_Q12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_HF_COLUMN_Q12W<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_HF_COLUMN_Q12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RCOSCLF_CTUNE_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSCLF_CTUNE_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RCOSCLF_RTUNE_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSCLF_RTUNE_TRIMW<'a> {
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
    #[doc = "Bits 26:29 - 29:26\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_row_q12(&self) -> XOSC_HF_ROW_Q12R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSC_HF_ROW_Q12R { bits }
    }
    #[doc = "Bits 10:25 - 25:10\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_column_q12(&self) -> XOSC_HF_COLUMN_Q12R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        XOSC_HF_COLUMN_Q12R { bits }
    }
    #[doc = "Bits 2:9 - 9:2\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosclf_ctune_trim(&self) -> RCOSCLF_CTUNE_TRIMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSCLF_CTUNE_TRIMR { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosclf_rtune_trim(&self) -> RCOSCLF_RTUNE_TRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSCLF_RTUNE_TRIMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4227922944 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 26:29 - 29:26\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_row_q12(&mut self) -> _XOSC_HF_ROW_Q12W {
        _XOSC_HF_ROW_Q12W { w: self }
    }
    #[doc = "Bits 10:25 - 25:10\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_column_q12(&mut self) -> _XOSC_HF_COLUMN_Q12W {
        _XOSC_HF_COLUMN_Q12W { w: self }
    }
    #[doc = "Bits 2:9 - 9:2\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosclf_ctune_trim(&mut self) -> _RCOSCLF_CTUNE_TRIMW {
        _RCOSCLF_CTUNE_TRIMW { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosclf_rtune_trim(&mut self) -> _RCOSCLF_RTUNE_TRIMW {
        _RCOSCLF_RTUNE_TRIMW { w: self }
    }
}
