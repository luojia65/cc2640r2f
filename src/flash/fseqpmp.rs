#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FSEQPMP {
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
pub struct TRIM_3P4R {
    bits: u8,
}
impl TRIM_3P4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED22R {
    bits: u8,
}
impl RESERVED22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_1P7R {
    bits: u8,
}
impl TRIM_1P7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRIM_0P8R {
    bits: u8,
}
impl TRIM_0P8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED15R {
    bits: bool,
}
impl RESERVED15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VIN_AT_XR {
    bits: u8,
}
impl VIN_AT_XR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED9R {
    bits: u8,
}
impl RESERVED9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VIN_BY_PASSR {
    bits: bool,
}
impl VIN_BY_PASSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SEQ_PUMPR {
    bits: u8,
}
impl SEQ_PUMPR {
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
pub struct _TRIM_3P4W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_3P4W<'a> {
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
pub struct _RESERVED22W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED22W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_1P7W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_1P7W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_0P8W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_0P8W<'a> {
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
pub struct _RESERVED15W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED15W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIN_AT_XW<'a> {
    w: &'a mut W,
}
impl<'a> _VIN_AT_XW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED9W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIN_BY_PASSW<'a> {
    w: &'a mut W,
}
impl<'a> _VIN_BY_PASSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SEQ_PUMPW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQ_PUMPW<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved28(&self) -> RESERVED28R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED28R { bits }
    }
    #[doc = "Bits 24:27 - 27:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trim_3p4(&self) -> TRIM_3P4R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_3P4R { bits }
    }
    #[doc = "Bits 22:23 - 23:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved22(&self) -> RESERVED22R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED22R { bits }
    }
    #[doc = "Bits 20:21 - 21:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trim_1p7(&self) -> TRIM_1P7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_1P7R { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trim_0p8(&self) -> TRIM_0P8R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_0P8R { bits }
    }
    #[doc = "Bit 15 - 15:15\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved15(&self) -> RESERVED15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED15R { bits }
    }
    #[doc = "Bits 12:14 - 14:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x(&self) -> VIN_AT_XR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VIN_AT_XR { bits }
    }
    #[doc = "Bits 9:11 - 11:9\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved9(&self) -> RESERVED9R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED9R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_by_pass(&self) -> VIN_BY_PASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VIN_BY_PASSR { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn seq_pump(&self) -> SEQ_PUMPR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEQ_PUMPR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2231894016 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - 31:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved28(&mut self) -> _RESERVED28W {
        _RESERVED28W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trim_3p4(&mut self) -> _TRIM_3P4W {
        _TRIM_3P4W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved22(&mut self) -> _RESERVED22W {
        _RESERVED22W { w: self }
    }
    #[doc = "Bits 20:21 - 21:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trim_1p7(&mut self) -> _TRIM_1P7W {
        _TRIM_1P7W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trim_0p8(&mut self) -> _TRIM_0P8W {
        _TRIM_0P8W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved15(&mut self) -> _RESERVED15W {
        _RESERVED15W { w: self }
    }
    #[doc = "Bits 12:14 - 14:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x(&mut self) -> _VIN_AT_XW {
        _VIN_AT_XW { w: self }
    }
    #[doc = "Bits 9:11 - 11:9\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved9(&mut self) -> _RESERVED9W {
        _RESERVED9W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_by_pass(&mut self) -> _VIN_BY_PASSW {
        _VIN_BY_PASSW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn seq_pump(&mut self) -> _SEQ_PUMPW {
        _SEQ_PUMPW { w: self }
    }
}
