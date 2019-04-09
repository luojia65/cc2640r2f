#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT2 {
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
pub struct ADC_DCBIASR {
    bits: u8,
}
impl ADC_DCBIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPM_RAMP1_THMETR {
    bits: bool,
}
impl HPM_RAMP1_THMETR {
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
pub struct HPM_RAMP2_THMETR {
    bits: bool,
}
impl HPM_RAMP2_THMETR {
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
pub struct HPM_RAMP3_THMETR {
    bits: bool,
}
impl HPM_RAMP3_THMETR {
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
pub struct RESERVED16R {
    bits: u8,
}
impl RESERVED16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RAMPSTATER {
    bits: u8,
}
impl RAMPSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED4R {
    bits: u8,
}
impl RESERVED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AMPCOMP_REQR {
    bits: bool,
}
impl AMPCOMP_REQR {
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
pub struct XOSC_HF_AMPGOODR {
    bits: bool,
}
impl XOSC_HF_AMPGOODR {
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
pub struct XOSC_HF_FREQGOODR {
    bits: bool,
}
impl XOSC_HF_FREQGOODR {
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
pub struct XOSC_HF_RF_FREQGOODR {
    bits: bool,
}
impl XOSC_HF_RF_FREQGOODR {
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
#[doc = r" Proxy"]
pub struct _ADC_DCBIASW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCBIASW<'a> {
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
pub struct _HPM_RAMP1_THMETW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_RAMP1_THMETW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_RAMP2_THMETW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_RAMP2_THMETW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_RAMP3_THMETW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_RAMP3_THMETW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED16W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED16W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RAMPSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMPSTATEW<'a> {
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
pub struct _RESERVED4W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AMPCOMP_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _AMPCOMP_REQW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_HF_AMPGOODW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_HF_AMPGOODW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_HF_FREQGOODW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_HF_FREQGOODW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_HF_RF_FREQGOODW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_HF_RF_FREQGOODW<'a> {
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
    #[doc = "Bits 26:31 - 31:26\\] DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[inline]
    pub fn adc_dcbias(&self) -> ADC_DCBIASR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADC_DCBIASR { bits }
    }
    #[doc = "Bit 25 - 25:25\\] Indication of threshold is met for hpm_ramp1"]
    #[inline]
    pub fn hpm_ramp1_thmet(&self) -> HPM_RAMP1_THMETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_RAMP1_THMETR { bits }
    }
    #[doc = "Bit 24 - 24:24\\] Indication of threshold is met for hpm_ramp2"]
    #[inline]
    pub fn hpm_ramp2_thmet(&self) -> HPM_RAMP2_THMETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_RAMP2_THMETR { bits }
    }
    #[doc = "Bit 23 - 23:23\\] Indication of threshold is met for hpm_ramp3"]
    #[inline]
    pub fn hpm_ramp3_thmet(&self) -> HPM_RAMP3_THMETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPM_RAMP3_THMETR { bits }
    }
    #[doc = "Bits 16:22 - 22:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&self) -> RESERVED16R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED16R { bits }
    }
    #[doc = "Bits 12:15 - 15:12\\] xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[inline]
    pub fn rampstate(&self) -> RAMPSTATER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RAMPSTATER { bits }
    }
    #[doc = "Bits 4:11 - 11:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED4R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] ampcomp_req"]
    #[inline]
    pub fn ampcomp_req(&self) -> AMPCOMP_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AMPCOMP_REQR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
    #[inline]
    pub fn xosc_hf_ampgood(&self) -> XOSC_HF_AMPGOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_HF_AMPGOODR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] frequency of xosc_hf is good to use for the digital clocks"]
    #[inline]
    pub fn xosc_hf_freqgood(&self) -> XOSC_HF_FREQGOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_HF_FREQGOODR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[inline]
    pub fn xosc_hf_rf_freqgood(&self) -> XOSC_HF_RF_FREQGOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_HF_RF_FREQGOODR { bits }
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
    #[doc = "Bits 26:31 - 31:26\\] DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[inline]
    pub fn adc_dcbias(&mut self) -> _ADC_DCBIASW {
        _ADC_DCBIASW { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Indication of threshold is met for hpm_ramp1"]
    #[inline]
    pub fn hpm_ramp1_thmet(&mut self) -> _HPM_RAMP1_THMETW {
        _HPM_RAMP1_THMETW { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Indication of threshold is met for hpm_ramp2"]
    #[inline]
    pub fn hpm_ramp2_thmet(&mut self) -> _HPM_RAMP2_THMETW {
        _HPM_RAMP2_THMETW { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Indication of threshold is met for hpm_ramp3"]
    #[inline]
    pub fn hpm_ramp3_thmet(&mut self) -> _HPM_RAMP3_THMETW {
        _HPM_RAMP3_THMETW { w: self }
    }
    #[doc = "Bits 16:22 - 22:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&mut self) -> _RESERVED16W {
        _RESERVED16W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[inline]
    pub fn rampstate(&mut self) -> _RAMPSTATEW {
        _RAMPSTATEW { w: self }
    }
    #[doc = "Bits 4:11 - 11:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] ampcomp_req"]
    #[inline]
    pub fn ampcomp_req(&mut self) -> _AMPCOMP_REQW {
        _AMPCOMP_REQW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
    #[inline]
    pub fn xosc_hf_ampgood(&mut self) -> _XOSC_HF_AMPGOODW {
        _XOSC_HF_AMPGOODW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] frequency of xosc_hf is good to use for the digital clocks"]
    #[inline]
    pub fn xosc_hf_freqgood(&mut self) -> _XOSC_HF_FREQGOODW {
        _XOSC_HF_FREQGOODW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[inline]
    pub fn xosc_hf_rf_freqgood(&mut self) -> _XOSC_HF_RF_FREQGOODW {
        _XOSC_HF_RF_FREQGOODW { w: self }
    }
}
