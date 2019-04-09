#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NVIC_IPR2 {
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
pub struct PRI_11R {
    bits: u8,
}
impl PRI_11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_10R {
    bits: u8,
}
impl PRI_10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_9R {
    bits: u8,
}
impl PRI_9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_8R {
    bits: u8,
}
impl PRI_8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PRI_11W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_11W<'a> {
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
pub struct _PRI_10W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_10W<'a> {
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
pub struct _PRI_9W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_9W<'a> {
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
pub struct _PRI_8W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_8W<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\] Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline]
    pub fn pri_11(&self) -> PRI_11R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_11R { bits }
    }
    #[doc = "Bits 16:23 - 23:16\\] Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline]
    pub fn pri_10(&self) -> PRI_10R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_10R { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline]
    pub fn pri_9(&self) -> PRI_9R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_9R { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline]
    pub fn pri_8(&self) -> PRI_8R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_8R { bits }
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
    #[doc = "Bits 24:31 - 31:24\\] Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline]
    pub fn pri_11(&mut self) -> _PRI_11W {
        _PRI_11W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\] Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline]
    pub fn pri_10(&mut self) -> _PRI_10W {
        _PRI_10W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline]
    pub fn pri_9(&mut self) -> _PRI_9W {
        _PRI_9W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline]
    pub fn pri_8(&mut self) -> _PRI_8W {
        _PRI_8W { w: self }
    }
}
