#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWD_CURR_35C {
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
pub struct DELTA_CACHE_REFR {
    bits: u8,
}
impl DELTA_CACHE_REFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DELTA_RFMEM_RETR {
    bits: u8,
}
impl DELTA_RFMEM_RETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DELTA_XOSC_LPMR {
    bits: u8,
}
impl DELTA_XOSC_LPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BASELINER {
    bits: u8,
}
impl BASELINER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DELTA_CACHE_REFW<'a> {
    w: &'a mut W,
}
impl<'a> _DELTA_CACHE_REFW<'a> {
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
pub struct _DELTA_RFMEM_RETW<'a> {
    w: &'a mut W,
}
impl<'a> _DELTA_RFMEM_RETW<'a> {
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
pub struct _DELTA_XOSC_LPMW<'a> {
    w: &'a mut W,
}
impl<'a> _DELTA_XOSC_LPMW<'a> {
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
pub struct _BASELINEW<'a> {
    w: &'a mut W,
}
impl<'a> _BASELINEW<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\] Additional maximum current, in units of 1uA, with cache retention"]
    #[inline]
    pub fn delta_cache_ref(&self) -> DELTA_CACHE_REFR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DELTA_CACHE_REFR { bits }
    }
    #[doc = "Bits 16:23 - 23:16\\] Additional maximum current, in 1uA units, with RF memory retention"]
    #[inline]
    pub fn delta_rfmem_ret(&self) -> DELTA_RFMEM_RETR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DELTA_RFMEM_RETR { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode"]
    #[inline]
    pub fn delta_xosc_lpm(&self) -> DELTA_XOSC_LPMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DELTA_XOSC_LPMR { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA"]
    #[inline]
    pub fn baseline(&self) -> BASELINER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BASELINER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 202417418 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:31 - 31:24\\] Additional maximum current, in units of 1uA, with cache retention"]
    #[inline]
    pub fn delta_cache_ref(&mut self) -> _DELTA_CACHE_REFW {
        _DELTA_CACHE_REFW { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\] Additional maximum current, in 1uA units, with RF memory retention"]
    #[inline]
    pub fn delta_rfmem_ret(&mut self) -> _DELTA_RFMEM_RETW {
        _DELTA_RFMEM_RETW { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode"]
    #[inline]
    pub fn delta_xosc_lpm(&mut self) -> _DELTA_XOSC_LPMW {
        _DELTA_XOSC_LPMW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Worst-case baseline maximum powerdown current, in units of 0.5uA"]
    #[inline]
    pub fn baseline(&mut self) -> _BASELINEW {
        _BASELINEW { w: self }
    }
}
