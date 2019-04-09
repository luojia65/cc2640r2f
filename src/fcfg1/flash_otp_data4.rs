#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASH_OTP_DATA4 {
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
pub struct STANDBY_MODE_SEL_INT_WRTR {
    bits: bool,
}
impl STANDBY_MODE_SEL_INT_WRTR {
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
pub struct STANDBY_PW_SEL_INT_WRTR {
    bits: u8,
}
impl STANDBY_PW_SEL_INT_WRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIS_STANDBY_INT_WRTR {
    bits: bool,
}
impl DIS_STANDBY_INT_WRTR {
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
pub struct DIS_IDLE_INT_WRTR {
    bits: bool,
}
impl DIS_IDLE_INT_WRTR {
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
pub struct VIN_AT_X_INT_WRTR {
    bits: u8,
}
impl VIN_AT_X_INT_WRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STANDBY_MODE_SEL_EXT_WRTR {
    bits: bool,
}
impl STANDBY_MODE_SEL_EXT_WRTR {
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
pub struct STANDBY_PW_SEL_EXT_WRTR {
    bits: u8,
}
impl STANDBY_PW_SEL_EXT_WRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIS_STANDBY_EXT_WRTR {
    bits: bool,
}
impl DIS_STANDBY_EXT_WRTR {
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
pub struct DIS_IDLE_EXT_WRTR {
    bits: bool,
}
impl DIS_IDLE_EXT_WRTR {
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
pub struct VIN_AT_X_EXT_WRTR {
    bits: u8,
}
impl VIN_AT_X_EXT_WRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STANDBY_MODE_SEL_INT_RDR {
    bits: bool,
}
impl STANDBY_MODE_SEL_INT_RDR {
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
pub struct STANDBY_PW_SEL_INT_RDR {
    bits: u8,
}
impl STANDBY_PW_SEL_INT_RDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIS_STANDBY_INT_RDR {
    bits: bool,
}
impl DIS_STANDBY_INT_RDR {
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
pub struct DIS_IDLE_INT_RDR {
    bits: bool,
}
impl DIS_IDLE_INT_RDR {
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
pub struct VIN_AT_X_INT_RDR {
    bits: u8,
}
impl VIN_AT_X_INT_RDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STANDBY_MODE_SEL_EXT_RDR {
    bits: bool,
}
impl STANDBY_MODE_SEL_EXT_RDR {
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
pub struct STANDBY_PW_SEL_EXT_RDR {
    bits: u8,
}
impl STANDBY_PW_SEL_EXT_RDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIS_STANDBY_EXT_RDR {
    bits: bool,
}
impl DIS_STANDBY_EXT_RDR {
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
pub struct DIS_IDLE_EXT_RDR {
    bits: bool,
}
impl DIS_IDLE_EXT_RDR {
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
pub struct VIN_AT_X_EXT_RDR {
    bits: u8,
}
impl VIN_AT_X_EXT_RDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _STANDBY_MODE_SEL_INT_WRTW<'a> {
    w: &'a mut W,
}
impl<'a> _STANDBY_MODE_SEL_INT_WRTW<'a> {
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STANDBY_PW_SEL_INT_WRTW<'a> {
    w: &'a mut W,
}
impl<'a> _STANDBY_PW_SEL_INT_WRTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIS_STANDBY_INT_WRTW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_STANDBY_INT_WRTW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIS_IDLE_INT_WRTW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_IDLE_INT_WRTW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIN_AT_X_INT_WRTW<'a> {
    w: &'a mut W,
}
impl<'a> _VIN_AT_X_INT_WRTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STANDBY_MODE_SEL_EXT_WRTW<'a> {
    w: &'a mut W,
}
impl<'a> _STANDBY_MODE_SEL_EXT_WRTW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STANDBY_PW_SEL_EXT_WRTW<'a> {
    w: &'a mut W,
}
impl<'a> _STANDBY_PW_SEL_EXT_WRTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIS_STANDBY_EXT_WRTW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_STANDBY_EXT_WRTW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIS_IDLE_EXT_WRTW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_IDLE_EXT_WRTW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIN_AT_X_EXT_WRTW<'a> {
    w: &'a mut W,
}
impl<'a> _VIN_AT_X_EXT_WRTW<'a> {
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
pub struct _STANDBY_MODE_SEL_INT_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _STANDBY_MODE_SEL_INT_RDW<'a> {
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
pub struct _STANDBY_PW_SEL_INT_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _STANDBY_PW_SEL_INT_RDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIS_STANDBY_INT_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_STANDBY_INT_RDW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIS_IDLE_INT_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_IDLE_INT_RDW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIN_AT_X_INT_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _VIN_AT_X_INT_RDW<'a> {
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
pub struct _STANDBY_MODE_SEL_EXT_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _STANDBY_MODE_SEL_EXT_RDW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STANDBY_PW_SEL_EXT_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _STANDBY_PW_SEL_EXT_RDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIS_STANDBY_EXT_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_STANDBY_EXT_RDW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIS_IDLE_EXT_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_IDLE_EXT_RDW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VIN_AT_X_EXT_RDW<'a> {
    w: &'a mut W,
}
impl<'a> _VIN_AT_X_EXT_RDW<'a> {
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
    #[doc = "Bit 31 - 31:31\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_mode_sel_int_wrt(&self) -> STANDBY_MODE_SEL_INT_WRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STANDBY_MODE_SEL_INT_WRTR { bits }
    }
    #[doc = "Bits 29:30 - 30:29\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_pw_sel_int_wrt(&self) -> STANDBY_PW_SEL_INT_WRTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STANDBY_PW_SEL_INT_WRTR { bits }
    }
    #[doc = "Bit 28 - 28:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_standby_int_wrt(&self) -> DIS_STANDBY_INT_WRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_STANDBY_INT_WRTR { bits }
    }
    #[doc = "Bit 27 - 27:27\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_idle_int_wrt(&self) -> DIS_IDLE_INT_WRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_IDLE_INT_WRTR { bits }
    }
    #[doc = "Bits 24:26 - 26:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x_int_wrt(&self) -> VIN_AT_X_INT_WRTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VIN_AT_X_INT_WRTR { bits }
    }
    #[doc = "Bit 23 - 23:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_mode_sel_ext_wrt(&self) -> STANDBY_MODE_SEL_EXT_WRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STANDBY_MODE_SEL_EXT_WRTR { bits }
    }
    #[doc = "Bits 21:22 - 22:21\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_pw_sel_ext_wrt(&self) -> STANDBY_PW_SEL_EXT_WRTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STANDBY_PW_SEL_EXT_WRTR { bits }
    }
    #[doc = "Bit 20 - 20:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_standby_ext_wrt(&self) -> DIS_STANDBY_EXT_WRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_STANDBY_EXT_WRTR { bits }
    }
    #[doc = "Bit 19 - 19:19\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_idle_ext_wrt(&self) -> DIS_IDLE_EXT_WRTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_IDLE_EXT_WRTR { bits }
    }
    #[doc = "Bits 16:18 - 18:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x_ext_wrt(&self) -> VIN_AT_X_EXT_WRTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VIN_AT_X_EXT_WRTR { bits }
    }
    #[doc = "Bit 15 - 15:15\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_mode_sel_int_rd(&self) -> STANDBY_MODE_SEL_INT_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STANDBY_MODE_SEL_INT_RDR { bits }
    }
    #[doc = "Bits 13:14 - 14:13\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_pw_sel_int_rd(&self) -> STANDBY_PW_SEL_INT_RDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STANDBY_PW_SEL_INT_RDR { bits }
    }
    #[doc = "Bit 12 - 12:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_standby_int_rd(&self) -> DIS_STANDBY_INT_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_STANDBY_INT_RDR { bits }
    }
    #[doc = "Bit 11 - 11:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_idle_int_rd(&self) -> DIS_IDLE_INT_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_IDLE_INT_RDR { bits }
    }
    #[doc = "Bits 8:10 - 10:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x_int_rd(&self) -> VIN_AT_X_INT_RDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VIN_AT_X_INT_RDR { bits }
    }
    #[doc = "Bit 7 - 7:7\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_mode_sel_ext_rd(&self) -> STANDBY_MODE_SEL_EXT_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STANDBY_MODE_SEL_EXT_RDR { bits }
    }
    #[doc = "Bits 5:6 - 6:5\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_pw_sel_ext_rd(&self) -> STANDBY_PW_SEL_EXT_RDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        STANDBY_PW_SEL_EXT_RDR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_standby_ext_rd(&self) -> DIS_STANDBY_EXT_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_STANDBY_EXT_RDR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_idle_ext_rd(&self) -> DIS_IDLE_EXT_RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_IDLE_EXT_RDR { bits }
    }
    #[doc = "Bits 0:2 - 2:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x_ext_rd(&self) -> VIN_AT_X_EXT_RDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VIN_AT_X_EXT_RDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2560139167 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - 31:31\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_mode_sel_int_wrt(&mut self) -> _STANDBY_MODE_SEL_INT_WRTW {
        _STANDBY_MODE_SEL_INT_WRTW { w: self }
    }
    #[doc = "Bits 29:30 - 30:29\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_pw_sel_int_wrt(&mut self) -> _STANDBY_PW_SEL_INT_WRTW {
        _STANDBY_PW_SEL_INT_WRTW { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_standby_int_wrt(&mut self) -> _DIS_STANDBY_INT_WRTW {
        _DIS_STANDBY_INT_WRTW { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_idle_int_wrt(&mut self) -> _DIS_IDLE_INT_WRTW {
        _DIS_IDLE_INT_WRTW { w: self }
    }
    #[doc = "Bits 24:26 - 26:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x_int_wrt(&mut self) -> _VIN_AT_X_INT_WRTW {
        _VIN_AT_X_INT_WRTW { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_mode_sel_ext_wrt(&mut self) -> _STANDBY_MODE_SEL_EXT_WRTW {
        _STANDBY_MODE_SEL_EXT_WRTW { w: self }
    }
    #[doc = "Bits 21:22 - 22:21\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_pw_sel_ext_wrt(&mut self) -> _STANDBY_PW_SEL_EXT_WRTW {
        _STANDBY_PW_SEL_EXT_WRTW { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_standby_ext_wrt(&mut self) -> _DIS_STANDBY_EXT_WRTW {
        _DIS_STANDBY_EXT_WRTW { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_idle_ext_wrt(&mut self) -> _DIS_IDLE_EXT_WRTW {
        _DIS_IDLE_EXT_WRTW { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x_ext_wrt(&mut self) -> _VIN_AT_X_EXT_WRTW {
        _VIN_AT_X_EXT_WRTW { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_mode_sel_int_rd(&mut self) -> _STANDBY_MODE_SEL_INT_RDW {
        _STANDBY_MODE_SEL_INT_RDW { w: self }
    }
    #[doc = "Bits 13:14 - 14:13\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_pw_sel_int_rd(&mut self) -> _STANDBY_PW_SEL_INT_RDW {
        _STANDBY_PW_SEL_INT_RDW { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_standby_int_rd(&mut self) -> _DIS_STANDBY_INT_RDW {
        _DIS_STANDBY_INT_RDW { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_idle_int_rd(&mut self) -> _DIS_IDLE_INT_RDW {
        _DIS_IDLE_INT_RDW { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x_int_rd(&mut self) -> _VIN_AT_X_INT_RDW {
        _VIN_AT_X_INT_RDW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_mode_sel_ext_rd(&mut self) -> _STANDBY_MODE_SEL_EXT_RDW {
        _STANDBY_MODE_SEL_EXT_RDW { w: self }
    }
    #[doc = "Bits 5:6 - 6:5\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn standby_pw_sel_ext_rd(&mut self) -> _STANDBY_PW_SEL_EXT_RDW {
        _STANDBY_PW_SEL_EXT_RDW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_standby_ext_rd(&mut self) -> _DIS_STANDBY_EXT_RDW {
        _DIS_STANDBY_EXT_RDW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn dis_idle_ext_rd(&mut self) -> _DIS_IDLE_EXT_RDW {
        _DIS_IDLE_EXT_RDW { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn vin_at_x_ext_rd(&mut self) -> _VIN_AT_X_EXT_RDW {
        _VIN_AT_X_EXT_RDW { w: self }
    }
}
