#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCFG_TAP_DAP_0 {
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
pub struct CPU_DAP_ENABLER {
    bits: u8,
}
impl CPU_DAP_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRCM_TAP_ENABLER {
    bits: u8,
}
impl PRCM_TAP_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TEST_TAP_ENABLER {
    bits: u8,
}
impl TEST_TAP_ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CPU_DAP_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CPU_DAP_ENABLEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRCM_TAP_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRCM_TAP_ENABLEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TEST_TAP_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEST_TAP_ENABLEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 16:23 - 23:16\\] Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[inline]
    pub fn cpu_dap_enable(&self) -> CPU_DAP_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CPU_DAP_ENABLER { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
    #[inline]
    pub fn prcm_tap_enable(&self) -> PRCM_TAP_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRCM_TAP_ENABLER { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[inline]
    pub fn test_tap_enable(&self) -> TEST_TAP_ENABLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TEST_TAP_ENABLER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4291151301 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:23 - 23:16\\] Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[inline]
    pub fn cpu_dap_enable(&mut self) -> _CPU_DAP_ENABLEW {
        _CPU_DAP_ENABLEW { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
    #[inline]
    pub fn prcm_tap_enable(&mut self) -> _PRCM_TAP_ENABLEW {
        _PRCM_TAP_ENABLEW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[inline]
    pub fn test_tap_enable(&mut self) -> _TEST_TAP_ENABLEW {
        _TEST_TAP_ENABLEW { w: self }
    }
}
