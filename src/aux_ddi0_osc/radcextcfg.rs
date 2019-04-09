#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RADCEXTCFG {
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
#[doc = r" Value of the field"]
pub struct RADC_DAC_THR {
    bits: u8,
}
impl RADC_DAC_THR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RADC_MODE_IS_SARR {
    bits: bool,
}
impl RADC_MODE_IS_SARR {
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
pub struct RESERVED0R {
    bits: u8,
}
impl RESERVED0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
        const OFFSET: u8 = 22;
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
        const OFFSET: u8 = 16;
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RADC_DAC_THW<'a> {
    w: &'a mut W,
}
impl<'a> _RADC_DAC_THW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RADC_MODE_IS_SARW<'a> {
    w: &'a mut W,
}
impl<'a> _RADC_MODE_IS_SARW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED0W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED0W<'a> {
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
    #[doc = "Bits 22:31 - 31:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpm_ibias_wait_cnt(&self) -> HPM_IBIAS_WAIT_CNTR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HPM_IBIAS_WAIT_CNTR { bits }
    }
    #[doc = "Bits 16:21 - 21:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lpm_ibias_wait_cnt(&self) -> LPM_IBIAS_WAIT_CNTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPM_IBIAS_WAIT_CNTR { bits }
    }
    #[doc = "Bits 12:15 - 15:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn idac_step(&self) -> IDAC_STEPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IDAC_STEPR { bits }
    }
    #[doc = "Bits 6:11 - 11:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn radc_dac_th(&self) -> RADC_DAC_THR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RADC_DAC_THR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn radc_mode_is_sar(&self) -> RADC_MODE_IS_SARR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RADC_MODE_IS_SARR { bits }
    }
    #[doc = "Bits 0:4 - 4:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED0R { bits }
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
    #[doc = "Bits 22:31 - 31:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hpm_ibias_wait_cnt(&mut self) -> _HPM_IBIAS_WAIT_CNTW {
        _HPM_IBIAS_WAIT_CNTW { w: self }
    }
    #[doc = "Bits 16:21 - 21:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn lpm_ibias_wait_cnt(&mut self) -> _LPM_IBIAS_WAIT_CNTW {
        _LPM_IBIAS_WAIT_CNTW { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn idac_step(&mut self) -> _IDAC_STEPW {
        _IDAC_STEPW { w: self }
    }
    #[doc = "Bits 6:11 - 11:6\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn radc_dac_th(&mut self) -> _RADC_DAC_THW {
        _RADC_DAC_THW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn radc_mode_is_sar(&mut self) -> _RADC_MODE_IS_SARW {
        _RADC_MODE_IS_SARW { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved0(&mut self) -> _RESERVED0W {
        _RESERVED0W { w: self }
    }
}
