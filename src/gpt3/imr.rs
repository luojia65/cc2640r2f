#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IMR {
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
pub struct RESERVED14R {
    bits: u32,
}
impl RESERVED14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `DMABIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMABIMR {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl DMABIMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            DMABIMR::EN => true,
            DMABIMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMABIMR {
        match value {
            true => DMABIMR::EN,
            false => DMABIMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == DMABIMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == DMABIMR::DIS
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED12R {
    bits: bool,
}
impl RESERVED12R {
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
#[doc = "Possible values of the field `TBMIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBMIMR {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl TBMIMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            TBMIMR::EN => true,
            TBMIMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBMIMR {
        match value {
            true => TBMIMR::EN,
            false => TBMIMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TBMIMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TBMIMR::DIS
    }
}
#[doc = "Possible values of the field `CBEIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBEIMR {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl CBEIMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            CBEIMR::EN => true,
            CBEIMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CBEIMR {
        match value {
            true => CBEIMR::EN,
            false => CBEIMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CBEIMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CBEIMR::DIS
    }
}
#[doc = "Possible values of the field `CBMIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBMIMR {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl CBMIMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            CBMIMR::EN => true,
            CBMIMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CBMIMR {
        match value {
            true => CBMIMR::EN,
            false => CBMIMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CBMIMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CBMIMR::DIS
    }
}
#[doc = "Possible values of the field `TBTOIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBTOIMR {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl TBTOIMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            TBTOIMR::EN => true,
            TBTOIMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBTOIMR {
        match value {
            true => TBTOIMR::EN,
            false => TBTOIMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TBTOIMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TBTOIMR::DIS
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED6R {
    bits: u8,
}
impl RESERVED6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DMAAIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAAIMR {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl DMAAIMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            DMAAIMR::EN => true,
            DMAAIMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAAIMR {
        match value {
            true => DMAAIMR::EN,
            false => DMAAIMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == DMAAIMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == DMAAIMR::DIS
    }
}
#[doc = "Possible values of the field `TAMIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMIMR {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl TAMIMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            TAMIMR::EN => true,
            TAMIMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAMIMR {
        match value {
            true => TAMIMR::EN,
            false => TAMIMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TAMIMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TAMIMR::DIS
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED3R {
    bits: bool,
}
impl RESERVED3R {
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
#[doc = "Possible values of the field `CAEIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAEIMR {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl CAEIMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            CAEIMR::EN => true,
            CAEIMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAEIMR {
        match value {
            true => CAEIMR::EN,
            false => CAEIMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CAEIMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CAEIMR::DIS
    }
}
#[doc = "Possible values of the field `CAMIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAMIMR {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl CAMIMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            CAMIMR::EN => true,
            CAMIMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAMIMR {
        match value {
            true => CAMIMR::EN,
            false => CAMIMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CAMIMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CAMIMR::DIS
    }
}
#[doc = "Possible values of the field `TATOIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TATOIMR {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl TATOIMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
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
            TATOIMR::EN => true,
            TATOIMR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TATOIMR {
        match value {
            true => TATOIMR::EN,
            false => TATOIMR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TATOIMR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TATOIMR::DIS
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED14W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 262143;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMABIM`"]
pub enum DMABIMW {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl DMABIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMABIMW::EN => true,
            DMABIMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMABIMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMABIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMABIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(DMABIMW::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMABIMW::DIS)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED12W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED12W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `TBMIM`"]
pub enum TBMIMW {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl TBMIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBMIMW::EN => true,
            TBMIMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TBMIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBMIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TBMIMW::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBMIMW::DIS)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CBEIM`"]
pub enum CBEIMW {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl CBEIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CBEIMW::EN => true,
            CBEIMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CBEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _CBEIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CBEIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CBEIMW::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CBEIMW::DIS)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CBMIM`"]
pub enum CBMIMW {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl CBMIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CBMIMW::EN => true,
            CBMIMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CBMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _CBMIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CBMIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CBMIMW::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CBMIMW::DIS)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TBTOIM`"]
pub enum TBTOIMW {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl TBTOIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBTOIMW::EN => true,
            TBTOIMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBTOIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TBTOIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBTOIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TBTOIMW::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBTOIMW::DIS)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED6W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAAIM`"]
pub enum DMAAIMW {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl DMAAIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAAIMW::EN => true,
            DMAAIMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAAIMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAAIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAAIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(DMAAIMW::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(DMAAIMW::DIS)
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
#[doc = "Values that can be written to the field `TAMIM`"]
pub enum TAMIMW {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl TAMIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAMIMW::EN => true,
            TAMIMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TAMIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAMIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TAMIMW::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAMIMW::DIS)
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
#[doc = r" Proxy"]
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Values that can be written to the field `CAEIM`"]
pub enum CAEIMW {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl CAEIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAEIMW::EN => true,
            CAEIMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _CAEIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAEIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CAEIMW::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CAEIMW::DIS)
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
#[doc = "Values that can be written to the field `CAMIM`"]
pub enum CAMIMW {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl CAMIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAMIMW::EN => true,
            CAMIMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _CAMIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAMIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CAMIMW::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CAMIMW::DIS)
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
#[doc = "Values that can be written to the field `TATOIM`"]
pub enum TATOIMW {
    #[doc = "Enable Interrupt"]
    EN,
    #[doc = "Disable Interrupt"]
    DIS,
}
impl TATOIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TATOIMW::EN => true,
            TATOIMW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TATOIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TATOIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TATOIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Interrupt"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TATOIMW::EN)
    }
    #[doc = "Disable Interrupt"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TATOIMW::DIS)
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
    #[doc = "Bits 14:31 - 31:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved14(&self) -> RESERVED14R {
        let bits = {
            const MASK: u32 = 262143;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED14R { bits }
    }
    #[doc = "Bit 13 - 13:13\\] Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
    #[inline]
    pub fn dmabim(&self) -> DMABIMR {
        DMABIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - 12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved12(&self) -> RESERVED12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED12R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
    #[inline]
    pub fn tbmim(&self) -> TBMIMR {
        TBMIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - 10:10\\] Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
    #[inline]
    pub fn cbeim(&self) -> CBEIMR {
        CBEIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - 9:9\\] Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
    #[inline]
    pub fn cbmim(&self) -> CBMIMR {
        CBMIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - 8:8\\] Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
    #[inline]
    pub fn tbtoim(&self) -> TBTOIMR {
        TBTOIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - 7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&self) -> RESERVED6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED6R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
    #[inline]
    pub fn dmaaim(&self) -> DMAAIMR {
        DMAAIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 4:4\\] Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
    #[inline]
    pub fn tamim(&self) -> TAMIMR {
        TAMIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 3:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
    #[inline]
    pub fn caeim(&self) -> CAEIMR {
        CAEIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
    #[inline]
    pub fn camim(&self) -> CAMIMR {
        CAMIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
    #[inline]
    pub fn tatoim(&self) -> TATOIMR {
        TATOIMR::_from({
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
    #[doc = "Bits 14:31 - 31:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved14(&mut self) -> _RESERVED14W {
        _RESERVED14W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Enabling this bit will make the RIS.DMABRIS interrupt propagate to MIS.DMABMIS"]
    #[inline]
    pub fn dmabim(&mut self) -> _DMABIMW {
        _DMABIMW { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved12(&mut self) -> _RESERVED12W {
        _RESERVED12W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Enabling this bit will make the RIS.TBMRIS interrupt propagate to MIS.TBMMIS"]
    #[inline]
    pub fn tbmim(&mut self) -> _TBMIMW {
        _TBMIMW { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Enabling this bit will make the RIS.CBERIS interrupt propagate to MIS.CBEMIS"]
    #[inline]
    pub fn cbeim(&mut self) -> _CBEIMW {
        _CBEIMW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Enabling this bit will make the RIS.CBMRIS interrupt propagate to MIS.CBMMIS"]
    #[inline]
    pub fn cbmim(&mut self) -> _CBMIMW {
        _CBMIMW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Enabling this bit will make the RIS.TBTORIS interrupt propagate to MIS.TBTOMIS"]
    #[inline]
    pub fn tbtoim(&mut self) -> _TBTOIMW {
        _TBTOIMW { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Enabling this bit will make the RIS.DMAARIS interrupt propagate to MIS.DMAAMIS"]
    #[inline]
    pub fn dmaaim(&mut self) -> _DMAAIMW {
        _DMAAIMW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Enabling this bit will make the RIS.TAMRIS interrupt propagate to MIS.TAMMIS"]
    #[inline]
    pub fn tamim(&mut self) -> _TAMIMW {
        _TAMIMW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Enabling this bit will make the RIS.CAERIS interrupt propagate to MIS.CAEMIS"]
    #[inline]
    pub fn caeim(&mut self) -> _CAEIMW {
        _CAEIMW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Enabling this bit will make the RIS.CAMRIS interrupt propagate to MIS.CAMMIS"]
    #[inline]
    pub fn camim(&mut self) -> _CAMIMW {
        _CAMIMW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Enabling this bit will make the RIS.TATORIS interrupt propagate to MIS.TATOMIS"]
    #[inline]
    pub fn tatoim(&mut self) -> _TATOIMW {
        _TATOIMW { w: self }
    }
}
