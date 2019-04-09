#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INFRCLKDIVDS {
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
pub struct RESERVED2R {
    bits: u32,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `RATIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATIOR {
    #[doc = "Divide by 32"]
    DIV32,
    #[doc = "Divide by 8"]
    DIV8,
    #[doc = "Divide by 2"]
    DIV2,
    #[doc = "Divide by 1"]
    DIV1,
}
impl RATIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RATIOR::DIV32 => 3,
            RATIOR::DIV8 => 2,
            RATIOR::DIV2 => 1,
            RATIOR::DIV1 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RATIOR {
        match value {
            3 => RATIOR::DIV32,
            2 => RATIOR::DIV8,
            1 => RATIOR::DIV2,
            0 => RATIOR::DIV1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline]
    pub fn is_div32(&self) -> bool {
        *self == RATIOR::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == RATIOR::DIV8
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
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 1073741823;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RATIO`"]
pub enum RATIOW {
    #[doc = "Divide by 32"]
    DIV32,
    #[doc = "Divide by 8"]
    DIV8,
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
            RATIOW::DIV32 => 3,
            RATIOW::DIV8 => 2,
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
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn div32(self) -> &'a mut W {
        self.variant(RATIOW::DIV32)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(RATIOW::DIV8)
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
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 2:31 - 31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] Division rate for clocks driving modules in the MCU_AON domain when system CPU is in seepsleep mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline]
    pub fn ratio(&self) -> RATIOR {
        RATIOR::_from({
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
    #[doc = "Bits 2:31 - 31:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Division rate for clocks driving modules in the MCU_AON domain when system CPU is in seepsleep mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline]
    pub fn ratio(&mut self) -> _RATIOW {
        _RATIOW { w: self }
    }
}
