#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DONEMASK {
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
pub struct CHNLSR {
    bits: u32,
}
impl CHNLSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CHNLSW<'a> {
    w: &'a mut W,
}
impl<'a> _CHNLSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - 31:0\\] Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\] = 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\] is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\] = 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\] is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\] = 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\] from contributing to generation of combined uDMA done signal Bit \\[Ch\\] = 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\] to contribute to generation of combined uDMA done signal."]
    #[inline]
    pub fn chnls(&self) -> CHNLSR {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CHNLSR { bits }
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
    #[doc = "Bits 0:31 - 31:0\\] Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\] = 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\] is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\] = 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\] is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\] = 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\] from contributing to generation of combined uDMA done signal Bit \\[Ch\\] = 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\] to contribute to generation of combined uDMA done signal."]
    #[inline]
    pub fn chnls(&mut self) -> _CHNLSW {
        _CHNLSW { w: self }
    }
}
