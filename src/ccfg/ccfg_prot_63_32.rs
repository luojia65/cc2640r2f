#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCFG_PROT_63_32 {
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
pub struct WRT_PROT_SEC_63R {
    bits: bool,
}
impl WRT_PROT_SEC_63R {
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
pub struct WRT_PROT_SEC_62R {
    bits: bool,
}
impl WRT_PROT_SEC_62R {
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
pub struct WRT_PROT_SEC_61R {
    bits: bool,
}
impl WRT_PROT_SEC_61R {
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
pub struct WRT_PROT_SEC_60R {
    bits: bool,
}
impl WRT_PROT_SEC_60R {
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
pub struct WRT_PROT_SEC_59R {
    bits: bool,
}
impl WRT_PROT_SEC_59R {
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
pub struct WRT_PROT_SEC_58R {
    bits: bool,
}
impl WRT_PROT_SEC_58R {
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
pub struct WRT_PROT_SEC_57R {
    bits: bool,
}
impl WRT_PROT_SEC_57R {
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
pub struct WRT_PROT_SEC_56R {
    bits: bool,
}
impl WRT_PROT_SEC_56R {
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
pub struct WRT_PROT_SEC_55R {
    bits: bool,
}
impl WRT_PROT_SEC_55R {
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
pub struct WRT_PROT_SEC_54R {
    bits: bool,
}
impl WRT_PROT_SEC_54R {
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
pub struct WRT_PROT_SEC_53R {
    bits: bool,
}
impl WRT_PROT_SEC_53R {
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
pub struct WRT_PROT_SEC_52R {
    bits: bool,
}
impl WRT_PROT_SEC_52R {
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
pub struct WRT_PROT_SEC_51R {
    bits: bool,
}
impl WRT_PROT_SEC_51R {
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
pub struct WRT_PROT_SEC_50R {
    bits: bool,
}
impl WRT_PROT_SEC_50R {
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
pub struct WRT_PROT_SEC_49R {
    bits: bool,
}
impl WRT_PROT_SEC_49R {
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
pub struct WRT_PROT_SEC_48R {
    bits: bool,
}
impl WRT_PROT_SEC_48R {
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
pub struct WRT_PROT_SEC_47R {
    bits: bool,
}
impl WRT_PROT_SEC_47R {
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
pub struct WRT_PROT_SEC_46R {
    bits: bool,
}
impl WRT_PROT_SEC_46R {
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
pub struct WRT_PROT_SEC_45R {
    bits: bool,
}
impl WRT_PROT_SEC_45R {
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
pub struct WRT_PROT_SEC_44R {
    bits: bool,
}
impl WRT_PROT_SEC_44R {
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
pub struct WRT_PROT_SEC_43R {
    bits: bool,
}
impl WRT_PROT_SEC_43R {
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
pub struct WRT_PROT_SEC_42R {
    bits: bool,
}
impl WRT_PROT_SEC_42R {
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
pub struct WRT_PROT_SEC_41R {
    bits: bool,
}
impl WRT_PROT_SEC_41R {
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
pub struct WRT_PROT_SEC_40R {
    bits: bool,
}
impl WRT_PROT_SEC_40R {
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
pub struct WRT_PROT_SEC_39R {
    bits: bool,
}
impl WRT_PROT_SEC_39R {
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
pub struct WRT_PROT_SEC_38R {
    bits: bool,
}
impl WRT_PROT_SEC_38R {
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
pub struct WRT_PROT_SEC_37R {
    bits: bool,
}
impl WRT_PROT_SEC_37R {
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
pub struct WRT_PROT_SEC_36R {
    bits: bool,
}
impl WRT_PROT_SEC_36R {
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
pub struct WRT_PROT_SEC_35R {
    bits: bool,
}
impl WRT_PROT_SEC_35R {
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
pub struct WRT_PROT_SEC_34R {
    bits: bool,
}
impl WRT_PROT_SEC_34R {
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
pub struct WRT_PROT_SEC_33R {
    bits: bool,
}
impl WRT_PROT_SEC_33R {
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
pub struct WRT_PROT_SEC_32R {
    bits: bool,
}
impl WRT_PROT_SEC_32R {
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
pub struct _WRT_PROT_SEC_63W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_63W<'a> {
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
pub struct _WRT_PROT_SEC_62W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_62W<'a> {
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
pub struct _WRT_PROT_SEC_61W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_61W<'a> {
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
pub struct _WRT_PROT_SEC_60W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_60W<'a> {
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
pub struct _WRT_PROT_SEC_59W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_59W<'a> {
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
pub struct _WRT_PROT_SEC_58W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_58W<'a> {
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
pub struct _WRT_PROT_SEC_57W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_57W<'a> {
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
pub struct _WRT_PROT_SEC_56W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_56W<'a> {
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
pub struct _WRT_PROT_SEC_55W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_55W<'a> {
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
pub struct _WRT_PROT_SEC_54W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_54W<'a> {
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
pub struct _WRT_PROT_SEC_53W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_53W<'a> {
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
pub struct _WRT_PROT_SEC_52W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_52W<'a> {
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
pub struct _WRT_PROT_SEC_51W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_51W<'a> {
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
pub struct _WRT_PROT_SEC_50W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_50W<'a> {
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
pub struct _WRT_PROT_SEC_49W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_49W<'a> {
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
pub struct _WRT_PROT_SEC_48W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_48W<'a> {
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
pub struct _WRT_PROT_SEC_47W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_47W<'a> {
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
pub struct _WRT_PROT_SEC_46W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_46W<'a> {
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
pub struct _WRT_PROT_SEC_45W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_45W<'a> {
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
pub struct _WRT_PROT_SEC_44W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_44W<'a> {
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
pub struct _WRT_PROT_SEC_43W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_43W<'a> {
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
pub struct _WRT_PROT_SEC_42W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_42W<'a> {
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
pub struct _WRT_PROT_SEC_41W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_41W<'a> {
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
pub struct _WRT_PROT_SEC_40W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_40W<'a> {
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
pub struct _WRT_PROT_SEC_39W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_39W<'a> {
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
pub struct _WRT_PROT_SEC_38W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_38W<'a> {
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
pub struct _WRT_PROT_SEC_37W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_37W<'a> {
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
pub struct _WRT_PROT_SEC_36W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_36W<'a> {
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
pub struct _WRT_PROT_SEC_35W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_35W<'a> {
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
pub struct _WRT_PROT_SEC_34W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_34W<'a> {
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
pub struct _WRT_PROT_SEC_33W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_33W<'a> {
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
pub struct _WRT_PROT_SEC_32W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_32W<'a> {
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
    pub fn wrt_prot_sec_63(&self) -> WRT_PROT_SEC_63R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_63R { bits }
    }
    #[doc = "Bit 30 - 30:30\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_62(&self) -> WRT_PROT_SEC_62R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_62R { bits }
    }
    #[doc = "Bit 29 - 29:29\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_61(&self) -> WRT_PROT_SEC_61R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_61R { bits }
    }
    #[doc = "Bit 28 - 28:28\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_60(&self) -> WRT_PROT_SEC_60R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_60R { bits }
    }
    #[doc = "Bit 27 - 27:27\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_59(&self) -> WRT_PROT_SEC_59R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_59R { bits }
    }
    #[doc = "Bit 26 - 26:26\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_58(&self) -> WRT_PROT_SEC_58R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_58R { bits }
    }
    #[doc = "Bit 25 - 25:25\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_57(&self) -> WRT_PROT_SEC_57R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_57R { bits }
    }
    #[doc = "Bit 24 - 24:24\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_56(&self) -> WRT_PROT_SEC_56R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_56R { bits }
    }
    #[doc = "Bit 23 - 23:23\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_55(&self) -> WRT_PROT_SEC_55R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_55R { bits }
    }
    #[doc = "Bit 22 - 22:22\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_54(&self) -> WRT_PROT_SEC_54R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_54R { bits }
    }
    #[doc = "Bit 21 - 21:21\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_53(&self) -> WRT_PROT_SEC_53R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_53R { bits }
    }
    #[doc = "Bit 20 - 20:20\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_52(&self) -> WRT_PROT_SEC_52R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_52R { bits }
    }
    #[doc = "Bit 19 - 19:19\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_51(&self) -> WRT_PROT_SEC_51R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_51R { bits }
    }
    #[doc = "Bit 18 - 18:18\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_50(&self) -> WRT_PROT_SEC_50R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_50R { bits }
    }
    #[doc = "Bit 17 - 17:17\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_49(&self) -> WRT_PROT_SEC_49R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_49R { bits }
    }
    #[doc = "Bit 16 - 16:16\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_48(&self) -> WRT_PROT_SEC_48R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_48R { bits }
    }
    #[doc = "Bit 15 - 15:15\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_47(&self) -> WRT_PROT_SEC_47R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_47R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_46(&self) -> WRT_PROT_SEC_46R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_46R { bits }
    }
    #[doc = "Bit 13 - 13:13\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_45(&self) -> WRT_PROT_SEC_45R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_45R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_44(&self) -> WRT_PROT_SEC_44R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_44R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_43(&self) -> WRT_PROT_SEC_43R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_43R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_42(&self) -> WRT_PROT_SEC_42R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_42R { bits }
    }
    #[doc = "Bit 9 - 9:9\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_41(&self) -> WRT_PROT_SEC_41R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_41R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_40(&self) -> WRT_PROT_SEC_40R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_40R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_39(&self) -> WRT_PROT_SEC_39R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_39R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_38(&self) -> WRT_PROT_SEC_38R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_38R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_37(&self) -> WRT_PROT_SEC_37R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_37R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_36(&self) -> WRT_PROT_SEC_36R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_36R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_35(&self) -> WRT_PROT_SEC_35R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_35R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_34(&self) -> WRT_PROT_SEC_34R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_34R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_33(&self) -> WRT_PROT_SEC_33R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_33R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_32(&self) -> WRT_PROT_SEC_32R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_32R { bits }
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
    pub fn wrt_prot_sec_63(&mut self) -> _WRT_PROT_SEC_63W {
        _WRT_PROT_SEC_63W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_62(&mut self) -> _WRT_PROT_SEC_62W {
        _WRT_PROT_SEC_62W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_61(&mut self) -> _WRT_PROT_SEC_61W {
        _WRT_PROT_SEC_61W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_60(&mut self) -> _WRT_PROT_SEC_60W {
        _WRT_PROT_SEC_60W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_59(&mut self) -> _WRT_PROT_SEC_59W {
        _WRT_PROT_SEC_59W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_58(&mut self) -> _WRT_PROT_SEC_58W {
        _WRT_PROT_SEC_58W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_57(&mut self) -> _WRT_PROT_SEC_57W {
        _WRT_PROT_SEC_57W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_56(&mut self) -> _WRT_PROT_SEC_56W {
        _WRT_PROT_SEC_56W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_55(&mut self) -> _WRT_PROT_SEC_55W {
        _WRT_PROT_SEC_55W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_54(&mut self) -> _WRT_PROT_SEC_54W {
        _WRT_PROT_SEC_54W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_53(&mut self) -> _WRT_PROT_SEC_53W {
        _WRT_PROT_SEC_53W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_52(&mut self) -> _WRT_PROT_SEC_52W {
        _WRT_PROT_SEC_52W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_51(&mut self) -> _WRT_PROT_SEC_51W {
        _WRT_PROT_SEC_51W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_50(&mut self) -> _WRT_PROT_SEC_50W {
        _WRT_PROT_SEC_50W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_49(&mut self) -> _WRT_PROT_SEC_49W {
        _WRT_PROT_SEC_49W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_48(&mut self) -> _WRT_PROT_SEC_48W {
        _WRT_PROT_SEC_48W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_47(&mut self) -> _WRT_PROT_SEC_47W {
        _WRT_PROT_SEC_47W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_46(&mut self) -> _WRT_PROT_SEC_46W {
        _WRT_PROT_SEC_46W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_45(&mut self) -> _WRT_PROT_SEC_45W {
        _WRT_PROT_SEC_45W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_44(&mut self) -> _WRT_PROT_SEC_44W {
        _WRT_PROT_SEC_44W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_43(&mut self) -> _WRT_PROT_SEC_43W {
        _WRT_PROT_SEC_43W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_42(&mut self) -> _WRT_PROT_SEC_42W {
        _WRT_PROT_SEC_42W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_41(&mut self) -> _WRT_PROT_SEC_41W {
        _WRT_PROT_SEC_41W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_40(&mut self) -> _WRT_PROT_SEC_40W {
        _WRT_PROT_SEC_40W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_39(&mut self) -> _WRT_PROT_SEC_39W {
        _WRT_PROT_SEC_39W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_38(&mut self) -> _WRT_PROT_SEC_38W {
        _WRT_PROT_SEC_38W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_37(&mut self) -> _WRT_PROT_SEC_37W {
        _WRT_PROT_SEC_37W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_36(&mut self) -> _WRT_PROT_SEC_36W {
        _WRT_PROT_SEC_36W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_35(&mut self) -> _WRT_PROT_SEC_35W {
        _WRT_PROT_SEC_35W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_34(&mut self) -> _WRT_PROT_SEC_34W {
        _WRT_PROT_SEC_34W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_33(&mut self) -> _WRT_PROT_SEC_33W {
        _WRT_PROT_SEC_33W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_32(&mut self) -> _WRT_PROT_SEC_32W {
        _WRT_PROT_SEC_32W { w: self }
    }
}
