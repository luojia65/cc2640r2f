#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NVIC_IPR5 {
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
pub struct PRI_23R {
    bits: u8,
}
impl PRI_23R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_22R {
    bits: u8,
}
impl PRI_22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_21R {
    bits: u8,
}
impl PRI_21R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_20R {
    bits: u8,
}
impl PRI_20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PRI_23W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_23W<'a> {
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
pub struct _PRI_22W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_22W<'a> {
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
pub struct _PRI_21W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_21W<'a> {
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
pub struct _PRI_20W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_20W<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\] Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline]
    pub fn pri_23(&self) -> PRI_23R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_23R { bits }
    }
    #[doc = "Bits 16:23 - 23:16\\] Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline]
    pub fn pri_22(&self) -> PRI_22R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_22R { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline]
    pub fn pri_21(&self) -> PRI_21R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_21R { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline]
    pub fn pri_20(&self) -> PRI_20R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_20R { bits }
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
    #[doc = "Bits 24:31 - 31:24\\] Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline]
    pub fn pri_23(&mut self) -> _PRI_23W {
        _PRI_23W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\] Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline]
    pub fn pri_22(&mut self) -> _PRI_22W {
        _PRI_22W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline]
    pub fn pri_21(&mut self) -> _PRI_21W {
        _PRI_21W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline]
    pub fn pri_20(&mut self) -> _PRI_20W {
        _PRI_20W { w: self }
    }
}
