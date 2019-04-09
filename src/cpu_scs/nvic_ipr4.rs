#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NVIC_IPR4 {
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
pub struct PRI_19R {
    bits: u8,
}
impl PRI_19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_18R {
    bits: u8,
}
impl PRI_18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_17R {
    bits: u8,
}
impl PRI_17R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_16R {
    bits: u8,
}
impl PRI_16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PRI_19W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_19W<'a> {
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
pub struct _PRI_18W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_18W<'a> {
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
pub struct _PRI_17W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_17W<'a> {
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
pub struct _PRI_16W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_16W<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\] Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline]
    pub fn pri_19(&self) -> PRI_19R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_19R { bits }
    }
    #[doc = "Bits 16:23 - 23:16\\] Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline]
    pub fn pri_18(&self) -> PRI_18R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_18R { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline]
    pub fn pri_17(&self) -> PRI_17R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_17R { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline]
    pub fn pri_16(&self) -> PRI_16R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_16R { bits }
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
    #[doc = "Bits 24:31 - 31:24\\] Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline]
    pub fn pri_19(&mut self) -> _PRI_19W {
        _PRI_19W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\] Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline]
    pub fn pri_18(&mut self) -> _PRI_18W {
        _PRI_18W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline]
    pub fn pri_17(&mut self) -> _PRI_17W {
        _PRI_17W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline]
    pub fn pri_16(&mut self) -> _PRI_16W {
        _PRI_16W { w: self }
    }
}
