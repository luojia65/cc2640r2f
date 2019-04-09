#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::COMP {
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
pub struct COMPA_REF_RES_ENR {
    bits: bool,
}
impl COMPA_REF_RES_ENR {
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
pub struct COMPA_REF_CURR_ENR {
    bits: bool,
}
impl COMPA_REF_CURR_ENR {
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
#[doc = "Possible values of the field `COMPB_TRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPB_TRIMR {
    #[doc = "Divide reference by 4"]
    DIV4,
    #[doc = "Divide reference by 3"]
    DIV3,
    #[doc = "Divide reference by 2"]
    DIV2,
    #[doc = "No reference division"]
    DIV1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl COMPB_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COMPB_TRIMR::DIV4 => 7,
            COMPB_TRIMR::DIV3 => 3,
            COMPB_TRIMR::DIV2 => 1,
            COMPB_TRIMR::DIV1 => 0,
            COMPB_TRIMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COMPB_TRIMR {
        match value {
            7 => COMPB_TRIMR::DIV4,
            3 => COMPB_TRIMR::DIV3,
            1 => COMPB_TRIMR::DIV2,
            0 => COMPB_TRIMR::DIV1,
            i => COMPB_TRIMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == COMPB_TRIMR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline]
    pub fn is_div3(&self) -> bool {
        *self == COMPB_TRIMR::DIV3
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == COMPB_TRIMR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == COMPB_TRIMR::DIV1
    }
}
#[doc = r" Value of the field"]
pub struct COMPB_ENR {
    bits: bool,
}
impl COMPB_ENR {
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
pub struct RESERVED1R {
    bits: bool,
}
impl RESERVED1R {
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
pub struct COMPA_ENR {
    bits: bool,
}
impl COMPA_ENR {
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
pub struct _COMPA_REF_RES_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPA_REF_RES_ENW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPA_REF_CURR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPA_REF_CURR_ENW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMPB_TRIM`"]
pub enum COMPB_TRIMW {
    #[doc = "Divide reference by 4"]
    DIV4,
    #[doc = "Divide reference by 3"]
    DIV3,
    #[doc = "Divide reference by 2"]
    DIV2,
    #[doc = "No reference division"]
    DIV1,
}
impl COMPB_TRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COMPB_TRIMW::DIV4 => 7,
            COMPB_TRIMW::DIV3 => 3,
            COMPB_TRIMW::DIV2 => 1,
            COMPB_TRIMW::DIV1 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMPB_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPB_TRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMPB_TRIMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide reference by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(COMPB_TRIMW::DIV4)
    }
    #[doc = "Divide reference by 3"]
    #[inline]
    pub fn div3(self) -> &'a mut W {
        self.variant(COMPB_TRIMW::DIV3)
    }
    #[doc = "Divide reference by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(COMPB_TRIMW::DIV2)
    }
    #[doc = "No reference division"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(COMPB_TRIMW::DIV1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPB_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPB_ENW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED1W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED1W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMPA_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMPA_ENW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 7 - 7:7\\] Enables 400kohm resistance from COMPA reference node to ground. Used with COMPA_REF_CURR_EN to generate voltage reference for cap-sense."]
    #[inline]
    pub fn compa_ref_res_en(&self) -> COMPA_REF_RES_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        COMPA_REF_RES_ENR { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Enables 2uA IPTAT current from ISRC to COMPA reference node. Requires ISRC.EN = 1. Used with COMPA_REF_RES_EN to generate voltage reference for cap-sense."]
    #[inline]
    pub fn compa_ref_curr_en(&self) -> COMPA_REF_CURR_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        COMPA_REF_CURR_ENR { bits }
    }
    #[doc = "Bits 3:5 - 5:3\\] COMPB voltage reference trim temperature coded:"]
    #[inline]
    pub fn compb_trim(&self) -> COMPB_TRIMR {
        COMPB_TRIMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 2 - 2:2\\] COMPB enable"]
    #[inline]
    pub fn compb_en(&self) -> COMPB_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        COMPB_ENR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        RESERVED1R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] COMPA enable"]
    #[inline]
    pub fn compa_en(&self) -> COMPA_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        COMPA_ENR { bits }
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - 7:7\\] Enables 400kohm resistance from COMPA reference node to ground. Used with COMPA_REF_CURR_EN to generate voltage reference for cap-sense."]
    #[inline]
    pub fn compa_ref_res_en(&mut self) -> _COMPA_REF_RES_ENW {
        _COMPA_REF_RES_ENW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Enables 2uA IPTAT current from ISRC to COMPA reference node. Requires ISRC.EN = 1. Used with COMPA_REF_RES_EN to generate voltage reference for cap-sense."]
    #[inline]
    pub fn compa_ref_curr_en(&mut self) -> _COMPA_REF_CURR_ENW {
        _COMPA_REF_CURR_ENW { w: self }
    }
    #[doc = "Bits 3:5 - 5:3\\] COMPB voltage reference trim temperature coded:"]
    #[inline]
    pub fn compb_trim(&mut self) -> _COMPB_TRIMW {
        _COMPB_TRIMW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] COMPB enable"]
    #[inline]
    pub fn compb_en(&mut self) -> _COMPB_ENW {
        _COMPB_ENW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved1(&mut self) -> _RESERVED1W {
        _RESERVED1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] COMPA enable"]
    #[inline]
    pub fn compa_en(&mut self) -> _COMPA_ENW {
        _COMPA_ENW { w: self }
    }
}
