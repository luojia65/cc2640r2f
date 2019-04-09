#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRECTL {
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
#[doc = "Possible values of the field `RATIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATIOR {
    #[doc = "Prescaler divides input by 64. \n\nAUX_TDC_PRE event has a rising edge for every 64 rising edges of the input. AUX_TDC_PRE event toggles on every 32nd rising edge of the input. "]
    DIV64,
    #[doc = "Prescaler divides input by 16. \n\nAUX_TDC_PRE event has a rising edge for every 16 rising edges of the input. AUX_TDC_PRE event toggles on every 8th rising edge of the input. "]
    DIV16,
}
impl RATIOR {
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
            RATIOR::DIV64 => true,
            RATIOR::DIV16 => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RATIOR {
        match value {
            true => RATIOR::DIV64,
            false => RATIOR::DIV16,
        }
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == RATIOR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == RATIOR::DIV16
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED5R {
    bits: bool,
}
impl RESERVED5R {
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
#[doc = "Possible values of the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCR {
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    ADC_IRQ,
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    MCU_EV,
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF"]
    ACLK_REF,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15"]
    AUXIO15,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14 "]
    AUXIO14,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13 "]
    AUXIO13,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12 "]
    AUXIO12,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11"]
    AUXIO11,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10"]
    AUXIO10,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9  "]
    AUXIO9,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8  "]
    AUXIO8,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7  "]
    AUXIO7,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6  "]
    AUXIO6,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5  "]
    AUXIO5,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4  "]
    AUXIO4,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3  "]
    AUXIO3,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    AUXIO2,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    AUXIO1,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    AUXIO0,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    AON_PROG_WU,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW"]
    AON_SW,
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX1"]
    OBSMUX1,
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX0"]
    OBSMUX0,
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    ADC_FIFO_ALMOST_FULL,
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE"]
    ADC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    TIMER1_EV,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    TIMER0_EV,
    #[doc = "AUX_ANAIF:ISRCCTL.RESET_N"]
    ISRC_RESET,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    AON_RTC_CH2,
}
impl SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCR::ADC_IRQ => 31,
            SRCR::MCU_EV => 30,
            SRCR::ACLK_REF => 29,
            SRCR::AUXIO15 => 28,
            SRCR::AUXIO14 => 27,
            SRCR::AUXIO13 => 26,
            SRCR::AUXIO12 => 25,
            SRCR::AUXIO11 => 24,
            SRCR::AUXIO10 => 23,
            SRCR::AUXIO9 => 22,
            SRCR::AUXIO8 => 21,
            SRCR::AUXIO7 => 20,
            SRCR::AUXIO6 => 19,
            SRCR::AUXIO5 => 18,
            SRCR::AUXIO4 => 17,
            SRCR::AUXIO3 => 16,
            SRCR::AUXIO2 => 15,
            SRCR::AUXIO1 => 14,
            SRCR::AUXIO0 => 13,
            SRCR::AON_PROG_WU => 12,
            SRCR::AON_SW => 11,
            SRCR::OBSMUX1 => 10,
            SRCR::OBSMUX0 => 9,
            SRCR::ADC_FIFO_ALMOST_FULL => 8,
            SRCR::ADC_DONE => 7,
            SRCR::SMPH_AUTOTAKE_DONE => 6,
            SRCR::TIMER1_EV => 5,
            SRCR::TIMER0_EV => 4,
            SRCR::ISRC_RESET => 3,
            SRCR::AUX_COMPB => 2,
            SRCR::AUX_COMPA => 1,
            SRCR::AON_RTC_CH2 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCR {
        match value {
            31 => SRCR::ADC_IRQ,
            30 => SRCR::MCU_EV,
            29 => SRCR::ACLK_REF,
            28 => SRCR::AUXIO15,
            27 => SRCR::AUXIO14,
            26 => SRCR::AUXIO13,
            25 => SRCR::AUXIO12,
            24 => SRCR::AUXIO11,
            23 => SRCR::AUXIO10,
            22 => SRCR::AUXIO9,
            21 => SRCR::AUXIO8,
            20 => SRCR::AUXIO7,
            19 => SRCR::AUXIO6,
            18 => SRCR::AUXIO5,
            17 => SRCR::AUXIO4,
            16 => SRCR::AUXIO3,
            15 => SRCR::AUXIO2,
            14 => SRCR::AUXIO1,
            13 => SRCR::AUXIO0,
            12 => SRCR::AON_PROG_WU,
            11 => SRCR::AON_SW,
            10 => SRCR::OBSMUX1,
            9 => SRCR::OBSMUX0,
            8 => SRCR::ADC_FIFO_ALMOST_FULL,
            7 => SRCR::ADC_DONE,
            6 => SRCR::SMPH_AUTOTAKE_DONE,
            5 => SRCR::TIMER1_EV,
            4 => SRCR::TIMER0_EV,
            3 => SRCR::ISRC_RESET,
            2 => SRCR::AUX_COMPB,
            1 => SRCR::AUX_COMPA,
            0 => SRCR::AON_RTC_CH2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_IRQ`"]
    #[inline]
    pub fn is_adc_irq(&self) -> bool {
        *self == SRCR::ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline]
    pub fn is_mcu_ev(&self) -> bool {
        *self == SRCR::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline]
    pub fn is_aclk_ref(&self) -> bool {
        *self == SRCR::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline]
    pub fn is_auxio15(&self) -> bool {
        *self == SRCR::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline]
    pub fn is_auxio14(&self) -> bool {
        *self == SRCR::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline]
    pub fn is_auxio13(&self) -> bool {
        *self == SRCR::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline]
    pub fn is_auxio12(&self) -> bool {
        *self == SRCR::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline]
    pub fn is_auxio11(&self) -> bool {
        *self == SRCR::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline]
    pub fn is_auxio10(&self) -> bool {
        *self == SRCR::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline]
    pub fn is_auxio9(&self) -> bool {
        *self == SRCR::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline]
    pub fn is_auxio8(&self) -> bool {
        *self == SRCR::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == SRCR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == SRCR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == SRCR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == SRCR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == SRCR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == SRCR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == SRCR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == SRCR::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AON_PROG_WU`"]
    #[inline]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == SRCR::AON_PROG_WU
    }
    #[doc = "Checks if the value of the field is `AON_SW`"]
    #[inline]
    pub fn is_aon_sw(&self) -> bool {
        *self == SRCR::AON_SW
    }
    #[doc = "Checks if the value of the field is `OBSMUX1`"]
    #[inline]
    pub fn is_obsmux1(&self) -> bool {
        *self == SRCR::OBSMUX1
    }
    #[doc = "Checks if the value of the field is `OBSMUX0`"]
    #[inline]
    pub fn is_obsmux0(&self) -> bool {
        *self == SRCR::OBSMUX0
    }
    #[doc = "Checks if the value of the field is `ADC_FIFO_ALMOST_FULL`"]
    #[inline]
    pub fn is_adc_fifo_almost_full(&self) -> bool {
        *self == SRCR::ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `ADC_DONE`"]
    #[inline]
    pub fn is_adc_done(&self) -> bool {
        *self == SRCR::ADC_DONE
    }
    #[doc = "Checks if the value of the field is `SMPH_AUTOTAKE_DONE`"]
    #[inline]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == SRCR::SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `TIMER1_EV`"]
    #[inline]
    pub fn is_timer1_ev(&self) -> bool {
        *self == SRCR::TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `TIMER0_EV`"]
    #[inline]
    pub fn is_timer0_ev(&self) -> bool {
        *self == SRCR::TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `ISRC_RESET`"]
    #[inline]
    pub fn is_isrc_reset(&self) -> bool {
        *self == SRCR::ISRC_RESET
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == SRCR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == SRCR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == SRCR::AON_RTC_CH2
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RATIO`"]
pub enum RATIOW {
    #[doc = "Prescaler divides input by 64. \n\nAUX_TDC_PRE event has a rising edge for every 64 rising edges of the input. AUX_TDC_PRE event toggles on every 32nd rising edge of the input. "]
    DIV64,
    #[doc = "Prescaler divides input by 16. \n\nAUX_TDC_PRE event has a rising edge for every 16 rising edges of the input. AUX_TDC_PRE event toggles on every 8th rising edge of the input. "]
    DIV16,
}
impl RATIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RATIOW::DIV64 => true,
            RATIOW::DIV16 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RATIOW<'a> {
    w: &'a mut W,
}
impl<'a> _RATIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RATIOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prescaler divides input by 64. AUX_TDC_PRE event has a rising edge for every 64 rising edges of the input. AUX_TDC_PRE event toggles on every 32nd rising edge of the input."]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(RATIOW::DIV64)
    }
    #[doc = "Prescaler divides input by 16. AUX_TDC_PRE event has a rising edge for every 16 rising edges of the input. AUX_TDC_PRE event toggles on every 8th rising edge of the input."]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(RATIOW::DIV16)
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
pub struct _RESERVED5W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED5W<'a> {
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
#[doc = "Values that can be written to the field `SRC`"]
pub enum SRCW {
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    ADC_IRQ,
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    MCU_EV,
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF"]
    ACLK_REF,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15"]
    AUXIO15,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14 "]
    AUXIO14,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13 "]
    AUXIO13,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12 "]
    AUXIO12,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11"]
    AUXIO11,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10"]
    AUXIO10,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9  "]
    AUXIO9,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8  "]
    AUXIO8,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7  "]
    AUXIO7,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6  "]
    AUXIO6,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5  "]
    AUXIO5,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4  "]
    AUXIO4,
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3  "]
    AUXIO3,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    AUXIO2,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    AUXIO1,
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    AUXIO0,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    AON_PROG_WU,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW"]
    AON_SW,
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX1"]
    OBSMUX1,
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX0"]
    OBSMUX0,
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    ADC_FIFO_ALMOST_FULL,
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE"]
    ADC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    TIMER1_EV,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    TIMER0_EV,
    #[doc = "AUX_ANAIF:ISRCCTL.RESET_N"]
    ISRC_RESET,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    AON_RTC_CH2,
}
impl SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCW::ADC_IRQ => 31,
            SRCW::MCU_EV => 30,
            SRCW::ACLK_REF => 29,
            SRCW::AUXIO15 => 28,
            SRCW::AUXIO14 => 27,
            SRCW::AUXIO13 => 26,
            SRCW::AUXIO12 => 25,
            SRCW::AUXIO11 => 24,
            SRCW::AUXIO10 => 23,
            SRCW::AUXIO9 => 22,
            SRCW::AUXIO8 => 21,
            SRCW::AUXIO7 => 20,
            SRCW::AUXIO6 => 19,
            SRCW::AUXIO5 => 18,
            SRCW::AUXIO4 => 17,
            SRCW::AUXIO3 => 16,
            SRCW::AUXIO2 => 15,
            SRCW::AUXIO1 => 14,
            SRCW::AUXIO0 => 13,
            SRCW::AON_PROG_WU => 12,
            SRCW::AON_SW => 11,
            SRCW::OBSMUX1 => 10,
            SRCW::OBSMUX0 => 9,
            SRCW::ADC_FIFO_ALMOST_FULL => 8,
            SRCW::ADC_DONE => 7,
            SRCW::SMPH_AUTOTAKE_DONE => 6,
            SRCW::TIMER1_EV => 5,
            SRCW::TIMER0_EV => 4,
            SRCW::ISRC_RESET => 3,
            SRCW::AUX_COMPB => 2,
            SRCW::AUX_COMPA => 1,
            SRCW::AON_RTC_CH2 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    #[inline]
    pub fn adc_irq(self) -> &'a mut W {
        self.variant(SRCW::ADC_IRQ)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    #[inline]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(SRCW::MCU_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF"]
    #[inline]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(SRCW::ACLK_REF)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15"]
    #[inline]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(SRCW::AUXIO15)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14"]
    #[inline]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(SRCW::AUXIO14)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13"]
    #[inline]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(SRCW::AUXIO13)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12"]
    #[inline]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(SRCW::AUXIO12)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11"]
    #[inline]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(SRCW::AUXIO11)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10"]
    #[inline]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(SRCW::AUXIO10)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9"]
    #[inline]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(SRCW::AUXIO9)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8"]
    #[inline]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(SRCW::AUXIO8)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7"]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(SRCW::AUXIO7)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6"]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(SRCW::AUXIO6)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5"]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(SRCW::AUXIO5)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4"]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(SRCW::AUXIO4)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3"]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(SRCW::AUXIO3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(SRCW::AUXIO2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(SRCW::AUXIO1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(SRCW::AUXIO0)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    #[inline]
    pub fn aon_prog_wu(self) -> &'a mut W {
        self.variant(SRCW::AON_PROG_WU)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW"]
    #[inline]
    pub fn aon_sw(self) -> &'a mut W {
        self.variant(SRCW::AON_SW)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX1"]
    #[inline]
    pub fn obsmux1(self) -> &'a mut W {
        self.variant(SRCW::OBSMUX1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX0"]
    #[inline]
    pub fn obsmux0(self) -> &'a mut W {
        self.variant(SRCW::OBSMUX0)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline]
    pub fn adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(SRCW::ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE"]
    #[inline]
    pub fn adc_done(self) -> &'a mut W {
        self.variant(SRCW::ADC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline]
    pub fn smph_autotake_done(self) -> &'a mut W {
        self.variant(SRCW::SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    #[inline]
    pub fn timer1_ev(self) -> &'a mut W {
        self.variant(SRCW::TIMER1_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    #[inline]
    pub fn timer0_ev(self) -> &'a mut W {
        self.variant(SRCW::TIMER0_EV)
    }
    #[doc = "AUX_ANAIF:ISRCCTL.RESET_N"]
    #[inline]
    pub fn isrc_reset(self) -> &'a mut W {
        self.variant(SRCW::ISRC_RESET)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(SRCW::AUX_COMPB)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(SRCW::AUX_COMPA)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    #[inline]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(SRCW::AON_RTC_CH2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bit 7 - 7:7\\] Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
    #[inline]
    pub fn reset_n(&self) -> RESET_NR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESET_NR { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
    #[inline]
    pub fn ratio(&self) -> RATIOR {
        RATIOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - 5:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved5(&self) -> RESERVED5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED5R { bits }
    }
    #[doc = "Bits 0:4 - 4:0\\] Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 31 }
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
    #[doc = "Bit 7 - 7:7\\] Prescaler reset. 0: Reset prescaler. 1: Release reset of prescaler. AUX_TDC_PRE event becomes 0 when you reset the prescaler."]
    #[inline]
    pub fn reset_n(&mut self) -> _RESET_NW {
        _RESET_NW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Prescaler ratio. This controls how often the AUX_TDC_PRE event is generated by the prescaler."]
    #[inline]
    pub fn ratio(&mut self) -> _RATIOW {
        _RATIOW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved5(&mut self) -> _RESERVED5W {
        _RESERVED5W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\] Prescaler event source. Select an event from the asynchronous AUX event bus to connect to the prescaler input. Configure only while RESET_N is 0."]
    #[inline]
    pub fn src(&mut self) -> _SRCW {
        _SRCW { w: self }
    }
}
