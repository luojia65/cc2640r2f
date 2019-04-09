#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCFG_PROT_31_0 {
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
pub struct WRT_PROT_SEC_31R {
    bits: bool,
}
impl WRT_PROT_SEC_31R {
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
pub struct WRT_PROT_SEC_30R {
    bits: bool,
}
impl WRT_PROT_SEC_30R {
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
pub struct WRT_PROT_SEC_29R {
    bits: bool,
}
impl WRT_PROT_SEC_29R {
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
pub struct WRT_PROT_SEC_28R {
    bits: bool,
}
impl WRT_PROT_SEC_28R {
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
pub struct WRT_PROT_SEC_27R {
    bits: bool,
}
impl WRT_PROT_SEC_27R {
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
pub struct WRT_PROT_SEC_26R {
    bits: bool,
}
impl WRT_PROT_SEC_26R {
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
pub struct WRT_PROT_SEC_25R {
    bits: bool,
}
impl WRT_PROT_SEC_25R {
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
pub struct WRT_PROT_SEC_24R {
    bits: bool,
}
impl WRT_PROT_SEC_24R {
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
pub struct WRT_PROT_SEC_23R {
    bits: bool,
}
impl WRT_PROT_SEC_23R {
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
pub struct WRT_PROT_SEC_22R {
    bits: bool,
}
impl WRT_PROT_SEC_22R {
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
pub struct WRT_PROT_SEC_21R {
    bits: bool,
}
impl WRT_PROT_SEC_21R {
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
pub struct WRT_PROT_SEC_20R {
    bits: bool,
}
impl WRT_PROT_SEC_20R {
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
pub struct WRT_PROT_SEC_19R {
    bits: bool,
}
impl WRT_PROT_SEC_19R {
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
pub struct WRT_PROT_SEC_18R {
    bits: bool,
}
impl WRT_PROT_SEC_18R {
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
pub struct WRT_PROT_SEC_17R {
    bits: bool,
}
impl WRT_PROT_SEC_17R {
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
pub struct WRT_PROT_SEC_16R {
    bits: bool,
}
impl WRT_PROT_SEC_16R {
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
pub struct WRT_PROT_SEC_15R {
    bits: bool,
}
impl WRT_PROT_SEC_15R {
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
pub struct WRT_PROT_SEC_14R {
    bits: bool,
}
impl WRT_PROT_SEC_14R {
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
pub struct WRT_PROT_SEC_13R {
    bits: bool,
}
impl WRT_PROT_SEC_13R {
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
pub struct WRT_PROT_SEC_12R {
    bits: bool,
}
impl WRT_PROT_SEC_12R {
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
pub struct WRT_PROT_SEC_11R {
    bits: bool,
}
impl WRT_PROT_SEC_11R {
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
pub struct WRT_PROT_SEC_10R {
    bits: bool,
}
impl WRT_PROT_SEC_10R {
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
pub struct WRT_PROT_SEC_9R {
    bits: bool,
}
impl WRT_PROT_SEC_9R {
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
pub struct WRT_PROT_SEC_8R {
    bits: bool,
}
impl WRT_PROT_SEC_8R {
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
pub struct WRT_PROT_SEC_7R {
    bits: bool,
}
impl WRT_PROT_SEC_7R {
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
pub struct WRT_PROT_SEC_6R {
    bits: bool,
}
impl WRT_PROT_SEC_6R {
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
pub struct WRT_PROT_SEC_5R {
    bits: bool,
}
impl WRT_PROT_SEC_5R {
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
pub struct WRT_PROT_SEC_4R {
    bits: bool,
}
impl WRT_PROT_SEC_4R {
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
pub struct WRT_PROT_SEC_3R {
    bits: bool,
}
impl WRT_PROT_SEC_3R {
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
pub struct WRT_PROT_SEC_2R {
    bits: bool,
}
impl WRT_PROT_SEC_2R {
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
pub struct WRT_PROT_SEC_1R {
    bits: bool,
}
impl WRT_PROT_SEC_1R {
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
pub struct WRT_PROT_SEC_0R {
    bits: bool,
}
impl WRT_PROT_SEC_0R {
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
pub struct _WRT_PROT_SEC_31W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_31W<'a> {
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
pub struct _WRT_PROT_SEC_30W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_30W<'a> {
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
pub struct _WRT_PROT_SEC_29W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_29W<'a> {
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
pub struct _WRT_PROT_SEC_28W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_28W<'a> {
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
pub struct _WRT_PROT_SEC_27W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_27W<'a> {
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
pub struct _WRT_PROT_SEC_26W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_26W<'a> {
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
pub struct _WRT_PROT_SEC_25W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_25W<'a> {
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
pub struct _WRT_PROT_SEC_24W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_24W<'a> {
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
pub struct _WRT_PROT_SEC_23W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_23W<'a> {
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
pub struct _WRT_PROT_SEC_22W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_22W<'a> {
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
pub struct _WRT_PROT_SEC_21W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_21W<'a> {
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
pub struct _WRT_PROT_SEC_20W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_20W<'a> {
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
pub struct _WRT_PROT_SEC_19W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_19W<'a> {
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
pub struct _WRT_PROT_SEC_18W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_18W<'a> {
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
pub struct _WRT_PROT_SEC_17W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_17W<'a> {
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
pub struct _WRT_PROT_SEC_16W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_16W<'a> {
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
pub struct _WRT_PROT_SEC_15W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_15W<'a> {
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
pub struct _WRT_PROT_SEC_14W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_14W<'a> {
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
pub struct _WRT_PROT_SEC_13W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_13W<'a> {
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
pub struct _WRT_PROT_SEC_12W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_12W<'a> {
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
pub struct _WRT_PROT_SEC_11W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_11W<'a> {
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
pub struct _WRT_PROT_SEC_10W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_10W<'a> {
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
pub struct _WRT_PROT_SEC_9W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_9W<'a> {
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
pub struct _WRT_PROT_SEC_8W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_8W<'a> {
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
pub struct _WRT_PROT_SEC_7W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_7W<'a> {
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
pub struct _WRT_PROT_SEC_6W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_6W<'a> {
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
pub struct _WRT_PROT_SEC_5W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_5W<'a> {
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
pub struct _WRT_PROT_SEC_4W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_4W<'a> {
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
pub struct _WRT_PROT_SEC_3W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_3W<'a> {
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
pub struct _WRT_PROT_SEC_2W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_2W<'a> {
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
pub struct _WRT_PROT_SEC_1W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_1W<'a> {
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
pub struct _WRT_PROT_SEC_0W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_0W<'a> {
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
    #[doc = "Bit 31 - 31:31\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_31(&self) -> WRT_PROT_SEC_31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_31R { bits }
    }
    #[doc = "Bit 30 - 30:30\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_30(&self) -> WRT_PROT_SEC_30R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_30R { bits }
    }
    #[doc = "Bit 29 - 29:29\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_29(&self) -> WRT_PROT_SEC_29R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_29R { bits }
    }
    #[doc = "Bit 28 - 28:28\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_28(&self) -> WRT_PROT_SEC_28R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_28R { bits }
    }
    #[doc = "Bit 27 - 27:27\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_27(&self) -> WRT_PROT_SEC_27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_27R { bits }
    }
    #[doc = "Bit 26 - 26:26\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_26(&self) -> WRT_PROT_SEC_26R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_26R { bits }
    }
    #[doc = "Bit 25 - 25:25\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_25(&self) -> WRT_PROT_SEC_25R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_25R { bits }
    }
    #[doc = "Bit 24 - 24:24\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_24(&self) -> WRT_PROT_SEC_24R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_24R { bits }
    }
    #[doc = "Bit 23 - 23:23\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_23(&self) -> WRT_PROT_SEC_23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_23R { bits }
    }
    #[doc = "Bit 22 - 22:22\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_22(&self) -> WRT_PROT_SEC_22R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_22R { bits }
    }
    #[doc = "Bit 21 - 21:21\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_21(&self) -> WRT_PROT_SEC_21R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_21R { bits }
    }
    #[doc = "Bit 20 - 20:20\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_20(&self) -> WRT_PROT_SEC_20R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_20R { bits }
    }
    #[doc = "Bit 19 - 19:19\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_19(&self) -> WRT_PROT_SEC_19R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_19R { bits }
    }
    #[doc = "Bit 18 - 18:18\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_18(&self) -> WRT_PROT_SEC_18R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_18R { bits }
    }
    #[doc = "Bit 17 - 17:17\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_17(&self) -> WRT_PROT_SEC_17R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_17R { bits }
    }
    #[doc = "Bit 16 - 16:16\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_16(&self) -> WRT_PROT_SEC_16R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_16R { bits }
    }
    #[doc = "Bit 15 - 15:15\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_15(&self) -> WRT_PROT_SEC_15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_15R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_14(&self) -> WRT_PROT_SEC_14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_14R { bits }
    }
    #[doc = "Bit 13 - 13:13\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_13(&self) -> WRT_PROT_SEC_13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_13R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_12(&self) -> WRT_PROT_SEC_12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_12R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_11(&self) -> WRT_PROT_SEC_11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_11R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_10(&self) -> WRT_PROT_SEC_10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_10R { bits }
    }
    #[doc = "Bit 9 - 9:9\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_9(&self) -> WRT_PROT_SEC_9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_9R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_8(&self) -> WRT_PROT_SEC_8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_8R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_7(&self) -> WRT_PROT_SEC_7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_7R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_6(&self) -> WRT_PROT_SEC_6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_6R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_5(&self) -> WRT_PROT_SEC_5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_5R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_4(&self) -> WRT_PROT_SEC_4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_4R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_3(&self) -> WRT_PROT_SEC_3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_2(&self) -> WRT_PROT_SEC_2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_1(&self) -> WRT_PROT_SEC_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_1R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_0(&self) -> WRT_PROT_SEC_0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_0R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - 31:31\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_31(&mut self) -> _WRT_PROT_SEC_31W {
        _WRT_PROT_SEC_31W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_30(&mut self) -> _WRT_PROT_SEC_30W {
        _WRT_PROT_SEC_30W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_29(&mut self) -> _WRT_PROT_SEC_29W {
        _WRT_PROT_SEC_29W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_28(&mut self) -> _WRT_PROT_SEC_28W {
        _WRT_PROT_SEC_28W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_27(&mut self) -> _WRT_PROT_SEC_27W {
        _WRT_PROT_SEC_27W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_26(&mut self) -> _WRT_PROT_SEC_26W {
        _WRT_PROT_SEC_26W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_25(&mut self) -> _WRT_PROT_SEC_25W {
        _WRT_PROT_SEC_25W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_24(&mut self) -> _WRT_PROT_SEC_24W {
        _WRT_PROT_SEC_24W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_23(&mut self) -> _WRT_PROT_SEC_23W {
        _WRT_PROT_SEC_23W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_22(&mut self) -> _WRT_PROT_SEC_22W {
        _WRT_PROT_SEC_22W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_21(&mut self) -> _WRT_PROT_SEC_21W {
        _WRT_PROT_SEC_21W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_20(&mut self) -> _WRT_PROT_SEC_20W {
        _WRT_PROT_SEC_20W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_19(&mut self) -> _WRT_PROT_SEC_19W {
        _WRT_PROT_SEC_19W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_18(&mut self) -> _WRT_PROT_SEC_18W {
        _WRT_PROT_SEC_18W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_17(&mut self) -> _WRT_PROT_SEC_17W {
        _WRT_PROT_SEC_17W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_16(&mut self) -> _WRT_PROT_SEC_16W {
        _WRT_PROT_SEC_16W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_15(&mut self) -> _WRT_PROT_SEC_15W {
        _WRT_PROT_SEC_15W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_14(&mut self) -> _WRT_PROT_SEC_14W {
        _WRT_PROT_SEC_14W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_13(&mut self) -> _WRT_PROT_SEC_13W {
        _WRT_PROT_SEC_13W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_12(&mut self) -> _WRT_PROT_SEC_12W {
        _WRT_PROT_SEC_12W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_11(&mut self) -> _WRT_PROT_SEC_11W {
        _WRT_PROT_SEC_11W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_10(&mut self) -> _WRT_PROT_SEC_10W {
        _WRT_PROT_SEC_10W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_9(&mut self) -> _WRT_PROT_SEC_9W {
        _WRT_PROT_SEC_9W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_8(&mut self) -> _WRT_PROT_SEC_8W {
        _WRT_PROT_SEC_8W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_7(&mut self) -> _WRT_PROT_SEC_7W {
        _WRT_PROT_SEC_7W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_6(&mut self) -> _WRT_PROT_SEC_6W {
        _WRT_PROT_SEC_6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_5(&mut self) -> _WRT_PROT_SEC_5W {
        _WRT_PROT_SEC_5W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_4(&mut self) -> _WRT_PROT_SEC_4W {
        _WRT_PROT_SEC_4W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_3(&mut self) -> _WRT_PROT_SEC_3W {
        _WRT_PROT_SEC_3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_2(&mut self) -> _WRT_PROT_SEC_2W {
        _WRT_PROT_SEC_2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_1(&mut self) -> _WRT_PROT_SEC_1W {
        _WRT_PROT_SEC_1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_0(&mut self) -> _WRT_PROT_SEC_0W {
        _WRT_PROT_SEC_0W { w: self }
    }
}
