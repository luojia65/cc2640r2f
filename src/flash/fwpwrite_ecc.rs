#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FWPWRITE_ECC {
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
pub struct ECCBYTES07_00R {
    bits: u8,
}
impl ECCBYTES07_00R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ECCBYTES15_08R {
    bits: u8,
}
impl ECCBYTES15_08R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ECCBYTES23_16R {
    bits: u8,
}
impl ECCBYTES23_16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ECCBYTES31_24R {
    bits: u8,
}
impl ECCBYTES31_24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ECCBYTES07_00W<'a> {
    w: &'a mut W,
}
impl<'a> _ECCBYTES07_00W<'a> {
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
pub struct _ECCBYTES15_08W<'a> {
    w: &'a mut W,
}
impl<'a> _ECCBYTES15_08W<'a> {
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
pub struct _ECCBYTES23_16W<'a> {
    w: &'a mut W,
}
impl<'a> _ECCBYTES23_16W<'a> {
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
pub struct _ECCBYTES31_24W<'a> {
    w: &'a mut W,
}
impl<'a> _ECCBYTES31_24W<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn eccbytes07_00(&self) -> ECCBYTES07_00R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECCBYTES07_00R { bits }
    }
    #[doc = "Bits 16:23 - 23:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn eccbytes15_08(&self) -> ECCBYTES15_08R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECCBYTES15_08R { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn eccbytes23_16(&self) -> ECCBYTES23_16R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECCBYTES23_16R { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn eccbytes31_24(&self) -> ECCBYTES31_24R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECCBYTES31_24R { bits }
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
    #[doc = "Bits 24:31 - 31:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn eccbytes07_00(&mut self) -> _ECCBYTES07_00W {
        _ECCBYTES07_00W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn eccbytes15_08(&mut self) -> _ECCBYTES15_08W {
        _ECCBYTES15_08W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn eccbytes23_16(&mut self) -> _ECCBYTES23_16W {
        _ECCBYTES23_16W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn eccbytes31_24(&mut self) -> _ECCBYTES31_24W {
        _ECCBYTES31_24W { w: self }
    }
}
