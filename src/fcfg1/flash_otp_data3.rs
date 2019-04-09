#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASH_OTP_DATA3 {
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
pub struct EC_STEP_SIZER {
    bits: u16,
}
impl EC_STEP_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DO_PRECONDR {
    bits: bool,
}
impl DO_PRECONDR {
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
pub struct MAX_EC_LEVELR {
    bits: u8,
}
impl MAX_EC_LEVELR {
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
pub struct FLASH_SIZER {
    bits: u8,
}
impl FLASH_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WAIT_SYSCODER {
    bits: u8,
}
impl WAIT_SYSCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _EC_STEP_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _EC_STEP_SIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DO_PRECONDW<'a> {
    w: &'a mut W,
}
impl<'a> _DO_PRECONDW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAX_EC_LEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _MAX_EC_LEVELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 18;
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLASH_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_SIZEW<'a> {
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
pub struct _WAIT_SYSCODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAIT_SYSCODEW<'a> {
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
    #[doc = "Bits 23:31 - 31:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ec_step_size(&self) -> EC_STEP_SIZER {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        EC_STEP_SIZER { bits }
    }
    #[doc = "Bit 22 - 22:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn do_precond(&self) -> DO_PRECONDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DO_PRECONDR { bits }
    }
    #[doc = "Bits 18:21 - 21:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn max_ec_level(&self) -> MAX_EC_LEVELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAX_EC_LEVELR { bits }
    }
    #[doc = "Bits 16:17 - 17:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trim_1p7(&self) -> TRIM_1P7R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRIM_1P7R { bits }
    }
    #[doc = "Bits 8:15 - 15:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn flash_size(&self) -> FLASH_SIZER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FLASH_SIZER { bits }
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn wait_syscode(&self) -> WAIT_SYSCODER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WAIT_SYSCODER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1114115 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 23:31 - 31:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn ec_step_size(&mut self) -> _EC_STEP_SIZEW {
        _EC_STEP_SIZEW { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn do_precond(&mut self) -> _DO_PRECONDW {
        _DO_PRECONDW { w: self }
    }
    #[doc = "Bits 18:21 - 21:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn max_ec_level(&mut self) -> _MAX_EC_LEVELW {
        _MAX_EC_LEVELW { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn trim_1p7(&mut self) -> _TRIM_1P7W {
        _TRIM_1P7W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn flash_size(&mut self) -> _FLASH_SIZEW {
        _FLASH_SIZEW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn wait_syscode(&mut self) -> _WAIT_SYSCODEW {
        _WAIT_SYSCODEW { w: self }
    }
}
