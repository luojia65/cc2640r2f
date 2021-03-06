#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSCCFG {
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
pub struct RESERVED8R {
    bits: u32,
}
impl RESERVED8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PER_MR {
    bits: u8,
}
impl PER_MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PER_ER {
    bits: u8,
}
impl PER_ER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED8W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PER_MW<'a> {
    w: &'a mut W,
}
impl<'a> _PER_MW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PER_EW<'a> {
    w: &'a mut W,
}
impl<'a> _PER_EW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&self) -> RESERVED8R {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED8R { bits }
    }
    #[doc = "Bits 3:7 - 7:3\\] Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the mantissa Note: Oscillator amplitude calibration is turned of when both this bitfield and PER_E are set to 0"]
    #[inline]
    pub fn per_m(&self) -> PER_MR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PER_MR { bits }
    }
    #[doc = "Bits 0:2 - 2:0\\] Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the exponent Note: Oscillator amplitude calibration is turned of when both PER_M and this bitfield are set to 0"]
    #[inline]
    pub fn per_e(&self) -> PER_ER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PER_ER { bits }
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&mut self) -> _RESERVED8W {
        _RESERVED8W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\] Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the mantissa Note: Oscillator amplitude calibration is turned of when both this bitfield and PER_E are set to 0"]
    #[inline]
    pub fn per_m(&mut self) -> _PER_MW {
        _PER_MW { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Number of 32 KHz clocks between oscillator amplitude calibrations. When this counter expires, an oscillator amplitude compensation is triggered immediately in Active mode. When this counter expires in Powerdown mode an internal flag is set such that the amplitude compensation is postponed until the next recharge occurs. The Period will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent PERIOD=(PER_M*16+15)*2^PER_E This field sets the exponent Note: Oscillator amplitude calibration is turned of when both PER_M and this bitfield are set to 0"]
    #[inline]
    pub fn per_e(&mut self) -> _PER_EW {
        _PER_EW { w: self }
    }
}
