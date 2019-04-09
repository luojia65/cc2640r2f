#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG_IF_ADC {
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
pub struct FF2ADJR {
    bits: u8,
}
impl FF2ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FF3ADJR {
    bits: u8,
}
impl FF3ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INT3ADJR {
    bits: u8,
}
impl INT3ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FF1ADJR {
    bits: u8,
}
impl FF1ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AAFCAPR {
    bits: u8,
}
impl AAFCAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INT2ADJR {
    bits: u8,
}
impl INT2ADJR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IFDIGLDO_TRIM_OUTPUTR {
    bits: u8,
}
impl IFDIGLDO_TRIM_OUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IFANALDO_TRIM_OUTPUTR {
    bits: u8,
}
impl IFANALDO_TRIM_OUTPUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _FF2ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _FF2ADJW<'a> {
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
pub struct _FF3ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _FF3ADJW<'a> {
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
pub struct _INT3ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _INT3ADJW<'a> {
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
pub struct _FF1ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _FF1ADJW<'a> {
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
pub struct _AAFCAPW<'a> {
    w: &'a mut W,
}
impl<'a> _AAFCAPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INT2ADJW<'a> {
    w: &'a mut W,
}
impl<'a> _INT2ADJW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IFDIGLDO_TRIM_OUTPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _IFDIGLDO_TRIM_OUTPUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IFANALDO_TRIM_OUTPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _IFANALDO_TRIM_OUTPUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    pub fn ff2adj(&self) -> FF2ADJR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FF2ADJR { bits }
    }
    #[doc = "Bits 24:27 - 27:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ff3adj(&self) -> FF3ADJR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FF3ADJR { bits }
    }
    #[doc = "Bits 20:23 - 23:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn int3adj(&self) -> INT3ADJR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INT3ADJR { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ff1adj(&self) -> FF1ADJR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FF1ADJR { bits }
    }
    #[doc = "Bits 14:15 - 15:14\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn aafcap(&self) -> AAFCAPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AAFCAPR { bits }
    }
    #[doc = "Bits 10:13 - 13:10\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn int2adj(&self) -> INT2ADJR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INT2ADJR { bits }
    }
    #[doc = "Bits 5:9 - 9:5\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ifdigldo_trim_output(&self) -> IFDIGLDO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IFDIGLDO_TRIM_OUTPUTR { bits }
    }
    #[doc = "Bits 0:4 - 4:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ifanaldo_trim_output(&self) -> IFANALDO_TRIM_OUTPUTR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IFANALDO_TRIM_OUTPUTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 878769152 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ff2adj(&mut self) -> _FF2ADJW {
        _FF2ADJW { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ff3adj(&mut self) -> _FF3ADJW {
        _FF3ADJW { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn int3adj(&mut self) -> _INT3ADJW {
        _INT3ADJW { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ff1adj(&mut self) -> _FF1ADJW {
        _FF1ADJW { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn aafcap(&mut self) -> _AAFCAPW {
        _AAFCAPW { w: self }
    }
    #[doc = "Bits 10:13 - 13:10\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn int2adj(&mut self) -> _INT2ADJW {
        _INT2ADJW { w: self }
    }
    #[doc = "Bits 5:9 - 9:5\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ifdigldo_trim_output(&mut self) -> _IFDIGLDO_TRIM_OUTPUTW {
        _IFDIGLDO_TRIM_OUTPUTW { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ifanaldo_trim_output(&mut self) -> _IFANALDO_TRIM_OUTPUTW {
        _IFANALDO_TRIM_OUTPUTW { w: self }
    }
}
