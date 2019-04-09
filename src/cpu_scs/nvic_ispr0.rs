#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NVIC_ISPR0 {
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
pub struct SETPEND31R {
    bits: bool,
}
impl SETPEND31R {
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
pub struct SETPEND30R {
    bits: bool,
}
impl SETPEND30R {
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
pub struct SETPEND29R {
    bits: bool,
}
impl SETPEND29R {
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
pub struct SETPEND28R {
    bits: bool,
}
impl SETPEND28R {
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
pub struct SETPEND27R {
    bits: bool,
}
impl SETPEND27R {
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
pub struct SETPEND26R {
    bits: bool,
}
impl SETPEND26R {
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
pub struct SETPEND25R {
    bits: bool,
}
impl SETPEND25R {
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
pub struct SETPEND24R {
    bits: bool,
}
impl SETPEND24R {
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
pub struct SETPEND23R {
    bits: bool,
}
impl SETPEND23R {
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
pub struct SETPEND22R {
    bits: bool,
}
impl SETPEND22R {
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
pub struct SETPEND21R {
    bits: bool,
}
impl SETPEND21R {
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
pub struct SETPEND20R {
    bits: bool,
}
impl SETPEND20R {
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
pub struct SETPEND19R {
    bits: bool,
}
impl SETPEND19R {
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
pub struct SETPEND18R {
    bits: bool,
}
impl SETPEND18R {
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
pub struct SETPEND17R {
    bits: bool,
}
impl SETPEND17R {
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
pub struct SETPEND16R {
    bits: bool,
}
impl SETPEND16R {
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
pub struct SETPEND15R {
    bits: bool,
}
impl SETPEND15R {
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
pub struct SETPEND14R {
    bits: bool,
}
impl SETPEND14R {
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
pub struct SETPEND13R {
    bits: bool,
}
impl SETPEND13R {
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
pub struct SETPEND12R {
    bits: bool,
}
impl SETPEND12R {
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
pub struct SETPEND11R {
    bits: bool,
}
impl SETPEND11R {
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
pub struct SETPEND10R {
    bits: bool,
}
impl SETPEND10R {
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
pub struct SETPEND9R {
    bits: bool,
}
impl SETPEND9R {
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
pub struct SETPEND8R {
    bits: bool,
}
impl SETPEND8R {
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
pub struct SETPEND7R {
    bits: bool,
}
impl SETPEND7R {
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
pub struct SETPEND6R {
    bits: bool,
}
impl SETPEND6R {
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
pub struct SETPEND5R {
    bits: bool,
}
impl SETPEND5R {
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
pub struct SETPEND4R {
    bits: bool,
}
impl SETPEND4R {
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
pub struct SETPEND3R {
    bits: bool,
}
impl SETPEND3R {
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
pub struct SETPEND2R {
    bits: bool,
}
impl SETPEND2R {
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
pub struct SETPEND1R {
    bits: bool,
}
impl SETPEND1R {
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
pub struct SETPEND0R {
    bits: bool,
}
impl SETPEND0R {
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
pub struct _SETPEND31W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND31W<'a> {
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
pub struct _SETPEND30W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND30W<'a> {
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
pub struct _SETPEND29W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND29W<'a> {
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
pub struct _SETPEND28W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND28W<'a> {
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
pub struct _SETPEND27W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND27W<'a> {
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
pub struct _SETPEND26W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND26W<'a> {
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
pub struct _SETPEND25W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND25W<'a> {
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
pub struct _SETPEND24W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND24W<'a> {
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
pub struct _SETPEND23W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND23W<'a> {
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
pub struct _SETPEND22W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND22W<'a> {
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
pub struct _SETPEND21W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND21W<'a> {
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
pub struct _SETPEND20W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND20W<'a> {
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
pub struct _SETPEND19W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND19W<'a> {
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
pub struct _SETPEND18W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND18W<'a> {
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
pub struct _SETPEND17W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND17W<'a> {
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
pub struct _SETPEND16W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND16W<'a> {
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
pub struct _SETPEND15W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND15W<'a> {
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
pub struct _SETPEND14W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND14W<'a> {
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
pub struct _SETPEND13W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND13W<'a> {
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
pub struct _SETPEND12W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND12W<'a> {
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
pub struct _SETPEND11W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND11W<'a> {
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
pub struct _SETPEND10W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND10W<'a> {
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
pub struct _SETPEND9W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND9W<'a> {
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
pub struct _SETPEND8W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND8W<'a> {
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
pub struct _SETPEND7W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND7W<'a> {
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
pub struct _SETPEND6W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND6W<'a> {
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
pub struct _SETPEND5W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND5W<'a> {
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
pub struct _SETPEND4W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND4W<'a> {
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
pub struct _SETPEND3W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND3W<'a> {
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
pub struct _SETPEND2W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND2W<'a> {
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
pub struct _SETPEND1W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND1W<'a> {
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
pub struct _SETPEND0W<'a> {
    w: &'a mut W,
}
impl<'a> _SETPEND0W<'a> {
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
    #[doc = "Bit 31 - 31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend31(&self) -> SETPEND31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND31R { bits }
    }
    #[doc = "Bit 30 - 30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend30(&self) -> SETPEND30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND30R { bits }
    }
    #[doc = "Bit 29 - 29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend29(&self) -> SETPEND29R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND29R { bits }
    }
    #[doc = "Bit 28 - 28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend28(&self) -> SETPEND28R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND28R { bits }
    }
    #[doc = "Bit 27 - 27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend27(&self) -> SETPEND27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND27R { bits }
    }
    #[doc = "Bit 26 - 26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend26(&self) -> SETPEND26R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND26R { bits }
    }
    #[doc = "Bit 25 - 25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend25(&self) -> SETPEND25R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND25R { bits }
    }
    #[doc = "Bit 24 - 24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend24(&self) -> SETPEND24R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND24R { bits }
    }
    #[doc = "Bit 23 - 23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend23(&self) -> SETPEND23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND23R { bits }
    }
    #[doc = "Bit 22 - 22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend22(&self) -> SETPEND22R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND22R { bits }
    }
    #[doc = "Bit 21 - 21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend21(&self) -> SETPEND21R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND21R { bits }
    }
    #[doc = "Bit 20 - 20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend20(&self) -> SETPEND20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND20R { bits }
    }
    #[doc = "Bit 19 - 19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend19(&self) -> SETPEND19R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND19R { bits }
    }
    #[doc = "Bit 18 - 18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend18(&self) -> SETPEND18R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND18R { bits }
    }
    #[doc = "Bit 17 - 17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend17(&self) -> SETPEND17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND17R { bits }
    }
    #[doc = "Bit 16 - 16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend16(&self) -> SETPEND16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND16R { bits }
    }
    #[doc = "Bit 15 - 15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend15(&self) -> SETPEND15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND15R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend14(&self) -> SETPEND14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND14R { bits }
    }
    #[doc = "Bit 13 - 13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend13(&self) -> SETPEND13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND13R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend12(&self) -> SETPEND12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND12R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend11(&self) -> SETPEND11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND11R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend10(&self) -> SETPEND10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND10R { bits }
    }
    #[doc = "Bit 9 - 9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend9(&self) -> SETPEND9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND9R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend8(&self) -> SETPEND8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND8R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend7(&self) -> SETPEND7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND7R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend6(&self) -> SETPEND6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND6R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend5(&self) -> SETPEND5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND5R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend4(&self) -> SETPEND4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND4R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend3(&self) -> SETPEND3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend2(&self) -> SETPEND2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend1(&self) -> SETPEND1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND1R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend0(&self) -> SETPEND0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETPEND0R { bits }
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
    #[doc = "Bit 31 - 31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend31(&mut self) -> _SETPEND31W {
        _SETPEND31W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend30(&mut self) -> _SETPEND30W {
        _SETPEND30W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend29(&mut self) -> _SETPEND29W {
        _SETPEND29W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend28(&mut self) -> _SETPEND28W {
        _SETPEND28W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend27(&mut self) -> _SETPEND27W {
        _SETPEND27W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend26(&mut self) -> _SETPEND26W {
        _SETPEND26W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend25(&mut self) -> _SETPEND25W {
        _SETPEND25W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend24(&mut self) -> _SETPEND24W {
        _SETPEND24W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend23(&mut self) -> _SETPEND23W {
        _SETPEND23W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend22(&mut self) -> _SETPEND22W {
        _SETPEND22W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend21(&mut self) -> _SETPEND21W {
        _SETPEND21W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend20(&mut self) -> _SETPEND20W {
        _SETPEND20W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend19(&mut self) -> _SETPEND19W {
        _SETPEND19W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend18(&mut self) -> _SETPEND18W {
        _SETPEND18W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend17(&mut self) -> _SETPEND17W {
        _SETPEND17W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend16(&mut self) -> _SETPEND16W {
        _SETPEND16W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend15(&mut self) -> _SETPEND15W {
        _SETPEND15W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend14(&mut self) -> _SETPEND14W {
        _SETPEND14W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend13(&mut self) -> _SETPEND13W {
        _SETPEND13W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend12(&mut self) -> _SETPEND12W {
        _SETPEND12W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend11(&mut self) -> _SETPEND11W {
        _SETPEND11W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend10(&mut self) -> _SETPEND10W {
        _SETPEND10W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend9(&mut self) -> _SETPEND9W {
        _SETPEND9W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend8(&mut self) -> _SETPEND8W {
        _SETPEND8W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend7(&mut self) -> _SETPEND7W {
        _SETPEND7W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend6(&mut self) -> _SETPEND6W {
        _SETPEND6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend5(&mut self) -> _SETPEND5W {
        _SETPEND5W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend4(&mut self) -> _SETPEND4W {
        _SETPEND4W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend3(&mut self) -> _SETPEND3W {
        _SETPEND3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend2(&mut self) -> _SETPEND2W {
        _SETPEND2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend1(&mut self) -> _SETPEND1W {
        _SETPEND1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn setpend0(&mut self) -> _SETPEND0W {
        _SETPEND0W { w: self }
    }
}
