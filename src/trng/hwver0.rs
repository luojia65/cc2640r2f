#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HWVER0 {
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
pub struct RESERVED28R {
    bits: u8,
}
impl RESERVED28R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HW_MAJOR_VERR {
    bits: u8,
}
impl HW_MAJOR_VERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HW_MINOR_VERR {
    bits: u8,
}
impl HW_MINOR_VERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HW_PATCH_LVLR {
    bits: u8,
}
impl HW_PATCH_LVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EIP_NUM_COMPLR {
    bits: u8,
}
impl EIP_NUM_COMPLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EIP_NUMR {
    bits: u8,
}
impl EIP_NUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED28W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED28W<'a> {
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
pub struct _HW_MAJOR_VERW<'a> {
    w: &'a mut W,
}
impl<'a> _HW_MAJOR_VERW<'a> {
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
pub struct _HW_MINOR_VERW<'a> {
    w: &'a mut W,
}
impl<'a> _HW_MINOR_VERW<'a> {
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
pub struct _HW_PATCH_LVLW<'a> {
    w: &'a mut W,
}
impl<'a> _HW_PATCH_LVLW<'a> {
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
pub struct _EIP_NUM_COMPLW<'a> {
    w: &'a mut W,
}
impl<'a> _EIP_NUM_COMPLW<'a> {
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
pub struct _EIP_NUMW<'a> {
    w: &'a mut W,
}
impl<'a> _EIP_NUMW<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved28(&self) -> RESERVED28R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED28R { bits }
    }
    #[doc = "Bits 24:27 - 27:24\\] 4 bits binary encoding of the major hardware revision number."]
    #[inline]
    pub fn hw_major_ver(&self) -> HW_MAJOR_VERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HW_MAJOR_VERR { bits }
    }
    #[doc = "Bits 20:23 - 23:20\\] 4 bits binary encoding of the minor hardware revision number."]
    #[inline]
    pub fn hw_minor_ver(&self) -> HW_MINOR_VERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HW_MINOR_VERR { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] 4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
    #[inline]
    pub fn hw_patch_lvl(&self) -> HW_PATCH_LVLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HW_PATCH_LVLR { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
    #[inline]
    pub fn eip_num_compl(&self) -> EIP_NUM_COMPLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EIP_NUM_COMPLR { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] 8 bits binary encoding of the module number. This TRNG gives 0x4B."]
    #[inline]
    pub fn eip_num(&self) -> EIP_NUMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EIP_NUMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 33600587 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved28(&mut self) -> _RESERVED28W {
        _RESERVED28W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] 4 bits binary encoding of the major hardware revision number."]
    #[inline]
    pub fn hw_major_ver(&mut self) -> _HW_MAJOR_VERW {
        _HW_MAJOR_VERW { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] 4 bits binary encoding of the minor hardware revision number."]
    #[inline]
    pub fn hw_minor_ver(&mut self) -> _HW_MINOR_VERW {
        _HW_MINOR_VERW { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] 4 bits binary encoding of the hardware patch level, initial release will carry value zero."]
    #[inline]
    pub fn hw_patch_lvl(&mut self) -> _HW_PATCH_LVLW {
        _HW_PATCH_LVLW { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Bit-by-bit logic complement of bits \\[7:0\\]. This TRNG gives 0xB4."]
    #[inline]
    pub fn eip_num_compl(&mut self) -> _EIP_NUM_COMPLW {
        _EIP_NUM_COMPLW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] 8 bits binary encoding of the module number. This TRNG gives 0x4B."]
    #[inline]
    pub fn eip_num(&mut self) -> _EIP_NUMW {
        _EIP_NUMW { w: self }
    }
}
