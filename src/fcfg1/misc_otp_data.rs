#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISC_OTP_DATA {
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
pub struct RCOSC_HF_ITUNER {
    bits: u8,
}
impl RCOSC_HF_ITUNER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RCOSC_HF_CRIMR {
    bits: u8,
}
impl RCOSC_HF_CRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PER_MR {
    bits: u8,
}
impl PER_MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PER_ER {
    bits: u8,
}
impl PER_ER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PO_TAIL_RES_TRIMR {
    bits: u8,
}
impl PO_TAIL_RES_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TEST_PROGRAM_REVR {
    bits: u8,
}
impl TEST_PROGRAM_REVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RCOSC_HF_ITUNEW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSC_HF_ITUNEW<'a> {
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
pub struct _RCOSC_HF_CRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSC_HF_CRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PER_MW<'a> {
    w: &'a mut W,
}
impl<'a> _PER_MW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PER_EW<'a> {
    w: &'a mut W,
}
impl<'a> _PER_EW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PO_TAIL_RES_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _PO_TAIL_RES_TRIMW<'a> {
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
pub struct _TEST_PROGRAM_REVW<'a> {
    w: &'a mut W,
}
impl<'a> _TEST_PROGRAM_REVW<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosc_hf_itune(&self) -> RCOSC_HF_ITUNER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSC_HF_ITUNER { bits }
    }
    #[doc = "Bits 20:27 - 27:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosc_hf_crim(&self) -> RCOSC_HF_CRIMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RCOSC_HF_CRIMR { bits }
    }
    #[doc = "Bits 15:19 - 19:15\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn per_m(&self) -> PER_MR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PER_MR { bits }
    }
    #[doc = "Bits 12:14 - 14:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn per_e(&self) -> PER_ER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PER_ER { bits }
    }
    #[doc = "Bits 8:11 - 11:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn po_tail_res_trim(&self) -> PO_TAIL_RES_TRIMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PO_TAIL_RES_TRIMR { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
    #[inline]
    pub fn test_program_rev(&self) -> TEST_PROGRAM_REVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TEST_PROGRAM_REVR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 50688 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosc_hf_itune(&mut self) -> _RCOSC_HF_ITUNEW {
        _RCOSC_HF_ITUNEW { w: self }
    }
    #[doc = "Bits 20:27 - 27:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosc_hf_crim(&mut self) -> _RCOSC_HF_CRIMW {
        _RCOSC_HF_CRIMW { w: self }
    }
    #[doc = "Bits 15:19 - 19:15\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn per_m(&mut self) -> _PER_MW {
        _PER_MW { w: self }
    }
    #[doc = "Bits 12:14 - 14:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn per_e(&mut self) -> _PER_EW {
        _PER_EW { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn po_tail_res_trim(&mut self) -> _PO_TAIL_RES_TRIMW {
        _PO_TAIL_RES_TRIMW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
    #[inline]
    pub fn test_program_rev(&mut self) -> _TEST_PROGRAM_REVW {
        _TEST_PROGRAM_REVW { w: self }
    }
}
