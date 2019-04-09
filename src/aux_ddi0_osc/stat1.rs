#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT1 {
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
#[doc = "Possible values of the field `RAMPSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMPSTATER {
    #[doc = "FAST_START_SETTLE"]
    FAST_START_SETTLE,
    #[doc = "FAST_START"]
    FAST_START,
    #[doc = "DUMMY_TO_INIT_1"]
    DUMMY_TO_INIT_1,
    #[doc = "IDAC_DECREMENT_WITH_MEASURE"]
    IDAC_DEC_W_MEASURE,
    #[doc = "IBIAS_INCREMENT"]
    IBIAS_INC,
    #[doc = "LPM_UPDATE"]
    LPM_UPDATE,
    #[doc = "IBIAS_DECREMENT_WITH_MEASURE"]
    IBIAS_DEC_W_MEASURE,
    #[doc = "IBIAS_CAP_UPDATE"]
    IBIAS_CAP_UPDATE,
    #[doc = "IDAC_INCREMENT"]
    IDAC_INCREMENT,
    #[doc = "HPM_UPDATE"]
    HPM_UPDATE,
    #[doc = "HPM_RAMP3"]
    HPM_RAMP3,
    #[doc = "HPM_RAMP2"]
    HPM_RAMP2,
    #[doc = "HPM_RAMP1"]
    HPM_RAMP1,
    #[doc = "INITIALIZATION"]
    INITIALIZATION,
    #[doc = "RESET"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAMPSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMPSTATER::FAST_START_SETTLE => 14,
            RAMPSTATER::FAST_START => 13,
            RAMPSTATER::DUMMY_TO_INIT_1 => 12,
            RAMPSTATER::IDAC_DEC_W_MEASURE => 11,
            RAMPSTATER::IBIAS_INC => 10,
            RAMPSTATER::LPM_UPDATE => 9,
            RAMPSTATER::IBIAS_DEC_W_MEASURE => 8,
            RAMPSTATER::IBIAS_CAP_UPDATE => 7,
            RAMPSTATER::IDAC_INCREMENT => 6,
            RAMPSTATER::HPM_UPDATE => 5,
            RAMPSTATER::HPM_RAMP3 => 4,
            RAMPSTATER::HPM_RAMP2 => 3,
            RAMPSTATER::HPM_RAMP1 => 2,
            RAMPSTATER::INITIALIZATION => 1,
            RAMPSTATER::RESET => 0,
            RAMPSTATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMPSTATER {
        match value {
            14 => RAMPSTATER::FAST_START_SETTLE,
            13 => RAMPSTATER::FAST_START,
            12 => RAMPSTATER::DUMMY_TO_INIT_1,
            11 => RAMPSTATER::IDAC_DEC_W_MEASURE,
            10 => RAMPSTATER::IBIAS_INC,
            9 => RAMPSTATER::LPM_UPDATE,
            8 => RAMPSTATER::IBIAS_DEC_W_MEASURE,
            7 => RAMPSTATER::IBIAS_CAP_UPDATE,
            6 => RAMPSTATER::IDAC_INCREMENT,
            5 => RAMPSTATER::HPM_UPDATE,
            4 => RAMPSTATER::HPM_RAMP3,
            3 => RAMPSTATER::HPM_RAMP2,
            2 => RAMPSTATER::HPM_RAMP1,
            1 => RAMPSTATER::INITIALIZATION,
            0 => RAMPSTATER::RESET,
            i => RAMPSTATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FAST_START_SETTLE`"]
    #[inline]
    pub fn is_fast_start_settle(&self) -> bool {
        *self == RAMPSTATER::FAST_START_SETTLE
    }
    #[doc = "Checks if the value of the field is `FAST_START`"]
    #[inline]
    pub fn is_fast_start(&self) -> bool {
        *self == RAMPSTATER::FAST_START
    }
    #[doc = "Checks if the value of the field is `DUMMY_TO_INIT_1`"]
    #[inline]
    pub fn is_dummy_to_init_1(&self) -> bool {
        *self == RAMPSTATER::DUMMY_TO_INIT_1
    }
    #[doc = "Checks if the value of the field is `IDAC_DEC_W_MEASURE`"]
    #[inline]
    pub fn is_idac_dec_w_measure(&self) -> bool {
        *self == RAMPSTATER::IDAC_DEC_W_MEASURE
    }
    #[doc = "Checks if the value of the field is `IBIAS_INC`"]
    #[inline]
    pub fn is_ibias_inc(&self) -> bool {
        *self == RAMPSTATER::IBIAS_INC
    }
    #[doc = "Checks if the value of the field is `LPM_UPDATE`"]
    #[inline]
    pub fn is_lpm_update(&self) -> bool {
        *self == RAMPSTATER::LPM_UPDATE
    }
    #[doc = "Checks if the value of the field is `IBIAS_DEC_W_MEASURE`"]
    #[inline]
    pub fn is_ibias_dec_w_measure(&self) -> bool {
        *self == RAMPSTATER::IBIAS_DEC_W_MEASURE
    }
    #[doc = "Checks if the value of the field is `IBIAS_CAP_UPDATE`"]
    #[inline]
    pub fn is_ibias_cap_update(&self) -> bool {
        *self == RAMPSTATER::IBIAS_CAP_UPDATE
    }
    #[doc = "Checks if the value of the field is `IDAC_INCREMENT`"]
    #[inline]
    pub fn is_idac_increment(&self) -> bool {
        *self == RAMPSTATER::IDAC_INCREMENT
    }
    #[doc = "Checks if the value of the field is `HPM_UPDATE`"]
    #[inline]
    pub fn is_hpm_update(&self) -> bool {
        *self == RAMPSTATER::HPM_UPDATE
    }
    #[doc = "Checks if the value of the field is `HPM_RAMP3`"]
    #[inline]
    pub fn is_hpm_ramp3(&self) -> bool {
        *self == RAMPSTATER::HPM_RAMP3
    }
    #[doc = "Checks if the value of the field is `HPM_RAMP2`"]
    #[inline]
    pub fn is_hpm_ramp2(&self) -> bool {
        *self == RAMPSTATER::HPM_RAMP2
    }
    #[doc = "Checks if the value of the field is `HPM_RAMP1`"]
    #[inline]
    pub fn is_hpm_ramp1(&self) -> bool {
        *self == RAMPSTATER::HPM_RAMP1
    }
    #[doc = "Checks if the value of the field is `INITIALIZATION`"]
    #[inline]
    pub fn is_initialization(&self) -> bool {
        *self == RAMPSTATER::INITIALIZATION
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == RAMPSTATER::RESET
    }
}
#[doc = r" Value of the field"]
pub struct HPM_UPDATE_AMPR {
    bits: u8,
}
impl HPM_UPDATE_AMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPM_UPDATE_AMPR {
    bits: u8,
}
impl LPM_UPDATE_AMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FORCE_RCOSC_HFR {
    bits: bool,
}
impl FORCE_RCOSC_HFR {
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
pub struct SCLK_HF_ENR {
    bits: bool,
}
impl SCLK_HF_ENR {
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
pub struct SCLK_MF_ENR {
    bits: bool,
}
impl SCLK_MF_ENR {
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
pub struct ACLK_ADC_ENR {
    bits: bool,
}
impl ACLK_ADC_ENR {
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
pub struct ACLK_TDC_ENR {
    bits: bool,
}
impl ACLK_TDC_ENR {
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
pub struct ACLK_REF_ENR {
    bits: bool,
}
impl ACLK_REF_ENR {
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
pub struct CLK_CHP_ENR {
    bits: bool,
}
impl CLK_CHP_ENR {
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
pub struct CLK_DCDC_ENR {
    bits: bool,
}
impl CLK_DCDC_ENR {
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
pub struct SCLK_HF_GOODR {
    bits: bool,
}
impl SCLK_HF_GOODR {
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
pub struct SCLK_MF_GOODR {
    bits: bool,
}
impl SCLK_MF_GOODR {
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
pub struct SCLK_LF_GOODR {
    bits: bool,
}
impl SCLK_LF_GOODR {
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
pub struct ACLK_ADC_GOODR {
    bits: bool,
}
impl ACLK_ADC_GOODR {
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
pub struct ACLK_TDC_GOODR {
    bits: bool,
}
impl ACLK_TDC_GOODR {
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
pub struct ACLK_REF_GOODR {
    bits: bool,
}
impl ACLK_REF_GOODR {
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
pub struct CLK_CHP_GOODR {
    bits: bool,
}
impl CLK_CHP_GOODR {
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
pub struct CLK_DCDC_GOODR {
    bits: bool,
}
impl CLK_DCDC_GOODR {
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
#[doc = "Values that can be written to the field `RAMPSTATE`"]
pub enum RAMPSTATEW {
    #[doc = "FAST_START_SETTLE"]
    FAST_START_SETTLE,
    #[doc = "FAST_START"]
    FAST_START,
    #[doc = "DUMMY_TO_INIT_1"]
    DUMMY_TO_INIT_1,
    #[doc = "IDAC_DECREMENT_WITH_MEASURE"]
    IDAC_DEC_W_MEASURE,
    #[doc = "IBIAS_INCREMENT"]
    IBIAS_INC,
    #[doc = "LPM_UPDATE"]
    LPM_UPDATE,
    #[doc = "IBIAS_DECREMENT_WITH_MEASURE"]
    IBIAS_DEC_W_MEASURE,
    #[doc = "IBIAS_CAP_UPDATE"]
    IBIAS_CAP_UPDATE,
    #[doc = "IDAC_INCREMENT"]
    IDAC_INCREMENT,
    #[doc = "HPM_UPDATE"]
    HPM_UPDATE,
    #[doc = "HPM_RAMP3"]
    HPM_RAMP3,
    #[doc = "HPM_RAMP2"]
    HPM_RAMP2,
    #[doc = "HPM_RAMP1"]
    HPM_RAMP1,
    #[doc = "INITIALIZATION"]
    INITIALIZATION,
    #[doc = "RESET"]
    RESET,
}
impl RAMPSTATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAMPSTATEW::FAST_START_SETTLE => 14,
            RAMPSTATEW::FAST_START => 13,
            RAMPSTATEW::DUMMY_TO_INIT_1 => 12,
            RAMPSTATEW::IDAC_DEC_W_MEASURE => 11,
            RAMPSTATEW::IBIAS_INC => 10,
            RAMPSTATEW::LPM_UPDATE => 9,
            RAMPSTATEW::IBIAS_DEC_W_MEASURE => 8,
            RAMPSTATEW::IBIAS_CAP_UPDATE => 7,
            RAMPSTATEW::IDAC_INCREMENT => 6,
            RAMPSTATEW::HPM_UPDATE => 5,
            RAMPSTATEW::HPM_RAMP3 => 4,
            RAMPSTATEW::HPM_RAMP2 => 3,
            RAMPSTATEW::HPM_RAMP1 => 2,
            RAMPSTATEW::INITIALIZATION => 1,
            RAMPSTATEW::RESET => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMPSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMPSTATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMPSTATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FAST_START_SETTLE"]
    #[inline]
    pub fn fast_start_settle(self) -> &'a mut W {
        self.variant(RAMPSTATEW::FAST_START_SETTLE)
    }
    #[doc = "FAST_START"]
    #[inline]
    pub fn fast_start(self) -> &'a mut W {
        self.variant(RAMPSTATEW::FAST_START)
    }
    #[doc = "DUMMY_TO_INIT_1"]
    #[inline]
    pub fn dummy_to_init_1(self) -> &'a mut W {
        self.variant(RAMPSTATEW::DUMMY_TO_INIT_1)
    }
    #[doc = "IDAC_DECREMENT_WITH_MEASURE"]
    #[inline]
    pub fn idac_dec_w_measure(self) -> &'a mut W {
        self.variant(RAMPSTATEW::IDAC_DEC_W_MEASURE)
    }
    #[doc = "IBIAS_INCREMENT"]
    #[inline]
    pub fn ibias_inc(self) -> &'a mut W {
        self.variant(RAMPSTATEW::IBIAS_INC)
    }
    #[doc = "LPM_UPDATE"]
    #[inline]
    pub fn lpm_update(self) -> &'a mut W {
        self.variant(RAMPSTATEW::LPM_UPDATE)
    }
    #[doc = "IBIAS_DECREMENT_WITH_MEASURE"]
    #[inline]
    pub fn ibias_dec_w_measure(self) -> &'a mut W {
        self.variant(RAMPSTATEW::IBIAS_DEC_W_MEASURE)
    }
    #[doc = "IBIAS_CAP_UPDATE"]
    #[inline]
    pub fn ibias_cap_update(self) -> &'a mut W {
        self.variant(RAMPSTATEW::IBIAS_CAP_UPDATE)
    }
    #[doc = "IDAC_INCREMENT"]
    #[inline]
    pub fn idac_increment(self) -> &'a mut W {
        self.variant(RAMPSTATEW::IDAC_INCREMENT)
    }
    #[doc = "HPM_UPDATE"]
    #[inline]
    pub fn hpm_update(self) -> &'a mut W {
        self.variant(RAMPSTATEW::HPM_UPDATE)
    }
    #[doc = "HPM_RAMP3"]
    #[inline]
    pub fn hpm_ramp3(self) -> &'a mut W {
        self.variant(RAMPSTATEW::HPM_RAMP3)
    }
    #[doc = "HPM_RAMP2"]
    #[inline]
    pub fn hpm_ramp2(self) -> &'a mut W {
        self.variant(RAMPSTATEW::HPM_RAMP2)
    }
    #[doc = "HPM_RAMP1"]
    #[inline]
    pub fn hpm_ramp1(self) -> &'a mut W {
        self.variant(RAMPSTATEW::HPM_RAMP1)
    }
    #[doc = "INITIALIZATION"]
    #[inline]
    pub fn initialization(self) -> &'a mut W {
        self.variant(RAMPSTATEW::INITIALIZATION)
    }
    #[doc = "RESET"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(RAMPSTATEW::RESET)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HPM_UPDATE_AMPW<'a> {
    w: &'a mut W,
}
impl<'a> _HPM_UPDATE_AMPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPM_UPDATE_AMPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_UPDATE_AMPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FORCE_RCOSC_HFW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCE_RCOSC_HFW<'a> {
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
pub struct _SCLK_HF_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_HF_ENW<'a> {
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
pub struct _SCLK_MF_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_MF_ENW<'a> {
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
pub struct _ACLK_ADC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACLK_ADC_ENW<'a> {
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
pub struct _ACLK_TDC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACLK_TDC_ENW<'a> {
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
pub struct _ACLK_REF_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACLK_REF_ENW<'a> {
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
pub struct _CLK_CHP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_CHP_ENW<'a> {
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
pub struct _CLK_DCDC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_DCDC_ENW<'a> {
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
pub struct _SCLK_HF_GOODW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_HF_GOODW<'a> {
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
pub struct _SCLK_MF_GOODW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_MF_GOODW<'a> {
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
pub struct _SCLK_LF_GOODW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLK_LF_GOODW<'a> {
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
pub struct _ACLK_ADC_GOODW<'a> {
    w: &'a mut W,
}
impl<'a> _ACLK_ADC_GOODW<'a> {
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
pub struct _ACLK_TDC_GOODW<'a> {
    w: &'a mut W,
}
impl<'a> _ACLK_TDC_GOODW<'a> {
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
pub struct _ACLK_REF_GOODW<'a> {
    w: &'a mut W,
}
impl<'a> _ACLK_REF_GOODW<'a> {
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
pub struct _CLK_CHP_GOODW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_CHP_GOODW<'a> {
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
pub struct _CLK_DCDC_GOODW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_DCDC_GOODW<'a> {
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
    #[doc = "Bits 28:31 - 31:28\\] AMPCOMP FSM State"]
    #[inline]
    pub fn rampstate(&self) -> RAMPSTATER {
        RAMPSTATER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:27 - 27:22\\] OSC amplitude during HPM_UPDATE state. When amplitude compensation of XOSC_HF is enabled in high performance mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline]
    pub fn hpm_update_amp(&self) -> HPM_UPDATE_AMPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HPM_UPDATE_AMPR { bits }
    }
    #[doc = "Bits 16:21 - 21:16\\] OSC amplitude during LPM_UPDATE state When amplitude compensation of XOSC_HF is enabled in low power mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline]
    pub fn lpm_update_amp(&self) -> LPM_UPDATE_AMPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPM_UPDATE_AMPR { bits }
    }
    #[doc = "Bit 15 - 15:15\\] force_rcosc_hf"]
    #[inline]
    pub fn force_rcosc_hf(&self) -> FORCE_RCOSC_HFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FORCE_RCOSC_HFR { bits }
    }
    #[doc = "Bit 14 - 14:14\\] SCLK_HF_EN"]
    #[inline]
    pub fn sclk_hf_en(&self) -> SCLK_HF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLK_HF_ENR { bits }
    }
    #[doc = "Bit 13 - 13:13\\] SCLK_MF_EN"]
    #[inline]
    pub fn sclk_mf_en(&self) -> SCLK_MF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLK_MF_ENR { bits }
    }
    #[doc = "Bit 12 - 12:12\\] ACLK_ADC_EN"]
    #[inline]
    pub fn aclk_adc_en(&self) -> ACLK_ADC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_ADC_ENR { bits }
    }
    #[doc = "Bit 11 - 11:11\\] ACLK_TDC_EN"]
    #[inline]
    pub fn aclk_tdc_en(&self) -> ACLK_TDC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_TDC_ENR { bits }
    }
    #[doc = "Bit 10 - 10:10\\] ACLK_REF_EN"]
    #[inline]
    pub fn aclk_ref_en(&self) -> ACLK_REF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_REF_ENR { bits }
    }
    #[doc = "Bit 9 - 9:9\\] CLK_CHP_EN"]
    #[inline]
    pub fn clk_chp_en(&self) -> CLK_CHP_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK_CHP_ENR { bits }
    }
    #[doc = "Bit 8 - 8:8\\] CLK_DCDC_EN"]
    #[inline]
    pub fn clk_dcdc_en(&self) -> CLK_DCDC_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK_DCDC_ENR { bits }
    }
    #[doc = "Bit 7 - 7:7\\] SCLK_HF_GOOD"]
    #[inline]
    pub fn sclk_hf_good(&self) -> SCLK_HF_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLK_HF_GOODR { bits }
    }
    #[doc = "Bit 6 - 6:6\\] SCLK_MF_GOOD"]
    #[inline]
    pub fn sclk_mf_good(&self) -> SCLK_MF_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLK_MF_GOODR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] SCLK_LF_GOOD"]
    #[inline]
    pub fn sclk_lf_good(&self) -> SCLK_LF_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCLK_LF_GOODR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] ACLK_ADC_GOOD"]
    #[inline]
    pub fn aclk_adc_good(&self) -> ACLK_ADC_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_ADC_GOODR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] ACLK_TDC_GOOD"]
    #[inline]
    pub fn aclk_tdc_good(&self) -> ACLK_TDC_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_TDC_GOODR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] ACLK_REF_GOOD"]
    #[inline]
    pub fn aclk_ref_good(&self) -> ACLK_REF_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACLK_REF_GOODR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] CLK_CHP_GOOD"]
    #[inline]
    pub fn clk_chp_good(&self) -> CLK_CHP_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK_CHP_GOODR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] CLK_DCDC_GOOD"]
    #[inline]
    pub fn clk_dcdc_good(&self) -> CLK_DCDC_GOODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK_DCDC_GOODR { bits }
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
    #[doc = "Bits 28:31 - 31:28\\] AMPCOMP FSM State"]
    #[inline]
    pub fn rampstate(&mut self) -> _RAMPSTATEW {
        _RAMPSTATEW { w: self }
    }
    #[doc = "Bits 22:27 - 27:22\\] OSC amplitude during HPM_UPDATE state. When amplitude compensation of XOSC_HF is enabled in high performance mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline]
    pub fn hpm_update_amp(&mut self) -> _HPM_UPDATE_AMPW {
        _HPM_UPDATE_AMPW { w: self }
    }
    #[doc = "Bits 16:21 - 21:16\\] OSC amplitude during LPM_UPDATE state When amplitude compensation of XOSC_HF is enabled in low power mode, this value is the amplitude of the crystal oscillations measured by the on-chip oscillator ADC, divided by 15 mV. For example, a value of 0x20 would indicate that the amplitude of the crystal is approximately 480 mV. To enable amplitude compensation, AON_WUC OSCCFG must be set to a non-zero value."]
    #[inline]
    pub fn lpm_update_amp(&mut self) -> _LPM_UPDATE_AMPW {
        _LPM_UPDATE_AMPW { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] force_rcosc_hf"]
    #[inline]
    pub fn force_rcosc_hf(&mut self) -> _FORCE_RCOSC_HFW {
        _FORCE_RCOSC_HFW { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] SCLK_HF_EN"]
    #[inline]
    pub fn sclk_hf_en(&mut self) -> _SCLK_HF_ENW {
        _SCLK_HF_ENW { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] SCLK_MF_EN"]
    #[inline]
    pub fn sclk_mf_en(&mut self) -> _SCLK_MF_ENW {
        _SCLK_MF_ENW { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] ACLK_ADC_EN"]
    #[inline]
    pub fn aclk_adc_en(&mut self) -> _ACLK_ADC_ENW {
        _ACLK_ADC_ENW { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] ACLK_TDC_EN"]
    #[inline]
    pub fn aclk_tdc_en(&mut self) -> _ACLK_TDC_ENW {
        _ACLK_TDC_ENW { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] ACLK_REF_EN"]
    #[inline]
    pub fn aclk_ref_en(&mut self) -> _ACLK_REF_ENW {
        _ACLK_REF_ENW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] CLK_CHP_EN"]
    #[inline]
    pub fn clk_chp_en(&mut self) -> _CLK_CHP_ENW {
        _CLK_CHP_ENW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] CLK_DCDC_EN"]
    #[inline]
    pub fn clk_dcdc_en(&mut self) -> _CLK_DCDC_ENW {
        _CLK_DCDC_ENW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] SCLK_HF_GOOD"]
    #[inline]
    pub fn sclk_hf_good(&mut self) -> _SCLK_HF_GOODW {
        _SCLK_HF_GOODW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] SCLK_MF_GOOD"]
    #[inline]
    pub fn sclk_mf_good(&mut self) -> _SCLK_MF_GOODW {
        _SCLK_MF_GOODW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] SCLK_LF_GOOD"]
    #[inline]
    pub fn sclk_lf_good(&mut self) -> _SCLK_LF_GOODW {
        _SCLK_LF_GOODW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] ACLK_ADC_GOOD"]
    #[inline]
    pub fn aclk_adc_good(&mut self) -> _ACLK_ADC_GOODW {
        _ACLK_ADC_GOODW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] ACLK_TDC_GOOD"]
    #[inline]
    pub fn aclk_tdc_good(&mut self) -> _ACLK_TDC_GOODW {
        _ACLK_TDC_GOODW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] ACLK_REF_GOOD"]
    #[inline]
    pub fn aclk_ref_good(&mut self) -> _ACLK_REF_GOODW {
        _ACLK_REF_GOODW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] CLK_CHP_GOOD"]
    #[inline]
    pub fn clk_chp_good(&mut self) -> _CLK_CHP_GOODW {
        _CLK_CHP_GOODW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] CLK_DCDC_GOOD"]
    #[inline]
    pub fn clk_dcdc_good(&mut self) -> _CLK_DCDC_GOODW {
        _CLK_DCDC_GOODW { w: self }
    }
}
