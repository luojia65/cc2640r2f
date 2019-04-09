#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADCCTL {
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
#[doc = "Possible values of the field `START_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_POLR {
    #[doc = "Set ADC trigger on falling edge of event source."]
    FALL,
    #[doc = "Set ADC trigger on rising edge of event source."]
    RISE,
}
impl START_POLR {
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
            START_POLR::FALL => true,
            START_POLR::RISE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> START_POLR {
        match value {
            true => START_POLR::FALL,
            false => START_POLR::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == START_POLR::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == START_POLR::RISE
    }
}
#[doc = "Possible values of the field `START_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_SRCR {
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
    #[doc = "No event."]
    NO_EVENT1,
    #[doc = "No event."]
    NO_EVENT0,
    #[doc = "Reserved - Do not use."]
    RESERVED1,
    #[doc = "Reserved - Do not use."]
    RESERVED0,
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    TIMER1_EV,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    TIMER0_EV,
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE"]
    TDC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    RTC_CH2_EV,
}
impl START_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            START_SRCR::ADC_IRQ => 31,
            START_SRCR::MCU_EV => 30,
            START_SRCR::ACLK_REF => 29,
            START_SRCR::AUXIO15 => 28,
            START_SRCR::AUXIO14 => 27,
            START_SRCR::AUXIO13 => 26,
            START_SRCR::AUXIO12 => 25,
            START_SRCR::AUXIO11 => 24,
            START_SRCR::AUXIO10 => 23,
            START_SRCR::AUXIO9 => 22,
            START_SRCR::AUXIO8 => 21,
            START_SRCR::AUXIO7 => 20,
            START_SRCR::AUXIO6 => 19,
            START_SRCR::AUXIO5 => 18,
            START_SRCR::AUXIO4 => 17,
            START_SRCR::AUXIO3 => 16,
            START_SRCR::AUXIO2 => 15,
            START_SRCR::AUXIO1 => 14,
            START_SRCR::AUXIO0 => 13,
            START_SRCR::AON_PROG_WU => 12,
            START_SRCR::AON_SW => 11,
            START_SRCR::NO_EVENT1 => 10,
            START_SRCR::NO_EVENT0 => 9,
            START_SRCR::RESERVED1 => 8,
            START_SRCR::RESERVED0 => 7,
            START_SRCR::SMPH_AUTOTAKE_DONE => 6,
            START_SRCR::TIMER1_EV => 5,
            START_SRCR::TIMER0_EV => 4,
            START_SRCR::TDC_DONE => 3,
            START_SRCR::AUX_COMPB => 2,
            START_SRCR::AUX_COMPA => 1,
            START_SRCR::RTC_CH2_EV => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> START_SRCR {
        match value {
            31 => START_SRCR::ADC_IRQ,
            30 => START_SRCR::MCU_EV,
            29 => START_SRCR::ACLK_REF,
            28 => START_SRCR::AUXIO15,
            27 => START_SRCR::AUXIO14,
            26 => START_SRCR::AUXIO13,
            25 => START_SRCR::AUXIO12,
            24 => START_SRCR::AUXIO11,
            23 => START_SRCR::AUXIO10,
            22 => START_SRCR::AUXIO9,
            21 => START_SRCR::AUXIO8,
            20 => START_SRCR::AUXIO7,
            19 => START_SRCR::AUXIO6,
            18 => START_SRCR::AUXIO5,
            17 => START_SRCR::AUXIO4,
            16 => START_SRCR::AUXIO3,
            15 => START_SRCR::AUXIO2,
            14 => START_SRCR::AUXIO1,
            13 => START_SRCR::AUXIO0,
            12 => START_SRCR::AON_PROG_WU,
            11 => START_SRCR::AON_SW,
            10 => START_SRCR::NO_EVENT1,
            9 => START_SRCR::NO_EVENT0,
            8 => START_SRCR::RESERVED1,
            7 => START_SRCR::RESERVED0,
            6 => START_SRCR::SMPH_AUTOTAKE_DONE,
            5 => START_SRCR::TIMER1_EV,
            4 => START_SRCR::TIMER0_EV,
            3 => START_SRCR::TDC_DONE,
            2 => START_SRCR::AUX_COMPB,
            1 => START_SRCR::AUX_COMPA,
            0 => START_SRCR::RTC_CH2_EV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_IRQ`"]
    #[inline]
    pub fn is_adc_irq(&self) -> bool {
        *self == START_SRCR::ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline]
    pub fn is_mcu_ev(&self) -> bool {
        *self == START_SRCR::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline]
    pub fn is_aclk_ref(&self) -> bool {
        *self == START_SRCR::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline]
    pub fn is_auxio15(&self) -> bool {
        *self == START_SRCR::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline]
    pub fn is_auxio14(&self) -> bool {
        *self == START_SRCR::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline]
    pub fn is_auxio13(&self) -> bool {
        *self == START_SRCR::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline]
    pub fn is_auxio12(&self) -> bool {
        *self == START_SRCR::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline]
    pub fn is_auxio11(&self) -> bool {
        *self == START_SRCR::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline]
    pub fn is_auxio10(&self) -> bool {
        *self == START_SRCR::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline]
    pub fn is_auxio9(&self) -> bool {
        *self == START_SRCR::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline]
    pub fn is_auxio8(&self) -> bool {
        *self == START_SRCR::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == START_SRCR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == START_SRCR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == START_SRCR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == START_SRCR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == START_SRCR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == START_SRCR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == START_SRCR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == START_SRCR::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AON_PROG_WU`"]
    #[inline]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == START_SRCR::AON_PROG_WU
    }
    #[doc = "Checks if the value of the field is `AON_SW`"]
    #[inline]
    pub fn is_aon_sw(&self) -> bool {
        *self == START_SRCR::AON_SW
    }
    #[doc = "Checks if the value of the field is `NO_EVENT1`"]
    #[inline]
    pub fn is_no_event1(&self) -> bool {
        *self == START_SRCR::NO_EVENT1
    }
    #[doc = "Checks if the value of the field is `NO_EVENT0`"]
    #[inline]
    pub fn is_no_event0(&self) -> bool {
        *self == START_SRCR::NO_EVENT0
    }
    #[doc = "Checks if the value of the field is `RESERVED1`"]
    #[inline]
    pub fn is_reserved1(&self) -> bool {
        *self == START_SRCR::RESERVED1
    }
    #[doc = "Checks if the value of the field is `RESERVED0`"]
    #[inline]
    pub fn is_reserved0(&self) -> bool {
        *self == START_SRCR::RESERVED0
    }
    #[doc = "Checks if the value of the field is `SMPH_AUTOTAKE_DONE`"]
    #[inline]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == START_SRCR::SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `TIMER1_EV`"]
    #[inline]
    pub fn is_timer1_ev(&self) -> bool {
        *self == START_SRCR::TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `TIMER0_EV`"]
    #[inline]
    pub fn is_timer0_ev(&self) -> bool {
        *self == START_SRCR::TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `TDC_DONE`"]
    #[inline]
    pub fn is_tdc_done(&self) -> bool {
        *self == START_SRCR::TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == START_SRCR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == START_SRCR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_EV`"]
    #[inline]
    pub fn is_rtc_ch2_ev(&self) -> bool {
        *self == START_SRCR::RTC_CH2_EV
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
#[doc = "Possible values of the field `CMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDR {
    #[doc = "Flush ADC FIFO.\n\nYou must set CMD to EN or DIS after flush.\n\nSystem CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    FLUSH,
    #[doc = "Enable ADC interface."]
    EN,
    #[doc = "Disable ADC interface."]
    DIS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDR::FLUSH => 3,
            CMDR::EN => 1,
            CMDR::DIS => 0,
            CMDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDR {
        match value {
            3 => CMDR::FLUSH,
            1 => CMDR::EN,
            0 => CMDR::DIS,
            i => CMDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline]
    pub fn is_flush(&self) -> bool {
        *self == CMDR::FLUSH
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == CMDR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == CMDR::DIS
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
#[doc = "Values that can be written to the field `START_POL`"]
pub enum START_POLW {
    #[doc = "Set ADC trigger on falling edge of event source."]
    FALL,
    #[doc = "Set ADC trigger on rising edge of event source."]
    RISE,
}
impl START_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            START_POLW::FALL => true,
            START_POLW::RISE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _START_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _START_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: START_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Set ADC trigger on falling edge of event source."]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(START_POLW::FALL)
    }
    #[doc = "Set ADC trigger on rising edge of event source."]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(START_POLW::RISE)
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
#[doc = "Values that can be written to the field `START_SRC`"]
pub enum START_SRCW {
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
    #[doc = "No event."]
    NO_EVENT1,
    #[doc = "No event."]
    NO_EVENT0,
    #[doc = "Reserved - Do not use."]
    RESERVED1,
    #[doc = "Reserved - Do not use."]
    RESERVED0,
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    TIMER1_EV,
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    TIMER0_EV,
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE"]
    TDC_DONE,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    RTC_CH2_EV,
}
impl START_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            START_SRCW::ADC_IRQ => 31,
            START_SRCW::MCU_EV => 30,
            START_SRCW::ACLK_REF => 29,
            START_SRCW::AUXIO15 => 28,
            START_SRCW::AUXIO14 => 27,
            START_SRCW::AUXIO13 => 26,
            START_SRCW::AUXIO12 => 25,
            START_SRCW::AUXIO11 => 24,
            START_SRCW::AUXIO10 => 23,
            START_SRCW::AUXIO9 => 22,
            START_SRCW::AUXIO8 => 21,
            START_SRCW::AUXIO7 => 20,
            START_SRCW::AUXIO6 => 19,
            START_SRCW::AUXIO5 => 18,
            START_SRCW::AUXIO4 => 17,
            START_SRCW::AUXIO3 => 16,
            START_SRCW::AUXIO2 => 15,
            START_SRCW::AUXIO1 => 14,
            START_SRCW::AUXIO0 => 13,
            START_SRCW::AON_PROG_WU => 12,
            START_SRCW::AON_SW => 11,
            START_SRCW::NO_EVENT1 => 10,
            START_SRCW::NO_EVENT0 => 9,
            START_SRCW::RESERVED1 => 8,
            START_SRCW::RESERVED0 => 7,
            START_SRCW::SMPH_AUTOTAKE_DONE => 6,
            START_SRCW::TIMER1_EV => 5,
            START_SRCW::TIMER0_EV => 4,
            START_SRCW::TDC_DONE => 3,
            START_SRCW::AUX_COMPB => 2,
            START_SRCW::AUX_COMPA => 1,
            START_SRCW::RTC_CH2_EV => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _START_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _START_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: START_SRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ADC_IRQ"]
    #[inline]
    pub fn adc_irq(self) -> &'a mut W {
        self.variant(START_SRCW::ADC_IRQ)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.MCU_EV"]
    #[inline]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(START_SRCW::MCU_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.ACLK_REF"]
    #[inline]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(START_SRCW::ACLK_REF)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO15"]
    #[inline]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO15)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO14"]
    #[inline]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO14)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO13"]
    #[inline]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO13)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO12"]
    #[inline]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO12)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO11"]
    #[inline]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO11)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO10"]
    #[inline]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO10)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO9"]
    #[inline]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO9)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO8"]
    #[inline]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO8)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO7"]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO7)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO6"]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO6)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO5"]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO5)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO4"]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO4)
    }
    #[doc = "AUX_EVCTL:EVSTAT1.AUXIO3"]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO3)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO2"]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO2)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO1"]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO1)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUXIO0"]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(START_SRCW::AUXIO0)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_PROG_WU"]
    #[inline]
    pub fn aon_prog_wu(self) -> &'a mut W {
        self.variant(START_SRCW::AON_PROG_WU)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_SW"]
    #[inline]
    pub fn aon_sw(self) -> &'a mut W {
        self.variant(START_SRCW::AON_SW)
    }
    #[doc = "No event."]
    #[inline]
    pub fn no_event1(self) -> &'a mut W {
        self.variant(START_SRCW::NO_EVENT1)
    }
    #[doc = "No event."]
    #[inline]
    pub fn no_event0(self) -> &'a mut W {
        self.variant(START_SRCW::NO_EVENT0)
    }
    #[doc = "Reserved - Do not use."]
    #[inline]
    pub fn reserved1(self) -> &'a mut W {
        self.variant(START_SRCW::RESERVED1)
    }
    #[doc = "Reserved - Do not use."]
    #[inline]
    pub fn reserved0(self) -> &'a mut W {
        self.variant(START_SRCW::RESERVED0)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline]
    pub fn smph_autotake_done(self) -> &'a mut W {
        self.variant(START_SRCW::SMPH_AUTOTAKE_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER1_EV"]
    #[inline]
    pub fn timer1_ev(self) -> &'a mut W {
        self.variant(START_SRCW::TIMER1_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TIMER0_EV"]
    #[inline]
    pub fn timer0_ev(self) -> &'a mut W {
        self.variant(START_SRCW::TIMER0_EV)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.TDC_DONE"]
    #[inline]
    pub fn tdc_done(self) -> &'a mut W {
        self.variant(START_SRCW::TDC_DONE)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPB"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_COMPB)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AUX_COMPA"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(START_SRCW::AUX_COMPA)
    }
    #[doc = "AUX_EVCTL:EVSTAT0.AON_RTC_CH2"]
    #[inline]
    pub fn rtc_ch2_ev(self) -> &'a mut W {
        self.variant(START_SRCW::RTC_CH2_EV)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
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
        const MASK: u8 = 63;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD`"]
pub enum CMDW {
    #[doc = "Flush ADC FIFO.\n\nYou must set CMD to EN or DIS after flush.\n\nSystem CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    FLUSH,
    #[doc = "Enable ADC interface."]
    EN,
    #[doc = "Disable ADC interface."]
    DIS,
}
impl CMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDW::FLUSH => 3,
            CMDW::EN => 1,
            CMDW::DIS => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Flush ADC FIFO. You must set CMD to EN or DIS after flush. System CPU must wait two clock cycles before it sets CMD to EN or DIS."]
    #[inline]
    pub fn flush(self) -> &'a mut W {
        self.variant(CMDW::FLUSH)
    }
    #[doc = "Enable ADC interface."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(CMDW::EN)
    }
    #[doc = "Disable ADC interface."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(CMDW::DIS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 13 - 13:13\\] Select active polarity for START_SRC event."]
    #[inline]
    pub fn start_pol(&self) -> START_POLR {
        START_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - 12:8\\] Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT<n> if you want to trigger the ADC manually through ADCTRIG.START."]
    #[inline]
    pub fn start_src(&self) -> START_SRCR {
        START_SRCR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:7 - 7:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED2R { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        CMDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 13 - 13:13\\] Select active polarity for START_SRC event."]
    #[inline]
    pub fn start_pol(&mut self) -> _START_POLW {
        _START_POLW { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\] Select ADC trigger event source from the asynchronous AUX event bus. Set START_SRC to NO_EVENT<n> if you want to trigger the ADC manually through ADCTRIG.START."]
    #[inline]
    pub fn start_src(&mut self) -> _START_SRCW {
        _START_SRCW { w: self }
    }
    #[doc = "Bits 2:7 - 7:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] ADC interface command. Non-enumerated values are not supported. The written value is returned when read."]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
}
