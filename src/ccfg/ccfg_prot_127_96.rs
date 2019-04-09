#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCFG_PROT_127_96 {
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
pub struct WRT_PROT_SEC_127R {
    bits: bool,
}
impl WRT_PROT_SEC_127R {
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
pub struct WRT_PROT_SEC_126R {
    bits: bool,
}
impl WRT_PROT_SEC_126R {
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
pub struct WRT_PROT_SEC_125R {
    bits: bool,
}
impl WRT_PROT_SEC_125R {
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
pub struct WRT_PROT_SEC_124R {
    bits: bool,
}
impl WRT_PROT_SEC_124R {
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
pub struct WRT_PROT_SEC_123R {
    bits: bool,
}
impl WRT_PROT_SEC_123R {
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
pub struct WRT_PROT_SEC_122R {
    bits: bool,
}
impl WRT_PROT_SEC_122R {
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
pub struct WRT_PROT_SEC_121R {
    bits: bool,
}
impl WRT_PROT_SEC_121R {
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
pub struct WRT_PROT_SEC_120R {
    bits: bool,
}
impl WRT_PROT_SEC_120R {
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
pub struct WRT_PROT_SEC_119R {
    bits: bool,
}
impl WRT_PROT_SEC_119R {
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
pub struct WRT_PROT_SEC_118R {
    bits: bool,
}
impl WRT_PROT_SEC_118R {
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
pub struct WRT_PROT_SEC_117R {
    bits: bool,
}
impl WRT_PROT_SEC_117R {
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
pub struct WRT_PROT_SEC_116R {
    bits: bool,
}
impl WRT_PROT_SEC_116R {
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
pub struct WRT_PROT_SEC_115R {
    bits: bool,
}
impl WRT_PROT_SEC_115R {
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
pub struct WRT_PROT_SEC_114R {
    bits: bool,
}
impl WRT_PROT_SEC_114R {
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
pub struct WRT_PROT_SEC_113R {
    bits: bool,
}
impl WRT_PROT_SEC_113R {
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
pub struct WRT_PROT_SEC_112R {
    bits: bool,
}
impl WRT_PROT_SEC_112R {
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
pub struct WRT_PROT_SEC_111R {
    bits: bool,
}
impl WRT_PROT_SEC_111R {
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
pub struct WRT_PROT_SEC_110R {
    bits: bool,
}
impl WRT_PROT_SEC_110R {
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
pub struct WRT_PROT_SEC_109R {
    bits: bool,
}
impl WRT_PROT_SEC_109R {
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
pub struct WRT_PROT_SEC_108R {
    bits: bool,
}
impl WRT_PROT_SEC_108R {
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
pub struct WRT_PROT_SEC_107R {
    bits: bool,
}
impl WRT_PROT_SEC_107R {
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
pub struct WRT_PROT_SEC_106R {
    bits: bool,
}
impl WRT_PROT_SEC_106R {
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
pub struct WRT_PROT_SEC_105R {
    bits: bool,
}
impl WRT_PROT_SEC_105R {
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
pub struct WRT_PROT_SEC_104R {
    bits: bool,
}
impl WRT_PROT_SEC_104R {
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
pub struct WRT_PROT_SEC_103R {
    bits: bool,
}
impl WRT_PROT_SEC_103R {
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
pub struct WRT_PROT_SEC_102R {
    bits: bool,
}
impl WRT_PROT_SEC_102R {
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
pub struct WRT_PROT_SEC_101R {
    bits: bool,
}
impl WRT_PROT_SEC_101R {
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
pub struct WRT_PROT_SEC_100R {
    bits: bool,
}
impl WRT_PROT_SEC_100R {
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
pub struct WRT_PROT_SEC_99R {
    bits: bool,
}
impl WRT_PROT_SEC_99R {
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
pub struct WRT_PROT_SEC_98R {
    bits: bool,
}
impl WRT_PROT_SEC_98R {
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
pub struct WRT_PROT_SEC_97R {
    bits: bool,
}
impl WRT_PROT_SEC_97R {
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
pub struct WRT_PROT_SEC_96R {
    bits: bool,
}
impl WRT_PROT_SEC_96R {
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
pub struct _WRT_PROT_SEC_127W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_127W<'a> {
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
pub struct _WRT_PROT_SEC_126W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_126W<'a> {
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
pub struct _WRT_PROT_SEC_125W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_125W<'a> {
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
pub struct _WRT_PROT_SEC_124W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_124W<'a> {
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
pub struct _WRT_PROT_SEC_123W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_123W<'a> {
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
pub struct _WRT_PROT_SEC_122W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_122W<'a> {
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
pub struct _WRT_PROT_SEC_121W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_121W<'a> {
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
pub struct _WRT_PROT_SEC_120W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_120W<'a> {
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
pub struct _WRT_PROT_SEC_119W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_119W<'a> {
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
pub struct _WRT_PROT_SEC_118W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_118W<'a> {
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
pub struct _WRT_PROT_SEC_117W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_117W<'a> {
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
pub struct _WRT_PROT_SEC_116W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_116W<'a> {
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
pub struct _WRT_PROT_SEC_115W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_115W<'a> {
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
pub struct _WRT_PROT_SEC_114W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_114W<'a> {
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
pub struct _WRT_PROT_SEC_113W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_113W<'a> {
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
pub struct _WRT_PROT_SEC_112W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_112W<'a> {
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
pub struct _WRT_PROT_SEC_111W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_111W<'a> {
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
pub struct _WRT_PROT_SEC_110W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_110W<'a> {
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
pub struct _WRT_PROT_SEC_109W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_109W<'a> {
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
pub struct _WRT_PROT_SEC_108W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_108W<'a> {
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
pub struct _WRT_PROT_SEC_107W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_107W<'a> {
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
pub struct _WRT_PROT_SEC_106W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_106W<'a> {
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
pub struct _WRT_PROT_SEC_105W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_105W<'a> {
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
pub struct _WRT_PROT_SEC_104W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_104W<'a> {
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
pub struct _WRT_PROT_SEC_103W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_103W<'a> {
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
pub struct _WRT_PROT_SEC_102W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_102W<'a> {
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
pub struct _WRT_PROT_SEC_101W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_101W<'a> {
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
pub struct _WRT_PROT_SEC_100W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_100W<'a> {
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
pub struct _WRT_PROT_SEC_99W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_99W<'a> {
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
pub struct _WRT_PROT_SEC_98W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_98W<'a> {
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
pub struct _WRT_PROT_SEC_97W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_97W<'a> {
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
pub struct _WRT_PROT_SEC_96W<'a> {
    w: &'a mut W,
}
impl<'a> _WRT_PROT_SEC_96W<'a> {
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
    pub fn wrt_prot_sec_127(&self) -> WRT_PROT_SEC_127R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_127R { bits }
    }
    #[doc = "Bit 30 - 30:30\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_126(&self) -> WRT_PROT_SEC_126R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_126R { bits }
    }
    #[doc = "Bit 29 - 29:29\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_125(&self) -> WRT_PROT_SEC_125R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_125R { bits }
    }
    #[doc = "Bit 28 - 28:28\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_124(&self) -> WRT_PROT_SEC_124R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_124R { bits }
    }
    #[doc = "Bit 27 - 27:27\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_123(&self) -> WRT_PROT_SEC_123R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_123R { bits }
    }
    #[doc = "Bit 26 - 26:26\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_122(&self) -> WRT_PROT_SEC_122R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_122R { bits }
    }
    #[doc = "Bit 25 - 25:25\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_121(&self) -> WRT_PROT_SEC_121R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_121R { bits }
    }
    #[doc = "Bit 24 - 24:24\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_120(&self) -> WRT_PROT_SEC_120R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_120R { bits }
    }
    #[doc = "Bit 23 - 23:23\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_119(&self) -> WRT_PROT_SEC_119R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_119R { bits }
    }
    #[doc = "Bit 22 - 22:22\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_118(&self) -> WRT_PROT_SEC_118R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_118R { bits }
    }
    #[doc = "Bit 21 - 21:21\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_117(&self) -> WRT_PROT_SEC_117R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_117R { bits }
    }
    #[doc = "Bit 20 - 20:20\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_116(&self) -> WRT_PROT_SEC_116R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_116R { bits }
    }
    #[doc = "Bit 19 - 19:19\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_115(&self) -> WRT_PROT_SEC_115R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_115R { bits }
    }
    #[doc = "Bit 18 - 18:18\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_114(&self) -> WRT_PROT_SEC_114R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_114R { bits }
    }
    #[doc = "Bit 17 - 17:17\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_113(&self) -> WRT_PROT_SEC_113R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_113R { bits }
    }
    #[doc = "Bit 16 - 16:16\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_112(&self) -> WRT_PROT_SEC_112R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_112R { bits }
    }
    #[doc = "Bit 15 - 15:15\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_111(&self) -> WRT_PROT_SEC_111R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_111R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_110(&self) -> WRT_PROT_SEC_110R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_110R { bits }
    }
    #[doc = "Bit 13 - 13:13\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_109(&self) -> WRT_PROT_SEC_109R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_109R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_108(&self) -> WRT_PROT_SEC_108R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_108R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_107(&self) -> WRT_PROT_SEC_107R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_107R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_106(&self) -> WRT_PROT_SEC_106R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_106R { bits }
    }
    #[doc = "Bit 9 - 9:9\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_105(&self) -> WRT_PROT_SEC_105R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_105R { bits }
    }
    #[doc = "Bit 8 - 8:8\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_104(&self) -> WRT_PROT_SEC_104R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_104R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_103(&self) -> WRT_PROT_SEC_103R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_103R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_102(&self) -> WRT_PROT_SEC_102R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_102R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_101(&self) -> WRT_PROT_SEC_101R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_101R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_100(&self) -> WRT_PROT_SEC_100R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_100R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_99(&self) -> WRT_PROT_SEC_99R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_99R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_98(&self) -> WRT_PROT_SEC_98R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_98R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_97(&self) -> WRT_PROT_SEC_97R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_97R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_96(&self) -> WRT_PROT_SEC_96R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRT_PROT_SEC_96R { bits }
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
    pub fn wrt_prot_sec_127(&mut self) -> _WRT_PROT_SEC_127W {
        _WRT_PROT_SEC_127W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_126(&mut self) -> _WRT_PROT_SEC_126W {
        _WRT_PROT_SEC_126W { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_125(&mut self) -> _WRT_PROT_SEC_125W {
        _WRT_PROT_SEC_125W { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_124(&mut self) -> _WRT_PROT_SEC_124W {
        _WRT_PROT_SEC_124W { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_123(&mut self) -> _WRT_PROT_SEC_123W {
        _WRT_PROT_SEC_123W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_122(&mut self) -> _WRT_PROT_SEC_122W {
        _WRT_PROT_SEC_122W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_121(&mut self) -> _WRT_PROT_SEC_121W {
        _WRT_PROT_SEC_121W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_120(&mut self) -> _WRT_PROT_SEC_120W {
        _WRT_PROT_SEC_120W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_119(&mut self) -> _WRT_PROT_SEC_119W {
        _WRT_PROT_SEC_119W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_118(&mut self) -> _WRT_PROT_SEC_118W {
        _WRT_PROT_SEC_118W { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_117(&mut self) -> _WRT_PROT_SEC_117W {
        _WRT_PROT_SEC_117W { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_116(&mut self) -> _WRT_PROT_SEC_116W {
        _WRT_PROT_SEC_116W { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_115(&mut self) -> _WRT_PROT_SEC_115W {
        _WRT_PROT_SEC_115W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_114(&mut self) -> _WRT_PROT_SEC_114W {
        _WRT_PROT_SEC_114W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_113(&mut self) -> _WRT_PROT_SEC_113W {
        _WRT_PROT_SEC_113W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_112(&mut self) -> _WRT_PROT_SEC_112W {
        _WRT_PROT_SEC_112W { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_111(&mut self) -> _WRT_PROT_SEC_111W {
        _WRT_PROT_SEC_111W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_110(&mut self) -> _WRT_PROT_SEC_110W {
        _WRT_PROT_SEC_110W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_109(&mut self) -> _WRT_PROT_SEC_109W {
        _WRT_PROT_SEC_109W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_108(&mut self) -> _WRT_PROT_SEC_108W {
        _WRT_PROT_SEC_108W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_107(&mut self) -> _WRT_PROT_SEC_107W {
        _WRT_PROT_SEC_107W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_106(&mut self) -> _WRT_PROT_SEC_106W {
        _WRT_PROT_SEC_106W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_105(&mut self) -> _WRT_PROT_SEC_105W {
        _WRT_PROT_SEC_105W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_104(&mut self) -> _WRT_PROT_SEC_104W {
        _WRT_PROT_SEC_104W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_103(&mut self) -> _WRT_PROT_SEC_103W {
        _WRT_PROT_SEC_103W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_102(&mut self) -> _WRT_PROT_SEC_102W {
        _WRT_PROT_SEC_102W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_101(&mut self) -> _WRT_PROT_SEC_101W {
        _WRT_PROT_SEC_101W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_100(&mut self) -> _WRT_PROT_SEC_100W {
        _WRT_PROT_SEC_100W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_99(&mut self) -> _WRT_PROT_SEC_99W {
        _WRT_PROT_SEC_99W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_98(&mut self) -> _WRT_PROT_SEC_98W {
        _WRT_PROT_SEC_98W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_97(&mut self) -> _WRT_PROT_SEC_97W {
        _WRT_PROT_SEC_97W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 0: Sector protected"]
    #[inline]
    pub fn wrt_prot_sec_96(&mut self) -> _WRT_PROT_SEC_96W {
        _WRT_PROT_SEC_96W { w: self }
    }
}
