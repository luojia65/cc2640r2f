#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::T0CFG {
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
#[doc = "Possible values of the field `TICK_SRC_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICK_SRC_POLR {
    #[doc = "Count on falling edges of TICK_SRC."]
    FALL,
    #[doc = "Count on rising edges of TICK_SRC."]
    RISE,
}
impl TICK_SRC_POLR {
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
            TICK_SRC_POLR::FALL => true,
            TICK_SRC_POLR::RISE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TICK_SRC_POLR {
        match value {
            true => TICK_SRC_POLR::FALL,
            false => TICK_SRC_POLR::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == TICK_SRC_POLR::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == TICK_SRC_POLR::RISE
    }
}
#[doc = "Possible values of the field `TICK_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICK_SRCR {
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    ADC_IRQ,
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    MCU_EVENT,
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
    #[doc = "AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    RTC_4KHZ,
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE"]
    ADC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    TIMER1_EV,
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE"]
    TDC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    RTC_CH2_EV,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TICK_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TICK_SRCR::ADC_IRQ => 31,
            TICK_SRCR::MCU_EVENT => 30,
            TICK_SRCR::ACLK_REF => 29,
            TICK_SRCR::AUXIO15 => 28,
            TICK_SRCR::AUXIO14 => 27,
            TICK_SRCR::AUXIO13 => 26,
            TICK_SRCR::AUXIO12 => 25,
            TICK_SRCR::AUXIO11 => 24,
            TICK_SRCR::AUXIO10 => 23,
            TICK_SRCR::AUXIO9 => 22,
            TICK_SRCR::AUXIO8 => 21,
            TICK_SRCR::AUXIO7 => 20,
            TICK_SRCR::AUXIO6 => 19,
            TICK_SRCR::AUXIO5 => 18,
            TICK_SRCR::AUXIO4 => 17,
            TICK_SRCR::AUXIO3 => 16,
            TICK_SRCR::AUXIO2 => 15,
            TICK_SRCR::AUXIO1 => 14,
            TICK_SRCR::AUXIO0 => 13,
            TICK_SRCR::AON_PROG_WU => 12,
            TICK_SRCR::AON_SW => 11,
            TICK_SRCR::OBSMUX1 => 10,
            TICK_SRCR::OBSMUX0 => 9,
            TICK_SRCR::RTC_4KHZ => 8,
            TICK_SRCR::ADC_DONE => 7,
            TICK_SRCR::SMPH_AUTOTAKE_DONE => 6,
            TICK_SRCR::TIMER1_EV => 5,
            TICK_SRCR::TDC_DONE => 3,
            TICK_SRCR::AUX_COMPB => 2,
            TICK_SRCR::AUX_COMPA => 1,
            TICK_SRCR::RTC_CH2_EV => 0,
            TICK_SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TICK_SRCR {
        match value {
            31 => TICK_SRCR::ADC_IRQ,
            30 => TICK_SRCR::MCU_EVENT,
            29 => TICK_SRCR::ACLK_REF,
            28 => TICK_SRCR::AUXIO15,
            27 => TICK_SRCR::AUXIO14,
            26 => TICK_SRCR::AUXIO13,
            25 => TICK_SRCR::AUXIO12,
            24 => TICK_SRCR::AUXIO11,
            23 => TICK_SRCR::AUXIO10,
            22 => TICK_SRCR::AUXIO9,
            21 => TICK_SRCR::AUXIO8,
            20 => TICK_SRCR::AUXIO7,
            19 => TICK_SRCR::AUXIO6,
            18 => TICK_SRCR::AUXIO5,
            17 => TICK_SRCR::AUXIO4,
            16 => TICK_SRCR::AUXIO3,
            15 => TICK_SRCR::AUXIO2,
            14 => TICK_SRCR::AUXIO1,
            13 => TICK_SRCR::AUXIO0,
            12 => TICK_SRCR::AON_PROG_WU,
            11 => TICK_SRCR::AON_SW,
            10 => TICK_SRCR::OBSMUX1,
            9 => TICK_SRCR::OBSMUX0,
            8 => TICK_SRCR::RTC_4KHZ,
            7 => TICK_SRCR::ADC_DONE,
            6 => TICK_SRCR::SMPH_AUTOTAKE_DONE,
            5 => TICK_SRCR::TIMER1_EV,
            3 => TICK_SRCR::TDC_DONE,
            2 => TICK_SRCR::AUX_COMPB,
            1 => TICK_SRCR::AUX_COMPA,
            0 => TICK_SRCR::RTC_CH2_EV,
            i => TICK_SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_IRQ`"]
    #[inline]
    pub fn is_adc_irq(&self) -> bool {
        *self == TICK_SRCR::ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `MCU_EVENT`"]
    #[inline]
    pub fn is_mcu_event(&self) -> bool {
        *self == TICK_SRCR::MCU_EVENT
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline]
    pub fn is_aclk_ref(&self) -> bool {
        *self == TICK_SRCR::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline]
    pub fn is_auxio15(&self) -> bool {
        *self == TICK_SRCR::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline]
    pub fn is_auxio14(&self) -> bool {
        *self == TICK_SRCR::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline]
    pub fn is_auxio13(&self) -> bool {
        *self == TICK_SRCR::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline]
    pub fn is_auxio12(&self) -> bool {
        *self == TICK_SRCR::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline]
    pub fn is_auxio11(&self) -> bool {
        *self == TICK_SRCR::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline]
    pub fn is_auxio10(&self) -> bool {
        *self == TICK_SRCR::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline]
    pub fn is_auxio9(&self) -> bool {
        *self == TICK_SRCR::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline]
    pub fn is_auxio8(&self) -> bool {
        *self == TICK_SRCR::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == TICK_SRCR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == TICK_SRCR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == TICK_SRCR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == TICK_SRCR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == TICK_SRCR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == TICK_SRCR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == TICK_SRCR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == TICK_SRCR::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AON_PROG_WU`"]
    #[inline]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == TICK_SRCR::AON_PROG_WU
    }
    #[doc = "Checks if the value of the field is `AON_SW`"]
    #[inline]
    pub fn is_aon_sw(&self) -> bool {
        *self == TICK_SRCR::AON_SW
    }
    #[doc = "Checks if the value of the field is `OBSMUX1`"]
    #[inline]
    pub fn is_obsmux1(&self) -> bool {
        *self == TICK_SRCR::OBSMUX1
    }
    #[doc = "Checks if the value of the field is `OBSMUX0`"]
    #[inline]
    pub fn is_obsmux0(&self) -> bool {
        *self == TICK_SRCR::OBSMUX0
    }
    #[doc = "Checks if the value of the field is `RTC_4KHZ`"]
    #[inline]
    pub fn is_rtc_4khz(&self) -> bool {
        *self == TICK_SRCR::RTC_4KHZ
    }
    #[doc = "Checks if the value of the field is `ADC_DONE`"]
    #[inline]
    pub fn is_adc_done(&self) -> bool {
        *self == TICK_SRCR::ADC_DONE
    }
    #[doc = "Checks if the value of the field is `SMPH_AUTOTAKE_DONE`"]
    #[inline]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == TICK_SRCR::SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `TIMER1_EV`"]
    #[inline]
    pub fn is_timer1_ev(&self) -> bool {
        *self == TICK_SRCR::TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `TDC_DONE`"]
    #[inline]
    pub fn is_tdc_done(&self) -> bool {
        *self == TICK_SRCR::TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == TICK_SRCR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == TICK_SRCR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_EV`"]
    #[inline]
    pub fn is_rtc_ch2_ev(&self) -> bool {
        *self == TICK_SRCR::RTC_CH2_EV
    }
}
#[doc = r" Value of the field"]
pub struct PRER {
    bits: u8,
}
impl PRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED2R {
    bits: u8,
}
impl RESERVED2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Use event set by TICK_SRC as source for prescaler.\n"]
    TICK,
    #[doc = "Use AUX clock as source for prescaler."]
    CLK,
}
impl MODER {
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
            MODER::TICK => true,
            MODER::CLK => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            true => MODER::TICK,
            false => MODER::CLK,
        }
    }
    #[doc = "Checks if the value of the field is `TICK`"]
    #[inline]
    pub fn is_tick(&self) -> bool {
        *self == MODER::TICK
    }
    #[doc = "Checks if the value of the field is `CLK`"]
    #[inline]
    pub fn is_clk(&self) -> bool {
        *self == MODER::CLK
    }
}
#[doc = "Possible values of the field `RELOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RELOADR {
    #[doc = "Continuous mode.\n\nTimer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    CONT,
    #[doc = "Manual mode.\n\nTimer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE.\n"]
    MAN,
}
impl RELOADR {
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
            RELOADR::CONT => true,
            RELOADR::MAN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RELOADR {
        match value {
            true => RELOADR::CONT,
            false => RELOADR::MAN,
        }
    }
    #[doc = "Checks if the value of the field is `CONT`"]
    #[inline]
    pub fn is_cont(&self) -> bool {
        *self == RELOADR::CONT
    }
    #[doc = "Checks if the value of the field is `MAN`"]
    #[inline]
    pub fn is_man(&self) -> bool {
        *self == RELOADR::MAN
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
#[doc = "Values that can be written to the field `TICK_SRC_POL`"]
pub enum TICK_SRC_POLW {
    #[doc = "Count on falling edges of TICK_SRC."]
    FALL,
    #[doc = "Count on rising edges of TICK_SRC."]
    RISE,
}
impl TICK_SRC_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TICK_SRC_POLW::FALL => true,
            TICK_SRC_POLW::RISE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TICK_SRC_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _TICK_SRC_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TICK_SRC_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Count on falling edges of TICK_SRC."]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(TICK_SRC_POLW::FALL)
    }
    #[doc = "Count on rising edges of TICK_SRC."]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(TICK_SRC_POLW::RISE)
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
#[doc = "Values that can be written to the field `TICK_SRC`"]
pub enum TICK_SRCW {
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    ADC_IRQ,
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    MCU_EVENT,
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
    #[doc = "AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    RTC_4KHZ,
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE"]
    ADC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    TIMER1_EV,
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE"]
    TDC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    RTC_CH2_EV,
}
impl TICK_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TICK_SRCW::ADC_IRQ => 31,
            TICK_SRCW::MCU_EVENT => 30,
            TICK_SRCW::ACLK_REF => 29,
            TICK_SRCW::AUXIO15 => 28,
            TICK_SRCW::AUXIO14 => 27,
            TICK_SRCW::AUXIO13 => 26,
            TICK_SRCW::AUXIO12 => 25,
            TICK_SRCW::AUXIO11 => 24,
            TICK_SRCW::AUXIO10 => 23,
            TICK_SRCW::AUXIO9 => 22,
            TICK_SRCW::AUXIO8 => 21,
            TICK_SRCW::AUXIO7 => 20,
            TICK_SRCW::AUXIO6 => 19,
            TICK_SRCW::AUXIO5 => 18,
            TICK_SRCW::AUXIO4 => 17,
            TICK_SRCW::AUXIO3 => 16,
            TICK_SRCW::AUXIO2 => 15,
            TICK_SRCW::AUXIO1 => 14,
            TICK_SRCW::AUXIO0 => 13,
            TICK_SRCW::AON_PROG_WU => 12,
            TICK_SRCW::AON_SW => 11,
            TICK_SRCW::OBSMUX1 => 10,
            TICK_SRCW::OBSMUX0 => 9,
            TICK_SRCW::RTC_4KHZ => 8,
            TICK_SRCW::ADC_DONE => 7,
            TICK_SRCW::SMPH_AUTOTAKE_DONE => 6,
            TICK_SRCW::TIMER1_EV => 5,
            TICK_SRCW::TDC_DONE => 3,
            TICK_SRCW::AUX_COMPB => 2,
            TICK_SRCW::AUX_COMPA => 1,
            TICK_SRCW::RTC_CH2_EV => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TICK_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TICK_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TICK_SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    #[inline]
    pub fn adc_irq(self) -> &'a mut W {
        self.variant(TICK_SRCW::ADC_IRQ)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    #[inline]
    pub fn mcu_event(self) -> &'a mut W {
        self.variant(TICK_SRCW::MCU_EVENT)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF"]
    #[inline]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(TICK_SRCW::ACLK_REF)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15"]
    #[inline]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO15)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14"]
    #[inline]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO14)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13"]
    #[inline]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO13)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12"]
    #[inline]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO12)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11"]
    #[inline]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO11)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10"]
    #[inline]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO10)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9"]
    #[inline]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO9)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8"]
    #[inline]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO8)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7"]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO7)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6"]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO6)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5"]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO5)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4"]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO4)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3"]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUXIO0)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    #[inline]
    pub fn aon_prog_wu(self) -> &'a mut W {
        self.variant(TICK_SRCW::AON_PROG_WU)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW"]
    #[inline]
    pub fn aon_sw(self) -> &'a mut W {
        self.variant(TICK_SRCW::AON_SW)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX1"]
    #[inline]
    pub fn obsmux1(self) -> &'a mut W {
        self.variant(TICK_SRCW::OBSMUX1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.OBSMUX0"]
    #[inline]
    pub fn obsmux0(self) -> &'a mut W {
        self.variant(TICK_SRCW::OBSMUX0)
    }
    #[doc = "AON_RTC:SUBSEC.VALUE bit 19. AON_RTC:CTL.RTC_4KHZ_EN enables this event."]
    #[inline]
    pub fn rtc_4khz(self) -> &'a mut W {
        self.variant(TICK_SRCW::RTC_4KHZ)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.ADC_DONE"]
    #[inline]
    pub fn adc_done(self) -> &'a mut W {
        self.variant(TICK_SRCW::ADC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline]
    pub fn smph_autotake_done(self) -> &'a mut W {
        self.variant(TICK_SRCW::SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    #[inline]
    pub fn timer1_ev(self) -> &'a mut W {
        self.variant(TICK_SRCW::TIMER1_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE"]
    #[inline]
    pub fn tdc_done(self) -> &'a mut W {
        self.variant(TICK_SRCW::TDC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_COMPB)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(TICK_SRCW::AUX_COMPA)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    #[inline]
    pub fn rtc_ch2_ev(self) -> &'a mut W {
        self.variant(TICK_SRCW::RTC_CH2_EV)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PREW<'a> {
    w: &'a mut W,
}
impl<'a> _PREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Use event set by TICK_SRC as source for prescaler.\n"]
    TICK,
    #[doc = "Use AUX clock as source for prescaler."]
    CLK,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::TICK => true,
            MODEW::CLK => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use event set by TICK_SRC as source for prescaler."]
    #[inline]
    pub fn tick(self) -> &'a mut W {
        self.variant(MODEW::TICK)
    }
    #[doc = "Use AUX clock as source for prescaler."]
    #[inline]
    pub fn clk(self) -> &'a mut W {
        self.variant(MODEW::CLK)
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
#[doc = "Values that can be written to the field `RELOAD`"]
pub enum RELOADW {
    #[doc = "Continuous mode.\n\nTimer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    CONT,
    #[doc = "Manual mode.\n\nTimer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE.\n"]
    MAN,
}
impl RELOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RELOADW::CONT => true,
            RELOADW::MAN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RELOADW<'a> {
    w: &'a mut W,
}
impl<'a> _RELOADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RELOADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Continuous mode. Timer 0 restarts when the counter value becomes equal to or greater than ( T0TARGET.VALUE - 1)."]
    #[inline]
    pub fn cont(self) -> &'a mut W {
        self.variant(RELOADW::CONT)
    }
    #[doc = "Manual mode. Timer 0 stops and T0CTL.EN becomes 0 when the counter value becomes equal to or greater than T0TARGET.VALUE."]
    #[inline]
    pub fn man(self) -> &'a mut W {
        self.variant(RELOADW::MAN)
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
    #[doc = "Bit 13 - 13:13\\] Tick source polarity for Timer 0."]
    #[inline]
    pub fn tick_src_pol(&self) -> TICK_SRC_POLR {
        TICK_SRC_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - 12:8\\] Select Timer 0 tick source from the synchronous event bus."]
    #[inline]
    pub fn tick_src(&self) -> TICK_SRCR {
        TICK_SRCR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - 7:4\\] Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline]
    pub fn pre(&self) -> PRER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRER { bits }
    }
    #[doc = "Bits 2:3 - 3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Timer 0 mode. Configure source for Timer 0 prescaler."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] Timer 0 reload mode."]
    #[inline]
    pub fn reload(&self) -> RELOADR {
        RELOADR::_from({
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
    #[doc = "Bit 13 - 13:13\\] Tick source polarity for Timer 0."]
    #[inline]
    pub fn tick_src_pol(&mut self) -> _TICK_SRC_POLW {
        _TICK_SRC_POLW { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\] Select Timer 0 tick source from the synchronous event bus."]
    #[inline]
    pub fn tick_src(&mut self) -> _TICK_SRCW {
        _TICK_SRCW { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Prescaler division ratio is 2^PRE: 0x0: Divide by 1. 0x1: Divide by 2. 0x2: Divide by 4. ... 0xF: Divide by 32,768."]
    #[inline]
    pub fn pre(&mut self) -> _PREW {
        _PREW { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Timer 0 mode. Configure source for Timer 0 prescaler."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Timer 0 reload mode."]
    #[inline]
    pub fn reload(&mut self) -> _RELOADW {
        _RELOADW { w: self }
    }
}
