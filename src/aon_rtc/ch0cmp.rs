#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CH0CMP {
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
pub struct VALUER {
    bits: u32,
}
impl VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _VALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _VALUEW<'a> {
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
    #[doc = "Bits 0:31 - 31:0\\] RTC Channel 0 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to 2 SCLK_LF clock cycles before event occurs due to synchronization."]
    #[inline]
    pub fn value(&self) -> VALUER {
        let bits = {
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        VALUER { bits }
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
    #[doc = "Bits 0:31 - 31:0\\] RTC Channel 0 compare value. Bit 31 to 16 represents seconds and bits 15 to 0 represents subseconds of the compare value. The compare value is compared against SEC.VALUE (15:0) and SUBSEC.VALUE (31:16) values of the Real Time Clock register. A Cannel 0 event is generated when {SEC.VALUE(15:0),SUBSEC.VALUE (31:16)} is reaching or exciting the compare value. Writing to this register can trigger an immediate*) event in case the new compare value matches a Real Time Clock value from 1 second in the past up till current Real Time Clock value. Example: To generate a compare 5.5 seconds RTC start,- set this value = 0x0005_8000 *) It can take up to 2 SCLK_LF clock cycles before event occurs due to synchronization."]
    #[inline]
    pub fn value(&mut self) -> _VALUEW {
        _VALUEW { w: self }
    }
}
