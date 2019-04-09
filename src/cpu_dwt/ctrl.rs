#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct RESERVED26R {
    bits: u8,
}
impl RESERVED26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NOCYCCNTR {
    bits: bool,
}
impl NOCYCCNTR {
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
pub struct NOPRFCNTR {
    bits: bool,
}
impl NOPRFCNTR {
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
pub struct RESERVED23R {
    bits: bool,
}
impl RESERVED23R {
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
pub struct CYCEVTENAR {
    bits: bool,
}
impl CYCEVTENAR {
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
pub struct FOLDEVTENAR {
    bits: bool,
}
impl FOLDEVTENAR {
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
pub struct LSUEVTENAR {
    bits: bool,
}
impl LSUEVTENAR {
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
pub struct SLEEPEVTENAR {
    bits: bool,
}
impl SLEEPEVTENAR {
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
pub struct EXCEVTENAR {
    bits: bool,
}
impl EXCEVTENAR {
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
pub struct CPIEVTENAR {
    bits: bool,
}
impl CPIEVTENAR {
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
pub struct EXCTRCENAR {
    bits: bool,
}
impl EXCTRCENAR {
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
pub struct RESERVED13R {
    bits: u8,
}
impl RESERVED13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PCSAMPLEENAR {
    bits: bool,
}
impl PCSAMPLEENAR {
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
#[doc = "Possible values of the field `SYNCTAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCTAPR {
    #[doc = "Tap at bit 28 of CYCCNT "]
    BIT28,
    #[doc = "Tap at bit 26 of CYCCNT "]
    BIT26,
    #[doc = "Tap at bit 24 of CYCCNT "]
    BIT24,
    #[doc = "Disabled. No synchronization packets"]
    DIS,
}
impl SYNCTAPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYNCTAPR::BIT28 => 3,
            SYNCTAPR::BIT26 => 2,
            SYNCTAPR::BIT24 => 1,
            SYNCTAPR::DIS => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYNCTAPR {
        match value {
            3 => SYNCTAPR::BIT28,
            2 => SYNCTAPR::BIT26,
            1 => SYNCTAPR::BIT24,
            0 => SYNCTAPR::DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIT28`"]
    #[inline]
    pub fn is_bit28(&self) -> bool {
        *self == SYNCTAPR::BIT28
    }
    #[doc = "Checks if the value of the field is `BIT26`"]
    #[inline]
    pub fn is_bit26(&self) -> bool {
        *self == SYNCTAPR::BIT26
    }
    #[doc = "Checks if the value of the field is `BIT24`"]
    #[inline]
    pub fn is_bit24(&self) -> bool {
        *self == SYNCTAPR::BIT24
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == SYNCTAPR::DIS
    }
}
#[doc = "Possible values of the field `CYCTAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CYCTAPR {
    #[doc = "Selects bit \\[10\\] to tap"]
    BIT10,
    #[doc = "Selects bit \\[6\\] to tap"]
    BIT6,
}
impl CYCTAPR {
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
            CYCTAPR::BIT10 => true,
            CYCTAPR::BIT6 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CYCTAPR {
        match value {
            true => CYCTAPR::BIT10,
            false => CYCTAPR::BIT6,
        }
    }
    #[doc = "Checks if the value of the field is `BIT10`"]
    #[inline]
    pub fn is_bit10(&self) -> bool {
        *self == CYCTAPR::BIT10
    }
    #[doc = "Checks if the value of the field is `BIT6`"]
    #[inline]
    pub fn is_bit6(&self) -> bool {
        *self == CYCTAPR::BIT6
    }
}
#[doc = r" Value of the field"]
pub struct POSTCNTR {
    bits: u8,
}
impl POSTCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct POSTPRESETR {
    bits: u8,
}
impl POSTPRESETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CYCCNTENAR {
    bits: bool,
}
impl CYCCNTENAR {
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
pub struct _RESERVED26W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED26W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOCYCCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _NOCYCCNTW<'a> {
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
pub struct _NOPRFCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _NOPRFCNTW<'a> {
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
pub struct _RESERVED23W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED23W<'a> {
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
pub struct _CYCEVTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _CYCEVTENAW<'a> {
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
pub struct _FOLDEVTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _FOLDEVTENAW<'a> {
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
pub struct _LSUEVTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _LSUEVTENAW<'a> {
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
pub struct _SLEEPEVTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPEVTENAW<'a> {
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
pub struct _EXCEVTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _EXCEVTENAW<'a> {
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
pub struct _CPIEVTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPIEVTENAW<'a> {
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
pub struct _EXCTRCENAW<'a> {
    w: &'a mut W,
}
impl<'a> _EXCTRCENAW<'a> {
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
pub struct _RESERVED13W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED13W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PCSAMPLEENAW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSAMPLEENAW<'a> {
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
#[doc = "Values that can be written to the field `SYNCTAP`"]
pub enum SYNCTAPW {
    #[doc = "Tap at bit 28 of CYCCNT "]
    BIT28,
    #[doc = "Tap at bit 26 of CYCCNT "]
    BIT26,
    #[doc = "Tap at bit 24 of CYCCNT "]
    BIT24,
    #[doc = "Disabled. No synchronization packets"]
    DIS,
}
impl SYNCTAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYNCTAPW::BIT28 => 3,
            SYNCTAPW::BIT26 => 2,
            SYNCTAPW::BIT24 => 1,
            SYNCTAPW::DIS => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCTAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCTAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCTAPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Tap at bit 28 of CYCCNT"]
    #[inline]
    pub fn bit28(self) -> &'a mut W {
        self.variant(SYNCTAPW::BIT28)
    }
    #[doc = "Tap at bit 26 of CYCCNT"]
    #[inline]
    pub fn bit26(self) -> &'a mut W {
        self.variant(SYNCTAPW::BIT26)
    }
    #[doc = "Tap at bit 24 of CYCCNT"]
    #[inline]
    pub fn bit24(self) -> &'a mut W {
        self.variant(SYNCTAPW::BIT24)
    }
    #[doc = "Disabled. No synchronization packets"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(SYNCTAPW::DIS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CYCTAP`"]
pub enum CYCTAPW {
    #[doc = "Selects bit \\[10\\] to tap"]
    BIT10,
    #[doc = "Selects bit \\[6\\] to tap"]
    BIT6,
}
impl CYCTAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CYCTAPW::BIT10 => true,
            CYCTAPW::BIT6 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CYCTAPW<'a> {
    w: &'a mut W,
}
impl<'a> _CYCTAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CYCTAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Selects bit \\[10\\] to tap"]
    #[inline]
    pub fn bit10(self) -> &'a mut W {
        self.variant(CYCTAPW::BIT10)
    }
    #[doc = "Selects bit \\[6\\] to tap"]
    #[inline]
    pub fn bit6(self) -> &'a mut W {
        self.variant(CYCTAPW::BIT6)
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
#[doc = r" Proxy"]
pub struct _POSTCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _POSTCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _POSTPRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _POSTPRESETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CYCCNTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _CYCCNTENAW<'a> {
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
    #[doc = "Bits 26:31 - 31:26\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved26(&self) -> RESERVED26R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED26R { bits }
    }
    #[doc = "Bit 25 - 25:25\\] When set, CYCCNT is not supported."]
    #[inline]
    pub fn nocyccnt(&self) -> NOCYCCNTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOCYCCNTR { bits }
    }
    #[doc = "Bit 24 - 24:24\\] When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
    #[inline]
    pub fn noprfcnt(&self) -> NOPRFCNTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOPRFCNTR { bits }
    }
    #[doc = "Bit 23 - 23:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved23(&self) -> RESERVED23R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED23R { bits }
    }
    #[doc = "Bit 22 - 22:22\\] Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled"]
    #[inline]
    pub fn cycevtena(&self) -> CYCEVTENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CYCEVTENAR { bits }
    }
    #[doc = "Bit 21 - 21:21\\] Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
    #[inline]
    pub fn foldevtena(&self) -> FOLDEVTENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FOLDEVTENAR { bits }
    }
    #[doc = "Bit 20 - 20:20\\] Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
    #[inline]
    pub fn lsuevtena(&self) -> LSUEVTENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LSUEVTENAR { bits }
    }
    #[doc = "Bit 19 - 19:19\\] Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
    #[inline]
    pub fn sleepevtena(&self) -> SLEEPEVTENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLEEPEVTENAR { bits }
    }
    #[doc = "Bit 18 - 18:18\\] Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
    #[inline]
    pub fn excevtena(&self) -> EXCEVTENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXCEVTENAR { bits }
    }
    #[doc = "Bit 17 - 17:17\\] Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
    #[inline]
    pub fn cpievtena(&self) -> CPIEVTENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPIEVTENAR { bits }
    }
    #[doc = "Bit 16 - 16:16\\] Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
    #[inline]
    pub fn exctrcena(&self) -> EXCTRCENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXCTRCENAR { bits }
    }
    #[doc = "Bits 13:15 - 15:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved13(&self) -> RESERVED13R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED13R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
    #[inline]
    pub fn pcsampleena(&self) -> PCSAMPLEENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCSAMPLEENAR { bits }
    }
    #[doc = "Bits 10:11 - 11:10\\] Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
    #[inline]
    pub fn synctap(&self) -> SYNCTAPR {
        SYNCTAPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 9 - 9:9\\] Selects a tap on CYCCNT. These are spaced at bits \\[6\\] and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
    #[inline]
    pub fn cyctap(&self) -> CYCTAPR {
        CYCTAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:8 - 8:5\\] Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
    #[inline]
    pub fn postcnt(&self) -> POSTCNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        POSTCNTR { bits }
    }
    #[doc = "Bits 1:4 - 4:1\\] Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
    #[inline]
    pub fn postpreset(&self) -> POSTPRESETR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        POSTPRESETR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
    #[inline]
    pub fn cyccntena(&self) -> CYCCNTENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CYCCNTENAR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1073741824 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 26:31 - 31:26\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved26(&mut self) -> _RESERVED26W {
        _RESERVED26W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] When set, CYCCNT is not supported."]
    #[inline]
    pub fn nocyccnt(&mut self) -> _NOCYCCNTW {
        _NOCYCCNTW { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] When set, FOLDCNT, LSUCNT, SLEEPCNT, EXCCNT, and CPICNT are not supported."]
    #[inline]
    pub fn noprfcnt(&mut self) -> _NOPRFCNTW {
        _NOPRFCNTW { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved23(&mut self) -> _RESERVED23W {
        _RESERVED23W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Enables Cycle count event. Emits an event when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. This event is only emitted if PCSAMPLEENA is disabled. PCSAMPLEENA overrides the setting of this bit. 0: Cycle count events disabled 1: Cycle count events enabled"]
    #[inline]
    pub fn cycevtena(&mut self) -> _CYCEVTENAW {
        _CYCEVTENAW { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Enables Folded instruction count event. Emits an event when FOLDCNT overflows (every 256 cycles of folded instructions). A folded instruction is one that does not incur even one cycle to execute. For example, an IT instruction is folded away and so does not use up one cycle. 0: Folded instruction count events disabled. 1: Folded instruction count events enabled."]
    #[inline]
    pub fn foldevtena(&mut self) -> _FOLDEVTENAW {
        _FOLDEVTENAW { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Enables LSU count event. Emits an event when LSUCNT overflows (every 256 cycles of LSU operation). LSU counts include all LSU costs after the initial cycle for the instruction. 0: LSU count events disabled. 1: LSU count events enabled."]
    #[inline]
    pub fn lsuevtena(&mut self) -> _LSUEVTENAW {
        _LSUEVTENAW { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Enables Sleep count event. Emits an event when SLEEPCNT overflows (every 256 cycles that the processor is sleeping). 0: Sleep count events disabled. 1: Sleep count events enabled."]
    #[inline]
    pub fn sleepevtena(&mut self) -> _SLEEPEVTENAW {
        _SLEEPEVTENAW { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] Enables Interrupt overhead event. Emits an event when EXCCNT overflows (every 256 cycles of interrupt overhead). 0x0: Interrupt overhead event disabled. 0x1: Interrupt overhead event enabled."]
    #[inline]
    pub fn excevtena(&mut self) -> _EXCEVTENAW {
        _EXCEVTENAW { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Enables CPI count event. Emits an event when CPICNT overflows (every 256 cycles of multi-cycle instructions). 0: CPI counter events disabled. 1: CPI counter events enabled."]
    #[inline]
    pub fn cpievtena(&mut self) -> _CPIEVTENAW {
        _CPIEVTENAW { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Enables Interrupt event tracing. 0: Interrupt event trace disabled. 1: Interrupt event trace enabled."]
    #[inline]
    pub fn exctrcena(&mut self) -> _EXCTRCENAW {
        _EXCTRCENAW { w: self }
    }
    #[doc = "Bits 13:15 - 15:13\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved13(&mut self) -> _RESERVED13W {
        _RESERVED13W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Enables PC Sampling event. A PC sample event is emitted when the POSTCNT counter triggers it. See CYCTAP and POSTPRESET for details. Enabling this bit overrides CYCEVTENA. 0: PC Sampling event disabled. 1: Sampling event enabled."]
    #[inline]
    pub fn pcsampleena(&mut self) -> _PCSAMPLEENAW {
        _PCSAMPLEENAW { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\] Selects a synchronization packet rate. CYCCNTENA and CPU_ITM:TCR.SYNCENA must also be enabled for this feature. Synchronization packets (if enabled) are generated on tap transitions (0 to1 or 1 to 0)."]
    #[inline]
    pub fn synctap(&mut self) -> _SYNCTAPW {
        _SYNCTAPW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Selects a tap on CYCCNT. These are spaced at bits \\[6\\] and \\[10\\]. When the selected bit in CYCCNT changes from 0 to 1 or 1 to 0, it emits into the POSTCNT, post-scalar counter. That counter then counts down. On a bit change when post-scalar is 0, it triggers an event for PC sampling or cycle count event (see details in CYCEVTENA)."]
    #[inline]
    pub fn cyctap(&mut self) -> _CYCTAPW {
        _CYCTAPW { w: self }
    }
    #[doc = "Bits 5:8 - 8:5\\] Post-scalar counter for CYCTAP. When the selected tapped bit changes from 0 to 1 or 1 to 0, the post scalar counter is down-counted when not 0. If 0, it triggers an event for PCSAMPLEENA or CYCEVTENA use. It also reloads with the value from POSTPRESET."]
    #[inline]
    pub fn postcnt(&mut self) -> _POSTCNTW {
        _POSTCNTW { w: self }
    }
    #[doc = "Bits 1:4 - 4:1\\] Reload value for post-scalar counter POSTCNT. When 0, events are triggered on each tap change (a power of 2). If this field has a non-0 value, it forms a count-down value, to be reloaded into POSTCNT each time it reaches 0. For example, a value 1 in this register means an event is formed every other tap change."]
    #[inline]
    pub fn postpreset(&mut self) -> _POSTPRESETW {
        _POSTPRESETW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Enable CYCCNT, allowing it to increment and generate synchronization and count events. If NOCYCCNT = 1, this bit reads zero and ignore writes."]
    #[inline]
    pub fn cyccntena(&mut self) -> _CYCCNTENAW {
        _CYCCNTENAW { w: self }
    }
}
