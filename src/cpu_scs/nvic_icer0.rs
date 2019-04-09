#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NVIC_ICER0 {
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
pub struct CLRENA31R {
    bits: bool,
}
impl CLRENA31R {
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
pub struct CLRENA30R {
    bits: bool,
}
impl CLRENA30R {
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
pub struct CLRENA29R {
    bits: bool,
}
impl CLRENA29R {
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
pub struct CLRENA28R {
    bits: bool,
}
impl CLRENA28R {
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
pub struct CLRENA27R {
    bits: bool,
}
impl CLRENA27R {
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
pub struct CLRENA26R {
    bits: bool,
}
impl CLRENA26R {
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
pub struct CLRENA25R {
    bits: bool,
}
impl CLRENA25R {
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
pub struct CLRENA24R {
    bits: bool,
}
impl CLRENA24R {
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
pub struct CLRENA23R {
    bits: bool,
}
impl CLRENA23R {
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
pub struct CLRENA22R {
    bits: bool,
}
impl CLRENA22R {
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
pub struct CLRENA21R {
    bits: bool,
}
impl CLRENA21R {
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
pub struct CLRENA20R {
    bits: bool,
}
impl CLRENA20R {
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
pub struct CLRENA19R {
    bits: bool,
}
impl CLRENA19R {
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
pub struct CLRENA18R {
    bits: bool,
}
impl CLRENA18R {
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
pub struct CLRENA17R {
    bits: bool,
}
impl CLRENA17R {
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
pub struct CLRENA16R {
    bits: bool,
}
impl CLRENA16R {
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
pub struct CLRENA15R {
    bits: bool,
}
impl CLRENA15R {
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
pub struct CLRENA14R {
    bits: bool,
}
impl CLRENA14R {
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
pub struct CLRENA13R {
    bits: bool,
}
impl CLRENA13R {
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
pub struct CLRENA12R {
    bits: bool,
}
impl CLRENA12R {
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
pub struct CLRENA11R {
    bits: bool,
}
impl CLRENA11R {
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
pub struct CLRENA10R {
    bits: bool,
}
impl CLRENA10R {
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
pub struct CLRENA9R {
    bits: bool,
}
impl CLRENA9R {
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
pub struct CLRENA8R {
    bits: bool,
}
impl CLRENA8R {
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
pub struct CLRENA7R {
    bits: bool,
}
impl CLRENA7R {
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
pub struct CLRENA6R {
    bits: bool,
}
impl CLRENA6R {
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
pub struct CLRENA5R {
    bits: bool,
}
impl CLRENA5R {
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
pub struct CLRENA4R {
    bits: bool,
}
impl CLRENA4R {
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
pub struct CLRENA3R {
    bits: bool,
}
impl CLRENA3R {
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
pub struct CLRENA2R {
    bits: bool,
}
impl CLRENA2R {
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
pub struct CLRENA1R {
    bits: bool,
}
impl CLRENA1R {
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
pub struct CLRENA0R {
    bits: bool,
}
impl CLRENA0R {
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
pub struct _CLRENA31W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA31W<'a> {
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
pub struct _CLRENA30W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA30W<'a> {
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
pub struct _CLRENA29W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA29W<'a> {
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
pub struct _CLRENA28W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA28W<'a> {
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
pub struct _CLRENA27W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA27W<'a> {
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
pub struct _CLRENA26W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA26W<'a> {
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
pub struct _CLRENA25W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA25W<'a> {
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
pub struct _CLRENA24W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA24W<'a> {
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
pub struct _CLRENA23W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA23W<'a> {
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
pub struct _CLRENA22W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA22W<'a> {
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
pub struct _CLRENA21W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA21W<'a> {
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
pub struct _CLRENA20W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA20W<'a> {
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
pub struct _CLRENA19W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA19W<'a> {
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
pub struct _CLRENA18W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA18W<'a> {
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
pub struct _CLRENA17W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA17W<'a> {
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
pub struct _CLRENA16W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA16W<'a> {
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
pub struct _CLRENA15W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA15W<'a> {
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
pub struct _CLRENA14W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA14W<'a> {
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
pub struct _CLRENA13W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA13W<'a> {
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
pub struct _CLRENA12W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA12W<'a> {
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
pub struct _CLRENA11W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA11W<'a> {
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
pub struct _CLRENA10W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA10W<'a> {
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
pub struct _CLRENA9W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA9W<'a> {
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
pub struct _CLRENA8W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA8W<'a> {
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
pub struct _CLRENA7W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA7W<'a> {
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
pub struct _CLRENA6W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA6W<'a> {
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
pub struct _CLRENA5W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA5W<'a> {
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
pub struct _CLRENA4W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA4W<'a> {
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
pub struct _CLRENA3W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA3W<'a> {
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
pub struct _CLRENA2W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA2W<'a> {
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
pub struct _CLRENA1W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA1W<'a> {
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
pub struct _CLRENA0W<'a> {
    w: &'a mut W,
}
impl<'a> _CLRENA0W<'a> {
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
    #[doc = "Bit 31 - 31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena31(&self) -> CLRENA31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA31R { bits }
    }
    #[doc = "Bit 30 - 30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena30(&self) -> CLRENA30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA30R { bits }
    }
    #[doc = "Bit 29 - 29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena29(&self) -> CLRENA29R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA29R { bits }
    }
    #[doc = "Bit 28 - 28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena28(&self) -> CLRENA28R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA28R { bits }
    }
    #[doc = "Bit 27 - 27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena27(&self) -> CLRENA27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA27R { bits }
    }
    #[doc = "Bit 26 - 26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena26(&self) -> CLRENA26R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA26R { bits }
    }
    #[doc = "Bit 25 - 25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena25(&self) -> CLRENA25R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA25R { bits }
    }
    #[doc = "Bit 24 - 24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena24(&self) -> CLRENA24R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA24R { bits }
    }
    #[doc = "Bit 23 - 23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena23(&self) -> CLRENA23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA23R { bits }
    }
    #[doc = "Bit 22 - 22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena22(&self) -> CLRENA22R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA22R { bits }
    }
    #[doc = "Bit 21 - 21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena21(&self) -> CLRENA21R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA21R { bits }
    }
    #[doc = "Bit 20 - 20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena20(&self) -> CLRENA20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA20R { bits }
    }
    #[doc = "Bit 19 - 19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena19(&self) -> CLRENA19R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA19R { bits }
    }
    #[doc = "Bit 18 - 18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena18(&self) -> CLRENA18R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA18R { bits }
    }
    #[doc = "Bit 17 - 17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena17(&self) -> CLRENA17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA17R { bits }
    }
    #[doc = "Bit 16 - 16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena16(&self) -> CLRENA16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA16R { bits }
    }
    #[doc = "Bit 15 - 15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena15(&self) -> CLRENA15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA15R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena14(&self) -> CLRENA14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA14R { bits }
    }
    #[doc = "Bit 13 - 13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena13(&self) -> CLRENA13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA13R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena12(&self) -> CLRENA12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA12R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena11(&self) -> CLRENA11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA11R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena10(&self) -> CLRENA10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA10R { bits }
    }
    #[doc = "Bit 9 - 9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena9(&self) -> CLRENA9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA9R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena8(&self) -> CLRENA8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA8R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena7(&self) -> CLRENA7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA7R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena6(&self) -> CLRENA6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA6R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena5(&self) -> CLRENA5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA5R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena4(&self) -> CLRENA4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA4R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena3(&self) -> CLRENA3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena2(&self) -> CLRENA2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena1(&self) -> CLRENA1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA1R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena0(&self) -> CLRENA0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLRENA0R { bits }
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
    #[doc = "Bit 31 - 31:31\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena31(&mut self) -> _CLRENA31W {
        _CLRENA31W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena30(&mut self) -> _CLRENA30W {
        _CLRENA30W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena29(&mut self) -> _CLRENA29W {
        _CLRENA29W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena28(&mut self) -> _CLRENA28W {
        _CLRENA28W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena27(&mut self) -> _CLRENA27W {
        _CLRENA27W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena26(&mut self) -> _CLRENA26W {
        _CLRENA26W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena25(&mut self) -> _CLRENA25W {
        _CLRENA25W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena24(&mut self) -> _CLRENA24W {
        _CLRENA24W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena23(&mut self) -> _CLRENA23W {
        _CLRENA23W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena22(&mut self) -> _CLRENA22W {
        _CLRENA22W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena21(&mut self) -> _CLRENA21W {
        _CLRENA21W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena20(&mut self) -> _CLRENA20W {
        _CLRENA20W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena19(&mut self) -> _CLRENA19W {
        _CLRENA19W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena18(&mut self) -> _CLRENA18W {
        _CLRENA18W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena17(&mut self) -> _CLRENA17W {
        _CLRENA17W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena16(&mut self) -> _CLRENA16W {
        _CLRENA16W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena15(&mut self) -> _CLRENA15W {
        _CLRENA15W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena14(&mut self) -> _CLRENA14W {
        _CLRENA14W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena13(&mut self) -> _CLRENA13W {
        _CLRENA13W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena12(&mut self) -> _CLRENA12W {
        _CLRENA12W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena11(&mut self) -> _CLRENA11W {
        _CLRENA11W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena10(&mut self) -> _CLRENA10W {
        _CLRENA10W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena9(&mut self) -> _CLRENA9W {
        _CLRENA9W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena8(&mut self) -> _CLRENA8W {
        _CLRENA8W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena7(&mut self) -> _CLRENA7W {
        _CLRENA7W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena6(&mut self) -> _CLRENA6W {
        _CLRENA6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena5(&mut self) -> _CLRENA5W {
        _CLRENA5W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena4(&mut self) -> _CLRENA4W {
        _CLRENA4W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena3(&mut self) -> _CLRENA3W {
        _CLRENA3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena2(&mut self) -> _CLRENA2W {
        _CLRENA2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena1(&mut self) -> _CLRENA1W {
        _CLRENA1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline]
    pub fn clrena0(&mut self) -> _CLRENA0W {
        _CLRENA0W { w: self }
    }
}
