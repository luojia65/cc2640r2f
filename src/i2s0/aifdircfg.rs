#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AIFDIRCFG {
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
pub struct RESERVED6R {
    bits: u32,
}
impl RESERVED6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `AD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD1R {
    #[doc = "Output mode"]
    OUT,
    #[doc = "Input mode"]
    IN,
    #[doc = "Not in use (disabled)"]
    DIS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AD1R::OUT => 2,
            AD1R::IN => 1,
            AD1R::DIS => 0,
            AD1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AD1R {
        match value {
            2 => AD1R::OUT,
            1 => AD1R::IN,
            0 => AD1R::DIS,
            i => AD1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == AD1R::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == AD1R::IN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == AD1R::DIS
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
#[doc = "Possible values of the field `AD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD0R {
    #[doc = "Output mode"]
    OUT,
    #[doc = "Input mode"]
    IN,
    #[doc = "Not in use (disabled)"]
    DIS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AD0R::OUT => 2,
            AD0R::IN => 1,
            AD0R::DIS => 0,
            AD0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AD0R {
        match value {
            2 => AD0R::OUT,
            1 => AD0R::IN,
            0 => AD0R::DIS,
            i => AD0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == AD0R::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == AD0R::IN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == AD0R::DIS
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED6W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 67108863;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AD1`"]
pub enum AD1W {
    #[doc = "Output mode"]
    OUT,
    #[doc = "Input mode"]
    IN,
    #[doc = "Not in use (disabled)"]
    DIS,
}
impl AD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AD1W::OUT => 2,
            AD1W::IN => 1,
            AD1W::DIS => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AD1W<'a> {
    w: &'a mut W,
}
impl<'a> _AD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AD1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Output mode"]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(AD1W::OUT)
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(AD1W::IN)
    }
    #[doc = "Not in use (disabled)"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(AD1W::DIS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AD0`"]
pub enum AD0W {
    #[doc = "Output mode"]
    OUT,
    #[doc = "Input mode"]
    IN,
    #[doc = "Not in use (disabled)"]
    DIS,
}
impl AD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AD0W::OUT => 2,
            AD0W::IN => 1,
            AD0W::DIS => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AD0W<'a> {
    w: &'a mut W,
}
impl<'a> _AD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AD0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Output mode"]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(AD0W::OUT)
    }
    #[doc = "Input mode"]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(AD0W::IN)
    }
    #[doc = "Not in use (disabled)"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(AD0W::DIS)
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
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&self) -> RESERVED6R {
        let bits = {
            const MASK: u32 = 67108863;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED6R { bits }
    }
    #[doc = "Bits 4:5 - 5:4\\] Configures the AD1 audio data pin usage: 0x3: Reserved"]
    #[inline]
    pub fn ad1(&self) -> AD1R {
        AD1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - 3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] Configures the AD0 audio data pin usage: 0x3: Reserved"]
    #[inline]
    pub fn ad0(&self) -> AD0R {
        AD0R::_from({
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
    #[doc = "Bits 6:31 - 31:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Configures the AD1 audio data pin usage: 0x3: Reserved"]
    #[inline]
    pub fn ad1(&mut self) -> _AD1W {
        _AD1W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Configures the AD0 audio data pin usage: 0x3: Reserved"]
    #[inline]
    pub fn ad0(&mut self) -> _AD0W {
        _AD0W { w: self }
    }
}
