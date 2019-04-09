#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE_CONF {
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
pub struct VDDR_TRIM_SLEEP_DELTAR {
    bits: u8,
}
impl VDDR_TRIM_SLEEP_DELTAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCDC_RECHARGER {
    bits: bool,
}
impl DCDC_RECHARGER {
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
pub struct DCDC_ACTIVER {
    bits: bool,
}
impl DCDC_ACTIVER {
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
pub struct VDDR_EXT_LOADR {
    bits: bool,
}
impl VDDR_EXT_LOADR {
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
pub struct VDDS_BOD_LEVELR {
    bits: bool,
}
impl VDDS_BOD_LEVELR {
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
#[doc = "Possible values of the field `SCLK_LF_OPTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_LF_OPTIONR {
    #[doc = "Low frequency RCOSC (default)"]
    RCOSC_LF,
    #[doc = "32.768kHz low frequency XOSC"]
    XOSC_LF,
    #[doc = "External low frequency clock on DIO defined by EXT_LF_CLK.DIO. The RTC tick speed AON_RTC:SUBSECINC is updated to EXT_LF_CLK.RTC_INCREMENT (done in the trimDevice() xxWare boot function). External clock must always be running when the chip is in standby for VDDR recharge timing."]
    EXTERNAL_LF,
    #[doc = "31.25kHz clock derived from 24MHz XOSC (dividing by 768 in HW). The RTC tick speed \\[AON_RTC.SUBSECINC.*\\] is updated to 0x8637BD, corresponding to a 31.25kHz clock (done in the trimDevice() xxWare boot function). Standby power mode is not supported when using this clock source."]
    XOSC_HF_DLF,
}
impl SCLK_LF_OPTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCLK_LF_OPTIONR::RCOSC_LF => 3,
            SCLK_LF_OPTIONR::XOSC_LF => 2,
            SCLK_LF_OPTIONR::EXTERNAL_LF => 1,
            SCLK_LF_OPTIONR::XOSC_HF_DLF => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCLK_LF_OPTIONR {
        match value {
            3 => SCLK_LF_OPTIONR::RCOSC_LF,
            2 => SCLK_LF_OPTIONR::XOSC_LF,
            1 => SCLK_LF_OPTIONR::EXTERNAL_LF,
            0 => SCLK_LF_OPTIONR::XOSC_HF_DLF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCOSC_LF`"]
    #[inline]
    pub fn is_rcosc_lf(&self) -> bool {
        *self == SCLK_LF_OPTIONR::RCOSC_LF
    }
    #[doc = "Checks if the value of the field is `XOSC_LF`"]
    #[inline]
    pub fn is_xosc_lf(&self) -> bool {
        *self == SCLK_LF_OPTIONR::XOSC_LF
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_LF`"]
    #[inline]
    pub fn is_external_lf(&self) -> bool {
        *self == SCLK_LF_OPTIONR::EXTERNAL_LF
    }
    #[doc = "Checks if the value of the field is `XOSC_HF_DLF`"]
    #[inline]
    pub fn is_xosc_hf_dlf(&self) -> bool {
        *self == SCLK_LF_OPTIONR::XOSC_HF_DLF
    }
}
#[doc = r" Value of the field"]
pub struct VDDR_TRIM_SLEEP_TCR {
    bits: bool,
}
impl VDDR_TRIM_SLEEP_TCR {
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
pub struct RTC_COMPR {
    bits: bool,
}
impl RTC_COMPR {
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
#[doc = "Possible values of the field `XOSC_FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XOSC_FREQR {
    #[doc = "24 MHz XOSC_HF"]
    _24M,
    #[doc = "48 MHz XOSC_HF"]
    _48M,
    #[doc = "HPOSC"]
    HPOSC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl XOSC_FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            XOSC_FREQR::_24M => 3,
            XOSC_FREQR::_48M => 2,
            XOSC_FREQR::HPOSC => 1,
            XOSC_FREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> XOSC_FREQR {
        match value {
            3 => XOSC_FREQR::_24M,
            2 => XOSC_FREQR::_48M,
            1 => XOSC_FREQR::HPOSC,
            i => XOSC_FREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_24M`"]
    #[inline]
    pub fn is_24m(&self) -> bool {
        *self == XOSC_FREQR::_24M
    }
    #[doc = "Checks if the value of the field is `_48M`"]
    #[inline]
    pub fn is_48m(&self) -> bool {
        *self == XOSC_FREQR::_48M
    }
    #[doc = "Checks if the value of the field is `HPOSC`"]
    #[inline]
    pub fn is_hposc(&self) -> bool {
        *self == XOSC_FREQR::HPOSC
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_CAP_MODR {
    bits: bool,
}
impl XOSC_CAP_MODR {
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
pub struct HF_COMPR {
    bits: bool,
}
impl HF_COMPR {
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
pub struct XOSC_CAPARRAY_DELTAR {
    bits: u8,
}
impl XOSC_CAPARRAY_DELTAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VDDR_CAPR {
    bits: u8,
}
impl VDDR_CAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _VDDR_TRIM_SLEEP_DELTAW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDR_TRIM_SLEEP_DELTAW<'a> {
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
pub struct _DCDC_RECHARGEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_RECHARGEW<'a> {
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
pub struct _DCDC_ACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDC_ACTIVEW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VDDR_EXT_LOADW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDR_EXT_LOADW<'a> {
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
pub struct _VDDS_BOD_LEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDS_BOD_LEVELW<'a> {
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
#[doc = "Values that can be written to the field `SCLK_LF_OPTION`"]
pub enum SCLK_LF_OPTIONW {
    #[doc = "Low frequency RCOSC (default)"]
    RCOSC_LF,
    #[doc = "32.768kHz low frequency XOSC"]
    XOSC_LF,
    #[doc = "External low frequency clock on DIO defined by EXT_LF_CLK.DIO. The RTC tick speed AON_RTC:SUBSECINC is updated to EXT_LF_CLK.RTC_INCREMENT (done in the trimDevice() xxWare boot function). External clock must always be running when the chip is in standby for VDDR recharge timing."]
    EXTERNAL_LF,
    #[doc = "31.25kHz clock derived from 24MHz XOSC (dividing by 768 in HW). The RTC tick speed \\[AON_RTC.SUBSECINC.*\\] is updated to 0x8637BD, corresponding to a 31.25kHz clock (done in the trimDevice() xxWare boot function). Standby power mode is not supported when using this clock source."]
    XOSC_HF_DLF,
}
impl SCLK_LF_OPTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCLK_LF_OPTIONW::RCOSC_LF => 3,
            SCLK_LF_OPTIONW::XOSC_LF => 2,
            SCLK_LF_OPTIONW::EXTERNAL_LF => 1,
            SCLK_LF_OPTIONW::XOSC_HF_DLF => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLK_LF_OPTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_LF_OPTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLK_LF_OPTIONW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low frequency RCOSC (default)"]
    #[inline]
    pub fn rcosc_lf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTIONW::RCOSC_LF)
    }
    #[doc = "32.768kHz low frequency XOSC"]
    #[inline]
    pub fn xosc_lf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTIONW::XOSC_LF)
    }
    #[doc = "External low frequency clock on DIO defined by EXT_LF_CLK.DIO. The RTC tick speed AON_RTC:SUBSECINC is updated to EXT_LF_CLK.RTC_INCREMENT (done in the trimDevice() xxWare boot function). External clock must always be running when the chip is in standby for VDDR recharge timing."]
    #[inline]
    pub fn external_lf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTIONW::EXTERNAL_LF)
    }
    #[doc = "31.25kHz clock derived from 24MHz XOSC (dividing by 768 in HW). The RTC tick speed \\[AON_RTC.SUBSECINC.*\\] is updated to 0x8637BD, corresponding to a 31.25kHz clock (done in the trimDevice() xxWare boot function). Standby power mode is not supported when using this clock source."]
    #[inline]
    pub fn xosc_hf_dlf(self) -> &'a mut W {
        self.variant(SCLK_LF_OPTIONW::XOSC_HF_DLF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VDDR_TRIM_SLEEP_TCW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDR_TRIM_SLEEP_TCW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTC_COMPW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_COMPW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `XOSC_FREQ`"]
pub enum XOSC_FREQW {
    #[doc = "24 MHz XOSC_HF"]
    _24M,
    #[doc = "48 MHz XOSC_HF"]
    _48M,
    #[doc = "HPOSC"]
    HPOSC,
}
impl XOSC_FREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            XOSC_FREQW::_24M => 3,
            XOSC_FREQW::_48M => 2,
            XOSC_FREQW::HPOSC => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_FREQW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_FREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XOSC_FREQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "24 MHz XOSC_HF"]
    #[inline]
    pub fn _24m(self) -> &'a mut W {
        self.variant(XOSC_FREQW::_24M)
    }
    #[doc = "48 MHz XOSC_HF"]
    #[inline]
    pub fn _48m(self) -> &'a mut W {
        self.variant(XOSC_FREQW::_48M)
    }
    #[doc = "HPOSC"]
    #[inline]
    pub fn hposc(self) -> &'a mut W {
        self.variant(XOSC_FREQW::HPOSC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_CAP_MODW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_CAP_MODW<'a> {
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
pub struct _HF_COMPW<'a> {
    w: &'a mut W,
}
impl<'a> _HF_COMPW<'a> {
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
pub struct _XOSC_CAPARRAY_DELTAW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_CAPARRAY_DELTAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VDDR_CAPW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDR_CAPW<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\] Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8"]
    #[inline]
    pub fn vddr_trim_sleep_delta(&self) -> VDDR_TRIM_SLEEP_DELTAR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VDDR_TRIM_SLEEP_DELTAR { bits }
    }
    #[doc = "Bit 27 - 27:27\\] DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline]
    pub fn dcdc_recharge(&self) -> DCDC_RECHARGER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_RECHARGER { bits }
    }
    #[doc = "Bit 26 - 26:26\\] DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline]
    pub fn dcdc_active(&self) -> DCDC_ACTIVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCDC_ACTIVER { bits }
    }
    #[doc = "Bit 25 - 25:25\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline]
    pub fn vddr_ext_load(&self) -> VDDR_EXT_LOADR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDDR_EXT_LOADR { bits }
    }
    #[doc = "Bit 24 - 24:24\\] VDDS BOD level. 0: VDDS BOD level is 2.0 V (necessary for maximum PA output power on CC13x0). 1: VDDS BOD level is 1.8 V (or 1.7 V for external regulator mode) (default)."]
    #[inline]
    pub fn vdds_bod_level(&self) -> VDDS_BOD_LEVELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDDS_BOD_LEVELR { bits }
    }
    #[doc = "Bits 22:23 - 23:22\\] Select source for SCLK_LF."]
    #[inline]
    pub fn sclk_lf_option(&self) -> SCLK_LF_OPTIONR {
        SCLK_LF_OPTIONR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - 21:21\\] 0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
    #[inline]
    pub fn vddr_trim_sleep_tc(&self) -> VDDR_TRIM_SLEEP_TCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDDR_TRIM_SLEEP_TCR { bits }
    }
    #[doc = "Bit 20 - 20:20\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline]
    pub fn rtc_comp(&self) -> RTC_COMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTC_COMPR { bits }
    }
    #[doc = "Bits 18:19 - 19:18\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline]
    pub fn xosc_freq(&self) -> XOSC_FREQR {
        XOSC_FREQR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 17 - 17:17\\] Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)"]
    #[inline]
    pub fn xosc_cap_mod(&self) -> XOSC_CAP_MODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_CAP_MODR { bits }
    }
    #[doc = "Bit 16 - 16:16\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline]
    pub fn hf_comp(&self) -> HF_COMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HF_COMPR { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
    #[inline]
    pub fn xosc_caparray_delta(&self) -> XOSC_CAPARRAY_DELTAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSC_CAPARRAY_DELTAR { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()"]
    #[inline]
    pub fn vddr_cap(&self) -> VDDR_CAPR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VDDR_CAPR { bits }
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
    #[doc = "Bits 28:31 - 31:28\\] Signed delta value to apply to the VDDR_TRIM_SLEEP target, minus one. See FCFG1:VOLT_TRIM.VDDR_TRIM_SLEEP_H. 0x8 (-8) : Delta = -7 ... 0xF (-1) : Delta = 0 0x0 (0) : Delta = +1 ... 0x7 (7) : Delta = +8"]
    #[inline]
    pub fn vddr_trim_sleep_delta(&mut self) -> _VDDR_TRIM_SLEEP_DELTAW {
        _VDDR_TRIM_SLEEP_DELTAW { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] DC/DC during recharge in powerdown. 0: Use the DC/DC during recharge in powerdown. 1: Do not use the DC/DC during recharge in powerdown (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline]
    pub fn dcdc_recharge(&mut self) -> _DCDC_RECHARGEW {
        _DCDC_RECHARGEW { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] DC/DC in active mode. 0: Use the DC/DC during active mode. 1: Do not use the DC/DC during active mode (default). NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline]
    pub fn dcdc_active(&mut self) -> _DCDC_ACTIVEW {
        _DCDC_ACTIVEW { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline]
    pub fn vddr_ext_load(&mut self) -> _VDDR_EXT_LOADW {
        _VDDR_EXT_LOADW { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] VDDS BOD level. 0: VDDS BOD level is 2.0 V (necessary for maximum PA output power on CC13x0). 1: VDDS BOD level is 1.8 V (or 1.7 V for external regulator mode) (default)."]
    #[inline]
    pub fn vdds_bod_level(&mut self) -> _VDDS_BOD_LEVELW {
        _VDDS_BOD_LEVELW { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\] Select source for SCLK_LF."]
    #[inline]
    pub fn sclk_lf_option(&mut self) -> _SCLK_LF_OPTIONW {
        _SCLK_LF_OPTIONW { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] 0x1: VDDR_TRIM_SLEEP_DELTA is not temperature compensated 0x0: RTOS/driver temperature compensates VDDR_TRIM_SLEEP_DELTA every time standby mode is entered. This improves low-temperature RCOSC_LF frequency stability in standby mode. When temperature compensation is performed, the delta is calculates this way: Delta = max (delta, min(8, floor(62-temp)/8)) Here, delta is given by VDDR_TRIM_SLEEP_DELTA, and temp is the current temperature in degrees C."]
    #[inline]
    pub fn vddr_trim_sleep_tc(&mut self) -> _VDDR_TRIM_SLEEP_TCW {
        _VDDR_TRIM_SLEEP_TCW { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline]
    pub fn rtc_comp(&mut self) -> _RTC_COMPW {
        _RTC_COMPW { w: self }
    }
    #[doc = "Bits 18:19 - 19:18\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline]
    pub fn xosc_freq(&mut self) -> _XOSC_FREQW {
        _XOSC_FREQW { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Enable modification (delta) to XOSC cap-array. Value specified in XOSC_CAPARRAY_DELTA. 0: Apply cap-array delta 1: Do not apply cap-array delta (default)"]
    #[inline]
    pub fn xosc_cap_mod(&mut self) -> _XOSC_CAP_MODW {
        _XOSC_CAP_MODW { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline]
    pub fn hf_comp(&mut self) -> _HF_COMPW {
        _HF_COMPW { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Signed 8-bit value, directly modifying trimmed XOSC cap-array step value. Enabled by XOSC_CAP_MOD."]
    #[inline]
    pub fn xosc_caparray_delta(&mut self) -> _XOSC_CAPARRAY_DELTAW {
        _XOSC_CAPARRAY_DELTAW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Unsigned 8-bit integer, representing the minimum decoupling capacitance (worst case) on VDDR, in units of 100nF. This should take into account capacitor tolerance and voltage dependent capacitance variation. This bit affects the recharge period calculation when going into powerdown or standby. NOTE! If using the following functions this field must be configured (used by TI RTOS): SysCtrlSetRechargeBeforePowerDown() SysCtrlAdjustRechargeAfterPowerDown()"]
    #[inline]
    pub fn vddr_cap(&mut self) -> _VDDR_CAPW {
        _VDDR_CAPW { w: self }
    }
}
