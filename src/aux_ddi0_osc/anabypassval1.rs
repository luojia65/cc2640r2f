#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ANABYPASSVAL1 {
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
pub struct RESERVED20R {
    bits: u16,
}
impl RESERVED20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_HF_ROW_Q12R {
    bits: u8,
}
impl XOSC_HF_ROW_Q12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XOSC_HF_COLUMN_Q12R {
    bits: u16,
}
impl XOSC_HF_COLUMN_Q12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED20W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED20W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_HF_ROW_Q12W<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_HF_ROW_Q12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XOSC_HF_COLUMN_Q12W<'a> {
    w: &'a mut W,
}
impl<'a> _XOSC_HF_COLUMN_Q12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 20:31 - 31:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved20(&self) -> RESERVED20R {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED20R { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_row_q12(&self) -> XOSC_HF_ROW_Q12R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XOSC_HF_ROW_Q12R { bits }
    }
    #[doc = "Bits 0:15 - 15:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_column_q12(&self) -> XOSC_HF_COLUMN_Q12R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        XOSC_HF_COLUMN_Q12R { bits }
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
    #[doc = "Bits 20:31 - 31:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved20(&mut self) -> _RESERVED20W {
        _RESERVED20W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_row_q12(&mut self) -> _XOSC_HF_ROW_Q12W {
        _XOSC_HF_ROW_Q12W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn xosc_hf_column_q12(&mut self) -> _XOSC_HF_COLUMN_Q12W {
        _XOSC_HF_COLUMN_Q12W { w: self }
    }
}
