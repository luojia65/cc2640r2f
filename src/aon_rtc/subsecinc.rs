#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SUBSECINC {
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
pub struct RESERVED24R {
    bits: u8,
}
impl RESERVED24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VALUEINCR {
    bits: u32,
}
impl VALUEINCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED24W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED24W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VALUEINCW<'a> {
    w: &'a mut W,
}
impl<'a> _VALUEINCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
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
    #[doc = "Bits 24:31 - 31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&self) -> RESERVED24R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED24R { bits }
    }
    #[doc = "Bits 0:23 - 23:0\\] This value compensates for a SCLK_LF clock which has an offset from 32768 Hz. The compensation value can be found as 2^38 / freq, where freq is SCLK_LF clock frequency in Hertz This value is added to SUBSEC.VALUE on every cycle, and carry of this is added to SEC.VALUE. To perform the addition, bits \\[23:6\\] are aligned with SUBSEC.VALUE bits \\[17:0\\]. The remaining bits \\[5:0\\] are accumulated in a hidden 6-bit register that generates a carry into the above mentioned addition on overflow. The default value corresponds to incrementing by precisely 1/32768 of a second. NOTE: This register is read only. Modification of the register value must be done using registers AUX_WUC:RTCSUBSECINC1 , AUX_WUC:RTCSUBSECINC0 and AUX_WUC:RTCSUBSECINCCTL"]
    #[inline]
    pub fn valueinc(&self) -> VALUEINCR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        VALUEINCR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8388608 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:31 - 31:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&mut self) -> _RESERVED24W {
        _RESERVED24W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\] This value compensates for a SCLK_LF clock which has an offset from 32768 Hz. The compensation value can be found as 2^38 / freq, where freq is SCLK_LF clock frequency in Hertz This value is added to SUBSEC.VALUE on every cycle, and carry of this is added to SEC.VALUE. To perform the addition, bits \\[23:6\\] are aligned with SUBSEC.VALUE bits \\[17:0\\]. The remaining bits \\[5:0\\] are accumulated in a hidden 6-bit register that generates a carry into the above mentioned addition on overflow. The default value corresponds to incrementing by precisely 1/32768 of a second. NOTE: This register is read only. Modification of the register value must be done using registers AUX_WUC:RTCSUBSECINC1 , AUX_WUC:RTCSUBSECINC0 and AUX_WUC:RTCSUBSECINCCTL"]
    #[inline]
    pub fn valueinc(&mut self) -> _VALUEINCW {
        _VALUEINCW { w: self }
    }
}
