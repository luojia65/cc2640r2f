#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LDO_TRIM {
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
pub struct RESERVED4R {
    bits: u8,
}
impl RESERVED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VDDR_TRIM_SLEEPR {
    bits: u8,
}
impl VDDR_TRIM_SLEEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED3R {
    bits: u8,
}
impl RESERVED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GLDO_CURSRCR {
    bits: u8,
}
impl GLDO_CURSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED2R {
    bits: u8,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ITRIM_DIGLDO_LOADR {
    bits: u8,
}
impl ITRIM_DIGLDO_LOADR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ITRIM_UDIGLDOR {
    bits: u8,
}
impl ITRIM_UDIGLDOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED1R {
    bits: u8,
}
impl RESERVED1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VTRIM_DELTAR {
    bits: u8,
}
impl VTRIM_DELTAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED4W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VDDR_TRIM_SLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDR_TRIM_SLEEPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GLDO_CURSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _GLDO_CURSRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ITRIM_DIGLDO_LOADW<'a> {
    w: &'a mut W,
}
impl<'a> _ITRIM_DIGLDO_LOADW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ITRIM_UDIGLDOW<'a> {
    w: &'a mut W,
}
impl<'a> _ITRIM_UDIGLDOW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED1W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VTRIM_DELTAW<'a> {
    w: &'a mut W,
}
impl<'a> _VTRIM_DELTAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 29:31 - 31:29\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED4R { bits }
    }
    #[doc = "Bits 24:28 - 28:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vddr_trim_sleep(&self) -> VDDR_TRIM_SLEEPR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VDDR_TRIM_SLEEPR { bits }
    }
    #[doc = "Bits 19:23 - 23:19\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED3R { bits }
    }
    #[doc = "Bits 16:18 - 18:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn gldo_cursrc(&self) -> GLDO_CURSRCR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GLDO_CURSRCR { bits }
    }
    #[doc = "Bits 13:15 - 15:13\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 11:12 - 12:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn itrim_digldo_load(&self) -> ITRIM_DIGLDO_LOADR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ITRIM_DIGLDO_LOADR { bits }
    }
    #[doc = "Bits 8:10 - 10:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn itrim_udigldo(&self) -> ITRIM_UDIGLDOR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ITRIM_UDIGLDOR { bits }
    }
    #[doc = "Bits 3:7 - 7:3\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved1(&self) -> RESERVED1R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED1R { bits }
    }
    #[doc = "Bits 0:2 - 2:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vtrim_delta(&self) -> VTRIM_DELTAR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VTRIM_DELTAR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3774406907 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 29:31 - 31:29\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bits 24:28 - 28:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vddr_trim_sleep(&mut self) -> _VDDR_TRIM_SLEEPW {
        _VDDR_TRIM_SLEEPW { w: self }
    }
    #[doc = "Bits 19:23 - 23:19\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn gldo_cursrc(&mut self) -> _GLDO_CURSRCW {
        _GLDO_CURSRCW { w: self }
    }
    #[doc = "Bits 13:15 - 15:13\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bits 11:12 - 12:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn itrim_digldo_load(&mut self) -> _ITRIM_DIGLDO_LOADW {
        _ITRIM_DIGLDO_LOADW { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn itrim_udigldo(&mut self) -> _ITRIM_UDIGLDOW {
        _ITRIM_UDIGLDOW { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved1(&mut self) -> _RESERVED1W {
        _RESERVED1W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vtrim_delta(&mut self) -> _VTRIM_DELTAW {
        _VTRIM_DELTAW { w: self }
    }
}
