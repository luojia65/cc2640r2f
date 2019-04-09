#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISC_OTP_DATA_1 {
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
pub struct PEAK_DET_ITRIMR {
    bits: u8,
}
impl PEAK_DET_ITRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HP_BUF_ITRIMR {
    bits: u8,
}
impl HP_BUF_ITRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LP_BUF_ITRIMR {
    bits: u8,
}
impl LP_BUF_ITRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DBLR_LOOP_FILTER_RESET_VOLTAGER {
    bits: u8,
}
impl DBLR_LOOP_FILTER_RESET_VOLTAGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPM_IBIAS_WAIT_CNTR {
    bits: u16,
}
impl HPM_IBIAS_WAIT_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPM_IBIAS_WAIT_CNTR {
    bits: u8,
}
impl LPM_IBIAS_WAIT_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDAC_STEPR {
    bits: u8,
}
impl IDAC_STEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PEAK_DET_ITRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _PEAK_DET_ITRIMW<'a> {
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
pub struct _HP_BUF_ITRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _HP_BUF_ITRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LP_BUF_ITRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _LP_BUF_ITRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DBLR_LOOP_FILTER_RESET_VOLTAGEW<'a> {
    w: &'a mut W,
}
impl<'a> _DBLR_LOOP_FILTER_RESET_VOLTAGEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_IBIAS_WAIT_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_IBIAS_WAIT_CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPM_IBIAS_WAIT_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_IBIAS_WAIT_CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IDAC_STEPW<'a> {
    w: &'a mut W,
}
impl<'a> _IDAC_STEPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    pub fn peak_det_itrim(&self) -> PEAK_DET_ITRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PEAK_DET_ITRIMR { bits }
    }
    #[doc = "Bits 24:26 - 26:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hp_buf_itrim(&self) -> HP_BUF_ITRIMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HP_BUF_ITRIMR { bits }
    }
    #[doc = "Bits 22:23 - 23:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lp_buf_itrim(&self) -> LP_BUF_ITRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LP_BUF_ITRIMR { bits }
    }
    #[doc = "Bits 20:21 - 21:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dblr_loop_filter_reset_voltage(&self) -> DBLR_LOOP_FILTER_RESET_VOLTAGER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DBLR_LOOP_FILTER_RESET_VOLTAGER { bits }
    }
    #[doc = "Bits 10:19 - 19:10\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpm_ibias_wait_cnt(&self) -> HPM_IBIAS_WAIT_CNTR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HPM_IBIAS_WAIT_CNTR { bits }
    }
    #[doc = "Bits 4:9 - 9:4\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lpm_ibias_wait_cnt(&self) -> LPM_IBIAS_WAIT_CNTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPM_IBIAS_WAIT_CNTR { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn idac_step(&self) -> IDAC_STEPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDAC_STEPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3758359544 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 27:28 - 28:27\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn peak_det_itrim(&mut self) -> _PEAK_DET_ITRIMW {
        _PEAK_DET_ITRIMW { w: self }
    }
    #[doc = "Bits 24:26 - 26:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hp_buf_itrim(&mut self) -> _HP_BUF_ITRIMW {
        _HP_BUF_ITRIMW { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lp_buf_itrim(&mut self) -> _LP_BUF_ITRIMW {
        _LP_BUF_ITRIMW { w: self }
    }
    #[doc = "Bits 20:21 - 21:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dblr_loop_filter_reset_voltage(&mut self) -> _DBLR_LOOP_FILTER_RESET_VOLTAGEW {
        _DBLR_LOOP_FILTER_RESET_VOLTAGEW { w: self }
    }
    #[doc = "Bits 10:19 - 19:10\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpm_ibias_wait_cnt(&mut self) -> _HPM_IBIAS_WAIT_CNTW {
        _HPM_IBIAS_WAIT_CNTW { w: self }
    }
    #[doc = "Bits 4:9 - 9:4\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lpm_ibias_wait_cnt(&mut self) -> _LPM_IBIAS_WAIT_CNTW {
        _LPM_IBIAS_WAIT_CNTW { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn idac_step(&mut self) -> _IDAC_STEPW {
        _IDAC_STEPW { w: self }
    }
}
