#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NVIC_IPR7 {
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
pub struct PRI_31R {
    bits: u8,
}
impl PRI_31R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_30R {
    bits: u8,
}
impl PRI_30R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_29R {
    bits: u8,
}
impl PRI_29R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_28R {
    bits: u8,
}
impl PRI_28R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PRI_31W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_31W<'a> {
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
pub struct _PRI_30W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_30W<'a> {
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
pub struct _PRI_29W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_29W<'a> {
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
pub struct _PRI_28W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_28W<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\] Priority of interrupt 31 (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline]
    pub fn pri_31(&self) -> PRI_31R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_31R { bits }
    }
    #[doc = "Bits 16:23 - 23:16\\] Priority of interrupt 30 (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline]
    pub fn pri_30(&self) -> PRI_30R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_30R { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Priority of interrupt 29 (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline]
    pub fn pri_29(&self) -> PRI_29R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_29R { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Priority of interrupt 28 (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline]
    pub fn pri_28(&self) -> PRI_28R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_28R { bits }
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
    #[doc = "Bits 24:31 - 31:24\\] Priority of interrupt 31 (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline]
    pub fn pri_31(&mut self) -> _PRI_31W {
        _PRI_31W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\] Priority of interrupt 30 (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline]
    pub fn pri_30(&mut self) -> _PRI_30W {
        _PRI_30W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Priority of interrupt 29 (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline]
    pub fn pri_29(&mut self) -> _PRI_29W {
        _PRI_29W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Priority of interrupt 28 (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline]
    pub fn pri_28(&mut self) -> _PRI_28W {
        _PRI_28W { w: self }
    }
}
