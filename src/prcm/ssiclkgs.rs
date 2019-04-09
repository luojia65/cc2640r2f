#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSICLKGS {
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
#[doc = "Possible values of the field `CLK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_ENR {
    #[doc = "Enable clock for SSI1"]
    SSI1,
    #[doc = "Enable clock for SSI0"]
    SSI0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLK_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_ENR::SSI1 => 2,
            CLK_ENR::SSI0 => 1,
            CLK_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_ENR {
        match value {
            2 => CLK_ENR::SSI1,
            1 => CLK_ENR::SSI0,
            i => CLK_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSI1`"]
    #[inline]
    pub fn is_ssi1(&self) -> bool {
        *self == CLK_ENR::SSI1
    }
    #[doc = "Checks if the value of the field is `SSI0`"]
    #[inline]
    pub fn is_ssi0(&self) -> bool {
        *self == CLK_ENR::SSI0
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
#[doc = "Values that can be written to the field `CLK_EN`"]
pub enum CLK_ENW {
    #[doc = "Enable clock for SSI1"]
    SSI1,
    #[doc = "Enable clock for SSI0"]
    SSI0,
}
impl CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_ENW::SSI1 => 2,
            CLK_ENW::SSI0 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Enable clock for SSI1"]
    #[inline]
    pub fn ssi1(self) -> &'a mut W {
        self.variant(CLK_ENW::SSI1)
    }
    #[doc = "Enable clock for SSI0"]
    #[inline]
    pub fn ssi0(self) -> &'a mut W {
        self.variant(CLK_ENW::SSI0)
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
    #[doc = "Bits 0:1 - 1:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline]
    pub fn clk_en(&self) -> CLK_ENR {
        CLK_ENR::_from({
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
    #[doc = "Bits 0:1 - 1:0\\] 0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline]
    pub fn clk_en(&mut self) -> _CLK_ENW {
        _CLK_ENW { w: self }
    }
}
