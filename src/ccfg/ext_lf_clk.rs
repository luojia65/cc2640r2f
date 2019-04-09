#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXT_LF_CLK {
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
pub struct DIOR {
    bits: u8,
}
impl DIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RTC_INCREMENTR {
    bits: u32,
}
impl RTC_INCREMENTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DIOW<'a> {
    w: &'a mut W,
}
impl<'a> _DIOW<'a> {
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
pub struct _RTC_INCREMENTW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_INCREMENTW<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\] Unsigned integer, selecting the DIO to supply external 32kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL. The selected DIO will be marked as reserved by the pin driver (TI-RTOS environment) and hence not selectable for other usage."]
    #[inline]
    pub fn dio(&self) -> DIOR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DIOR { bits }
    }
    #[doc = "Bits 0:23 - 23:0\\] Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
    #[inline]
    pub fn rtc_increment(&self) -> RTC_INCREMENTR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RTC_INCREMENTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:31 - 31:24\\] Unsigned integer, selecting the DIO to supply external 32kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL. The selected DIO will be marked as reserved by the pin driver (TI-RTOS environment) and hence not selectable for other usage."]
    #[inline]
    pub fn dio(&mut self) -> _DIOW {
        _DIOW { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\] Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
    #[inline]
    pub fn rtc_increment(&mut self) -> _RTC_INCREMENTW {
        _RTC_INCREMENTW { w: self }
    }
}
