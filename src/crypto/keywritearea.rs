#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::KEYWRITEAREA {
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
#[doc = "Possible values of the field `RAM_AREA7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA7R {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA7R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            RAM_AREA7R::SEL => true,
            RAM_AREA7R::NOT_SEL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA7R {
        match value {
            true => RAM_AREA7R::SEL,
            false => RAM_AREA7R::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA7R::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA7R::NOT_SEL
    }
}
#[doc = "Possible values of the field `RAM_AREA6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA6R {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA6R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            RAM_AREA6R::SEL => true,
            RAM_AREA6R::NOT_SEL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA6R {
        match value {
            true => RAM_AREA6R::SEL,
            false => RAM_AREA6R::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA6R::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA6R::NOT_SEL
    }
}
#[doc = "Possible values of the field `RAM_AREA5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA5R {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA5R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            RAM_AREA5R::SEL => true,
            RAM_AREA5R::NOT_SEL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA5R {
        match value {
            true => RAM_AREA5R::SEL,
            false => RAM_AREA5R::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA5R::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA5R::NOT_SEL
    }
}
#[doc = "Possible values of the field `RAM_AREA4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA4R {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA4R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            RAM_AREA4R::SEL => true,
            RAM_AREA4R::NOT_SEL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA4R {
        match value {
            true => RAM_AREA4R::SEL,
            false => RAM_AREA4R::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA4R::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA4R::NOT_SEL
    }
}
#[doc = "Possible values of the field `RAM_AREA3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA3R {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA3R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            RAM_AREA3R::SEL => true,
            RAM_AREA3R::NOT_SEL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA3R {
        match value {
            true => RAM_AREA3R::SEL,
            false => RAM_AREA3R::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA3R::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA3R::NOT_SEL
    }
}
#[doc = "Possible values of the field `RAM_AREA2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA2R {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA2R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            RAM_AREA2R::SEL => true,
            RAM_AREA2R::NOT_SEL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA2R {
        match value {
            true => RAM_AREA2R::SEL,
            false => RAM_AREA2R::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA2R::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA2R::NOT_SEL
    }
}
#[doc = "Possible values of the field `RAM_AREA1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA1R {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            RAM_AREA1R::SEL => true,
            RAM_AREA1R::NOT_SEL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA1R {
        match value {
            true => RAM_AREA1R::SEL,
            false => RAM_AREA1R::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA1R::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA1R::NOT_SEL
    }
}
#[doc = "Possible values of the field `RAM_AREA0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREA0R {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            RAM_AREA0R::SEL => true,
            RAM_AREA0R::NOT_SEL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAM_AREA0R {
        match value {
            true => RAM_AREA0R::SEL,
            false => RAM_AREA0R::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA0R::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA0R::NOT_SEL
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
#[doc = "Values that can be written to the field `RAM_AREA7`"]
pub enum RAM_AREA7W {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA7W::SEL => true,
            RAM_AREA7W::NOT_SEL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA7W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA7W::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA7W::NOT_SEL)
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
#[doc = "Values that can be written to the field `RAM_AREA6`"]
pub enum RAM_AREA6W {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA6W::SEL => true,
            RAM_AREA6W::NOT_SEL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA6W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA6W::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA6W::NOT_SEL)
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
#[doc = "Values that can be written to the field `RAM_AREA5`"]
pub enum RAM_AREA5W {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA5W::SEL => true,
            RAM_AREA5W::NOT_SEL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA5W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA5W::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA5W::NOT_SEL)
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
#[doc = "Values that can be written to the field `RAM_AREA4`"]
pub enum RAM_AREA4W {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA4W::SEL => true,
            RAM_AREA4W::NOT_SEL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA4W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA4W::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA4W::NOT_SEL)
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
#[doc = "Values that can be written to the field `RAM_AREA3`"]
pub enum RAM_AREA3W {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA3W::SEL => true,
            RAM_AREA3W::NOT_SEL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA3W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA3W::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA3W::NOT_SEL)
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
#[doc = "Values that can be written to the field `RAM_AREA2`"]
pub enum RAM_AREA2W {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA2W::SEL => true,
            RAM_AREA2W::NOT_SEL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA2W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA2W::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA2W::NOT_SEL)
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
#[doc = "Values that can be written to the field `RAM_AREA1`"]
pub enum RAM_AREA1W {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA1W::SEL => true,
            RAM_AREA1W::NOT_SEL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA1W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA1W::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA1W::NOT_SEL)
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
#[doc = "Values that can be written to the field `RAM_AREA0`"]
pub enum RAM_AREA0W {
    #[doc = "This RAM area is selected to be written"]
    SEL,
    #[doc = "This RAM area is not selected to be written"]
    NOT_SEL,
}
impl RAM_AREA0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAM_AREA0W::SEL => true,
            RAM_AREA0W::NOT_SEL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREA0W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREA0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREA0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA0W::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA0W::NOT_SEL)
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
    #[doc = "Bit 7 - 7:7\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area7(&self) -> RAM_AREA7R {
        RAM_AREA7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - 6:6\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area6(&self) -> RAM_AREA6R {
        RAM_AREA6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - 5:5\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area5(&self) -> RAM_AREA5R {
        RAM_AREA5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 4:4\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area4(&self) -> RAM_AREA4R {
        RAM_AREA4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 3:3\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area3(&self) -> RAM_AREA3R {
        RAM_AREA3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - 2:2\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area2(&self) -> RAM_AREA2R {
        RAM_AREA2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area1(&self) -> RAM_AREA1R {
        RAM_AREA1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area0(&self) -> RAM_AREA0R {
        RAM_AREA0R::_from({
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
    #[doc = "Bit 7 - 7:7\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area7(&mut self) -> _RAM_AREA7W {
        _RAM_AREA7W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area6(&mut self) -> _RAM_AREA6W {
        _RAM_AREA6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area5(&mut self) -> _RAM_AREA5W {
        _RAM_AREA5W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area4(&mut self) -> _RAM_AREA4W {
        _RAM_AREA4W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area3(&mut self) -> _RAM_AREA3W {
        _RAM_AREA3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area2(&mut self) -> _RAM_AREA2W {
        _RAM_AREA2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area1(&mut self) -> _RAM_AREA1W {
        _RAM_AREA1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline]
    pub fn ram_area0(&mut self) -> _RAM_AREA0W {
        _RAM_AREA0W { w: self }
    }
}
