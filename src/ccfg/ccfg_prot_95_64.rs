#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCFG_PROT_95_64 {
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
pub struct WRT_PROT_SEC_95R {
    bits: bool,
}
impl WRT_PROT_SEC_95R {
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
pub struct WRT_PROT_SEC_94R {
    bits: bool,
}
impl WRT_PROT_SEC_94R {
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
pub struct WRT_PROT_SEC_93R {
    bits: bool,
}
impl WRT_PROT_SEC_93R {
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
pub struct WRT_PROT_SEC_92R {
    bits: bool,
}
impl WRT_PROT_SEC_92R {
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
pub struct WRT_PROT_SEC_91R {
    bits: bool,
}
impl WRT_PROT_SEC_91R {
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
pub struct WRT_PROT_SEC_90R {
    bits: bool,
}
impl WRT_PROT_SEC_90R {
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
pub struct WRT_PROT_SEC_89R {
    bits: bool,
}
impl WRT_PROT_SEC_89R {
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
pub struct WRT_PROT_SEC_88R {
    bits: bool,
}
impl WRT_PROT_SEC_88R {
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
pub struct WRT_PROT_SEC_87R {
    bits: bool,
}
impl WRT_PROT_SEC_87R {
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
pub struct WRT_PROT_SEC_86R {
    bits: bool,
}
impl WRT_PROT_SEC_86R {
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
pub struct WRT_PROT_SEC_85R {
    bits: bool,
}
impl WRT_PROT_SEC_85R {
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
pub struct WRT_PROT_SEC_84R {
    bits: bool,
}
impl WRT_PROT_SEC_84R {
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
pub struct WRT_PROT_SEC_83R {
    bits: bool,
}
impl WRT_PROT_SEC_83R {
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
pub struct WRT_PROT_SEC_82R {
    bits: bool,
}
impl WRT_PROT_SEC_82R {
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
pub struct WRT_PROT_SEC_81R {
    bits: bool,
}
impl WRT_PROT_SEC_81R {
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
pub struct WRT_PROT_SEC_80R {
    bits: bool,
}
impl WRT_PROT_SEC_80R {
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
pub struct WRT_PROT_SEC_79R {
    bits: bool,
}
impl WRT_PROT_SEC_79R {
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
pub struct WRT_PROT_SEC_78R {
    bits: bool,
}
impl WRT_PROT_SEC_78R {
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
pub struct WRT_PROT_SEC_77R {
    bits: bool,
}
impl WRT_PROT_SEC_77R {
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
pub struct WRT_PROT_SEC_76R {
    bits: bool,
}
impl WRT_PROT_SEC_76R {
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
pub struct WRT_PROT_SEC_75R {
    bits: bool,
}
impl WRT_PROT_SEC_75R {
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
pub struct WRT_PROT_SEC_74R {
    bits: bool,
}
impl WRT_PROT_SEC_74R {
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
pub struct WRT_PROT_SEC_73R {
    bits: bool,
}
impl WRT_PROT_SEC_73R {
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
pub struct WRT_PROT_SEC_72R {
    bits: bool,
}
impl WRT_PROT_SEC_72R {
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
pub struct WRT_PROT_SEC_71R {
    bits: bool,
}
impl WRT_PROT_SEC_71R {
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
pub struct WRT_PROT_SEC_70R {
    bits: bool,
}
impl WRT_PROT_SEC_70R {
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
pub struct WRT_PROT_SEC_69R {
    bits: bool,
}
impl WRT_PROT_SEC_69R {
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
pub struct WRT_PROT_SEC_68R {
    bits: bool,
}
impl WRT_PROT_SEC_68R {
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
pub struct WRT_PROT_SEC_67R {
    bits: bool,
}
impl WRT_PROT_SEC_67R {
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
pub struct WRT_PROT_SEC_66R {
    bits: bool,
}
impl WRT_PROT_SEC_66R {
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
pub struct WRT_PROT_SEC_65R {
    bits: bool,
}
impl WRT_PROT_SEC_65R {
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
pub struct WRT_PROT_SEC_64R {
    bits: bool,
}
impl WRT_PROT_SEC_64R {
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
pub struct _WRT_PROT_SEC_95W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_95W<'a> {
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
pub struct _WRT_PROT_SEC_94W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_94W<'a> {
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
pub struct _WRT_PROT_SEC_93W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_93W<'a> {
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
pub struct _WRT_PROT_SEC_92W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_92W<'a> {
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
pub struct _WRT_PROT_SEC_91W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_91W<'a> {
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
pub struct _WRT_PROT_SEC_90W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_90W<'a> {
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
pub struct _WRT_PROT_SEC_89W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_89W<'a> {
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
pub struct _WRT_PROT_SEC_88W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_88W<'a> {
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
pub struct _WRT_PROT_SEC_87W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_87W<'a> {
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
pub struct _WRT_PROT_SEC_86W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_86W<'a> {
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
pub struct _WRT_PROT_SEC_85W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_85W<'a> {
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
pub struct _WRT_PROT_SEC_84W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_84W<'a> {
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
pub struct _WRT_PROT_SEC_83W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_83W<'a> {
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
pub struct _WRT_PROT_SEC_82W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_82W<'a> {
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
pub struct _WRT_PROT_SEC_81W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_81W<'a> {
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
pub struct _WRT_PROT_SEC_80W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_80W<'a> {
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
pub struct _WRT_PROT_SEC_79W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_79W<'a> {
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
pub struct _WRT_PROT_SEC_78W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_78W<'a> {
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
pub struct _WRT_PROT_SEC_77W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_77W<'a> {
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
pub struct _WRT_PROT_SEC_76W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_76W<'a> {
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
pub struct _WRT_PROT_SEC_75W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_75W<'a> {
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
pub struct _WRT_PROT_SEC_74W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_74W<'a> {
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
pub struct _WRT_PROT_SEC_73W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_73W<'a> {
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
pub struct _WRT_PROT_SEC_72W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_72W<'a> {
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
pub struct _WRT_PROT_SEC_71W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_71W<'a> {
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
pub struct _WRT_PROT_SEC_70W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_70W<'a> {
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
pub struct _WRT_PROT_SEC_69W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_69W<'a> {
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
pub struct _WRT_PROT_SEC_68W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_68W<'a> {
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
pub struct _WRT_PROT_SEC_67W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_67W<'a> {
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
pub struct _WRT_PROT_SEC_66W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_66W<'a> {
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
pub struct _WRT_PROT_SEC_65W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_65W<'a> {
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
pub struct _WRT_PROT_SEC_64W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_64W<'a> {
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
    pub fn wrt_prot_sec_95(&self) -> WRT_PROT_SEC_95R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_95R { bits }
    }
    #[doc = "Bit 30 - 30:30\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_94(&self) -> WRT_PROT_SEC_94R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_94R { bits }
    }
    #[doc = "Bit 29 - 29:29\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_93(&self) -> WRT_PROT_SEC_93R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_93R { bits }
    }
    #[doc = "Bit 28 - 28:28\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_92(&self) -> WRT_PROT_SEC_92R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_92R { bits }
    }
    #[doc = "Bit 27 - 27:27\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_91(&self) -> WRT_PROT_SEC_91R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_91R { bits }
    }
    #[doc = "Bit 26 - 26:26\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_90(&self) -> WRT_PROT_SEC_90R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_90R { bits }
    }
    #[doc = "Bit 25 - 25:25\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_89(&self) -> WRT_PROT_SEC_89R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_89R { bits }
    }
    #[doc = "Bit 24 - 24:24\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_88(&self) -> WRT_PROT_SEC_88R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_88R { bits }
    }
    #[doc = "Bit 23 - 23:23\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_87(&self) -> WRT_PROT_SEC_87R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_87R { bits }
    }
    #[doc = "Bit 22 - 22:22\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_86(&self) -> WRT_PROT_SEC_86R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_86R { bits }
    }
    #[doc = "Bit 21 - 21:21\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_85(&self) -> WRT_PROT_SEC_85R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_85R { bits }
    }
    #[doc = "Bit 20 - 20:20\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_84(&self) -> WRT_PROT_SEC_84R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_84R { bits }
    }
    #[doc = "Bit 19 - 19:19\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_83(&self) -> WRT_PROT_SEC_83R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_83R { bits }
    }
    #[doc = "Bit 18 - 18:18\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_82(&self) -> WRT_PROT_SEC_82R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_82R { bits }
    }
    #[doc = "Bit 17 - 17:17\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_81(&self) -> WRT_PROT_SEC_81R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_81R { bits }
    }
    #[doc = "Bit 16 - 16:16\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_80(&self) -> WRT_PROT_SEC_80R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_80R { bits }
    }
    #[doc = "Bit 15 - 15:15\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_79(&self) -> WRT_PROT_SEC_79R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_79R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_78(&self) -> WRT_PROT_SEC_78R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_78R { bits }
    }
    #[doc = "Bit 13 - 13:13\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_77(&self) -> WRT_PROT_SEC_77R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_77R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_76(&self) -> WRT_PROT_SEC_76R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_76R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_75(&self) -> WRT_PROT_SEC_75R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_75R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_74(&self) -> WRT_PROT_SEC_74R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_74R { bits }
    }
    #[doc = "Bit 9 - 9:9\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_73(&self) -> WRT_PROT_SEC_73R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_73R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_72(&self) -> WRT_PROT_SEC_72R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_72R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_71(&self) -> WRT_PROT_SEC_71R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_71R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_70(&self) -> WRT_PROT_SEC_70R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_70R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_69(&self) -> WRT_PROT_SEC_69R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_69R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_68(&self) -> WRT_PROT_SEC_68R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_68R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_67(&self) -> WRT_PROT_SEC_67R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_67R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_66(&self) -> WRT_PROT_SEC_66R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_66R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_65(&self) -> WRT_PROT_SEC_65R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_65R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_64(&self) -> WRT_PROT_SEC_64R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_64R { bits }
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
    pub fn wrt_prot_sec_95(&mut self) -> _WRT_PROT_SEC_95W {
        _WRT_PROT_SEC_95W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_94(&mut self) -> _WRT_PROT_SEC_94W {
        _WRT_PROT_SEC_94W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_93(&mut self) -> _WRT_PROT_SEC_93W {
        _WRT_PROT_SEC_93W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_92(&mut self) -> _WRT_PROT_SEC_92W {
        _WRT_PROT_SEC_92W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_91(&mut self) -> _WRT_PROT_SEC_91W {
        _WRT_PROT_SEC_91W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_90(&mut self) -> _WRT_PROT_SEC_90W {
        _WRT_PROT_SEC_90W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_89(&mut self) -> _WRT_PROT_SEC_89W {
        _WRT_PROT_SEC_89W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_88(&mut self) -> _WRT_PROT_SEC_88W {
        _WRT_PROT_SEC_88W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_87(&mut self) -> _WRT_PROT_SEC_87W {
        _WRT_PROT_SEC_87W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_86(&mut self) -> _WRT_PROT_SEC_86W {
        _WRT_PROT_SEC_86W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_85(&mut self) -> _WRT_PROT_SEC_85W {
        _WRT_PROT_SEC_85W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_84(&mut self) -> _WRT_PROT_SEC_84W {
        _WRT_PROT_SEC_84W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_83(&mut self) -> _WRT_PROT_SEC_83W {
        _WRT_PROT_SEC_83W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_82(&mut self) -> _WRT_PROT_SEC_82W {
        _WRT_PROT_SEC_82W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_81(&mut self) -> _WRT_PROT_SEC_81W {
        _WRT_PROT_SEC_81W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_80(&mut self) -> _WRT_PROT_SEC_80W {
        _WRT_PROT_SEC_80W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_79(&mut self) -> _WRT_PROT_SEC_79W {
        _WRT_PROT_SEC_79W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_78(&mut self) -> _WRT_PROT_SEC_78W {
        _WRT_PROT_SEC_78W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_77(&mut self) -> _WRT_PROT_SEC_77W {
        _WRT_PROT_SEC_77W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_76(&mut self) -> _WRT_PROT_SEC_76W {
        _WRT_PROT_SEC_76W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_75(&mut self) -> _WRT_PROT_SEC_75W {
        _WRT_PROT_SEC_75W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_74(&mut self) -> _WRT_PROT_SEC_74W {
        _WRT_PROT_SEC_74W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_73(&mut self) -> _WRT_PROT_SEC_73W {
        _WRT_PROT_SEC_73W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_72(&mut self) -> _WRT_PROT_SEC_72W {
        _WRT_PROT_SEC_72W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_71(&mut self) -> _WRT_PROT_SEC_71W {
        _WRT_PROT_SEC_71W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_70(&mut self) -> _WRT_PROT_SEC_70W {
        _WRT_PROT_SEC_70W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_69(&mut self) -> _WRT_PROT_SEC_69W {
        _WRT_PROT_SEC_69W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_68(&mut self) -> _WRT_PROT_SEC_68W {
        _WRT_PROT_SEC_68W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_67(&mut self) -> _WRT_PROT_SEC_67W {
        _WRT_PROT_SEC_67W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_66(&mut self) -> _WRT_PROT_SEC_66W {
        _WRT_PROT_SEC_66W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_65(&mut self) -> _WRT_PROT_SEC_65W {
        _WRT_PROT_SEC_65W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_64(&mut self) -> _WRT_PROT_SEC_64W {
        _WRT_PROT_SEC_64W { w: self }
    }
}
