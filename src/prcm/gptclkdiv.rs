#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPTCLKDIV {
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
pub struct RESERVED4R {
    bits: u32,
}
impl RESERVED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `RATIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATIOR {
    #[doc = "Divide by 256"]
    DIV256,
    #[doc = "Divide by 128"]
    DIV128,
    #[doc = "Divide by 64"]
    DIV64,
    #[doc = "Divide by 32"]
    DIV32,
    #[doc = "Divide by 16"]
    DIV16,
    #[doc = "Divide by 8"]
    DIV8,
    #[doc = "Divide by 4"]
    DIV4,
    #[doc = "Divide by 2"]
    DIV2,
    #[doc = "Divide by 1"]
    DIV1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RATIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RATIOR::DIV256 => 8,
            RATIOR::DIV128 => 7,
            RATIOR::DIV64 => 6,
            RATIOR::DIV32 => 5,
            RATIOR::DIV16 => 4,
            RATIOR::DIV8 => 3,
            RATIOR::DIV4 => 2,
            RATIOR::DIV2 => 1,
            RATIOR::DIV1 => 0,
            RATIOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RATIOR {
        match value {
            8 => RATIOR::DIV256,
            7 => RATIOR::DIV128,
            6 => RATIOR::DIV64,
            5 => RATIOR::DIV32,
            4 => RATIOR::DIV16,
            3 => RATIOR::DIV8,
            2 => RATIOR::DIV4,
            1 => RATIOR::DIV2,
            0 => RATIOR::DIV1,
            i => RATIOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == RATIOR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == RATIOR::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == RATIOR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == RATIOR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == RATIOR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == RATIOR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == RATIOR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == RATIOR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == RATIOR::DIV1
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED4W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RATIO`"]
pub enum RATIOW {
    #[doc = "Divide by 256"]
    DIV256,
    #[doc = "Divide by 128"]
    DIV128,
    #[doc = "Divide by 64"]
    DIV64,
    #[doc = "Divide by 32"]
    DIV32,
    #[doc = "Divide by 16"]
    DIV16,
    #[doc = "Divide by 8"]
    DIV8,
    #[doc = "Divide by 4"]
    DIV4,
    #[doc = "Divide by 2"]
    DIV2,
    #[doc = "Divide by 1"]
    DIV1,
}
impl RATIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RATIOW::DIV256 => 8,
            RATIOW::DIV128 => 7,
            RATIOW::DIV64 => 6,
            RATIOW::DIV32 => 5,
            RATIOW::DIV16 => 4,
            RATIOW::DIV8 => 3,
            RATIOW::DIV4 => 2,
            RATIOW::DIV2 => 1,
            RATIOW::DIV1 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RATIOW<'a> {
    w: &'a mut W,
}
impl<'a> _RATIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RATIOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(RATIOW::DIV256)
    }
    #[doc = "Divide by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(RATIOW::DIV128)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(RATIOW::DIV64)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(RATIOW::DIV32)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(RATIOW::DIV16)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(RATIOW::DIV8)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(RATIOW::DIV4)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(RATIOW::DIV2)
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(RATIOW::DIV1)
    }
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
    #[doc = "Bits 4:31 - 31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: u32 = 268435455;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED4R { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Scalar used for GPTs. The division rate will be constant and ungated for Run / Sleep / DeepSleep mode. For changes to take effect, CLKLOADCTL.LOAD needs to be written Other values are not supported."]
    #[inline]
    pub fn ratio(&self) -> RATIOR {
        RATIOR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 4:31 - 31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Scalar used for GPTs. The division rate will be constant and ungated for Run / Sleep / DeepSleep mode. For changes to take effect, CLKLOADCTL.LOAD needs to be written Other values are not supported."]
    #[inline]
    pub fn ratio(&mut self) -> _RATIOW {
        _RATIOW { w: self }
    }
}
