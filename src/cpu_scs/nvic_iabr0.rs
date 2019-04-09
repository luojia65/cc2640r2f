#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NVIC_IABR0 {
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
pub struct ACTIVE31R {
    bits: bool,
}
impl ACTIVE31R {
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
pub struct ACTIVE30R {
    bits: bool,
}
impl ACTIVE30R {
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
pub struct ACTIVE29R {
    bits: bool,
}
impl ACTIVE29R {
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
pub struct ACTIVE28R {
    bits: bool,
}
impl ACTIVE28R {
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
pub struct ACTIVE27R {
    bits: bool,
}
impl ACTIVE27R {
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
pub struct ACTIVE26R {
    bits: bool,
}
impl ACTIVE26R {
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
pub struct ACTIVE25R {
    bits: bool,
}
impl ACTIVE25R {
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
pub struct ACTIVE24R {
    bits: bool,
}
impl ACTIVE24R {
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
pub struct ACTIVE23R {
    bits: bool,
}
impl ACTIVE23R {
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
pub struct ACTIVE22R {
    bits: bool,
}
impl ACTIVE22R {
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
pub struct ACTIVE21R {
    bits: bool,
}
impl ACTIVE21R {
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
pub struct ACTIVE20R {
    bits: bool,
}
impl ACTIVE20R {
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
pub struct ACTIVE19R {
    bits: bool,
}
impl ACTIVE19R {
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
pub struct ACTIVE18R {
    bits: bool,
}
impl ACTIVE18R {
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
pub struct ACTIVE17R {
    bits: bool,
}
impl ACTIVE17R {
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
pub struct ACTIVE16R {
    bits: bool,
}
impl ACTIVE16R {
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
pub struct ACTIVE15R {
    bits: bool,
}
impl ACTIVE15R {
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
pub struct ACTIVE14R {
    bits: bool,
}
impl ACTIVE14R {
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
pub struct ACTIVE13R {
    bits: bool,
}
impl ACTIVE13R {
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
pub struct ACTIVE12R {
    bits: bool,
}
impl ACTIVE12R {
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
pub struct ACTIVE11R {
    bits: bool,
}
impl ACTIVE11R {
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
pub struct ACTIVE10R {
    bits: bool,
}
impl ACTIVE10R {
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
pub struct ACTIVE9R {
    bits: bool,
}
impl ACTIVE9R {
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
pub struct ACTIVE8R {
    bits: bool,
}
impl ACTIVE8R {
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
pub struct ACTIVE7R {
    bits: bool,
}
impl ACTIVE7R {
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
pub struct ACTIVE6R {
    bits: bool,
}
impl ACTIVE6R {
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
pub struct ACTIVE5R {
    bits: bool,
}
impl ACTIVE5R {
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
pub struct ACTIVE4R {
    bits: bool,
}
impl ACTIVE4R {
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
pub struct ACTIVE3R {
    bits: bool,
}
impl ACTIVE3R {
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
pub struct ACTIVE2R {
    bits: bool,
}
impl ACTIVE2R {
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
pub struct ACTIVE1R {
    bits: bool,
}
impl ACTIVE1R {
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
pub struct ACTIVE0R {
    bits: bool,
}
impl ACTIVE0R {
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
#[doc = r" Proxy"]
pub struct _ACTIVE31W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE31W<'a> {
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
pub struct _ACTIVE30W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE30W<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE29W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE29W<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE28W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE28W<'a> {
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
pub struct _ACTIVE27W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE27W<'a> {
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
pub struct _ACTIVE26W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE26W<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE25W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE25W<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE24W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE24W<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE23W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE23W<'a> {
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
pub struct _ACTIVE22W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE22W<'a> {
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
pub struct _ACTIVE21W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE21W<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE20W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE20W<'a> {
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
pub struct _ACTIVE19W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE19W<'a> {
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
pub struct _ACTIVE18W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE18W<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE17W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE17W<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE16W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE16W<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE15W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE15W<'a> {
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
pub struct _ACTIVE14W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE14W<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE13W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE13W<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE12W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE12W<'a> {
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
pub struct _ACTIVE11W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE11W<'a> {
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
pub struct _ACTIVE10W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE10W<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE9W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE9W<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE8W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE8W<'a> {
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
pub struct _ACTIVE7W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE7W<'a> {
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
pub struct _ACTIVE6W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE6W<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE5W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE5W<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE4W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE4W<'a> {
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
pub struct _ACTIVE3W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE3W<'a> {
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
pub struct _ACTIVE2W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE2W<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE1W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE1W<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVE0W<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVE0W<'a> {
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
    #[doc = "Bit 31 - 31:31\\] Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline]
    pub fn active31(&self) -> ACTIVE31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE31R { bits }
    }
    #[doc = "Bit 30 - 30:30\\] Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline]
    pub fn active30(&self) -> ACTIVE30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE30R { bits }
    }
    #[doc = "Bit 29 - 29:29\\] Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline]
    pub fn active29(&self) -> ACTIVE29R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE29R { bits }
    }
    #[doc = "Bit 28 - 28:28\\] Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline]
    pub fn active28(&self) -> ACTIVE28R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE28R { bits }
    }
    #[doc = "Bit 27 - 27:27\\] Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline]
    pub fn active27(&self) -> ACTIVE27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE27R { bits }
    }
    #[doc = "Bit 26 - 26:26\\] Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline]
    pub fn active26(&self) -> ACTIVE26R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE26R { bits }
    }
    #[doc = "Bit 25 - 25:25\\] Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline]
    pub fn active25(&self) -> ACTIVE25R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE25R { bits }
    }
    #[doc = "Bit 24 - 24:24\\] Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline]
    pub fn active24(&self) -> ACTIVE24R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE24R { bits }
    }
    #[doc = "Bit 23 - 23:23\\] Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline]
    pub fn active23(&self) -> ACTIVE23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE23R { bits }
    }
    #[doc = "Bit 22 - 22:22\\] Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline]
    pub fn active22(&self) -> ACTIVE22R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE22R { bits }
    }
    #[doc = "Bit 21 - 21:21\\] Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline]
    pub fn active21(&self) -> ACTIVE21R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE21R { bits }
    }
    #[doc = "Bit 20 - 20:20\\] Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline]
    pub fn active20(&self) -> ACTIVE20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE20R { bits }
    }
    #[doc = "Bit 19 - 19:19\\] Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline]
    pub fn active19(&self) -> ACTIVE19R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE19R { bits }
    }
    #[doc = "Bit 18 - 18:18\\] Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline]
    pub fn active18(&self) -> ACTIVE18R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE18R { bits }
    }
    #[doc = "Bit 17 - 17:17\\] Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline]
    pub fn active17(&self) -> ACTIVE17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE17R { bits }
    }
    #[doc = "Bit 16 - 16:16\\] Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline]
    pub fn active16(&self) -> ACTIVE16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE16R { bits }
    }
    #[doc = "Bit 15 - 15:15\\] Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline]
    pub fn active15(&self) -> ACTIVE15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE15R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline]
    pub fn active14(&self) -> ACTIVE14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE14R { bits }
    }
    #[doc = "Bit 13 - 13:13\\] Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline]
    pub fn active13(&self) -> ACTIVE13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE13R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline]
    pub fn active12(&self) -> ACTIVE12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE12R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline]
    pub fn active11(&self) -> ACTIVE11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE11R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline]
    pub fn active10(&self) -> ACTIVE10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE10R { bits }
    }
    #[doc = "Bit 9 - 9:9\\] Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline]
    pub fn active9(&self) -> ACTIVE9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE9R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline]
    pub fn active8(&self) -> ACTIVE8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE8R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline]
    pub fn active7(&self) -> ACTIVE7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE7R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline]
    pub fn active6(&self) -> ACTIVE6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE6R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline]
    pub fn active5(&self) -> ACTIVE5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE5R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline]
    pub fn active4(&self) -> ACTIVE4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE4R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
    #[inline]
    pub fn active3(&self) -> ACTIVE3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
    #[inline]
    pub fn active2(&self) -> ACTIVE2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
    #[inline]
    pub fn active1(&self) -> ACTIVE1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE1R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
    #[inline]
    pub fn active0(&self) -> ACTIVE0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACTIVE0R { bits }
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
    #[doc = "Bit 31 - 31:31\\] Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline]
    pub fn active31(&mut self) -> _ACTIVE31W {
        _ACTIVE31W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline]
    pub fn active30(&mut self) -> _ACTIVE30W {
        _ACTIVE30W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline]
    pub fn active29(&mut self) -> _ACTIVE29W {
        _ACTIVE29W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline]
    pub fn active28(&mut self) -> _ACTIVE28W {
        _ACTIVE28W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline]
    pub fn active27(&mut self) -> _ACTIVE27W {
        _ACTIVE27W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline]
    pub fn active26(&mut self) -> _ACTIVE26W {
        _ACTIVE26W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline]
    pub fn active25(&mut self) -> _ACTIVE25W {
        _ACTIVE25W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline]
    pub fn active24(&mut self) -> _ACTIVE24W {
        _ACTIVE24W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline]
    pub fn active23(&mut self) -> _ACTIVE23W {
        _ACTIVE23W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline]
    pub fn active22(&mut self) -> _ACTIVE22W {
        _ACTIVE22W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline]
    pub fn active21(&mut self) -> _ACTIVE21W {
        _ACTIVE21W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline]
    pub fn active20(&mut self) -> _ACTIVE20W {
        _ACTIVE20W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline]
    pub fn active19(&mut self) -> _ACTIVE19W {
        _ACTIVE19W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline]
    pub fn active18(&mut self) -> _ACTIVE18W {
        _ACTIVE18W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline]
    pub fn active17(&mut self) -> _ACTIVE17W {
        _ACTIVE17W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline]
    pub fn active16(&mut self) -> _ACTIVE16W {
        _ACTIVE16W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline]
    pub fn active15(&mut self) -> _ACTIVE15W {
        _ACTIVE15W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline]
    pub fn active14(&mut self) -> _ACTIVE14W {
        _ACTIVE14W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline]
    pub fn active13(&mut self) -> _ACTIVE13W {
        _ACTIVE13W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline]
    pub fn active12(&mut self) -> _ACTIVE12W {
        _ACTIVE12W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline]
    pub fn active11(&mut self) -> _ACTIVE11W {
        _ACTIVE11W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline]
    pub fn active10(&mut self) -> _ACTIVE10W {
        _ACTIVE10W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline]
    pub fn active9(&mut self) -> _ACTIVE9W {
        _ACTIVE9W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline]
    pub fn active8(&mut self) -> _ACTIVE8W {
        _ACTIVE8W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline]
    pub fn active7(&mut self) -> _ACTIVE7W {
        _ACTIVE7W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline]
    pub fn active6(&mut self) -> _ACTIVE6W {
        _ACTIVE6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline]
    pub fn active5(&mut self) -> _ACTIVE5W {
        _ACTIVE5W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline]
    pub fn active4(&mut self) -> _ACTIVE4W {
        _ACTIVE4W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
    #[inline]
    pub fn active3(&mut self) -> _ACTIVE3W {
        _ACTIVE3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
    #[inline]
    pub fn active2(&mut self) -> _ACTIVE2W {
        _ACTIVE2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
    #[inline]
    pub fn active1(&mut self) -> _ACTIVE1W {
        _ACTIVE1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
    #[inline]
    pub fn active0(&mut self) -> _ACTIVE0W {
        _ACTIVE0W { w: self }
    }
}
