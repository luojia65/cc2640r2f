#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCFG_BANK {
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
pub struct EE_BANK_WIDTHR {
    bits: u16,
}
impl EE_BANK_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EE_NUM_BANKR {
    bits: u8,
}
impl EE_NUM_BANKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAIN_BANK_WIDTHR {
    bits: u16,
}
impl MAIN_BANK_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAIN_NUM_BANKR {
    bits: u8,
}
impl MAIN_NUM_BANKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _EE_BANK_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _EE_BANK_WIDTHW<'a> {
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
pub struct _EE_NUM_BANKW<'a> {
    w: &'a mut W,
}
impl<'a> _EE_NUM_BANKW<'a> {
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
pub struct _MAIN_BANK_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _MAIN_BANK_WIDTHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAIN_NUM_BANKW<'a> {
    w: &'a mut W,
}
impl<'a> _MAIN_NUM_BANKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 20:31 - 31:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ee_bank_width(&self) -> EE_BANK_WIDTHR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        EE_BANK_WIDTHR { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ee_num_bank(&self) -> EE_NUM_BANKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EE_NUM_BANKR { bits }
    }
    #[doc = "Bits 4:15 - 15:4\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn main_bank_width(&self) -> MAIN_BANK_WIDTHR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MAIN_BANK_WIDTHR { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn main_num_bank(&self) -> MAIN_NUM_BANKR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAIN_NUM_BANKR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1025 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 20:31 - 31:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ee_bank_width(&mut self) -> _EE_BANK_WIDTHW {
        _EE_BANK_WIDTHW { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ee_num_bank(&mut self) -> _EE_NUM_BANKW {
        _EE_NUM_BANKW { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn main_bank_width(&mut self) -> _MAIN_BANK_WIDTHW {
        _MAIN_BANK_WIDTHW { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn main_num_bank(&mut self) -> _MAIN_NUM_BANKW {
        _MAIN_NUM_BANKW { w: self }
    }
}
