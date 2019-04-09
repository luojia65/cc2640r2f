#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AIFWCLKSRC {
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
pub struct RESERVED3R {
    bits: u32,
}
impl RESERVED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WCLK_INVR {
    bits: bool,
}
impl WCLK_INVR {
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
#[doc = "Possible values of the field `WCLK_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCLK_SRCR {
    #[doc = "Internal WCLK generator, from module PRCM"]
    INT,
    #[doc = "External WCLK generator, from pad"]
    EXT,
    #[doc = "None ('0')"]
    NONE,
}
impl WCLK_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WCLK_SRCR::INT => 2,
            WCLK_SRCR::EXT => 1,
            WCLK_SRCR::NONE => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WCLK_SRCR {
        match value {
            2 => WCLK_SRCR::INT,
            1 => WCLK_SRCR::EXT,
            0 => WCLK_SRCR::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline]
    pub fn is_int(&self) -> bool {
        *self == WCLK_SRCR::INT
    }
    #[doc = "Checks if the value of the field is `EXT`"]
    #[inline]
    pub fn is_ext(&self) -> bool {
        *self == WCLK_SRCR::EXT
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WCLK_SRCR::NONE
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WCLK_INVW<'a> {
    w: &'a mut W,
}
impl<'a> _WCLK_INVW<'a> {
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
#[doc = "Values that can be written to the field `WCLK_SRC`"]
pub enum WCLK_SRCW {
    #[doc = "Internal WCLK generator, from module PRCM"]
    INT,
    #[doc = "External WCLK generator, from pad"]
    EXT,
    #[doc = "None ('0')"]
    NONE,
}
impl WCLK_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WCLK_SRCW::INT => 2,
            WCLK_SRCW::EXT => 1,
            WCLK_SRCW::NONE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WCLK_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _WCLK_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WCLK_SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Internal WCLK generator, from module PRCM"]
    #[inline]
    pub fn int(self) -> &'a mut W {
        self.variant(WCLK_SRCW::INT)
    }
    #[doc = "External WCLK generator, from pad"]
    #[inline]
    pub fn ext(self) -> &'a mut W {
        self.variant(WCLK_SRCW::EXT)
    }
    #[doc = "None ('0')"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(WCLK_SRCW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
    #[inline]
    pub fn wclk_inv(&self) -> WCLK_INVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WCLK_INVR { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
    #[inline]
    pub fn wclk_src(&self) -> WCLK_SRCR {
        WCLK_SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Inverts WCLK source (pad or internal) when set. 0: Not inverted 1: Inverted"]
    #[inline]
    pub fn wclk_inv(&mut self) -> _WCLK_INVW {
        _WCLK_INVW { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Selects WCLK source for AIF (should be the same as the BCLK source). The BCLK source is defined in the PRCM:I2SBCLKSEL.SRC"]
    #[inline]
    pub fn wclk_src(&mut self) -> _WCLK_SRCW {
        _WCLK_SRCW { w: self }
    }
}
