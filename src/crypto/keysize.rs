#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::KEYSIZE {
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
#[doc = "Possible values of the field `SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZER {
    #[doc = "Not supported"]
    _256_BIT,
    #[doc = "Not supported"]
    _192_BIT,
    #[doc = "128 bits"]
    _128_BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SIZER::_256_BIT => 3,
            SIZER::_192_BIT => 2,
            SIZER::_128_BIT => 1,
            SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SIZER {
        match value {
            3 => SIZER::_256_BIT,
            2 => SIZER::_192_BIT,
            1 => SIZER::_128_BIT,
            i => SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_256_BIT`"]
    #[inline]
    pub fn is_256_bit(&self) -> bool {
        *self == SIZER::_256_BIT
    }
    #[doc = "Checks if the value of the field is `_192_BIT`"]
    #[inline]
    pub fn is_192_bit(&self) -> bool {
        *self == SIZER::_192_BIT
    }
    #[doc = "Checks if the value of the field is `_128_BIT`"]
    #[inline]
    pub fn is_128_bit(&self) -> bool {
        *self == SIZER::_128_BIT
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
#[doc = "Values that can be written to the field `SIZE`"]
pub enum SIZEW {
    #[doc = "Not supported"]
    _256_BIT,
    #[doc = "Not supported"]
    _192_BIT,
    #[doc = "128 bits"]
    _128_BIT,
}
impl SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SIZEW::_256_BIT => 3,
            SIZEW::_192_BIT => 2,
            SIZEW::_128_BIT => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Not supported"]
    #[inline]
    pub fn _256_bit(self) -> &'a mut W {
        self.variant(SIZEW::_256_BIT)
    }
    #[doc = "Not supported"]
    #[inline]
    pub fn _192_bit(self) -> &'a mut W {
        self.variant(SIZEW::_192_BIT)
    }
    #[doc = "128 bits"]
    #[inline]
    pub fn _128_bit(self) -> &'a mut W {
        self.variant(SIZEW::_128_BIT)
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
    #[doc = "Bits 0:1 - 1:0\\] Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register."]
    #[inline]
    pub fn size(&self) -> SIZER {
        SIZER::_from({
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
        W { bits: 1 }
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
    #[doc = "Bits 0:1 - 1:0\\] Key size When writing to this register, KEYWRITTENAREA will be reset. Note: For the Crypto peripheral this field is fixed to 128 bits. For software compatibility KEYWRITTENAREA will be reset when writing to this register."]
    #[inline]
    pub fn size(&mut self) -> _SIZEW {
        _SIZEW { w: self }
    }
}
