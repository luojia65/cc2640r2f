#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHDW_OSC_BIAS_LDO_TRIM {
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
pub struct SET_RCOSC_HF_COARSE_RESISTORR {
    bits: u8,
}
impl SET_RCOSC_HF_COARSE_RESISTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIMMAGR {
    bits: u8,
}
impl TRIMMAGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIMIREFR {
    bits: u8,
}
impl TRIMIREFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ITRIM_DIG_LDOR {
    bits: u8,
}
impl ITRIM_DIG_LDOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VTRIM_DIGR {
    bits: u8,
}
impl VTRIM_DIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VTRIM_COARSER {
    bits: u8,
}
impl VTRIM_COARSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSCHF_CTRIMR {
    bits: u8,
}
impl RCOSCHF_CTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SET_RCOSC_HF_COARSE_RESISTORW<'a> {
    w: &'a mut W,
}
impl<'a> _SET_RCOSC_HF_COARSE_RESISTORW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIMMAGW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIMMAGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIMIREFW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIMIREFW<'a> {
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
pub struct _ITRIM_DIG_LDOW<'a> {
    w: &'a mut W,
}
impl<'a> _ITRIM_DIG_LDOW<'a> {
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
pub struct _VTRIM_DIGW<'a> {
    w: &'a mut W,
}
impl<'a> _VTRIM_DIGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VTRIM_COARSEW<'a> {
    w: &'a mut W,
}
impl<'a> _VTRIM_COARSEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RCOSCHF_CTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSCHF_CTRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 27:28 - 28:27\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn set_rcosc_hf_coarse_resistor(&self) -> SET_RCOSC_HF_COARSE_RESISTORR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SET_RCOSC_HF_COARSE_RESISTORR { bits }
    }
    #[doc = "Bits 23:26 - 26:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimmag(&self) -> TRIMMAGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIMMAGR { bits }
    }
    #[doc = "Bits 18:22 - 22:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimiref(&self) -> TRIMIREFR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIMIREFR { bits }
    }
    #[doc = "Bits 16:17 - 17:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn itrim_dig_ldo(&self) -> ITRIM_DIG_LDOR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ITRIM_DIG_LDOR { bits }
    }
    #[doc = "Bits 12:15 - 15:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vtrim_dig(&self) -> VTRIM_DIGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VTRIM_DIGR { bits }
    }
    #[doc = "Bits 8:11 - 11:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vtrim_coarse(&self) -> VTRIM_COARSER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VTRIM_COARSER { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschf_ctrim(&self) -> RCOSCHF_CTRIMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSCHF_CTRIMR { bits }
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
    #[doc = "Bits 27:28 - 28:27\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn set_rcosc_hf_coarse_resistor(&mut self) -> _SET_RCOSC_HF_COARSE_RESISTORW {
        _SET_RCOSC_HF_COARSE_RESISTORW { w: self }
    }
    #[doc = "Bits 23:26 - 26:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimmag(&mut self) -> _TRIMMAGW {
        _TRIMMAGW { w: self }
    }
    #[doc = "Bits 18:22 - 22:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trimiref(&mut self) -> _TRIMIREFW {
        _TRIMIREFW { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn itrim_dig_ldo(&mut self) -> _ITRIM_DIG_LDOW {
        _ITRIM_DIG_LDOW { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vtrim_dig(&mut self) -> _VTRIM_DIGW {
        _VTRIM_DIGW { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vtrim_coarse(&mut self) -> _VTRIM_COARSEW {
        _VTRIM_COARSEW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschf_ctrim(&mut self) -> _RCOSCHF_CTRIMW {
        _RCOSCHF_CTRIMW { w: self }
    }
}
