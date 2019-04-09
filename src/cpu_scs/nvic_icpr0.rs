#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NVIC_ICPR0 {
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
pub struct CLRPEND31R {
    bits: bool,
}
impl CLRPEND31R {
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
pub struct CLRPEND30R {
    bits: bool,
}
impl CLRPEND30R {
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
pub struct CLRPEND29R {
    bits: bool,
}
impl CLRPEND29R {
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
pub struct CLRPEND28R {
    bits: bool,
}
impl CLRPEND28R {
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
pub struct CLRPEND27R {
    bits: bool,
}
impl CLRPEND27R {
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
pub struct CLRPEND26R {
    bits: bool,
}
impl CLRPEND26R {
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
pub struct CLRPEND25R {
    bits: bool,
}
impl CLRPEND25R {
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
pub struct CLRPEND24R {
    bits: bool,
}
impl CLRPEND24R {
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
pub struct CLRPEND23R {
    bits: bool,
}
impl CLRPEND23R {
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
pub struct CLRPEND22R {
    bits: bool,
}
impl CLRPEND22R {
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
pub struct CLRPEND21R {
    bits: bool,
}
impl CLRPEND21R {
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
pub struct CLRPEND20R {
    bits: bool,
}
impl CLRPEND20R {
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
pub struct CLRPEND19R {
    bits: bool,
}
impl CLRPEND19R {
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
pub struct CLRPEND18R {
    bits: bool,
}
impl CLRPEND18R {
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
pub struct CLRPEND17R {
    bits: bool,
}
impl CLRPEND17R {
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
pub struct CLRPEND16R {
    bits: bool,
}
impl CLRPEND16R {
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
pub struct CLRPEND15R {
    bits: bool,
}
impl CLRPEND15R {
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
pub struct CLRPEND14R {
    bits: bool,
}
impl CLRPEND14R {
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
pub struct CLRPEND13R {
    bits: bool,
}
impl CLRPEND13R {
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
pub struct CLRPEND12R {
    bits: bool,
}
impl CLRPEND12R {
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
pub struct CLRPEND11R {
    bits: bool,
}
impl CLRPEND11R {
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
pub struct CLRPEND10R {
    bits: bool,
}
impl CLRPEND10R {
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
pub struct CLRPEND9R {
    bits: bool,
}
impl CLRPEND9R {
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
pub struct CLRPEND8R {
    bits: bool,
}
impl CLRPEND8R {
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
pub struct CLRPEND7R {
    bits: bool,
}
impl CLRPEND7R {
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
pub struct CLRPEND6R {
    bits: bool,
}
impl CLRPEND6R {
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
pub struct CLRPEND5R {
    bits: bool,
}
impl CLRPEND5R {
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
pub struct CLRPEND4R {
    bits: bool,
}
impl CLRPEND4R {
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
pub struct CLRPEND3R {
    bits: bool,
}
impl CLRPEND3R {
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
pub struct CLRPEND2R {
    bits: bool,
}
impl CLRPEND2R {
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
pub struct CLRPEND1R {
    bits: bool,
}
impl CLRPEND1R {
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
pub struct CLRPEND0R {
    bits: bool,
}
impl CLRPEND0R {
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
pub struct _CLRPEND31W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND31W<'a> {
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
pub struct _CLRPEND30W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND30W<'a> {
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
pub struct _CLRPEND29W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND29W<'a> {
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
pub struct _CLRPEND28W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND28W<'a> {
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
pub struct _CLRPEND27W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND27W<'a> {
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
pub struct _CLRPEND26W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND26W<'a> {
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
pub struct _CLRPEND25W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND25W<'a> {
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
pub struct _CLRPEND24W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND24W<'a> {
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
pub struct _CLRPEND23W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND23W<'a> {
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
pub struct _CLRPEND22W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND22W<'a> {
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
pub struct _CLRPEND21W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND21W<'a> {
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
pub struct _CLRPEND20W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND20W<'a> {
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
pub struct _CLRPEND19W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND19W<'a> {
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
pub struct _CLRPEND18W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND18W<'a> {
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
pub struct _CLRPEND17W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND17W<'a> {
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
pub struct _CLRPEND16W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND16W<'a> {
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
pub struct _CLRPEND15W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND15W<'a> {
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
pub struct _CLRPEND14W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND14W<'a> {
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
pub struct _CLRPEND13W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND13W<'a> {
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
pub struct _CLRPEND12W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND12W<'a> {
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
pub struct _CLRPEND11W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND11W<'a> {
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
pub struct _CLRPEND10W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND10W<'a> {
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
pub struct _CLRPEND9W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND9W<'a> {
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
pub struct _CLRPEND8W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND8W<'a> {
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
pub struct _CLRPEND7W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND7W<'a> {
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
pub struct _CLRPEND6W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND6W<'a> {
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
pub struct _CLRPEND5W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND5W<'a> {
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
pub struct _CLRPEND4W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND4W<'a> {
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
pub struct _CLRPEND3W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND3W<'a> {
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
pub struct _CLRPEND2W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND2W<'a> {
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
pub struct _CLRPEND1W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND1W<'a> {
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
pub struct _CLRPEND0W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRPEND0W<'a> {
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
    #[doc = "Bit 31 - 31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend31(&self) -> CLRPEND31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND31R { bits }
    }
    #[doc = "Bit 30 - 30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend30(&self) -> CLRPEND30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND30R { bits }
    }
    #[doc = "Bit 29 - 29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend29(&self) -> CLRPEND29R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND29R { bits }
    }
    #[doc = "Bit 28 - 28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend28(&self) -> CLRPEND28R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND28R { bits }
    }
    #[doc = "Bit 27 - 27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend27(&self) -> CLRPEND27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND27R { bits }
    }
    #[doc = "Bit 26 - 26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend26(&self) -> CLRPEND26R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND26R { bits }
    }
    #[doc = "Bit 25 - 25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend25(&self) -> CLRPEND25R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND25R { bits }
    }
    #[doc = "Bit 24 - 24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend24(&self) -> CLRPEND24R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND24R { bits }
    }
    #[doc = "Bit 23 - 23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend23(&self) -> CLRPEND23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND23R { bits }
    }
    #[doc = "Bit 22 - 22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend22(&self) -> CLRPEND22R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND22R { bits }
    }
    #[doc = "Bit 21 - 21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend21(&self) -> CLRPEND21R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND21R { bits }
    }
    #[doc = "Bit 20 - 20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend20(&self) -> CLRPEND20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND20R { bits }
    }
    #[doc = "Bit 19 - 19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend19(&self) -> CLRPEND19R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND19R { bits }
    }
    #[doc = "Bit 18 - 18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend18(&self) -> CLRPEND18R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND18R { bits }
    }
    #[doc = "Bit 17 - 17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend17(&self) -> CLRPEND17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND17R { bits }
    }
    #[doc = "Bit 16 - 16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend16(&self) -> CLRPEND16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND16R { bits }
    }
    #[doc = "Bit 15 - 15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend15(&self) -> CLRPEND15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND15R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend14(&self) -> CLRPEND14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND14R { bits }
    }
    #[doc = "Bit 13 - 13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend13(&self) -> CLRPEND13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND13R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend12(&self) -> CLRPEND12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND12R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend11(&self) -> CLRPEND11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND11R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend10(&self) -> CLRPEND10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND10R { bits }
    }
    #[doc = "Bit 9 - 9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend9(&self) -> CLRPEND9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND9R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend8(&self) -> CLRPEND8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND8R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend7(&self) -> CLRPEND7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND7R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend6(&self) -> CLRPEND6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND6R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend5(&self) -> CLRPEND5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND5R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend4(&self) -> CLRPEND4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND4R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend3(&self) -> CLRPEND3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend2(&self) -> CLRPEND2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend1(&self) -> CLRPEND1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND1R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend0(&self) -> CLRPEND0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRPEND0R { bits }
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
    #[doc = "Bit 31 - 31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend31(&mut self) -> _CLRPEND31W {
        _CLRPEND31W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend30(&mut self) -> _CLRPEND30W {
        _CLRPEND30W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend29(&mut self) -> _CLRPEND29W {
        _CLRPEND29W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend28(&mut self) -> _CLRPEND28W {
        _CLRPEND28W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend27(&mut self) -> _CLRPEND27W {
        _CLRPEND27W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend26(&mut self) -> _CLRPEND26W {
        _CLRPEND26W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend25(&mut self) -> _CLRPEND25W {
        _CLRPEND25W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend24(&mut self) -> _CLRPEND24W {
        _CLRPEND24W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend23(&mut self) -> _CLRPEND23W {
        _CLRPEND23W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend22(&mut self) -> _CLRPEND22W {
        _CLRPEND22W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend21(&mut self) -> _CLRPEND21W {
        _CLRPEND21W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend20(&mut self) -> _CLRPEND20W {
        _CLRPEND20W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend19(&mut self) -> _CLRPEND19W {
        _CLRPEND19W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend18(&mut self) -> _CLRPEND18W {
        _CLRPEND18W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend17(&mut self) -> _CLRPEND17W {
        _CLRPEND17W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend16(&mut self) -> _CLRPEND16W {
        _CLRPEND16W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend15(&mut self) -> _CLRPEND15W {
        _CLRPEND15W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend14(&mut self) -> _CLRPEND14W {
        _CLRPEND14W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend13(&mut self) -> _CLRPEND13W {
        _CLRPEND13W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend12(&mut self) -> _CLRPEND12W {
        _CLRPEND12W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend11(&mut self) -> _CLRPEND11W {
        _CLRPEND11W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend10(&mut self) -> _CLRPEND10W {
        _CLRPEND10W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend9(&mut self) -> _CLRPEND9W {
        _CLRPEND9W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend8(&mut self) -> _CLRPEND8W {
        _CLRPEND8W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend7(&mut self) -> _CLRPEND7W {
        _CLRPEND7W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend6(&mut self) -> _CLRPEND6W {
        _CLRPEND6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend5(&mut self) -> _CLRPEND5W {
        _CLRPEND5W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend4(&mut self) -> _CLRPEND4W {
        _CLRPEND4W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend3(&mut self) -> _CLRPEND3W {
        _CLRPEND3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend2(&mut self) -> _CLRPEND2W {
        _CLRPEND2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend1(&mut self) -> _CLRPEND1W {
        _CLRPEND1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline]
    pub fn clrpend0(&mut self) -> _CLRPEND0W {
        _CLRPEND0W { w: self }
    }
}
