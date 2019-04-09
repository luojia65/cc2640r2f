#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EVTOMCUPOL {
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
pub struct RESERVED11R {
    bits: u32,
}
impl RESERVED11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `ADC_IRQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_IRQR {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl ADC_IRQR {
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
            ADC_IRQR::LOW => true,
            ADC_IRQR::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_IRQR {
        match value {
            true => ADC_IRQR::LOW,
            false => ADC_IRQR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == ADC_IRQR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == ADC_IRQR::HIGH
    }
}
#[doc = "Possible values of the field `OBSMUX0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OBSMUX0R {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl OBSMUX0R {
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
            OBSMUX0R::LOW => true,
            OBSMUX0R::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OBSMUX0R {
        match value {
            true => OBSMUX0R::LOW,
            false => OBSMUX0R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == OBSMUX0R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == OBSMUX0R::HIGH
    }
}
#[doc = "Possible values of the field `ADC_FIFO_ALMOST_FULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_FIFO_ALMOST_FULLR {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl ADC_FIFO_ALMOST_FULLR {
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
            ADC_FIFO_ALMOST_FULLR::LOW => true,
            ADC_FIFO_ALMOST_FULLR::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_FIFO_ALMOST_FULLR {
        match value {
            true => ADC_FIFO_ALMOST_FULLR::LOW,
            false => ADC_FIFO_ALMOST_FULLR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == ADC_FIFO_ALMOST_FULLR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == ADC_FIFO_ALMOST_FULLR::HIGH
    }
}
#[doc = "Possible values of the field `ADC_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DONER {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl ADC_DONER {
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
            ADC_DONER::LOW => true,
            ADC_DONER::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_DONER {
        match value {
            true => ADC_DONER::LOW,
            false => ADC_DONER::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == ADC_DONER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == ADC_DONER::HIGH
    }
}
#[doc = "Possible values of the field `SMPH_AUTOTAKE_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPH_AUTOTAKE_DONER {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl SMPH_AUTOTAKE_DONER {
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
            SMPH_AUTOTAKE_DONER::LOW => true,
            SMPH_AUTOTAKE_DONER::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMPH_AUTOTAKE_DONER {
        match value {
            true => SMPH_AUTOTAKE_DONER::LOW,
            false => SMPH_AUTOTAKE_DONER::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SMPH_AUTOTAKE_DONER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SMPH_AUTOTAKE_DONER::HIGH
    }
}
#[doc = "Possible values of the field `TIMER1_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1_EVR {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl TIMER1_EVR {
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
            TIMER1_EVR::LOW => true,
            TIMER1_EVR::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER1_EVR {
        match value {
            true => TIMER1_EVR::LOW,
            false => TIMER1_EVR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == TIMER1_EVR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == TIMER1_EVR::HIGH
    }
}
#[doc = "Possible values of the field `TIMER0_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0_EVR {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl TIMER0_EVR {
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
            TIMER0_EVR::LOW => true,
            TIMER0_EVR::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMER0_EVR {
        match value {
            true => TIMER0_EVR::LOW,
            false => TIMER0_EVR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == TIMER0_EVR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == TIMER0_EVR::HIGH
    }
}
#[doc = "Possible values of the field `TDC_DONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDC_DONER {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl TDC_DONER {
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
            TDC_DONER::LOW => true,
            TDC_DONER::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDC_DONER {
        match value {
            true => TDC_DONER::LOW,
            false => TDC_DONER::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == TDC_DONER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == TDC_DONER::HIGH
    }
}
#[doc = "Possible values of the field `AUX_COMPB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPBR {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_COMPBR {
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
            AUX_COMPBR::LOW => true,
            AUX_COMPBR::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_COMPBR {
        match value {
            true => AUX_COMPBR::LOW,
            false => AUX_COMPBR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_COMPBR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_COMPBR::HIGH
    }
}
#[doc = "Possible values of the field `AUX_COMPA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUX_COMPAR {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_COMPAR {
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
            AUX_COMPAR::LOW => true,
            AUX_COMPAR::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUX_COMPAR {
        match value {
            true => AUX_COMPAR::LOW,
            false => AUX_COMPAR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AUX_COMPAR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AUX_COMPAR::HIGH
    }
}
#[doc = "Possible values of the field `AON_WU_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AON_WU_EVR {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AON_WU_EVR {
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
            AON_WU_EVR::LOW => true,
            AON_WU_EVR::HIGH => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AON_WU_EVR {
        match value {
            true => AON_WU_EVR::LOW,
            false => AON_WU_EVR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == AON_WU_EVR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == AON_WU_EVR::HIGH
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED11W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 2097151;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC_IRQ`"]
pub enum ADC_IRQW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl ADC_IRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_IRQW::LOW => true,
            ADC_IRQW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_IRQW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_IRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(ADC_IRQW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(ADC_IRQW::HIGH)
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
#[doc = "Values that can be written to the field `OBSMUX0`"]
pub enum OBSMUX0W {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl OBSMUX0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OBSMUX0W::LOW => true,
            OBSMUX0W::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OBSMUX0W<'a> {
    w: &'a mut W,
}
impl<'a> _OBSMUX0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OBSMUX0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(OBSMUX0W::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(OBSMUX0W::HIGH)
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
#[doc = "Values that can be written to the field `ADC_FIFO_ALMOST_FULL`"]
pub enum ADC_FIFO_ALMOST_FULLW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl ADC_FIFO_ALMOST_FULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_FIFO_ALMOST_FULLW::LOW => true,
            ADC_FIFO_ALMOST_FULLW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_FIFO_ALMOST_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_FIFO_ALMOST_FULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_FIFO_ALMOST_FULLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(ADC_FIFO_ALMOST_FULLW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(ADC_FIFO_ALMOST_FULLW::HIGH)
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
#[doc = "Values that can be written to the field `ADC_DONE`"]
pub enum ADC_DONEW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl ADC_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_DONEW::LOW => true,
            ADC_DONEW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(ADC_DONEW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(ADC_DONEW::HIGH)
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
#[doc = "Values that can be written to the field `SMPH_AUTOTAKE_DONE`"]
pub enum SMPH_AUTOTAKE_DONEW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl SMPH_AUTOTAKE_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMPH_AUTOTAKE_DONEW::LOW => true,
            SMPH_AUTOTAKE_DONEW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMPH_AUTOTAKE_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPH_AUTOTAKE_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMPH_AUTOTAKE_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SMPH_AUTOTAKE_DONEW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SMPH_AUTOTAKE_DONEW::HIGH)
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
#[doc = "Values that can be written to the field `TIMER1_EV`"]
pub enum TIMER1_EVW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl TIMER1_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER1_EVW::LOW => true,
            TIMER1_EVW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER1_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER1_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER1_EVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(TIMER1_EVW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(TIMER1_EVW::HIGH)
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
#[doc = "Values that can be written to the field `TIMER0_EV`"]
pub enum TIMER0_EVW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl TIMER0_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMER0_EVW::LOW => true,
            TIMER0_EVW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMER0_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER0_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER0_EVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(TIMER0_EVW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(TIMER0_EVW::HIGH)
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
#[doc = "Values that can be written to the field `TDC_DONE`"]
pub enum TDC_DONEW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl TDC_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDC_DONEW::LOW => true,
            TDC_DONEW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDC_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDC_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDC_DONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(TDC_DONEW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(TDC_DONEW::HIGH)
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
#[doc = "Values that can be written to the field `AUX_COMPB`"]
pub enum AUX_COMPBW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_COMPBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_COMPBW::LOW => true,
            AUX_COMPBW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_COMPBW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_COMPBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_COMPBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_COMPBW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_COMPBW::HIGH)
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
#[doc = "Values that can be written to the field `AUX_COMPA`"]
pub enum AUX_COMPAW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AUX_COMPAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUX_COMPAW::LOW => true,
            AUX_COMPAW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUX_COMPAW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_COMPAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUX_COMPAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AUX_COMPAW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AUX_COMPAW::HIGH)
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
#[doc = "Values that can be written to the field `AON_WU_EV`"]
pub enum AON_WU_EVW {
    #[doc = "Low level"]
    LOW,
    #[doc = "High level"]
    HIGH,
}
impl AON_WU_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AON_WU_EVW::LOW => true,
            AON_WU_EVW::HIGH => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AON_WU_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _AON_WU_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AON_WU_EVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low level"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(AON_WU_EVW::LOW)
    }
    #[doc = "High level"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(AON_WU_EVW::HIGH)
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
    #[doc = "Bits 11:31 - 31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved11(&self) -> RESERVED11R {
        let bits = {
            const MASK: u32 = 2097151;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED11R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ."]
    #[inline]
    pub fn adc_irq(&self) -> ADC_IRQR {
        ADC_IRQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - 9:9\\] Select the event source level that sets EVTOMCUFLAGS.OBSMUX0."]
    #[inline]
    pub fn obsmux0(&self) -> OBSMUX0R {
        OBSMUX0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - 8:8\\] Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    #[inline]
    pub fn adc_fifo_almost_full(&self) -> ADC_FIFO_ALMOST_FULLR {
        ADC_FIFO_ALMOST_FULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - 7:7\\] Select the event source level that sets EVTOMCUFLAGS.ADC_DONE."]
    #[inline]
    pub fn adc_done(&self) -> ADC_DONER {
        ADC_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - 6:6\\] Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE."]
    #[inline]
    pub fn smph_autotake_done(&self) -> SMPH_AUTOTAKE_DONER {
        SMPH_AUTOTAKE_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - 5:5\\] Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV."]
    #[inline]
    pub fn timer1_ev(&self) -> TIMER1_EVR {
        TIMER1_EVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 4:4\\] Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV."]
    #[inline]
    pub fn timer0_ev(&self) -> TIMER0_EVR {
        TIMER0_EVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - 3:3\\] Select the event source level that sets EVTOMCUFLAGS.TDC_DONE."]
    #[inline]
    pub fn tdc_done(&self) -> TDC_DONER {
        TDC_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - 2:2\\] Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline]
    pub fn aux_compb(&self) -> AUX_COMPBR {
        AUX_COMPBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline]
    pub fn aux_compa(&self) -> AUX_COMPAR {
        AUX_COMPAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV."]
    #[inline]
    pub fn aon_wu_ev(&self) -> AON_WU_EVR {
        AON_WU_EVR::_from({
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
    #[doc = "Bits 11:31 - 31:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved11(&mut self) -> _RESERVED11W {
        _RESERVED11W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Select the event source level that sets EVTOMCUFLAGS.ADC_IRQ."]
    #[inline]
    pub fn adc_irq(&mut self) -> _ADC_IRQW {
        _ADC_IRQW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Select the event source level that sets EVTOMCUFLAGS.OBSMUX0."]
    #[inline]
    pub fn obsmux0(&mut self) -> _OBSMUX0W {
        _OBSMUX0W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Select the event source level that sets EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL."]
    #[inline]
    pub fn adc_fifo_almost_full(&mut self) -> _ADC_FIFO_ALMOST_FULLW {
        _ADC_FIFO_ALMOST_FULLW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Select the event source level that sets EVTOMCUFLAGS.ADC_DONE."]
    #[inline]
    pub fn adc_done(&mut self) -> _ADC_DONEW {
        _ADC_DONEW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Select the event source level that sets EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE."]
    #[inline]
    pub fn smph_autotake_done(&mut self) -> _SMPH_AUTOTAKE_DONEW {
        _SMPH_AUTOTAKE_DONEW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Select the event source level that sets EVTOMCUFLAGS.TIMER1_EV."]
    #[inline]
    pub fn timer1_ev(&mut self) -> _TIMER1_EVW {
        _TIMER1_EVW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Select the event source level that sets EVTOMCUFLAGS.TIMER0_EV."]
    #[inline]
    pub fn timer0_ev(&mut self) -> _TIMER0_EVW {
        _TIMER0_EVW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Select the event source level that sets EVTOMCUFLAGS.TDC_DONE."]
    #[inline]
    pub fn tdc_done(&mut self) -> _TDC_DONEW {
        _TDC_DONEW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Select the event source level that sets EVTOMCUFLAGS.AUX_COMPB."]
    #[inline]
    pub fn aux_compb(&mut self) -> _AUX_COMPBW {
        _AUX_COMPBW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Select the event source level that sets EVTOMCUFLAGS.AUX_COMPA."]
    #[inline]
    pub fn aux_compa(&mut self) -> _AUX_COMPAW {
        _AUX_COMPAW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Select the event source level that sets EVTOMCUFLAGS.AON_WU_EV."]
    #[inline]
    pub fn aon_wu_ev(&mut self) -> _AON_WU_EVW {
        _AON_WU_EVW { w: self }
    }
}
