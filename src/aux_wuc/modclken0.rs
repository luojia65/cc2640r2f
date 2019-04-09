#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODCLKEN0 {
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
#[doc = "Possible values of the field `AUX_ADI4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_ADI4R {
    #[doc = "System CPU has requested clock for AUX_ADI4"]
    EN,
    #[doc = "System CPU has not requested clock for AUX_ADI4"]
    DIS,
}
impl AUX_ADI4R {
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
            AUX_ADI4R::EN => true,
            AUX_ADI4R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_ADI4R {
        match value {
            true => AUX_ADI4R::EN,
            false => AUX_ADI4R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == AUX_ADI4R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == AUX_ADI4R::DIS
    }
}
#[doc = "Possible values of the field `AUX_DDI0_OSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_DDI0_OSCR {
    #[doc = "System CPU has requested clock for AUX_DDI0_OSC"]
    EN,
    #[doc = "System CPU has not requested clock for AUX_DDI0_OSC"]
    DIS,
}
impl AUX_DDI0_OSCR {
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
            AUX_DDI0_OSCR::EN => true,
            AUX_DDI0_OSCR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_DDI0_OSCR {
        match value {
            true => AUX_DDI0_OSCR::EN,
            false => AUX_DDI0_OSCR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == AUX_DDI0_OSCR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == AUX_DDI0_OSCR::DIS
    }
}
#[doc = "Possible values of the field `TDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDCR {
    #[doc = "System CPU has requested clock for TDC"]
    EN,
    #[doc = "System CPU has not requested clock for TDC"]
    DIS,
}
impl TDCR {
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
            TDCR::EN => true,
            TDCR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDCR {
        match value {
            true => TDCR::EN,
            false => TDCR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TDCR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TDCR::DIS
    }
}
#[doc = "Possible values of the field `ANAIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANAIFR {
    #[doc = "System CPU has requested clock for ANAIF"]
    EN,
    #[doc = "System CPU has not requested clock for ANAIF"]
    DIS,
}
impl ANAIFR {
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
            ANAIFR::EN => true,
            ANAIFR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANAIFR {
        match value {
            true => ANAIFR::EN,
            false => ANAIFR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == ANAIFR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ANAIFR::DIS
    }
}
#[doc = "Possible values of the field `TIMER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMERR {
    #[doc = "System CPU has requested clock for TIMER"]
    EN,
    #[doc = "System CPU has not requested clock for TIMER"]
    DIS,
}
impl TIMERR {
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
            TIMERR::EN => true,
            TIMERR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMERR {
        match value {
            true => TIMERR::EN,
            false => TIMERR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TIMERR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TIMERR::DIS
    }
}
#[doc = "Possible values of the field `AIODIO1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIODIO1R {
    #[doc = "System CPU has requested clock for AIODIO1"]
    EN,
    #[doc = "System CPU has not requested clock for AIODIO1"]
    DIS,
}
impl AIODIO1R {
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
            AIODIO1R::EN => true,
            AIODIO1R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AIODIO1R {
        match value {
            true => AIODIO1R::EN,
            false => AIODIO1R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == AIODIO1R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == AIODIO1R::DIS
    }
}
#[doc = "Possible values of the field `AIODIO0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIODIO0R {
    #[doc = "System CPU has requested clock for AIODIO0"]
    EN,
    #[doc = "System CPU has not requested clock for AIODIO0"]
    DIS,
}
impl AIODIO0R {
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
            AIODIO0R::EN => true,
            AIODIO0R::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AIODIO0R {
        match value {
            true => AIODIO0R::EN,
            false => AIODIO0R::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == AIODIO0R::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == AIODIO0R::DIS
    }
}
#[doc = "Possible values of the field `SMPH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPHR {
    #[doc = "System CPU has requested clock for SMPH"]
    EN,
    #[doc = "System CPU has not requested clock for SMPH"]
    DIS,
}
impl SMPHR {
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
            SMPHR::EN => true,
            SMPHR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMPHR {
        match value {
            true => SMPHR::EN,
            false => SMPHR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == SMPHR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == SMPHR::DIS
    }
}
#[doc = "Values that can be written to the field `AUX_ADI4`"]
pub enum AUX_ADI4W {
    #[doc = "System CPU has requested clock for AUX_ADI4"]
    EN,
    #[doc = "System CPU has not requested clock for AUX_ADI4"]
    DIS,
}
impl AUX_ADI4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_ADI4W::EN => true,
            AUX_ADI4W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_ADI4W<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_ADI4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_ADI4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System CPU has requested clock for AUX_ADI4"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(AUX_ADI4W::EN)
    }
    #[doc = "System CPU has not requested clock for AUX_ADI4"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(AUX_ADI4W::DIS)
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
#[doc = "Values that can be written to the field `AUX_DDI0_OSC`"]
pub enum AUX_DDI0_OSCW {
    #[doc = "System CPU has requested clock for AUX_DDI0_OSC"]
    EN,
    #[doc = "System CPU has not requested clock for AUX_DDI0_OSC"]
    DIS,
}
impl AUX_DDI0_OSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_DDI0_OSCW::EN => true,
            AUX_DDI0_OSCW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_DDI0_OSCW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_DDI0_OSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_DDI0_OSCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System CPU has requested clock for AUX_DDI0_OSC"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(AUX_DDI0_OSCW::EN)
    }
    #[doc = "System CPU has not requested clock for AUX_DDI0_OSC"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(AUX_DDI0_OSCW::DIS)
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
#[doc = "Values that can be written to the field `TDC`"]
pub enum TDCW {
    #[doc = "System CPU has requested clock for TDC"]
    EN,
    #[doc = "System CPU has not requested clock for TDC"]
    DIS,
}
impl TDCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDCW::EN => true,
            TDCW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDCW<'a> {
    w: &'a mut W,
}
impl<'a> _TDCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System CPU has requested clock for TDC"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TDCW::EN)
    }
    #[doc = "System CPU has not requested clock for TDC"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TDCW::DIS)
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
#[doc = "Values that can be written to the field `ANAIF`"]
pub enum ANAIFW {
    #[doc = "System CPU has requested clock for ANAIF"]
    EN,
    #[doc = "System CPU has not requested clock for ANAIF"]
    DIS,
}
impl ANAIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANAIFW::EN => true,
            ANAIFW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANAIFW<'a> {
    w: &'a mut W,
}
impl<'a> _ANAIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANAIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System CPU has requested clock for ANAIF"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(ANAIFW::EN)
    }
    #[doc = "System CPU has not requested clock for ANAIF"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ANAIFW::DIS)
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
#[doc = "Values that can be written to the field `TIMER`"]
pub enum TIMERW {
    #[doc = "System CPU has requested clock for TIMER"]
    EN,
    #[doc = "System CPU has not requested clock for TIMER"]
    DIS,
}
impl TIMERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMERW::EN => true,
            TIMERW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMERW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System CPU has requested clock for TIMER"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TIMERW::EN)
    }
    #[doc = "System CPU has not requested clock for TIMER"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TIMERW::DIS)
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
#[doc = "Values that can be written to the field `AIODIO1`"]
pub enum AIODIO1W {
    #[doc = "System CPU has requested clock for AIODIO1"]
    EN,
    #[doc = "System CPU has not requested clock for AIODIO1"]
    DIS,
}
impl AIODIO1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIODIO1W::EN => true,
            AIODIO1W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIODIO1W<'a> {
    w: &'a mut W,
}
impl<'a> _AIODIO1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIODIO1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System CPU has requested clock for AIODIO1"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(AIODIO1W::EN)
    }
    #[doc = "System CPU has not requested clock for AIODIO1"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(AIODIO1W::DIS)
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
#[doc = "Values that can be written to the field `AIODIO0`"]
pub enum AIODIO0W {
    #[doc = "System CPU has requested clock for AIODIO0"]
    EN,
    #[doc = "System CPU has not requested clock for AIODIO0"]
    DIS,
}
impl AIODIO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AIODIO0W::EN => true,
            AIODIO0W::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AIODIO0W<'a> {
    w: &'a mut W,
}
impl<'a> _AIODIO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AIODIO0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System CPU has requested clock for AIODIO0"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(AIODIO0W::EN)
    }
    #[doc = "System CPU has not requested clock for AIODIO0"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(AIODIO0W::DIS)
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
#[doc = "Values that can be written to the field `SMPH`"]
pub enum SMPHW {
    #[doc = "System CPU has requested clock for SMPH"]
    EN,
    #[doc = "System CPU has not requested clock for SMPH"]
    DIS,
}
impl SMPHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMPHW::EN => true,
            SMPHW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMPHW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMPHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System CPU has requested clock for SMPH"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(SMPHW::EN)
    }
    #[doc = "System CPU has not requested clock for SMPH"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(SMPHW::DIS)
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
    #[doc = "Bit 7 - 7:7\\] Enables (1) or disables (0) clock for AUX_ADI4."]
    #[inline]
    pub fn aux_adi4(&self) -> AUX_ADI4R {
        AUX_ADI4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - 6:6\\] Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
    #[inline]
    pub fn aux_ddi0_osc(&self) -> AUX_DDI0_OSCR {
        AUX_DDI0_OSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - 5:5\\] Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively."]
    #[inline]
    pub fn tdc(&self) -> TDCR {
        TDCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 4:4\\] Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL."]
    #[inline]
    pub fn anaif(&self) -> ANAIFR {
        ANAIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 3:3\\] Enables (1) or disables (0) clock for AUX_TIMER."]
    #[inline]
    pub fn timer(&self) -> TIMERR {
        TIMERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - 2:2\\] Enables (1) or disables (0) clock for AUX_AIODIO1."]
    #[inline]
    pub fn aiodio1(&self) -> AIODIO1R {
        AIODIO1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] Enables (1) or disables (0) clock for AUX_AIODIO0."]
    #[inline]
    pub fn aiodio0(&self) -> AIODIO0R {
        AIODIO0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Enables (1) or disables (0) clock for AUX_SMPH."]
    #[inline]
    pub fn smph(&self) -> SMPHR {
        SMPHR::_from({
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
    #[doc = "Bit 7 - 7:7\\] Enables (1) or disables (0) clock for AUX_ADI4."]
    #[inline]
    pub fn aux_adi4(&mut self) -> _AUX_ADI4W {
        _AUX_ADI4W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Enables (1) or disables (0) clock for AUX_DDI0_OSC."]
    #[inline]
    pub fn aux_ddi0_osc(&mut self) -> _AUX_DDI0_OSCW {
        _AUX_DDI0_OSCW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Enables (1) or disables (0) clock for AUX_TDCIF. Note that the TDC counter and reference clock sources must be requested separately using TDCCLKCTL and REFCLKCTL, respectively."]
    #[inline]
    pub fn tdc(&mut self) -> _TDCW {
        _TDCW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Enables (1) or disables (0) clock for AUX_ANAIF. Note that the ADC internal clock must be requested separately using ADCCLKCTL."]
    #[inline]
    pub fn anaif(&mut self) -> _ANAIFW {
        _ANAIFW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Enables (1) or disables (0) clock for AUX_TIMER."]
    #[inline]
    pub fn timer(&mut self) -> _TIMERW {
        _TIMERW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Enables (1) or disables (0) clock for AUX_AIODIO1."]
    #[inline]
    pub fn aiodio1(&mut self) -> _AIODIO1W {
        _AIODIO1W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Enables (1) or disables (0) clock for AUX_AIODIO0."]
    #[inline]
    pub fn aiodio0(&mut self) -> _AIODIO0W {
        _AIODIO0W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Enables (1) or disables (0) clock for AUX_SMPH."]
    #[inline]
    pub fn smph(&mut self) -> _SMPHW {
        _SMPHW { w: self }
    }
}
