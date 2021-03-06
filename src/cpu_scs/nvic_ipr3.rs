#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NVIC_IPR3 {
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
pub struct PRI_15R {
    bits: u8,
}
impl PRI_15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_14R {
    bits: u8,
}
impl PRI_14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_13R {
    bits: u8,
}
impl PRI_13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRI_12R {
    bits: u8,
}
impl PRI_12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PRI_15W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_15W<'a> {
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
pub struct _PRI_14W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_14W<'a> {
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
pub struct _PRI_13W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_13W<'a> {
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
pub struct _PRI_12W<'a> {
    w: &'a mut W,
}
impl<'a> _PRI_12W<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\] Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline]
    pub fn pri_15(&self) -> PRI_15R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_15R { bits }
    }
    #[doc = "Bits 16:23 - 23:16\\] Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline]
    pub fn pri_14(&self) -> PRI_14R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_14R { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline]
    pub fn pri_13(&self) -> PRI_13R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_13R { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline]
    pub fn pri_12(&self) -> PRI_12R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRI_12R { bits }
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
    #[doc = "Bits 24:31 - 31:24\\] Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline]
    pub fn pri_15(&mut self) -> _PRI_15W {
        _PRI_15W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\] Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline]
    pub fn pri_14(&mut self) -> _PRI_14W {
        _PRI_14W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline]
    pub fn pri_13(&mut self) -> _PRI_13W {
        _PRI_13W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline]
    pub fn pri_12(&mut self) -> _PRI_12W {
        _PRI_12W { w: self }
    }
}
