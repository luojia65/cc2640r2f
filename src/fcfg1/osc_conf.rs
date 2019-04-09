#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSC_CONF {
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
pub struct RESERVED1R {
    bits: u8,
}
impl RESERVED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADC_SH_VBUF_ENR {
    bits: bool,
}
impl ADC_SH_VBUF_ENR {
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
pub struct ADC_SH_MODE_ENR {
    bits: bool,
}
impl ADC_SH_MODE_ENR {
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
pub struct ATESTLF_RCOSCLF_IBIAS_TRIMR {
    bits: bool,
}
impl ATESTLF_RCOSCLF_IBIAS_TRIMR {
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
pub struct XOSCLF_REGULATOR_TRIMR {
    bits: u8,
}
impl XOSCLF_REGULATOR_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSCLF_CMIRRWR_RATIOR {
    bits: u8,
}
impl XOSCLF_CMIRRWR_RATIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_HF_FAST_STARTR {
    bits: u8,
}
impl XOSC_HF_FAST_STARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_OPTIONR {
    bits: bool,
}
impl XOSC_OPTIONR {
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
pub struct HPOSC_OPTIONR {
    bits: bool,
}
impl HPOSC_OPTIONR {
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
pub struct HPOSC_BIAS_HOLD_MODE_ENR {
    bits: bool,
}
impl HPOSC_BIAS_HOLD_MODE_ENR {
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
pub struct HPOSC_CURRMIRR_RATIOR {
    bits: u8,
}
impl HPOSC_CURRMIRR_RATIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPOSC_BIAS_RES_SETR {
    bits: u8,
}
impl HPOSC_BIAS_RES_SETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPOSC_FILTER_ENR {
    bits: bool,
}
impl HPOSC_FILTER_ENR {
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
pub struct HPOSC_BIAS_RECHARGE_DELAYR {
    bits: u8,
}
impl HPOSC_BIAS_RECHARGE_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED2R {
    bits: u8,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPOSC_SERIES_CAPR {
    bits: u8,
}
impl HPOSC_SERIES_CAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HPOSC_DIV3_BYPASSR {
    bits: bool,
}
impl HPOSC_DIV3_BYPASSR {
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
pub struct _RESERVED1W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC_SH_VBUF_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SH_VBUF_ENW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC_SH_MODE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SH_MODE_ENW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ATESTLF_RCOSCLF_IBIAS_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _ATESTLF_RCOSCLF_IBIAS_TRIMW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSCLF_REGULATOR_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCLF_REGULATOR_TRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSCLF_CMIRRWR_RATIOW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCLF_CMIRRWR_RATIOW<'a> {
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
pub struct _XOSC_HF_FAST_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_HF_FAST_STARTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_OPTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_OPTIONW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPOSC_OPTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _HPOSC_OPTIONW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPOSC_BIAS_HOLD_MODE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HPOSC_BIAS_HOLD_MODE_ENW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPOSC_CURRMIRR_RATIOW<'a> {
    w: &'a mut W,
}
impl<'a> _HPOSC_CURRMIRR_RATIOW<'a> {
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
pub struct _HPOSC_BIAS_RES_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _HPOSC_BIAS_RES_SETW<'a> {
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
pub struct _HPOSC_FILTER_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HPOSC_FILTER_ENW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPOSC_BIAS_RECHARGE_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _HPOSC_BIAS_RECHARGE_DELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPOSC_SERIES_CAPW<'a> {
    w: &'a mut W,
}
impl<'a> _HPOSC_SERIES_CAPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPOSC_DIV3_BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _HPOSC_DIV3_BYPASSW<'a> {
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
    #[doc = "Bits 30:31 - 31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED1R { bits }
    }
    #[doc = "Bit 29 - 29:29\\] Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
    #[inline]
    pub fn adc_sh_vbuf_en(&self) -> ADC_SH_VBUF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC_SH_VBUF_ENR { bits }
    }
    #[doc = "Bit 28 - 28:28\\] Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
    #[inline]
    pub fn adc_sh_mode_en(&self) -> ADC_SH_MODE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC_SH_MODE_ENR { bits }
    }
    #[doc = "Bit 27 - 27:27\\] Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
    #[inline]
    pub fn atestlf_rcosclf_ibias_trim(&self) -> ATESTLF_RCOSCLF_IBIAS_TRIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ATESTLF_RCOSCLF_IBIAS_TRIMR { bits }
    }
    #[doc = "Bits 25:26 - 26:25\\] Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
    #[inline]
    pub fn xosclf_regulator_trim(&self) -> XOSCLF_REGULATOR_TRIMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSCLF_REGULATOR_TRIMR { bits }
    }
    #[doc = "Bits 21:24 - 24:21\\] Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
    #[inline]
    pub fn xosclf_cmirrwr_ratio(&self) -> XOSCLF_CMIRRWR_RATIOR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSCLF_CMIRRWR_RATIOR { bits }
    }
    #[doc = "Bits 19:20 - 20:19\\] Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
    #[inline]
    pub fn xosc_hf_fast_start(&self) -> XOSC_HF_FAST_STARTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSC_HF_FAST_STARTR { bits }
    }
    #[doc = "Bit 18 - 18:18\\] 0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)"]
    #[inline]
    pub fn xosc_option(&self) -> XOSC_OPTIONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_OPTIONR { bits }
    }
    #[doc = "Bit 17 - 17:17\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_option(&self) -> HPOSC_OPTIONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPOSC_OPTIONR { bits }
    }
    #[doc = "Bit 16 - 16:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_bias_hold_mode_en(&self) -> HPOSC_BIAS_HOLD_MODE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPOSC_BIAS_HOLD_MODE_ENR { bits }
    }
    #[doc = "Bits 12:15 - 15:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_currmirr_ratio(&self) -> HPOSC_CURRMIRR_RATIOR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPOSC_CURRMIRR_RATIOR { bits }
    }
    #[doc = "Bits 8:11 - 11:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_bias_res_set(&self) -> HPOSC_BIAS_RES_SETR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPOSC_BIAS_RES_SETR { bits }
    }
    #[doc = "Bit 7 - 7:7\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_filter_en(&self) -> HPOSC_FILTER_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPOSC_FILTER_ENR { bits }
    }
    #[doc = "Bits 5:6 - 6:5\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_bias_recharge_delay(&self) -> HPOSC_BIAS_RECHARGE_DELAYR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPOSC_BIAS_RECHARGE_DELAYR { bits }
    }
    #[doc = "Bits 3:4 - 4:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 1:2 - 2:1\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_series_cap(&self) -> HPOSC_SERIES_CAPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPOSC_SERIES_CAPR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_div3_bypass(&self) -> HPOSC_DIV3_BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPOSC_DIV3_BYPASSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4027056128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 30:31 - 31:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved1(&mut self) -> _RESERVED1W {
        _RESERVED1W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_VBUF_EN."]
    #[inline]
    pub fn adc_sh_vbuf_en(&mut self) -> _ADC_SH_VBUF_ENW {
        _ADC_SH_VBUF_ENW { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] Trim value for DDI_0_OSC:ADCDOUBLERNANOAMPCTL.ADC_SH_MODE_EN."]
    #[inline]
    pub fn adc_sh_mode_en(&mut self) -> _ADC_SH_MODE_ENW {
        _ADC_SH_MODE_ENW { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] Trim value for DDI_0_OSC:ATESTCTL.ATESTLF_RCOSCLF_IBIAS_TRIM."]
    #[inline]
    pub fn atestlf_rcosclf_ibias_trim(&mut self) -> _ATESTLF_RCOSCLF_IBIAS_TRIMW {
        _ATESTLF_RCOSCLF_IBIAS_TRIMW { w: self }
    }
    #[doc = "Bits 25:26 - 26:25\\] Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_REGULATOR_TRIM."]
    #[inline]
    pub fn xosclf_regulator_trim(&mut self) -> _XOSCLF_REGULATOR_TRIMW {
        _XOSCLF_REGULATOR_TRIMW { w: self }
    }
    #[doc = "Bits 21:24 - 24:21\\] Trim value for DDI_0_OSC:LFOSCCTL.XOSCLF_CMIRRWR_RATIO."]
    #[inline]
    pub fn xosclf_cmirrwr_ratio(&mut self) -> _XOSCLF_CMIRRWR_RATIOW {
        _XOSCLF_CMIRRWR_RATIOW { w: self }
    }
    #[doc = "Bits 19:20 - 20:19\\] Trim value for DDI_0_OSC:CTL1.XOSC_HF_FAST_START."]
    #[inline]
    pub fn xosc_hf_fast_start(&mut self) -> _XOSC_HF_FAST_STARTW {
        _XOSC_HF_FAST_STARTW { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] 0: XOSC_HF unavailable (may not be bonded out) 1: XOSC_HF available (default)"]
    #[inline]
    pub fn xosc_option(&mut self) -> _XOSC_OPTIONW {
        _XOSC_OPTIONW { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_option(&mut self) -> _HPOSC_OPTIONW {
        _HPOSC_OPTIONW { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_bias_hold_mode_en(&mut self) -> _HPOSC_BIAS_HOLD_MODE_ENW {
        _HPOSC_BIAS_HOLD_MODE_ENW { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_currmirr_ratio(&mut self) -> _HPOSC_CURRMIRR_RATIOW {
        _HPOSC_CURRMIRR_RATIOW { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_bias_res_set(&mut self) -> _HPOSC_BIAS_RES_SETW {
        _HPOSC_BIAS_RES_SETW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_filter_en(&mut self) -> _HPOSC_FILTER_ENW {
        _HPOSC_FILTER_ENW { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_bias_recharge_delay(&mut self) -> _HPOSC_BIAS_RECHARGE_DELAYW {
        _HPOSC_BIAS_RECHARGE_DELAYW { w: self }
    }
    #[doc = "Bits 3:4 - 4:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_series_cap(&mut self) -> _HPOSC_SERIES_CAPW {
        _HPOSC_SERIES_CAPW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_div3_bypass(&mut self) -> _HPOSC_DIV3_BYPASSW {
        _HPOSC_DIV3_BYPASSW { w: self }
    }
}
