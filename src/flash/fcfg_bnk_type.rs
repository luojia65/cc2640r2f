#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCFG_BNK_TYPE {
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
pub struct B7_TYPER {
    bits: u8,
}
impl B7_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B6_TYPER {
    bits: u8,
}
impl B6_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B5_TYPER {
    bits: u8,
}
impl B5_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B4_TYPER {
    bits: u8,
}
impl B4_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B3_TYPER {
    bits: u8,
}
impl B3_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B2_TYPER {
    bits: u8,
}
impl B2_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B1_TYPER {
    bits: u8,
}
impl B1_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct B0_TYPER {
    bits: u8,
}
impl B0_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _B7_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _B7_TYPEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _B6_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _B6_TYPEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _B5_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _B5_TYPEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _B4_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _B4_TYPEW<'a> {
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
pub struct _B3_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _B3_TYPEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _B2_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _B2_TYPEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _B1_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _B1_TYPEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _B0_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _B0_TYPEW<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b7_type(&self) -> B7_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B7_TYPER { bits }
    }
    #[doc = "Bits 24:27 - 27:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b6_type(&self) -> B6_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B6_TYPER { bits }
    }
    #[doc = "Bits 20:23 - 23:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b5_type(&self) -> B5_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B5_TYPER { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b4_type(&self) -> B4_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B4_TYPER { bits }
    }
    #[doc = "Bits 12:15 - 15:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b3_type(&self) -> B3_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B3_TYPER { bits }
    }
    #[doc = "Bits 8:11 - 11:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b2_type(&self) -> B2_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B2_TYPER { bits }
    }
    #[doc = "Bits 4:7 - 7:4\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b1_type(&self) -> B1_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B1_TYPER { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b0_type(&self) -> B0_TYPER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        B0_TYPER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b7_type(&mut self) -> _B7_TYPEW {
        _B7_TYPEW { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b6_type(&mut self) -> _B6_TYPEW {
        _B6_TYPEW { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b5_type(&mut self) -> _B5_TYPEW {
        _B5_TYPEW { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b4_type(&mut self) -> _B4_TYPEW {
        _B4_TYPEW { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b3_type(&mut self) -> _B3_TYPEW {
        _B3_TYPEW { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b2_type(&mut self) -> _B2_TYPEW {
        _B2_TYPEW { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b1_type(&mut self) -> _B1_TYPEW {
        _B1_TYPEW { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn b0_type(&mut self) -> _B0_TYPEW {
        _B0_TYPEW { w: self }
    }
}
