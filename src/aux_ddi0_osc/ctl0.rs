#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL0 {
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
#[doc = "Possible values of the field `XTAL_IS_24M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTAL_IS_24MR {
    #[doc = "Internal. Only to be used through TI provided API."]
    _24M,
    #[doc = "Internal. Only to be used through TI provided API."]
    _48M,
}
impl XTAL_IS_24MR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            XTAL_IS_24MR::_24M => true,
            XTAL_IS_24MR::_48M => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTAL_IS_24MR {
        match value {
            true => XTAL_IS_24MR::_24M,
            false => XTAL_IS_24MR::_48M,
        }
    }
    #[doc = "Checks if the value of the field is `_24M`"]
    #[inline]
    pub fn is_24m(&self) -> bool {
        *self == XTAL_IS_24MR::_24M
    }
    #[doc = "Checks if the value of the field is `_48M`"]
    #[inline]
    pub fn is_48m(&self) -> bool {
        *self == XTAL_IS_24MR::_48M
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED30R {
    bits: bool,
}
impl RESERVED30R {
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
pub struct BYPASS_XOSC_LF_CLK_QUALR {
    bits: bool,
}
impl BYPASS_XOSC_LF_CLK_QUALR {
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
pub struct BYPASS_RCOSC_LF_CLK_QUALR {
    bits: bool,
}
impl BYPASS_RCOSC_LF_CLK_QUALR {
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
pub struct DOUBLER_START_DURATIONR {
    bits: u8,
}
impl DOUBLER_START_DURATIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DOUBLER_RESET_DURATIONR {
    bits: bool,
}
impl DOUBLER_RESET_DURATIONR {
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
pub struct RESERVED23R {
    bits: u8,
}
impl RESERVED23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FORCE_KICKSTART_ENR {
    bits: bool,
}
impl FORCE_KICKSTART_ENR {
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
pub struct RESERVED17R {
    bits: u8,
}
impl RESERVED17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ALLOW_SCLK_HF_SWITCHINGR {
    bits: bool,
}
impl ALLOW_SCLK_HF_SWITCHINGR {
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
pub struct RESERVED15R {
    bits: bool,
}
impl RESERVED15R {
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
pub struct HPOSC_MODE_ENR {
    bits: bool,
}
impl HPOSC_MODE_ENR {
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
pub struct RESERVED13R {
    bits: bool,
}
impl RESERVED13R {
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
pub struct RCOSC_LF_TRIMMEDR {
    bits: bool,
}
impl RCOSC_LF_TRIMMEDR {
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
pub struct XOSC_HF_POWER_MODER {
    bits: bool,
}
impl XOSC_HF_POWER_MODER {
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
pub struct XOSC_LF_DIG_BYPASSR {
    bits: bool,
}
impl XOSC_LF_DIG_BYPASSR {
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
pub struct CLK_LOSS_ENR {
    bits: bool,
}
impl CLK_LOSS_ENR {
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
pub struct ACLK_TDC_SRC_SELR {
    bits: u8,
}
impl ACLK_TDC_SRC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ACLK_REF_SRC_SELR {
    bits: u8,
}
impl ACLK_REF_SRC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPARE4R {
    bits: bool,
}
impl SPARE4R {
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
#[doc = "Possible values of the field `SCLK_LF_SRC_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_LF_SRC_SELR {
    #[doc = "Low frequency XOSC"]
    XOSCLF,
    #[doc = "Low frequency RCOSC"]
    RCOSCLF,
    #[doc = "Low frequency clock derived from High Frequency XOSC"]
    XOSCHFDLF,
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    RCOSCHFDLF,
}
impl SCLK_LF_SRC_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCLK_LF_SRC_SELR::XOSCLF => 3,
            SCLK_LF_SRC_SELR::RCOSCLF => 2,
            SCLK_LF_SRC_SELR::XOSCHFDLF => 1,
            SCLK_LF_SRC_SELR::RCOSCHFDLF => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCLK_LF_SRC_SELR {
        match value {
            3 => SCLK_LF_SRC_SELR::XOSCLF,
            2 => SCLK_LF_SRC_SELR::RCOSCLF,
            1 => SCLK_LF_SRC_SELR::XOSCHFDLF,
            0 => SCLK_LF_SRC_SELR::RCOSCHFDLF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XOSCLF`"]
    #[inline]
    pub fn is_xosclf(&self) -> bool {
        *self == SCLK_LF_SRC_SELR::XOSCLF
    }
    #[doc = "Checks if the value of the field is `RCOSCLF`"]
    #[inline]
    pub fn is_rcosclf(&self) -> bool {
        *self == SCLK_LF_SRC_SELR::RCOSCLF
    }
    #[doc = "Checks if the value of the field is `XOSCHFDLF`"]
    #[inline]
    pub fn is_xoschfdlf(&self) -> bool {
        *self == SCLK_LF_SRC_SELR::XOSCHFDLF
    }
    #[doc = "Checks if the value of the field is `RCOSCHFDLF`"]
    #[inline]
    pub fn is_rcoschfdlf(&self) -> bool {
        *self == SCLK_LF_SRC_SELR::RCOSCHFDLF
    }
}
#[doc = "Possible values of the field `SCLK_MF_SRC_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_MF_SRC_SELR {
    #[doc = "Medium frequency clock derived from high frequency XOSC."]
    XCOSCHFDMF,
    #[doc = "Internal. Only to be used through TI provided API."]
    RCOSCHFDMF,
}
impl SCLK_MF_SRC_SELR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SCLK_MF_SRC_SELR::XCOSCHFDMF => true,
            SCLK_MF_SRC_SELR::RCOSCHFDMF => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCLK_MF_SRC_SELR {
        match value {
            true => SCLK_MF_SRC_SELR::XCOSCHFDMF,
            false => SCLK_MF_SRC_SELR::RCOSCHFDMF,
        }
    }
    #[doc = "Checks if the value of the field is `XCOSCHFDMF`"]
    #[inline]
    pub fn is_xcoschfdmf(&self) -> bool {
        *self == SCLK_MF_SRC_SELR::XCOSCHFDMF
    }
    #[doc = "Checks if the value of the field is `RCOSCHFDMF`"]
    #[inline]
    pub fn is_rcoschfdmf(&self) -> bool {
        *self == SCLK_MF_SRC_SELR::RCOSCHFDMF
    }
}
#[doc = "Possible values of the field `SCLK_HF_SRC_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLK_HF_SRC_SELR {
    #[doc = "High frequency XOSC clk"]
    XOSC,
    #[doc = "High frequency RCOSC clock"]
    RCOSC,
}
impl SCLK_HF_SRC_SELR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SCLK_HF_SRC_SELR::XOSC => true,
            SCLK_HF_SRC_SELR::RCOSC => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCLK_HF_SRC_SELR {
        match value {
            true => SCLK_HF_SRC_SELR::XOSC,
            false => SCLK_HF_SRC_SELR::RCOSC,
        }
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline]
    pub fn is_xosc(&self) -> bool {
        *self == SCLK_HF_SRC_SELR::XOSC
    }
    #[doc = "Checks if the value of the field is `RCOSC`"]
    #[inline]
    pub fn is_rcosc(&self) -> bool {
        *self == SCLK_HF_SRC_SELR::RCOSC
    }
}
#[doc = "Values that can be written to the field `XTAL_IS_24M`"]
pub enum XTAL_IS_24MW {
    #[doc = "Internal. Only to be used through TI provided API."]
    _24M,
    #[doc = "Internal. Only to be used through TI provided API."]
    _48M,
}
impl XTAL_IS_24MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTAL_IS_24MW::_24M => true,
            XTAL_IS_24MW::_48M => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTAL_IS_24MW<'a> {
    w: &'a mut W,
}
impl<'a> _XTAL_IS_24MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTAL_IS_24MW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _24m(self) -> &'a mut W {
        self.variant(XTAL_IS_24MW::_24M)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn _48m(self) -> &'a mut W {
        self.variant(XTAL_IS_24MW::_48M)
    }
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED30W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED30W<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BYPASS_XOSC_LF_CLK_QUALW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASS_XOSC_LF_CLK_QUALW<'a> {
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
pub struct _BYPASS_RCOSC_LF_CLK_QUALW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASS_RCOSC_LF_CLK_QUALW<'a> {
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
pub struct _DOUBLER_START_DURATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _DOUBLER_START_DURATIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DOUBLER_RESET_DURATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _DOUBLER_RESET_DURATIONW<'a> {
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
pub struct _RESERVED23W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED23W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FORCE_KICKSTART_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCE_KICKSTART_ENW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED17W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED17W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALLOW_SCLK_HF_SWITCHINGW<'a> {
    w: &'a mut W,
}
impl<'a> _ALLOW_SCLK_HF_SWITCHINGW<'a> {
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
pub struct _RESERVED15W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED15W<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPOSC_MODE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HPOSC_MODE_ENW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED13W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED13W<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RCOSC_LF_TRIMMEDW<'a> {
    w: &'a mut W,
}
impl<'a> _RCOSC_LF_TRIMMEDW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_HF_POWER_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_HF_POWER_MODEW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_LF_DIG_BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_LF_DIG_BYPASSW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLK_LOSS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_LOSS_ENW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACLK_TDC_SRC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _ACLK_TDC_SRC_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACLK_REF_SRC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _ACLK_REF_SRC_SELW<'a> {
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
pub struct _SPARE4W<'a> {
    w: &'a mut W,
}
impl<'a> _SPARE4W<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCLK_LF_SRC_SEL`"]
pub enum SCLK_LF_SRC_SELW {
    #[doc = "Low frequency XOSC"]
    XOSCLF,
    #[doc = "Low frequency RCOSC"]
    RCOSCLF,
    #[doc = "Low frequency clock derived from High Frequency XOSC"]
    XOSCHFDLF,
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    RCOSCHFDLF,
}
impl SCLK_LF_SRC_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCLK_LF_SRC_SELW::XOSCLF => 3,
            SCLK_LF_SRC_SELW::RCOSCLF => 2,
            SCLK_LF_SRC_SELW::XOSCHFDLF => 1,
            SCLK_LF_SRC_SELW::RCOSCHFDLF => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLK_LF_SRC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_LF_SRC_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLK_LF_SRC_SELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low frequency XOSC"]
    #[inline]
    pub fn xosclf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SELW::XOSCLF)
    }
    #[doc = "Low frequency RCOSC"]
    #[inline]
    pub fn rcosclf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SELW::RCOSCLF)
    }
    #[doc = "Low frequency clock derived from High Frequency XOSC"]
    #[inline]
    pub fn xoschfdlf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SELW::XOSCHFDLF)
    }
    #[doc = "Low frequency clock derived from High Frequency RCOSC"]
    #[inline]
    pub fn rcoschfdlf(self) -> &'a mut W {
        self.variant(SCLK_LF_SRC_SELW::RCOSCHFDLF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCLK_MF_SRC_SEL`"]
pub enum SCLK_MF_SRC_SELW {
    #[doc = "Medium frequency clock derived from high frequency XOSC."]
    XCOSCHFDMF,
    #[doc = "Internal. Only to be used through TI provided API."]
    RCOSCHFDMF,
}
impl SCLK_MF_SRC_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCLK_MF_SRC_SELW::XCOSCHFDMF => true,
            SCLK_MF_SRC_SELW::RCOSCHFDMF => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLK_MF_SRC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_MF_SRC_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLK_MF_SRC_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Medium frequency clock derived from high frequency XOSC."]
    #[inline]
    pub fn xcoschfdmf(self) -> &'a mut W {
        self.variant(SCLK_MF_SRC_SELW::XCOSCHFDMF)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcoschfdmf(self) -> &'a mut W {
        self.variant(SCLK_MF_SRC_SELW::RCOSCHFDMF)
    }
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
#[doc = "Values that can be written to the field `SCLK_HF_SRC_SEL`"]
pub enum SCLK_HF_SRC_SELW {
    #[doc = "High frequency XOSC clk"]
    XOSC,
    #[doc = "High frequency RCOSC clock"]
    RCOSC,
}
impl SCLK_HF_SRC_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCLK_HF_SRC_SELW::XOSC => true,
            SCLK_HF_SRC_SELW::RCOSC => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLK_HF_SRC_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_HF_SRC_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLK_HF_SRC_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "High frequency XOSC clk"]
    #[inline]
    pub fn xosc(self) -> &'a mut W {
        self.variant(SCLK_HF_SRC_SELW::XOSC)
    }
    #[doc = "High frequency RCOSC clock"]
    #[inline]
    pub fn rcosc(self) -> &'a mut W {
        self.variant(SCLK_HF_SRC_SELW::RCOSC)
    }
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
    #[doc = "Bit 31 - 31:31\\] Set based on the accurate high frequency XTAL."]
    #[inline]
    pub fn xtal_is_24m(&self) -> XTAL_IS_24MR {
        XTAL_IS_24MR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - 30:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved30(&self) -> RESERVED30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED30R { bits }
    }
    #[doc = "Bit 29 - 29:29\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn bypass_xosc_lf_clk_qual(&self) -> BYPASS_XOSC_LF_CLK_QUALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYPASS_XOSC_LF_CLK_QUALR { bits }
    }
    #[doc = "Bit 28 - 28:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn bypass_rcosc_lf_clk_qual(&self) -> BYPASS_RCOSC_LF_CLK_QUALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYPASS_RCOSC_LF_CLK_QUALR { bits }
    }
    #[doc = "Bits 26:27 - 27:26\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn doubler_start_duration(&self) -> DOUBLER_START_DURATIONR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DOUBLER_START_DURATIONR { bits }
    }
    #[doc = "Bit 25 - 25:25\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn doubler_reset_duration(&self) -> DOUBLER_RESET_DURATIONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DOUBLER_RESET_DURATIONR { bits }
    }
    #[doc = "Bits 23:24 - 24:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved23(&self) -> RESERVED23R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED23R { bits }
    }
    #[doc = "Bit 22 - 22:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn force_kickstart_en(&self) -> FORCE_KICKSTART_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FORCE_KICKSTART_ENR { bits }
    }
    #[doc = "Bits 17:21 - 21:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved17(&self) -> RESERVED17R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED17R { bits }
    }
    #[doc = "Bit 16 - 16:16\\] 0: Default - Switching of HF clock source is disabled . 1: Allows switching of sclk_hf source. Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set. This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING) sclk_hf switching should be disabled to prevent flash corruption. Switching should not be enabled when running from flash."]
    #[inline]
    pub fn allow_sclk_hf_switching(&self) -> ALLOW_SCLK_HF_SWITCHINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALLOW_SCLK_HF_SWITCHINGR { bits }
    }
    #[doc = "Bit 15 - 15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved15(&self) -> RESERVED15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED15R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_mode_en(&self) -> HPOSC_MODE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HPOSC_MODE_ENR { bits }
    }
    #[doc = "Bit 13 - 13:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved13(&self) -> RESERVED13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED13R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosc_lf_trimmed(&self) -> RCOSC_LF_TRIMMEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RCOSC_LF_TRIMMEDR { bits }
    }
    #[doc = "Bit 11 - 11:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_power_mode(&self) -> XOSC_HF_POWER_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_HF_POWER_MODER { bits }
    }
    #[doc = "Bit 10 - 10:10\\] Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
    #[inline]
    pub fn xosc_lf_dig_bypass(&self) -> XOSC_LF_DIG_BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSC_LF_DIG_BYPASSR { bits }
    }
    #[doc = "Bit 9 - 9:9\\] Enable clock loss detection and hence the indicators to system controller. Checks both SCLK_HF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline]
    pub fn clk_loss_en(&self) -> CLK_LOSS_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK_LOSS_ENR { bits }
    }
    #[doc = "Bits 7:8 - 8:7\\] Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
    #[inline]
    pub fn aclk_tdc_src_sel(&self) -> ACLK_TDC_SRC_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ACLK_TDC_SRC_SELR { bits }
    }
    #[doc = "Bits 5:6 - 6:5\\] Source select for aclk_ref 00: RCOSC_HF derived (31.25kHz) 01: XOSC_HF derived (31.25kHz) 10: RCOSC_LF (32kHz) 11: XOSC_LF (32.768kHz)"]
    #[inline]
    pub fn aclk_ref_src_sel(&self) -> ACLK_REF_SRC_SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ACLK_REF_SRC_SELR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare4(&self) -> SPARE4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPARE4R { bits }
    }
    #[doc = "Bits 2:3 - 3:2\\] Source select for sclk_lf"]
    #[inline]
    pub fn sclk_lf_src_sel(&self) -> SCLK_LF_SRC_SELR {
        SCLK_LF_SRC_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 1 - 1:1\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sclk_mf_src_sel(&self) -> SCLK_MF_SRC_SELR {
        SCLK_MF_SRC_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running."]
    #[inline]
    pub fn sclk_hf_src_sel(&self) -> SCLK_HF_SRC_SELR {
        SCLK_HF_SRC_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 31 - 31:31\\] Set based on the accurate high frequency XTAL."]
    #[inline]
    pub fn xtal_is_24m(&mut self) -> _XTAL_IS_24MW {
        _XTAL_IS_24MW { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved30(&mut self) -> _RESERVED30W {
        _RESERVED30W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn bypass_xosc_lf_clk_qual(&mut self) -> _BYPASS_XOSC_LF_CLK_QUALW {
        _BYPASS_XOSC_LF_CLK_QUALW { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn bypass_rcosc_lf_clk_qual(&mut self) -> _BYPASS_RCOSC_LF_CLK_QUALW {
        _BYPASS_RCOSC_LF_CLK_QUALW { w: self }
    }
    #[doc = "Bits 26:27 - 27:26\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn doubler_start_duration(&mut self) -> _DOUBLER_START_DURATIONW {
        _DOUBLER_START_DURATIONW { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn doubler_reset_duration(&mut self) -> _DOUBLER_RESET_DURATIONW {
        _DOUBLER_RESET_DURATIONW { w: self }
    }
    #[doc = "Bits 23:24 - 24:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved23(&mut self) -> _RESERVED23W {
        _RESERVED23W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn force_kickstart_en(&mut self) -> _FORCE_KICKSTART_ENW {
        _FORCE_KICKSTART_ENW { w: self }
    }
    #[doc = "Bits 17:21 - 21:17\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved17(&mut self) -> _RESERVED17W {
        _RESERVED17W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] 0: Default - Switching of HF clock source is disabled . 1: Allows switching of sclk_hf source. Provided to prevent switching of the SCLK_HF source when running from flash (a long period during switching could corrupt flash). When sclk_hf switching is disabled, a new source can be started when SCLK_HF_SRC_SEL is changed, but the switch will not occur until this bit is set. This bit should be set to enable clock switching after STAT0.PENDINGSCLKHFSWITCHING indicates the new HF clock is ready. When switching completes (also indicated by STAT0.PENDINGSCLKHFSWITCHING) sclk_hf switching should be disabled to prevent flash corruption. Switching should not be enabled when running from flash."]
    #[inline]
    pub fn allow_sclk_hf_switching(&mut self) -> _ALLOW_SCLK_HF_SWITCHINGW {
        _ALLOW_SCLK_HF_SWITCHINGW { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved15(&mut self) -> _RESERVED15W {
        _RESERVED15W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn hposc_mode_en(&mut self) -> _HPOSC_MODE_ENW {
        _HPOSC_MODE_ENW { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved13(&mut self) -> _RESERVED13W {
        _RESERVED13W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn rcosc_lf_trimmed(&mut self) -> _RCOSC_LF_TRIMMEDW {
        _RCOSC_LF_TRIMMEDW { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_power_mode(&mut self) -> _XOSC_HF_POWER_MODEW {
        _XOSC_HF_POWER_MODEW { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Bypass XOSC_LF and use the digital input clock from AON for the xosc_lf clock. 0: Use 32kHz XOSC as xosc_lf clock source 1: Use digital input (from AON) as xosc_lf clock source. This bit will only have effect when SCLK_LF_SRC_SEL is selecting the xosc_lf as the sclk_lf source. The muxing performed by this bit is not glitch free. The following procedure must be followed when changing this field to avoid glitches on sclk_lf. 1) Set SCLK_LF_SRC_SEL to select any source other than the xosc_lf clock source. 2) Set or clear this bit to bypass or not bypass the xosc_lf. 3) Set SCLK_LF_SRC_SEL to use xosc_lf. It is recommended that either the rcosc_hf or xosc_hf (whichever is currently active) be selected as the source in step 1 above. This provides a faster clock change."]
    #[inline]
    pub fn xosc_lf_dig_bypass(&mut self) -> _XOSC_LF_DIG_BYPASSW {
        _XOSC_LF_DIG_BYPASSW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Enable clock loss detection and hence the indicators to system controller. Checks both SCLK_HF and SCLK_LF clock loss indicators. 0: Disable 1: Enable Clock loss detection must be disabled when changing the sclk_lf source. STAT0.SCLK_LF_SRC can be polled to determine when a change to a new sclk_lf source has completed."]
    #[inline]
    pub fn clk_loss_en(&mut self) -> _CLK_LOSS_ENW {
        _CLK_LOSS_ENW { w: self }
    }
    #[doc = "Bits 7:8 - 8:7\\] Source select for aclk_tdc. 00: RCOSC_HF (48MHz) 01: RCOSC_HF (24MHz) 10: XOSC_HF (24MHz) 11: Not used"]
    #[inline]
    pub fn aclk_tdc_src_sel(&mut self) -> _ACLK_TDC_SRC_SELW {
        _ACLK_TDC_SRC_SELW { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\] Source select for aclk_ref 00: RCOSC_HF derived (31.25kHz) 01: XOSC_HF derived (31.25kHz) 10: RCOSC_LF (32kHz) 11: XOSC_LF (32.768kHz)"]
    #[inline]
    pub fn aclk_ref_src_sel(&mut self) -> _ACLK_REF_SRC_SELW {
        _ACLK_REF_SRC_SELW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn spare4(&mut self) -> _SPARE4W {
        _SPARE4W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Source select for sclk_lf"]
    #[inline]
    pub fn sclk_lf_src_sel(&mut self) -> _SCLK_LF_SRC_SELW {
        _SCLK_LF_SRC_SELW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sclk_mf_src_sel(&mut self) -> _SCLK_MF_SRC_SELW {
        _SCLK_MF_SRC_SELW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Source select for sclk_hf. XOSC option is supported for test and debug only and should be used when the XOSC_HF is running."]
    #[inline]
    pub fn sclk_hf_src_sel(&mut self) -> _SCLK_HF_SRC_SELW {
        _SCLK_HF_SRC_SELW { w: self }
    }
}
