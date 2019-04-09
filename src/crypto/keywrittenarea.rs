#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::KEYWRITTENAREA {
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
pub struct RESERVED8R {
    bits: u32,
}
impl RESERVED8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `RAM_AREA_WRITTEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN7R {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN7R::WRITTEN => true,
            RAM_AREA_WRITTEN7R::NOT_WRITTEN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA_WRITTEN7R {
        match value {
            true => RAM_AREA_WRITTEN7R::WRITTEN,
            false => RAM_AREA_WRITTEN7R::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline]
    pub fn is_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN7R::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline]
    pub fn is_not_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN7R::NOT_WRITTEN
    }
}
#[doc = "Possible values of the field `RAM_AREA_WRITTEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN6R {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN6R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN6R::WRITTEN => true,
            RAM_AREA_WRITTEN6R::NOT_WRITTEN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA_WRITTEN6R {
        match value {
            true => RAM_AREA_WRITTEN6R::WRITTEN,
            false => RAM_AREA_WRITTEN6R::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline]
    pub fn is_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN6R::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline]
    pub fn is_not_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN6R::NOT_WRITTEN
    }
}
#[doc = "Possible values of the field `RAM_AREA_WRITTEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN5R {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN5R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN5R::WRITTEN => true,
            RAM_AREA_WRITTEN5R::NOT_WRITTEN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA_WRITTEN5R {
        match value {
            true => RAM_AREA_WRITTEN5R::WRITTEN,
            false => RAM_AREA_WRITTEN5R::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline]
    pub fn is_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN5R::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline]
    pub fn is_not_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN5R::NOT_WRITTEN
    }
}
#[doc = "Possible values of the field `RAM_AREA_WRITTEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN4R {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN4R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN4R::WRITTEN => true,
            RAM_AREA_WRITTEN4R::NOT_WRITTEN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA_WRITTEN4R {
        match value {
            true => RAM_AREA_WRITTEN4R::WRITTEN,
            false => RAM_AREA_WRITTEN4R::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline]
    pub fn is_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN4R::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline]
    pub fn is_not_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN4R::NOT_WRITTEN
    }
}
#[doc = "Possible values of the field `RAM_AREA_WRITTEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN3R {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN3R::WRITTEN => true,
            RAM_AREA_WRITTEN3R::NOT_WRITTEN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA_WRITTEN3R {
        match value {
            true => RAM_AREA_WRITTEN3R::WRITTEN,
            false => RAM_AREA_WRITTEN3R::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline]
    pub fn is_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN3R::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline]
    pub fn is_not_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN3R::NOT_WRITTEN
    }
}
#[doc = "Possible values of the field `RAM_AREA_WRITTEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN2R {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN2R::WRITTEN => true,
            RAM_AREA_WRITTEN2R::NOT_WRITTEN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA_WRITTEN2R {
        match value {
            true => RAM_AREA_WRITTEN2R::WRITTEN,
            false => RAM_AREA_WRITTEN2R::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline]
    pub fn is_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN2R::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline]
    pub fn is_not_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN2R::NOT_WRITTEN
    }
}
#[doc = "Possible values of the field `RAM_AREA_WRITTEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN1R {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN1R::WRITTEN => true,
            RAM_AREA_WRITTEN1R::NOT_WRITTEN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA_WRITTEN1R {
        match value {
            true => RAM_AREA_WRITTEN1R::WRITTEN,
            false => RAM_AREA_WRITTEN1R::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline]
    pub fn is_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN1R::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline]
    pub fn is_not_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN1R::NOT_WRITTEN
    }
}
#[doc = "Possible values of the field `RAM_AREA_WRITTEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA_WRITTEN0R {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN0R::WRITTEN => true,
            RAM_AREA_WRITTEN0R::NOT_WRITTEN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA_WRITTEN0R {
        match value {
            true => RAM_AREA_WRITTEN0R::WRITTEN,
            false => RAM_AREA_WRITTEN0R::NOT_WRITTEN,
        }
    }
    #[doc = "Checks if the value of the field is `WRITTEN`"]
    #[inline]
    pub fn is_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN0R::WRITTEN
    }
    #[doc = "Checks if the value of the field is `NOT_WRITTEN`"]
    #[inline]
    pub fn is_not_written(&self) -> bool {
        *self == RAM_AREA_WRITTEN0R::NOT_WRITTEN
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED8W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED8W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAM_AREA_WRITTEN7`"]
pub enum RAM_AREA_WRITTEN7W {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN7W::WRITTEN => true,
            RAM_AREA_WRITTEN7W::NOT_WRITTEN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA_WRITTEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA_WRITTEN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA_WRITTEN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN7W::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN7W::NOT_WRITTEN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `RAM_AREA_WRITTEN6`"]
pub enum RAM_AREA_WRITTEN6W {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN6W::WRITTEN => true,
            RAM_AREA_WRITTEN6W::NOT_WRITTEN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA_WRITTEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA_WRITTEN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA_WRITTEN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN6W::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN6W::NOT_WRITTEN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `RAM_AREA_WRITTEN5`"]
pub enum RAM_AREA_WRITTEN5W {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN5W::WRITTEN => true,
            RAM_AREA_WRITTEN5W::NOT_WRITTEN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA_WRITTEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA_WRITTEN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA_WRITTEN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN5W::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN5W::NOT_WRITTEN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `RAM_AREA_WRITTEN4`"]
pub enum RAM_AREA_WRITTEN4W {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN4W::WRITTEN => true,
            RAM_AREA_WRITTEN4W::NOT_WRITTEN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA_WRITTEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA_WRITTEN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA_WRITTEN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN4W::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN4W::NOT_WRITTEN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `RAM_AREA_WRITTEN3`"]
pub enum RAM_AREA_WRITTEN3W {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN3W::WRITTEN => true,
            RAM_AREA_WRITTEN3W::NOT_WRITTEN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA_WRITTEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA_WRITTEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA_WRITTEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN3W::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN3W::NOT_WRITTEN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `RAM_AREA_WRITTEN2`"]
pub enum RAM_AREA_WRITTEN2W {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN2W::WRITTEN => true,
            RAM_AREA_WRITTEN2W::NOT_WRITTEN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA_WRITTEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA_WRITTEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA_WRITTEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN2W::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN2W::NOT_WRITTEN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `RAM_AREA_WRITTEN1`"]
pub enum RAM_AREA_WRITTEN1W {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN1W::WRITTEN => true,
            RAM_AREA_WRITTEN1W::NOT_WRITTEN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA_WRITTEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA_WRITTEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA_WRITTEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN1W::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN1W::NOT_WRITTEN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `RAM_AREA_WRITTEN0`"]
pub enum RAM_AREA_WRITTEN0W {
    #[doc = "This RAM area is written with valid key information"]
    WRITTEN,
    #[doc = "This RAM area is not written with valid key information"]
    NOT_WRITTEN,
}
impl RAM_AREA_WRITTEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA_WRITTEN0W::WRITTEN => true,
            RAM_AREA_WRITTEN0W::NOT_WRITTEN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA_WRITTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA_WRITTEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA_WRITTEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is written with valid key information"]
    #[inline]
    pub fn written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN0W::WRITTEN)
    }
    #[doc = "This RAM area is not written with valid key information"]
    #[inline]
    pub fn not_written(self) -> &'a mut W {
        self.variant(RAM_AREA_WRITTEN0W::NOT_WRITTEN)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&self) -> RESERVED8R {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED8R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written7(&self) -> RAM_AREA_WRITTEN7R {
        RAM_AREA_WRITTEN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - 6:6\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written6(&self) -> RAM_AREA_WRITTEN6R {
        RAM_AREA_WRITTEN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - 5:5\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written5(&self) -> RAM_AREA_WRITTEN5R {
        RAM_AREA_WRITTEN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 4:4\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written4(&self) -> RAM_AREA_WRITTEN4R {
        RAM_AREA_WRITTEN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 3:3\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written3(&self) -> RAM_AREA_WRITTEN3R {
        RAM_AREA_WRITTEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - 2:2\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written2(&self) -> RAM_AREA_WRITTEN2R {
        RAM_AREA_WRITTEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written1(&self) -> RAM_AREA_WRITTEN1R {
        RAM_AREA_WRITTEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written0(&self) -> RAM_AREA_WRITTEN0R {
        RAM_AREA_WRITTEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bits 8:31 - 31:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&mut self) -> _RESERVED8W {
        _RESERVED8W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written7(&mut self) -> _RAM_AREA_WRITTEN7W {
        _RAM_AREA_WRITTEN7W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written6(&mut self) -> _RAM_AREA_WRITTEN6W {
        _RAM_AREA_WRITTEN6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written5(&mut self) -> _RAM_AREA_WRITTEN5W {
        _RAM_AREA_WRITTEN5W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written4(&mut self) -> _RAM_AREA_WRITTEN4W {
        _RAM_AREA_WRITTEN4W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written3(&mut self) -> _RAM_AREA_WRITTEN3W {
        _RAM_AREA_WRITTEN3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written2(&mut self) -> _RAM_AREA_WRITTEN2W {
        _RAM_AREA_WRITTEN2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written1(&mut self) -> _RAM_AREA_WRITTEN1W {
        _RAM_AREA_WRITTEN1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] On read this bit returns the key area written status. This bit can be reset by writing a 1. Note: This register will be reset on a soft reset initiated by writing to DMASWRESET.RESET. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline]
    pub fn ram_area_written0(&mut self) -> _RAM_AREA_WRITTEN0W {
        _RAM_AREA_WRITTEN0W { w: self }
    }
}
