#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
pub struct RESERVED15R {
    bits: u32,
}
impl RESERVED15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `TBPWML`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBPWMLR {
    #[doc = "Inverted"]
    INVERTED,
    #[doc = "Not inverted"]
    NORMAL,
}
impl TBPWMLR {
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
            TBPWMLR::INVERTED => true,
            TBPWMLR::NORMAL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBPWMLR {
        match value {
            true => TBPWMLR::INVERTED,
            false => TBPWMLR::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TBPWMLR::INVERTED
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TBPWMLR::NORMAL
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED12R {
    bits: u8,
}
impl RESERVED12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TBEVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBEVENTR {
    #[doc = "Both edges"]
    BOTH,
    #[doc = "Negative edge "]
    NEG,
    #[doc = "Positive edge"]
    POS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TBEVENTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TBEVENTR::BOTH => 3,
            TBEVENTR::NEG => 1,
            TBEVENTR::POS => 0,
            TBEVENTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TBEVENTR {
        match value {
            3 => TBEVENTR::BOTH,
            1 => TBEVENTR::NEG,
            0 => TBEVENTR::POS,
            i => TBEVENTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == TBEVENTR::BOTH
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline]
    pub fn is_neg(&self) -> bool {
        *self == TBEVENTR::NEG
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline]
    pub fn is_pos(&self) -> bool {
        *self == TBEVENTR::POS
    }
}
#[doc = "Possible values of the field `TBSTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBSTALLR {
    #[doc = "Timer B freezes counting while the processor is halted by the debugger. "]
    EN,
    #[doc = "Timer B continues counting while the processor is halted by the debugger. "]
    DIS,
}
impl TBSTALLR {
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
            TBSTALLR::EN => true,
            TBSTALLR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBSTALLR {
        match value {
            true => TBSTALLR::EN,
            false => TBSTALLR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TBSTALLR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TBSTALLR::DIS
    }
}
#[doc = "Possible values of the field `TBEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBENR {
    #[doc = "Timer B is enabled and begins counting or the capture logic is enabled based on CFG register. "]
    EN,
    #[doc = "Timer B is disabled. "]
    DIS,
}
impl TBENR {
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
            TBENR::EN => true,
            TBENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TBENR {
        match value {
            true => TBENR::EN,
            false => TBENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TBENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TBENR::DIS
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED7R {
    bits: bool,
}
impl RESERVED7R {
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
#[doc = "Possible values of the field `TAPWML`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAPWMLR {
    #[doc = "Inverted"]
    INVERTED,
    #[doc = "Not inverted"]
    NORMAL,
}
impl TAPWMLR {
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
            TAPWMLR::INVERTED => true,
            TAPWMLR::NORMAL => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAPWMLR {
        match value {
            true => TAPWMLR::INVERTED,
            false => TAPWMLR::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TAPWMLR::INVERTED
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TAPWMLR::NORMAL
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED4R {
    bits: u8,
}
impl RESERVED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TAEVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAEVENTR {
    #[doc = "Both edges"]
    BOTH,
    #[doc = "Negative edge "]
    NEG,
    #[doc = "Positive edge"]
    POS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TAEVENTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TAEVENTR::BOTH => 3,
            TAEVENTR::NEG => 1,
            TAEVENTR::POS => 0,
            TAEVENTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TAEVENTR {
        match value {
            3 => TAEVENTR::BOTH,
            1 => TAEVENTR::NEG,
            0 => TAEVENTR::POS,
            i => TAEVENTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == TAEVENTR::BOTH
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline]
    pub fn is_neg(&self) -> bool {
        *self == TAEVENTR::NEG
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline]
    pub fn is_pos(&self) -> bool {
        *self == TAEVENTR::POS
    }
}
#[doc = "Possible values of the field `TASTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASTALLR {
    #[doc = "Timer A freezes counting while the processor is halted by the debugger. "]
    EN,
    #[doc = "Timer A continues counting while the processor is halted by the debugger. "]
    DIS,
}
impl TASTALLR {
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
            TASTALLR::EN => true,
            TASTALLR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TASTALLR {
        match value {
            true => TASTALLR::EN,
            false => TASTALLR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TASTALLR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TASTALLR::DIS
    }
}
#[doc = "Possible values of the field `TAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAENR {
    #[doc = "Timer A is enabled and begins counting or the capture logic is enabled based on the CFG register. "]
    EN,
    #[doc = "Timer A is disabled. "]
    DIS,
}
impl TAENR {
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
            TAENR::EN => true,
            TAENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAENR {
        match value {
            true => TAENR::EN,
            false => TAENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TAENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TAENR::DIS
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED15W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED15W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 131071;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TBPWML`"]
pub enum TBPWMLW {
    #[doc = "Inverted"]
    INVERTED,
    #[doc = "Not inverted"]
    NORMAL,
}
impl TBPWMLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBPWMLW::INVERTED => true,
            TBPWMLW::NORMAL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBPWMLW<'a> {
    w: &'a mut W,
}
impl<'a> _TBPWMLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBPWMLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inverted"]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TBPWMLW::INVERTED)
    }
    #[doc = "Not inverted"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TBPWMLW::NORMAL)
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
        const OFFSET: u8 = 14;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TBEVENT`"]
pub enum TBEVENTW {
    #[doc = "Both edges"]
    BOTH,
    #[doc = "Negative edge "]
    NEG,
    #[doc = "Positive edge"]
    POS,
}
impl TBEVENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TBEVENTW::BOTH => 3,
            TBEVENTW::NEG => 1,
            TBEVENTW::POS => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBEVENTW<'a> {
    w: &'a mut W,
}
impl<'a> _TBEVENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBEVENTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Both edges"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(TBEVENTW::BOTH)
    }
    #[doc = "Negative edge"]
    #[inline]
    pub fn neg(self) -> &'a mut W {
        self.variant(TBEVENTW::NEG)
    }
    #[doc = "Positive edge"]
    #[inline]
    pub fn pos(self) -> &'a mut W {
        self.variant(TBEVENTW::POS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TBSTALL`"]
pub enum TBSTALLW {
    #[doc = "Timer B freezes counting while the processor is halted by the debugger. "]
    EN,
    #[doc = "Timer B continues counting while the processor is halted by the debugger. "]
    DIS,
}
impl TBSTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBSTALLW::EN => true,
            TBSTALLW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _TBSTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBSTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer B freezes counting while the processor is halted by the debugger."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TBSTALLW::EN)
    }
    #[doc = "Timer B continues counting while the processor is halted by the debugger."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBSTALLW::DIS)
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
#[doc = "Values that can be written to the field `TBEN`"]
pub enum TBENW {
    #[doc = "Timer B is enabled and begins counting or the capture logic is enabled based on CFG register. "]
    EN,
    #[doc = "Timer B is disabled. "]
    DIS,
}
impl TBENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TBENW::EN => true,
            TBENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBENW<'a> {
    w: &'a mut W,
}
impl<'a> _TBENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer B is enabled and begins counting or the capture logic is enabled based on CFG register."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TBENW::EN)
    }
    #[doc = "Timer B is disabled."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBENW::DIS)
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
pub struct _RESERVED7W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED7W<'a> {
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
#[doc = "Values that can be written to the field `TAPWML`"]
pub enum TAPWMLW {
    #[doc = "Inverted"]
    INVERTED,
    #[doc = "Not inverted"]
    NORMAL,
}
impl TAPWMLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAPWMLW::INVERTED => true,
            TAPWMLW::NORMAL => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAPWMLW<'a> {
    w: &'a mut W,
}
impl<'a> _TAPWMLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAPWMLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inverted"]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TAPWMLW::INVERTED)
    }
    #[doc = "Not inverted"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TAPWMLW::NORMAL)
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
#[doc = r" Proxy"]
pub struct _RESERVED4W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TAEVENT`"]
pub enum TAEVENTW {
    #[doc = "Both edges"]
    BOTH,
    #[doc = "Negative edge "]
    NEG,
    #[doc = "Positive edge"]
    POS,
}
impl TAEVENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TAEVENTW::BOTH => 3,
            TAEVENTW::NEG => 1,
            TAEVENTW::POS => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAEVENTW<'a> {
    w: &'a mut W,
}
impl<'a> _TAEVENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAEVENTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Both edges"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(TAEVENTW::BOTH)
    }
    #[doc = "Negative edge"]
    #[inline]
    pub fn neg(self) -> &'a mut W {
        self.variant(TAEVENTW::NEG)
    }
    #[doc = "Positive edge"]
    #[inline]
    pub fn pos(self) -> &'a mut W {
        self.variant(TAEVENTW::POS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TASTALL`"]
pub enum TASTALLW {
    #[doc = "Timer A freezes counting while the processor is halted by the debugger. "]
    EN,
    #[doc = "Timer A continues counting while the processor is halted by the debugger. "]
    DIS,
}
impl TASTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TASTALLW::EN => true,
            TASTALLW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TASTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _TASTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TASTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer A freezes counting while the processor is halted by the debugger."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TASTALLW::EN)
    }
    #[doc = "Timer A continues counting while the processor is halted by the debugger."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TASTALLW::DIS)
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
#[doc = "Values that can be written to the field `TAEN`"]
pub enum TAENW {
    #[doc = "Timer A is enabled and begins counting or the capture logic is enabled based on the CFG register. "]
    EN,
    #[doc = "Timer A is disabled. "]
    DIS,
}
impl TAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAENW::EN => true,
            TAENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer A is enabled and begins counting or the capture logic is enabled based on the CFG register."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TAENW::EN)
    }
    #[doc = "Timer A is disabled."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAENW::DIS)
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
    #[doc = "Bits 15:31 - 31:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved15(&self) -> RESERVED15R {
        let bits = {
            const MASK: u32 = 131071;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED15R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
    #[inline]
    pub fn tbpwml(&self) -> TBPWMLR {
        TBPWMLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - 13:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved12(&self) -> RESERVED12R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED12R { bits }
    }
    #[doc = "Bits 10:11 - 11:10\\] GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline]
    pub fn tbevent(&self) -> TBEVENTR {
        TBEVENTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 9 - 9:9\\] GPT Timer B Stall Enable"]
    #[inline]
    pub fn tbstall(&self) -> TBSTALLR {
        TBSTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - 8:8\\] GPT Timer B Enable"]
    #[inline]
    pub fn tben(&self) -> TBENR {
        TBENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - 7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved7(&self) -> RESERVED7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED7R { bits }
    }
    #[doc = "Bit 6 - 6:6\\] GPT Timer A PWM Output Level"]
    #[inline]
    pub fn tapwml(&self) -> TAPWMLR {
        TAPWMLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - 5:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED4R { bits }
    }
    #[doc = "Bits 2:3 - 3:2\\] GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline]
    pub fn taevent(&self) -> TAEVENTR {
        TAEVENTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 1 - 1:1\\] GPT Timer A Stall Enable"]
    #[inline]
    pub fn tastall(&self) -> TASTALLR {
        TASTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] GPT Timer A Enable"]
    #[inline]
    pub fn taen(&self) -> TAENR {
        TAENR::_from({
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
    #[doc = "Bits 15:31 - 31:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved15(&mut self) -> _RESERVED15W {
        _RESERVED15W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] GPT Timer B PWM Output Level 0: Output is unaffected. 1: Output is inverted."]
    #[inline]
    pub fn tbpwml(&mut self) -> _TBPWMLW {
        _TBPWMLW { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved12(&mut self) -> _RESERVED12W {
        _RESERVED12W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\] GPT Timer B Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline]
    pub fn tbevent(&mut self) -> _TBEVENTW {
        _TBEVENTW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] GPT Timer B Stall Enable"]
    #[inline]
    pub fn tbstall(&mut self) -> _TBSTALLW {
        _TBSTALLW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] GPT Timer B Enable"]
    #[inline]
    pub fn tben(&mut self) -> _TBENW {
        _TBENW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved7(&mut self) -> _RESERVED7W {
        _RESERVED7W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] GPT Timer A PWM Output Level"]
    #[inline]
    pub fn tapwml(&mut self) -> _TAPWMLW {
        _TAPWMLW { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] GPT Timer A Event Mode The values in this register are defined as follows: Value Description 0x0 Positive edge 0x1 Negative edge 0x2 Reserved 0x3 Both edges Note: If PWM output inversion is enabled, edge detection interrupt behavior is reversed. Thus, if a positive-edge interrupt trigger has been set and the PWM inversion generates a postive edge, no event-trigger interrupt asserts. Instead, the interrupt is generated on the negative edge of the PWM signal."]
    #[inline]
    pub fn taevent(&mut self) -> _TAEVENTW {
        _TAEVENTW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] GPT Timer A Stall Enable"]
    #[inline]
    pub fn tastall(&mut self) -> _TASTALLW {
        _TASTALLW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] GPT Timer A Enable"]
    #[inline]
    pub fn taen(&mut self) -> _TAENW {
        _TAENW { w: self }
    }
}
