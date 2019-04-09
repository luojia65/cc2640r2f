#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::ADC0 {
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
pub struct SMPL_MODER {
    bits: bool,
}
impl SMPL_MODER {
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
#[doc = "Possible values of the field `SMPL_CYCLE_EXP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPL_CYCLE_EXPR {
    #[doc = "65536x 6 MHz clock periods = 10.9ms"]
    _10P9_MS,
    #[doc = "32768x 6 MHz clock periods = 5.46ms"]
    _5P46_MS,
    #[doc = "16384x 6 MHz clock periods = 2.73ms"]
    _2P73_MS,
    #[doc = "8192x 6 MHz clock periods = 1.37ms"]
    _1P37_MS,
    #[doc = "4096x 6 MHz clock periods = 682us"]
    _682_US,
    #[doc = "2048x 6 MHz clock periods = 341us"]
    _341_US,
    #[doc = "1024x 6 MHz clock periods = 170us"]
    _170_US,
    #[doc = "512x 6 MHz clock periods = 85.3us"]
    _85P3_US,
    #[doc = "256x 6 MHz clock periods = 42.6us"]
    _42P6_US,
    #[doc = "128x 6 MHz clock periods = 21.3us"]
    _21P3_US,
    #[doc = "64x 6 MHz clock periods = 10.6us"]
    _10P6_US,
    #[doc = "32x 6 MHz clock periods = 5.3us"]
    _5P3_US,
    #[doc = "16x 6 MHz clock periods = 2.7us"]
    _2P7_US,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SMPL_CYCLE_EXPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMPL_CYCLE_EXPR::_10P9_MS => 15,
            SMPL_CYCLE_EXPR::_5P46_MS => 14,
            SMPL_CYCLE_EXPR::_2P73_MS => 13,
            SMPL_CYCLE_EXPR::_1P37_MS => 12,
            SMPL_CYCLE_EXPR::_682_US => 11,
            SMPL_CYCLE_EXPR::_341_US => 10,
            SMPL_CYCLE_EXPR::_170_US => 9,
            SMPL_CYCLE_EXPR::_85P3_US => 8,
            SMPL_CYCLE_EXPR::_42P6_US => 7,
            SMPL_CYCLE_EXPR::_21P3_US => 6,
            SMPL_CYCLE_EXPR::_10P6_US => 5,
            SMPL_CYCLE_EXPR::_5P3_US => 4,
            SMPL_CYCLE_EXPR::_2P7_US => 3,
            SMPL_CYCLE_EXPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMPL_CYCLE_EXPR {
        match value {
            15 => SMPL_CYCLE_EXPR::_10P9_MS,
            14 => SMPL_CYCLE_EXPR::_5P46_MS,
            13 => SMPL_CYCLE_EXPR::_2P73_MS,
            12 => SMPL_CYCLE_EXPR::_1P37_MS,
            11 => SMPL_CYCLE_EXPR::_682_US,
            10 => SMPL_CYCLE_EXPR::_341_US,
            9 => SMPL_CYCLE_EXPR::_170_US,
            8 => SMPL_CYCLE_EXPR::_85P3_US,
            7 => SMPL_CYCLE_EXPR::_42P6_US,
            6 => SMPL_CYCLE_EXPR::_21P3_US,
            5 => SMPL_CYCLE_EXPR::_10P6_US,
            4 => SMPL_CYCLE_EXPR::_5P3_US,
            3 => SMPL_CYCLE_EXPR::_2P7_US,
            i => SMPL_CYCLE_EXPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_10P9_MS`"]
    #[inline]
    pub fn is_10p9_ms(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_10P9_MS
    }
    #[doc = "Checks if the value of the field is `_5P46_MS`"]
    #[inline]
    pub fn is_5p46_ms(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_5P46_MS
    }
    #[doc = "Checks if the value of the field is `_2P73_MS`"]
    #[inline]
    pub fn is_2p73_ms(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_2P73_MS
    }
    #[doc = "Checks if the value of the field is `_1P37_MS`"]
    #[inline]
    pub fn is_1p37_ms(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_1P37_MS
    }
    #[doc = "Checks if the value of the field is `_682_US`"]
    #[inline]
    pub fn is_682_us(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_682_US
    }
    #[doc = "Checks if the value of the field is `_341_US`"]
    #[inline]
    pub fn is_341_us(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_341_US
    }
    #[doc = "Checks if the value of the field is `_170_US`"]
    #[inline]
    pub fn is_170_us(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_170_US
    }
    #[doc = "Checks if the value of the field is `_85P3_US`"]
    #[inline]
    pub fn is_85p3_us(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_85P3_US
    }
    #[doc = "Checks if the value of the field is `_42P6_US`"]
    #[inline]
    pub fn is_42p6_us(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_42P6_US
    }
    #[doc = "Checks if the value of the field is `_21P3_US`"]
    #[inline]
    pub fn is_21p3_us(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_21P3_US
    }
    #[doc = "Checks if the value of the field is `_10P6_US`"]
    #[inline]
    pub fn is_10p6_us(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_10P6_US
    }
    #[doc = "Checks if the value of the field is `_5P3_US`"]
    #[inline]
    pub fn is_5p3_us(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_5P3_US
    }
    #[doc = "Checks if the value of the field is `_2P7_US`"]
    #[inline]
    pub fn is_2p7_us(&self) -> bool {
        *self == SMPL_CYCLE_EXPR::_2P7_US
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED2R {
    bits: bool,
}
impl RESERVED2R {
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
pub struct RESET_NR {
    bits: bool,
}
impl RESET_NR {
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
pub struct ENR {
    bits: bool,
}
impl ENR {
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
pub struct _SMPL_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPL_MODEW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMPL_CYCLE_EXP`"]
pub enum SMPL_CYCLE_EXPW {
    #[doc = "65536x 6 MHz clock periods = 10.9ms"]
    _10P9_MS,
    #[doc = "32768x 6 MHz clock periods = 5.46ms"]
    _5P46_MS,
    #[doc = "16384x 6 MHz clock periods = 2.73ms"]
    _2P73_MS,
    #[doc = "8192x 6 MHz clock periods = 1.37ms"]
    _1P37_MS,
    #[doc = "4096x 6 MHz clock periods = 682us"]
    _682_US,
    #[doc = "2048x 6 MHz clock periods = 341us"]
    _341_US,
    #[doc = "1024x 6 MHz clock periods = 170us"]
    _170_US,
    #[doc = "512x 6 MHz clock periods = 85.3us"]
    _85P3_US,
    #[doc = "256x 6 MHz clock periods = 42.6us"]
    _42P6_US,
    #[doc = "128x 6 MHz clock periods = 21.3us"]
    _21P3_US,
    #[doc = "64x 6 MHz clock periods = 10.6us"]
    _10P6_US,
    #[doc = "32x 6 MHz clock periods = 5.3us"]
    _5P3_US,
    #[doc = "16x 6 MHz clock periods = 2.7us"]
    _2P7_US,
}
impl SMPL_CYCLE_EXPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMPL_CYCLE_EXPW::_10P9_MS => 15,
            SMPL_CYCLE_EXPW::_5P46_MS => 14,
            SMPL_CYCLE_EXPW::_2P73_MS => 13,
            SMPL_CYCLE_EXPW::_1P37_MS => 12,
            SMPL_CYCLE_EXPW::_682_US => 11,
            SMPL_CYCLE_EXPW::_341_US => 10,
            SMPL_CYCLE_EXPW::_170_US => 9,
            SMPL_CYCLE_EXPW::_85P3_US => 8,
            SMPL_CYCLE_EXPW::_42P6_US => 7,
            SMPL_CYCLE_EXPW::_21P3_US => 6,
            SMPL_CYCLE_EXPW::_10P6_US => 5,
            SMPL_CYCLE_EXPW::_5P3_US => 4,
            SMPL_CYCLE_EXPW::_2P7_US => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMPL_CYCLE_EXPW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPL_CYCLE_EXPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMPL_CYCLE_EXPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "65536x 6 MHz clock periods = 10.9ms"]
    #[inline]
    pub fn _10p9_ms(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_10P9_MS)
    }
    #[doc = "32768x 6 MHz clock periods = 5.46ms"]
    #[inline]
    pub fn _5p46_ms(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_5P46_MS)
    }
    #[doc = "16384x 6 MHz clock periods = 2.73ms"]
    #[inline]
    pub fn _2p73_ms(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_2P73_MS)
    }
    #[doc = "8192x 6 MHz clock periods = 1.37ms"]
    #[inline]
    pub fn _1p37_ms(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_1P37_MS)
    }
    #[doc = "4096x 6 MHz clock periods = 682us"]
    #[inline]
    pub fn _682_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_682_US)
    }
    #[doc = "2048x 6 MHz clock periods = 341us"]
    #[inline]
    pub fn _341_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_341_US)
    }
    #[doc = "1024x 6 MHz clock periods = 170us"]
    #[inline]
    pub fn _170_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_170_US)
    }
    #[doc = "512x 6 MHz clock periods = 85.3us"]
    #[inline]
    pub fn _85p3_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_85P3_US)
    }
    #[doc = "256x 6 MHz clock periods = 42.6us"]
    #[inline]
    pub fn _42p6_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_42P6_US)
    }
    #[doc = "128x 6 MHz clock periods = 21.3us"]
    #[inline]
    pub fn _21p3_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_21P3_US)
    }
    #[doc = "64x 6 MHz clock periods = 10.6us"]
    #[inline]
    pub fn _10p6_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_10P6_US)
    }
    #[doc = "32x 6 MHz clock periods = 5.3us"]
    #[inline]
    pub fn _5p3_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_5P3_US)
    }
    #[doc = "16x 6 MHz clock periods = 2.7us"]
    #[inline]
    pub fn _2p7_us(self) -> &'a mut W {
        self.variant(SMPL_CYCLE_EXPW::_2P7_US)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESET_NW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_NW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 7 - 7:7\\] ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal"]
    #[inline]
    pub fn smpl_mode(&self) -> SMPL_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        SMPL_MODER { bits }
    }
    #[doc = "Bits 3:6 - 6:3\\] Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
    #[inline]
    pub fn smpl_cycle_exp(&self) -> SMPL_CYCLE_EXPR {
        SMPL_CYCLE_EXPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 2 - 2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        RESERVED2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation"]
    #[inline]
    pub fn reset_n(&self) -> RESET_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        RESET_NR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] ADC Enable 0: Disable 1: Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        ENR { bits }
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 7 - 7:7\\] ADC Sampling mode: 0: Synchronous mode 1: Asynchronous mode The ADC does a sample-and-hold before conversion. In synchronous mode the sampling starts when the ADC clock detects a rising edge on the trigger signal. Jitter/uncertainty will be inferred in the detection if the trigger signal originates from a domain that is asynchronous to the ADC clock. SMPL_CYCLE_EXP determines the the duration of sampling. Conversion starts immediately after sampling ends. In asynchronous mode the sampling is continuous when enabled. Sampling ends and conversion starts immediately with the rising edge of the trigger signal. Sampling restarts when the conversion has finished. Asynchronous mode is useful when it is important to avoid jitter in the sampling instant of an externally driven signal"]
    #[inline]
    pub fn smpl_mode(&mut self) -> _SMPL_MODEW {
        _SMPL_MODEW { w: self }
    }
    #[doc = "Bits 3:6 - 6:3\\] Controls the sampling duration before conversion when the ADC is operated in synchronous mode (SMPL_MODE = 0). The setting has no effect in asynchronous mode. The sampling duration is given as 2^(SMPL_CYCLE_EXP + 1) / 6 us."]
    #[inline]
    pub fn smpl_cycle_exp(&mut self) -> _SMPL_CYCLE_EXPW {
        _SMPL_CYCLE_EXPW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Reset ADC digital subchip, active low. ADC must be reset every time it is reconfigured. 0: Reset 1: Normal operation"]
    #[inline]
    pub fn reset_n(&mut self) -> _RESET_NW {
        _RESET_NW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] ADC Enable 0: Disable 1: Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
}
