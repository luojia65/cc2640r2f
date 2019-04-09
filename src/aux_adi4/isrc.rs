#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::ISRC {
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
#[doc = "Possible values of the field `TRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIMR {
    #[doc = "11.75 uA"]
    _11P75U,
    #[doc = "4.5 uA"]
    _4P5U,
    #[doc = "2.0 uA"]
    _2P0U,
    #[doc = "1.0 uA"]
    _1P0U,
    #[doc = "0.5 uA"]
    _0P5U,
    #[doc = "0.25 uA"]
    _0P25U,
    #[doc = "No current connected"]
    NC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIMR::_11P75U => 32,
            TRIMR::_4P5U => 16,
            TRIMR::_2P0U => 8,
            TRIMR::_1P0U => 4,
            TRIMR::_0P5U => 2,
            TRIMR::_0P25U => 1,
            TRIMR::NC => 0,
            TRIMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIMR {
        match value {
            32 => TRIMR::_11P75U,
            16 => TRIMR::_4P5U,
            8 => TRIMR::_2P0U,
            4 => TRIMR::_1P0U,
            2 => TRIMR::_0P5U,
            1 => TRIMR::_0P25U,
            0 => TRIMR::NC,
            i => TRIMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_11P75U`"]
    #[inline]
    pub fn is_11p75u(&self) -> bool {
        *self == TRIMR::_11P75U
    }
    #[doc = "Checks if the value of the field is `_4P5U`"]
    #[inline]
    pub fn is_4p5u(&self) -> bool {
        *self == TRIMR::_4P5U
    }
    #[doc = "Checks if the value of the field is `_2P0U`"]
    #[inline]
    pub fn is_2p0u(&self) -> bool {
        *self == TRIMR::_2P0U
    }
    #[doc = "Checks if the value of the field is `_1P0U`"]
    #[inline]
    pub fn is_1p0u(&self) -> bool {
        *self == TRIMR::_1P0U
    }
    #[doc = "Checks if the value of the field is `_0P5U`"]
    #[inline]
    pub fn is_0p5u(&self) -> bool {
        *self == TRIMR::_0P5U
    }
    #[doc = "Checks if the value of the field is `_0P25U`"]
    #[inline]
    pub fn is_0p25u(&self) -> bool {
        *self == TRIMR::_0P25U
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline]
    pub fn is_nc(&self) -> bool {
        *self == TRIMR::NC
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
pub struct ENR {
    bits: bool,
}
impl ENR {
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
#[doc = "Values that can be written to the field `TRIM`"]
pub enum TRIMW {
    #[doc = "11.75 uA"]
    _11P75U,
    #[doc = "4.5 uA"]
    _4P5U,
    #[doc = "2.0 uA"]
    _2P0U,
    #[doc = "1.0 uA"]
    _1P0U,
    #[doc = "0.5 uA"]
    _0P5U,
    #[doc = "0.25 uA"]
    _0P25U,
    #[doc = "No current connected"]
    NC,
}
impl TRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIMW::_11P75U => 32,
            TRIMW::_4P5U => 16,
            TRIMW::_2P0U => 8,
            TRIMW::_1P0U => 4,
            TRIMW::_0P5U => 2,
            TRIMW::_0P25U => 1,
            TRIMW::NC => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "11.75 uA"]
    #[inline]
    pub fn _11p75u(self) -> &'a mut W {
        self.variant(TRIMW::_11P75U)
    }
    #[doc = "4.5 uA"]
    #[inline]
    pub fn _4p5u(self) -> &'a mut W {
        self.variant(TRIMW::_4P5U)
    }
    #[doc = "2.0 uA"]
    #[inline]
    pub fn _2p0u(self) -> &'a mut W {
        self.variant(TRIMW::_2P0U)
    }
    #[doc = "1.0 uA"]
    #[inline]
    pub fn _1p0u(self) -> &'a mut W {
        self.variant(TRIMW::_1P0U)
    }
    #[doc = "0.5 uA"]
    #[inline]
    pub fn _0p5u(self) -> &'a mut W {
        self.variant(TRIMW::_0P5U)
    }
    #[doc = "0.25 uA"]
    #[inline]
    pub fn _0p25u(self) -> &'a mut W {
        self.variant(TRIMW::_0P25U)
    }
    #[doc = "No current connected"]
    #[inline]
    pub fn nc(self) -> &'a mut W {
        self.variant(TRIMW::NC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
    #[doc = "Bits 2:7 - 7:2\\] Adjust current from current source. Output currents may be combined to get desired total current."]
    #[inline]
    pub fn trim(&self) -> TRIMR {
        TRIMR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
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
    #[doc = "Bit 0 - 0:0\\] Current source enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        ENR { bits }
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
    #[doc = "Bits 2:7 - 7:2\\] Adjust current from current source. Output currents may be combined to get desired total current."]
    #[inline]
    pub fn trim(&mut self) -> _TRIMW {
        _TRIMW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved1(&mut self) -> _RESERVED1W {
        _RESERVED1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Current source enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
}
