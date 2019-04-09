#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FSM_PGM {
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
pub struct RESERVED26R {
    bits: u8,
}
impl RESERVED26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PGM_BANKR {
    bits: u8,
}
impl PGM_BANKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PGM_ADDRR {
    bits: u32,
}
impl PGM_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED26W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED26W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PGM_BANKW<'a> {
    w: &'a mut W,
}
impl<'a> _PGM_BANKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PGM_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _PGM_ADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 8388607;
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
    #[doc = "Bits 26:31 - 31:26\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved26(&self) -> RESERVED26R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED26R { bits }
    }
    #[doc = "Bits 23:25 - 25:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pgm_bank(&self) -> PGM_BANKR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PGM_BANKR { bits }
    }
    #[doc = "Bits 0:22 - 22:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pgm_addr(&self) -> PGM_ADDRR {
        let bits = {
            const MASK: u32 = 8388607;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        PGM_ADDRR { bits }
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
    #[doc = "Bits 26:31 - 31:26\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved26(&mut self) -> _RESERVED26W {
        _RESERVED26W { w: self }
    }
    #[doc = "Bits 23:25 - 25:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pgm_bank(&mut self) -> _PGM_BANKW {
        _PGM_BANKW { w: self }
    }
    #[doc = "Bits 0:22 - 22:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn pgm_addr(&mut self) -> _PGM_ADDRW {
        _PGM_ADDRW { w: self }
    }
}
