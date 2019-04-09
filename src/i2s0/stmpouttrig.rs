#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STMPOUTTRIG {
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
pub struct RESERVED16R {
    bits: u16,
}
impl RESERVED16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OUT_START_WCNTR {
    bits: u16,
}
impl OUT_START_WCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED16W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED16W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUT_START_WCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _OUT_START_WCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&self) -> RESERVED16R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED16R { bits }
    }
    #[doc = "Bits 0:15 - 15:0\\] Compare value used to start the outgoing audio streams. This bit field must equal the WCLK counter value during the WCLK period in which the first output word(s) read from memory are clocked out (that is the sample at the start of the very first DMA output buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as outputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and 32 BCLK cycle ticks have happened. - 2 samples have been preloaded from memory (examine the AIFOUTPTR register if necessary). Note: The memory read access is only performed when required, that is channels 0/1 must be selected in AIFWMASK0/AIFWMASK1. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[inline]
    pub fn out_start_wcnt(&self) -> OUT_START_WCNTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        OUT_START_WCNTR { bits }
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
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&mut self) -> _RESERVED16W {
        _RESERVED16W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\] Compare value used to start the outgoing audio streams. This bit field must equal the WCLK counter value during the WCLK period in which the first output word(s) read from memory are clocked out (that is the sample at the start of the very first DMA output buffer). The value of this register takes effect when the following conditions are met: - One or more pins are configured as outputs in AIFDIRCFG. - AIFDMACFG has been configured for the correct buffer size, and 32 BCLK cycle ticks have happened. - 2 samples have been preloaded from memory (examine the AIFOUTPTR register if necessary). Note: The memory read access is only performed when required, that is channels 0/1 must be selected in AIFWMASK0/AIFWMASK1. Note: To avoid false triggers, this bit field should be set higher than STMPWPER.VALUE."]
    #[inline]
    pub fn out_start_wcnt(&mut self) -> _OUT_START_WCNTW {
        _OUT_START_WCNTW { w: self }
    }
}
