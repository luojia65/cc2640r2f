#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VECCFG0 {
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
#[doc = "Possible values of the field `VEC1_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VEC1_POLR {
    #[doc = "Falling edge triggers vector 1 execution."]
    FALL,
    #[doc = "Rising edge triggers vector 1 execution."]
    RISE,
}
impl VEC1_POLR {
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
            VEC1_POLR::FALL => true,
            VEC1_POLR::RISE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VEC1_POLR {
        match value {
            true => VEC1_POLR::FALL,
            false => VEC1_POLR::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == VEC1_POLR::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == VEC1_POLR::RISE
    }
}
#[doc = "Possible values of the field `VEC1_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VEC1_ENR {
    #[doc = "Enable vector 1 trigger."]
    EN,
    #[doc = "Disable vector 1 trigger."]
    DIS,
}
impl VEC1_ENR {
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
            VEC1_ENR::EN => true,
            VEC1_ENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VEC1_ENR {
        match value {
            true => VEC1_ENR::EN,
            false => VEC1_ENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == VEC1_ENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == VEC1_ENR::DIS
    }
}
#[doc = "Possible values of the field `VEC1_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VEC1_EVR {
    #[doc = "EVSTAT1.ADC_IRQ"]
    ADC_IRQ,
    #[doc = "EVSTAT1.MCU_EV"]
    MCU_EV,
    #[doc = "EVSTAT1.ACLK_REF"]
    ACLK_REF,
    #[doc = "EVSTAT1.AUXIO15"]
    AUXIO15,
    #[doc = "EVSTAT1.AUXIO14 "]
    AUXIO14,
    #[doc = "EVSTAT1.AUXIO13 "]
    AUXIO13,
    #[doc = "EVSTAT1.AUXIO12 "]
    AUXIO12,
    #[doc = "EVSTAT1.AUXIO11"]
    AUXIO11,
    #[doc = "EVSTAT1.AUXIO10"]
    AUXIO10,
    #[doc = "EVSTAT1.AUXIO9  "]
    AUXIO9,
    #[doc = "EVSTAT1.AUXIO8  "]
    AUXIO8,
    #[doc = "EVSTAT1.AUXIO7  "]
    AUXIO7,
    #[doc = "EVSTAT1.AUXIO6  "]
    AUXIO6,
    #[doc = "EVSTAT1.AUXIO5  "]
    AUXIO5,
    #[doc = "EVSTAT1.AUXIO4  "]
    AUXIO4,
    #[doc = "EVSTAT1.AUXIO3  "]
    AUXIO3,
    #[doc = "EVSTAT0.AUXIO2"]
    AUXIO2,
    #[doc = "EVSTAT0.AUXIO1"]
    AUXIO1,
    #[doc = "EVSTAT0.AUXIO0"]
    AUXIO0,
    #[doc = "EVSTAT0.AON_PROG_WU"]
    AON_PROG_WU,
    #[doc = "EVSTAT0.AON_SW"]
    AON_SW,
    #[doc = "EVSTAT0.OBSMUX1"]
    OBSMUX1,
    #[doc = "EVSTAT0.OBSMUX0"]
    OBSMUX0,
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    ADC_FIFO_ALMOST_FULL,
    #[doc = "EVSTAT0.ADC_DONE"]
    ADC_DONE,
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE,
    #[doc = "EVSTAT0.TIMER1_EV"]
    TIMER1_EV,
    #[doc = "EVSTAT0.TIMER0_EV"]
    TIMER0_EV,
    #[doc = "EVSTAT0.TDC_DONE"]
    TDC_DONE,
    #[doc = "EVSTAT0.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "EVSTAT0.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    AON_RTC_CH2,
}
impl VEC1_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VEC1_EVR::ADC_IRQ => 31,
            VEC1_EVR::MCU_EV => 30,
            VEC1_EVR::ACLK_REF => 29,
            VEC1_EVR::AUXIO15 => 28,
            VEC1_EVR::AUXIO14 => 27,
            VEC1_EVR::AUXIO13 => 26,
            VEC1_EVR::AUXIO12 => 25,
            VEC1_EVR::AUXIO11 => 24,
            VEC1_EVR::AUXIO10 => 23,
            VEC1_EVR::AUXIO9 => 22,
            VEC1_EVR::AUXIO8 => 21,
            VEC1_EVR::AUXIO7 => 20,
            VEC1_EVR::AUXIO6 => 19,
            VEC1_EVR::AUXIO5 => 18,
            VEC1_EVR::AUXIO4 => 17,
            VEC1_EVR::AUXIO3 => 16,
            VEC1_EVR::AUXIO2 => 15,
            VEC1_EVR::AUXIO1 => 14,
            VEC1_EVR::AUXIO0 => 13,
            VEC1_EVR::AON_PROG_WU => 12,
            VEC1_EVR::AON_SW => 11,
            VEC1_EVR::OBSMUX1 => 10,
            VEC1_EVR::OBSMUX0 => 9,
            VEC1_EVR::ADC_FIFO_ALMOST_FULL => 8,
            VEC1_EVR::ADC_DONE => 7,
            VEC1_EVR::SMPH_AUTOTAKE_DONE => 6,
            VEC1_EVR::TIMER1_EV => 5,
            VEC1_EVR::TIMER0_EV => 4,
            VEC1_EVR::TDC_DONE => 3,
            VEC1_EVR::AUX_COMPB => 2,
            VEC1_EVR::AUX_COMPA => 1,
            VEC1_EVR::AON_RTC_CH2 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VEC1_EVR {
        match value {
            31 => VEC1_EVR::ADC_IRQ,
            30 => VEC1_EVR::MCU_EV,
            29 => VEC1_EVR::ACLK_REF,
            28 => VEC1_EVR::AUXIO15,
            27 => VEC1_EVR::AUXIO14,
            26 => VEC1_EVR::AUXIO13,
            25 => VEC1_EVR::AUXIO12,
            24 => VEC1_EVR::AUXIO11,
            23 => VEC1_EVR::AUXIO10,
            22 => VEC1_EVR::AUXIO9,
            21 => VEC1_EVR::AUXIO8,
            20 => VEC1_EVR::AUXIO7,
            19 => VEC1_EVR::AUXIO6,
            18 => VEC1_EVR::AUXIO5,
            17 => VEC1_EVR::AUXIO4,
            16 => VEC1_EVR::AUXIO3,
            15 => VEC1_EVR::AUXIO2,
            14 => VEC1_EVR::AUXIO1,
            13 => VEC1_EVR::AUXIO0,
            12 => VEC1_EVR::AON_PROG_WU,
            11 => VEC1_EVR::AON_SW,
            10 => VEC1_EVR::OBSMUX1,
            9 => VEC1_EVR::OBSMUX0,
            8 => VEC1_EVR::ADC_FIFO_ALMOST_FULL,
            7 => VEC1_EVR::ADC_DONE,
            6 => VEC1_EVR::SMPH_AUTOTAKE_DONE,
            5 => VEC1_EVR::TIMER1_EV,
            4 => VEC1_EVR::TIMER0_EV,
            3 => VEC1_EVR::TDC_DONE,
            2 => VEC1_EVR::AUX_COMPB,
            1 => VEC1_EVR::AUX_COMPA,
            0 => VEC1_EVR::AON_RTC_CH2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_IRQ`"]
    #[inline]
    pub fn is_adc_irq(&self) -> bool {
        *self == VEC1_EVR::ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline]
    pub fn is_mcu_ev(&self) -> bool {
        *self == VEC1_EVR::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline]
    pub fn is_aclk_ref(&self) -> bool {
        *self == VEC1_EVR::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline]
    pub fn is_auxio15(&self) -> bool {
        *self == VEC1_EVR::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline]
    pub fn is_auxio14(&self) -> bool {
        *self == VEC1_EVR::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline]
    pub fn is_auxio13(&self) -> bool {
        *self == VEC1_EVR::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline]
    pub fn is_auxio12(&self) -> bool {
        *self == VEC1_EVR::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline]
    pub fn is_auxio11(&self) -> bool {
        *self == VEC1_EVR::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline]
    pub fn is_auxio10(&self) -> bool {
        *self == VEC1_EVR::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline]
    pub fn is_auxio9(&self) -> bool {
        *self == VEC1_EVR::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline]
    pub fn is_auxio8(&self) -> bool {
        *self == VEC1_EVR::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == VEC1_EVR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == VEC1_EVR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == VEC1_EVR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == VEC1_EVR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == VEC1_EVR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == VEC1_EVR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == VEC1_EVR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == VEC1_EVR::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AON_PROG_WU`"]
    #[inline]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == VEC1_EVR::AON_PROG_WU
    }
    #[doc = "Checks if the value of the field is `AON_SW`"]
    #[inline]
    pub fn is_aon_sw(&self) -> bool {
        *self == VEC1_EVR::AON_SW
    }
    #[doc = "Checks if the value of the field is `OBSMUX1`"]
    #[inline]
    pub fn is_obsmux1(&self) -> bool {
        *self == VEC1_EVR::OBSMUX1
    }
    #[doc = "Checks if the value of the field is `OBSMUX0`"]
    #[inline]
    pub fn is_obsmux0(&self) -> bool {
        *self == VEC1_EVR::OBSMUX0
    }
    #[doc = "Checks if the value of the field is `ADC_FIFO_ALMOST_FULL`"]
    #[inline]
    pub fn is_adc_fifo_almost_full(&self) -> bool {
        *self == VEC1_EVR::ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `ADC_DONE`"]
    #[inline]
    pub fn is_adc_done(&self) -> bool {
        *self == VEC1_EVR::ADC_DONE
    }
    #[doc = "Checks if the value of the field is `SMPH_AUTOTAKE_DONE`"]
    #[inline]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == VEC1_EVR::SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `TIMER1_EV`"]
    #[inline]
    pub fn is_timer1_ev(&self) -> bool {
        *self == VEC1_EVR::TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `TIMER0_EV`"]
    #[inline]
    pub fn is_timer0_ev(&self) -> bool {
        *self == VEC1_EVR::TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `TDC_DONE`"]
    #[inline]
    pub fn is_tdc_done(&self) -> bool {
        *self == VEC1_EVR::TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == VEC1_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == VEC1_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == VEC1_EVR::AON_RTC_CH2
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
#[doc = "Possible values of the field `VEC0_POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VEC0_POLR {
    #[doc = "Falling edge triggers vector 0 execution."]
    FALL,
    #[doc = "Rising edge triggers vector 0 execution."]
    RISE,
}
impl VEC0_POLR {
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
            VEC0_POLR::FALL => true,
            VEC0_POLR::RISE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VEC0_POLR {
        match value {
            true => VEC0_POLR::FALL,
            false => VEC0_POLR::RISE,
        }
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == VEC0_POLR::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == VEC0_POLR::RISE
    }
}
#[doc = "Possible values of the field `VEC0_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VEC0_ENR {
    #[doc = "Enable vector 0 trigger."]
    EN,
    #[doc = "Disable vector 0 trigger."]
    DIS,
}
impl VEC0_ENR {
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
            VEC0_ENR::EN => true,
            VEC0_ENR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VEC0_ENR {
        match value {
            true => VEC0_ENR::EN,
            false => VEC0_ENR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == VEC0_ENR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == VEC0_ENR::DIS
    }
}
#[doc = "Possible values of the field `VEC0_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VEC0_EVR {
    #[doc = "EVSTAT1.ADC_IRQ"]
    ADC_IRQ,
    #[doc = "EVSTAT1.MCU_EV"]
    MCU_EV,
    #[doc = "EVSTAT1.ACLK_REF"]
    ACLK_REF,
    #[doc = "EVSTAT1.AUXIO15"]
    AUXIO15,
    #[doc = "EVSTAT1.AUXIO14 "]
    AUXIO14,
    #[doc = "EVSTAT1.AUXIO13 "]
    AUXIO13,
    #[doc = "EVSTAT1.AUXIO12 "]
    AUXIO12,
    #[doc = "EVSTAT1.AUXIO11"]
    AUXIO11,
    #[doc = "EVSTAT1.AUXIO10"]
    AUXIO10,
    #[doc = "EVSTAT1.AUXIO9  "]
    AUXIO9,
    #[doc = "EVSTAT1.AUXIO8  "]
    AUXIO8,
    #[doc = "EVSTAT1.AUXIO7  "]
    AUXIO7,
    #[doc = "EVSTAT1.AUXIO6  "]
    AUXIO6,
    #[doc = "EVSTAT1.AUXIO5  "]
    AUXIO5,
    #[doc = "EVSTAT1.AUXIO4  "]
    AUXIO4,
    #[doc = "EVSTAT1.AUXIO3  "]
    AUXIO3,
    #[doc = "EVSTAT0.AUXIO2"]
    AUXIO2,
    #[doc = "EVSTAT0.AUXIO1"]
    AUXIO1,
    #[doc = "EVSTAT0.AUXIO0"]
    AUXIO0,
    #[doc = "EVSTAT0.AON_PROG_WU"]
    AON_PROG_WU,
    #[doc = "EVSTAT0.AON_SW"]
    AON_SW,
    #[doc = "EVSTAT0.OBSMUX1"]
    OBSMUX1,
    #[doc = "EVSTAT0.OBSMUX0"]
    OBSMUX0,
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    ADC_FIFO_ALMOST_FULL,
    #[doc = "EVSTAT0.ADC_DONE"]
    ADC_DONE,
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE,
    #[doc = "EVSTAT0.TIMER1_EV"]
    TIMER1_EV,
    #[doc = "EVSTAT0.TIMER0_EV"]
    TIMER0_EV,
    #[doc = "EVSTAT0.TDC_DONE"]
    TDC_DONE,
    #[doc = "EVSTAT0.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "EVSTAT0.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    AON_RTC_CH2,
}
impl VEC0_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VEC0_EVR::ADC_IRQ => 31,
            VEC0_EVR::MCU_EV => 30,
            VEC0_EVR::ACLK_REF => 29,
            VEC0_EVR::AUXIO15 => 28,
            VEC0_EVR::AUXIO14 => 27,
            VEC0_EVR::AUXIO13 => 26,
            VEC0_EVR::AUXIO12 => 25,
            VEC0_EVR::AUXIO11 => 24,
            VEC0_EVR::AUXIO10 => 23,
            VEC0_EVR::AUXIO9 => 22,
            VEC0_EVR::AUXIO8 => 21,
            VEC0_EVR::AUXIO7 => 20,
            VEC0_EVR::AUXIO6 => 19,
            VEC0_EVR::AUXIO5 => 18,
            VEC0_EVR::AUXIO4 => 17,
            VEC0_EVR::AUXIO3 => 16,
            VEC0_EVR::AUXIO2 => 15,
            VEC0_EVR::AUXIO1 => 14,
            VEC0_EVR::AUXIO0 => 13,
            VEC0_EVR::AON_PROG_WU => 12,
            VEC0_EVR::AON_SW => 11,
            VEC0_EVR::OBSMUX1 => 10,
            VEC0_EVR::OBSMUX0 => 9,
            VEC0_EVR::ADC_FIFO_ALMOST_FULL => 8,
            VEC0_EVR::ADC_DONE => 7,
            VEC0_EVR::SMPH_AUTOTAKE_DONE => 6,
            VEC0_EVR::TIMER1_EV => 5,
            VEC0_EVR::TIMER0_EV => 4,
            VEC0_EVR::TDC_DONE => 3,
            VEC0_EVR::AUX_COMPB => 2,
            VEC0_EVR::AUX_COMPA => 1,
            VEC0_EVR::AON_RTC_CH2 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VEC0_EVR {
        match value {
            31 => VEC0_EVR::ADC_IRQ,
            30 => VEC0_EVR::MCU_EV,
            29 => VEC0_EVR::ACLK_REF,
            28 => VEC0_EVR::AUXIO15,
            27 => VEC0_EVR::AUXIO14,
            26 => VEC0_EVR::AUXIO13,
            25 => VEC0_EVR::AUXIO12,
            24 => VEC0_EVR::AUXIO11,
            23 => VEC0_EVR::AUXIO10,
            22 => VEC0_EVR::AUXIO9,
            21 => VEC0_EVR::AUXIO8,
            20 => VEC0_EVR::AUXIO7,
            19 => VEC0_EVR::AUXIO6,
            18 => VEC0_EVR::AUXIO5,
            17 => VEC0_EVR::AUXIO4,
            16 => VEC0_EVR::AUXIO3,
            15 => VEC0_EVR::AUXIO2,
            14 => VEC0_EVR::AUXIO1,
            13 => VEC0_EVR::AUXIO0,
            12 => VEC0_EVR::AON_PROG_WU,
            11 => VEC0_EVR::AON_SW,
            10 => VEC0_EVR::OBSMUX1,
            9 => VEC0_EVR::OBSMUX0,
            8 => VEC0_EVR::ADC_FIFO_ALMOST_FULL,
            7 => VEC0_EVR::ADC_DONE,
            6 => VEC0_EVR::SMPH_AUTOTAKE_DONE,
            5 => VEC0_EVR::TIMER1_EV,
            4 => VEC0_EVR::TIMER0_EV,
            3 => VEC0_EVR::TDC_DONE,
            2 => VEC0_EVR::AUX_COMPB,
            1 => VEC0_EVR::AUX_COMPA,
            0 => VEC0_EVR::AON_RTC_CH2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_IRQ`"]
    #[inline]
    pub fn is_adc_irq(&self) -> bool {
        *self == VEC0_EVR::ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline]
    pub fn is_mcu_ev(&self) -> bool {
        *self == VEC0_EVR::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline]
    pub fn is_aclk_ref(&self) -> bool {
        *self == VEC0_EVR::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline]
    pub fn is_auxio15(&self) -> bool {
        *self == VEC0_EVR::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline]
    pub fn is_auxio14(&self) -> bool {
        *self == VEC0_EVR::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline]
    pub fn is_auxio13(&self) -> bool {
        *self == VEC0_EVR::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline]
    pub fn is_auxio12(&self) -> bool {
        *self == VEC0_EVR::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline]
    pub fn is_auxio11(&self) -> bool {
        *self == VEC0_EVR::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline]
    pub fn is_auxio10(&self) -> bool {
        *self == VEC0_EVR::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline]
    pub fn is_auxio9(&self) -> bool {
        *self == VEC0_EVR::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline]
    pub fn is_auxio8(&self) -> bool {
        *self == VEC0_EVR::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == VEC0_EVR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == VEC0_EVR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == VEC0_EVR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == VEC0_EVR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == VEC0_EVR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == VEC0_EVR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == VEC0_EVR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == VEC0_EVR::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AON_PROG_WU`"]
    #[inline]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == VEC0_EVR::AON_PROG_WU
    }
    #[doc = "Checks if the value of the field is `AON_SW`"]
    #[inline]
    pub fn is_aon_sw(&self) -> bool {
        *self == VEC0_EVR::AON_SW
    }
    #[doc = "Checks if the value of the field is `OBSMUX1`"]
    #[inline]
    pub fn is_obsmux1(&self) -> bool {
        *self == VEC0_EVR::OBSMUX1
    }
    #[doc = "Checks if the value of the field is `OBSMUX0`"]
    #[inline]
    pub fn is_obsmux0(&self) -> bool {
        *self == VEC0_EVR::OBSMUX0
    }
    #[doc = "Checks if the value of the field is `ADC_FIFO_ALMOST_FULL`"]
    #[inline]
    pub fn is_adc_fifo_almost_full(&self) -> bool {
        *self == VEC0_EVR::ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `ADC_DONE`"]
    #[inline]
    pub fn is_adc_done(&self) -> bool {
        *self == VEC0_EVR::ADC_DONE
    }
    #[doc = "Checks if the value of the field is `SMPH_AUTOTAKE_DONE`"]
    #[inline]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == VEC0_EVR::SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `TIMER1_EV`"]
    #[inline]
    pub fn is_timer1_ev(&self) -> bool {
        *self == VEC0_EVR::TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `TIMER0_EV`"]
    #[inline]
    pub fn is_timer0_ev(&self) -> bool {
        *self == VEC0_EVR::TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `TDC_DONE`"]
    #[inline]
    pub fn is_tdc_done(&self) -> bool {
        *self == VEC0_EVR::TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == VEC0_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == VEC0_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == VEC0_EVR::AON_RTC_CH2
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
#[doc = "Values that can be written to the field `VEC1_POL`"]
pub enum VEC1_POLW {
    #[doc = "Falling edge triggers vector 1 execution."]
    FALL,
    #[doc = "Rising edge triggers vector 1 execution."]
    RISE,
}
impl VEC1_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VEC1_POLW::FALL => true,
            VEC1_POLW::RISE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VEC1_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _VEC1_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VEC1_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge triggers vector 1 execution."]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(VEC1_POLW::FALL)
    }
    #[doc = "Rising edge triggers vector 1 execution."]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(VEC1_POLW::RISE)
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
#[doc = "Values that can be written to the field `VEC1_EN`"]
pub enum VEC1_ENW {
    #[doc = "Enable vector 1 trigger."]
    EN,
    #[doc = "Disable vector 1 trigger."]
    DIS,
}
impl VEC1_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VEC1_ENW::EN => true,
            VEC1_ENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VEC1_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _VEC1_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VEC1_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable vector 1 trigger."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(VEC1_ENW::EN)
    }
    #[doc = "Disable vector 1 trigger."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(VEC1_ENW::DIS)
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
#[doc = "Values that can be written to the field `VEC1_EV`"]
pub enum VEC1_EVW {
    #[doc = "EVSTAT1.ADC_IRQ"]
    ADC_IRQ,
    #[doc = "EVSTAT1.MCU_EV"]
    MCU_EV,
    #[doc = "EVSTAT1.ACLK_REF"]
    ACLK_REF,
    #[doc = "EVSTAT1.AUXIO15"]
    AUXIO15,
    #[doc = "EVSTAT1.AUXIO14 "]
    AUXIO14,
    #[doc = "EVSTAT1.AUXIO13 "]
    AUXIO13,
    #[doc = "EVSTAT1.AUXIO12 "]
    AUXIO12,
    #[doc = "EVSTAT1.AUXIO11"]
    AUXIO11,
    #[doc = "EVSTAT1.AUXIO10"]
    AUXIO10,
    #[doc = "EVSTAT1.AUXIO9  "]
    AUXIO9,
    #[doc = "EVSTAT1.AUXIO8  "]
    AUXIO8,
    #[doc = "EVSTAT1.AUXIO7  "]
    AUXIO7,
    #[doc = "EVSTAT1.AUXIO6  "]
    AUXIO6,
    #[doc = "EVSTAT1.AUXIO5  "]
    AUXIO5,
    #[doc = "EVSTAT1.AUXIO4  "]
    AUXIO4,
    #[doc = "EVSTAT1.AUXIO3  "]
    AUXIO3,
    #[doc = "EVSTAT0.AUXIO2"]
    AUXIO2,
    #[doc = "EVSTAT0.AUXIO1"]
    AUXIO1,
    #[doc = "EVSTAT0.AUXIO0"]
    AUXIO0,
    #[doc = "EVSTAT0.AON_PROG_WU"]
    AON_PROG_WU,
    #[doc = "EVSTAT0.AON_SW"]
    AON_SW,
    #[doc = "EVSTAT0.OBSMUX1"]
    OBSMUX1,
    #[doc = "EVSTAT0.OBSMUX0"]
    OBSMUX0,
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    ADC_FIFO_ALMOST_FULL,
    #[doc = "EVSTAT0.ADC_DONE"]
    ADC_DONE,
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE,
    #[doc = "EVSTAT0.TIMER1_EV"]
    TIMER1_EV,
    #[doc = "EVSTAT0.TIMER0_EV"]
    TIMER0_EV,
    #[doc = "EVSTAT0.TDC_DONE"]
    TDC_DONE,
    #[doc = "EVSTAT0.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "EVSTAT0.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    AON_RTC_CH2,
}
impl VEC1_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VEC1_EVW::ADC_IRQ => 31,
            VEC1_EVW::MCU_EV => 30,
            VEC1_EVW::ACLK_REF => 29,
            VEC1_EVW::AUXIO15 => 28,
            VEC1_EVW::AUXIO14 => 27,
            VEC1_EVW::AUXIO13 => 26,
            VEC1_EVW::AUXIO12 => 25,
            VEC1_EVW::AUXIO11 => 24,
            VEC1_EVW::AUXIO10 => 23,
            VEC1_EVW::AUXIO9 => 22,
            VEC1_EVW::AUXIO8 => 21,
            VEC1_EVW::AUXIO7 => 20,
            VEC1_EVW::AUXIO6 => 19,
            VEC1_EVW::AUXIO5 => 18,
            VEC1_EVW::AUXIO4 => 17,
            VEC1_EVW::AUXIO3 => 16,
            VEC1_EVW::AUXIO2 => 15,
            VEC1_EVW::AUXIO1 => 14,
            VEC1_EVW::AUXIO0 => 13,
            VEC1_EVW::AON_PROG_WU => 12,
            VEC1_EVW::AON_SW => 11,
            VEC1_EVW::OBSMUX1 => 10,
            VEC1_EVW::OBSMUX0 => 9,
            VEC1_EVW::ADC_FIFO_ALMOST_FULL => 8,
            VEC1_EVW::ADC_DONE => 7,
            VEC1_EVW::SMPH_AUTOTAKE_DONE => 6,
            VEC1_EVW::TIMER1_EV => 5,
            VEC1_EVW::TIMER0_EV => 4,
            VEC1_EVW::TDC_DONE => 3,
            VEC1_EVW::AUX_COMPB => 2,
            VEC1_EVW::AUX_COMPA => 1,
            VEC1_EVW::AON_RTC_CH2 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VEC1_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _VEC1_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VEC1_EVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline]
    pub fn adc_irq(self) -> &'a mut W {
        self.variant(VEC1_EVW::ADC_IRQ)
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(VEC1_EVW::MCU_EV)
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(VEC1_EVW::ACLK_REF)
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO15)
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO14)
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO13)
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO12)
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO11)
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO10)
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO9)
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO8)
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO7)
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO6)
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO5)
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO4)
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUXIO0)
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline]
    pub fn aon_prog_wu(self) -> &'a mut W {
        self.variant(VEC1_EVW::AON_PROG_WU)
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline]
    pub fn aon_sw(self) -> &'a mut W {
        self.variant(VEC1_EVW::AON_SW)
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline]
    pub fn obsmux1(self) -> &'a mut W {
        self.variant(VEC1_EVW::OBSMUX1)
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline]
    pub fn obsmux0(self) -> &'a mut W {
        self.variant(VEC1_EVW::OBSMUX0)
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline]
    pub fn adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(VEC1_EVW::ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline]
    pub fn adc_done(self) -> &'a mut W {
        self.variant(VEC1_EVW::ADC_DONE)
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline]
    pub fn smph_autotake_done(self) -> &'a mut W {
        self.variant(VEC1_EVW::SMPH_AUTOTAKE_DONE)
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline]
    pub fn timer1_ev(self) -> &'a mut W {
        self.variant(VEC1_EVW::TIMER1_EV)
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline]
    pub fn timer0_ev(self) -> &'a mut W {
        self.variant(VEC1_EVW::TIMER0_EV)
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline]
    pub fn tdc_done(self) -> &'a mut W {
        self.variant(VEC1_EVW::TDC_DONE)
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUX_COMPB)
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(VEC1_EVW::AUX_COMPA)
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(VEC1_EVW::AON_RTC_CH2)
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
#[doc = "Values that can be written to the field `VEC0_POL`"]
pub enum VEC0_POLW {
    #[doc = "Falling edge triggers vector 0 execution."]
    FALL,
    #[doc = "Rising edge triggers vector 0 execution."]
    RISE,
}
impl VEC0_POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VEC0_POLW::FALL => true,
            VEC0_POLW::RISE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VEC0_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _VEC0_POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VEC0_POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Falling edge triggers vector 0 execution."]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(VEC0_POLW::FALL)
    }
    #[doc = "Rising edge triggers vector 0 execution."]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(VEC0_POLW::RISE)
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
#[doc = "Values that can be written to the field `VEC0_EN`"]
pub enum VEC0_ENW {
    #[doc = "Enable vector 0 trigger."]
    EN,
    #[doc = "Disable vector 0 trigger."]
    DIS,
}
impl VEC0_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VEC0_ENW::EN => true,
            VEC0_ENW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VEC0_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _VEC0_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VEC0_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable vector 0 trigger."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(VEC0_ENW::EN)
    }
    #[doc = "Disable vector 0 trigger."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(VEC0_ENW::DIS)
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
#[doc = "Values that can be written to the field `VEC0_EV`"]
pub enum VEC0_EVW {
    #[doc = "EVSTAT1.ADC_IRQ"]
    ADC_IRQ,
    #[doc = "EVSTAT1.MCU_EV"]
    MCU_EV,
    #[doc = "EVSTAT1.ACLK_REF"]
    ACLK_REF,
    #[doc = "EVSTAT1.AUXIO15"]
    AUXIO15,
    #[doc = "EVSTAT1.AUXIO14 "]
    AUXIO14,
    #[doc = "EVSTAT1.AUXIO13 "]
    AUXIO13,
    #[doc = "EVSTAT1.AUXIO12 "]
    AUXIO12,
    #[doc = "EVSTAT1.AUXIO11"]
    AUXIO11,
    #[doc = "EVSTAT1.AUXIO10"]
    AUXIO10,
    #[doc = "EVSTAT1.AUXIO9  "]
    AUXIO9,
    #[doc = "EVSTAT1.AUXIO8  "]
    AUXIO8,
    #[doc = "EVSTAT1.AUXIO7  "]
    AUXIO7,
    #[doc = "EVSTAT1.AUXIO6  "]
    AUXIO6,
    #[doc = "EVSTAT1.AUXIO5  "]
    AUXIO5,
    #[doc = "EVSTAT1.AUXIO4  "]
    AUXIO4,
    #[doc = "EVSTAT1.AUXIO3  "]
    AUXIO3,
    #[doc = "EVSTAT0.AUXIO2"]
    AUXIO2,
    #[doc = "EVSTAT0.AUXIO1"]
    AUXIO1,
    #[doc = "EVSTAT0.AUXIO0"]
    AUXIO0,
    #[doc = "EVSTAT0.AON_PROG_WU"]
    AON_PROG_WU,
    #[doc = "EVSTAT0.AON_SW"]
    AON_SW,
    #[doc = "EVSTAT0.OBSMUX1"]
    OBSMUX1,
    #[doc = "EVSTAT0.OBSMUX0"]
    OBSMUX0,
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    ADC_FIFO_ALMOST_FULL,
    #[doc = "EVSTAT0.ADC_DONE"]
    ADC_DONE,
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    SMPH_AUTOTAKE_DONE,
    #[doc = "EVSTAT0.TIMER1_EV"]
    TIMER1_EV,
    #[doc = "EVSTAT0.TIMER0_EV"]
    TIMER0_EV,
    #[doc = "EVSTAT0.TDC_DONE"]
    TDC_DONE,
    #[doc = "EVSTAT0.AUX_COMPB"]
    AUX_COMPB,
    #[doc = "EVSTAT0.AUX_COMPA"]
    AUX_COMPA,
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    AON_RTC_CH2,
}
impl VEC0_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VEC0_EVW::ADC_IRQ => 31,
            VEC0_EVW::MCU_EV => 30,
            VEC0_EVW::ACLK_REF => 29,
            VEC0_EVW::AUXIO15 => 28,
            VEC0_EVW::AUXIO14 => 27,
            VEC0_EVW::AUXIO13 => 26,
            VEC0_EVW::AUXIO12 => 25,
            VEC0_EVW::AUXIO11 => 24,
            VEC0_EVW::AUXIO10 => 23,
            VEC0_EVW::AUXIO9 => 22,
            VEC0_EVW::AUXIO8 => 21,
            VEC0_EVW::AUXIO7 => 20,
            VEC0_EVW::AUXIO6 => 19,
            VEC0_EVW::AUXIO5 => 18,
            VEC0_EVW::AUXIO4 => 17,
            VEC0_EVW::AUXIO3 => 16,
            VEC0_EVW::AUXIO2 => 15,
            VEC0_EVW::AUXIO1 => 14,
            VEC0_EVW::AUXIO0 => 13,
            VEC0_EVW::AON_PROG_WU => 12,
            VEC0_EVW::AON_SW => 11,
            VEC0_EVW::OBSMUX1 => 10,
            VEC0_EVW::OBSMUX0 => 9,
            VEC0_EVW::ADC_FIFO_ALMOST_FULL => 8,
            VEC0_EVW::ADC_DONE => 7,
            VEC0_EVW::SMPH_AUTOTAKE_DONE => 6,
            VEC0_EVW::TIMER1_EV => 5,
            VEC0_EVW::TIMER0_EV => 4,
            VEC0_EVW::TDC_DONE => 3,
            VEC0_EVW::AUX_COMPB => 2,
            VEC0_EVW::AUX_COMPA => 1,
            VEC0_EVW::AON_RTC_CH2 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VEC0_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _VEC0_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VEC0_EVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline]
    pub fn adc_irq(self) -> &'a mut W {
        self.variant(VEC0_EVW::ADC_IRQ)
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(VEC0_EVW::MCU_EV)
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(VEC0_EVW::ACLK_REF)
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO15)
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO14)
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO13)
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO12)
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO11)
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO10)
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO9)
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO8)
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO7)
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO6)
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO5)
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO4)
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUXIO0)
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline]
    pub fn aon_prog_wu(self) -> &'a mut W {
        self.variant(VEC0_EVW::AON_PROG_WU)
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline]
    pub fn aon_sw(self) -> &'a mut W {
        self.variant(VEC0_EVW::AON_SW)
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline]
    pub fn obsmux1(self) -> &'a mut W {
        self.variant(VEC0_EVW::OBSMUX1)
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline]
    pub fn obsmux0(self) -> &'a mut W {
        self.variant(VEC0_EVW::OBSMUX0)
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline]
    pub fn adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(VEC0_EVW::ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline]
    pub fn adc_done(self) -> &'a mut W {
        self.variant(VEC0_EVW::ADC_DONE)
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline]
    pub fn smph_autotake_done(self) -> &'a mut W {
        self.variant(VEC0_EVW::SMPH_AUTOTAKE_DONE)
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline]
    pub fn timer1_ev(self) -> &'a mut W {
        self.variant(VEC0_EVW::TIMER1_EV)
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline]
    pub fn timer0_ev(self) -> &'a mut W {
        self.variant(VEC0_EVW::TIMER0_EV)
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline]
    pub fn tdc_done(self) -> &'a mut W {
        self.variant(VEC0_EVW::TDC_DONE)
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUX_COMPB)
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(VEC0_EVW::AUX_COMPA)
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(VEC0_EVW::AON_RTC_CH2)
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
    #[doc = "Bit 14 - 14:14\\] Vector 1 trigger event polarity. To manually trigger vector 1 execution: - AUX_SCE must sleep. - Set VEC1_EV to a known static value. - Toggle VEC1_POL twice."]
    #[inline]
    pub fn vec1_pol(&self) -> VEC1_POLR {
        VEC1_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - 13:13\\] Vector 1 trigger enable. When enabled, VEC1_EV event with VEC1_POL polarity triggers a jump to vector # 1 when AUX_SCE sleeps. Lower vectors (0) have priority."]
    #[inline]
    pub fn vec1_en(&self) -> VEC1_ENR {
        VEC1_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - 12:8\\] Select vector 1 trigger source event."]
    #[inline]
    pub fn vec1_ev(&self) -> VEC1_EVR {
        VEC1_EVR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 6 - 6:6\\] Vector 0 trigger event polarity. To manually trigger vector 0 execution: - AUX_SCE must sleep. - Set VEC0_EV to a known static value. - Toggle VEC0_POL twice."]
    #[inline]
    pub fn vec0_pol(&self) -> VEC0_POLR {
        VEC0_POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - 5:5\\] Vector 0 trigger enable. When enabled, VEC0_EV event with VEC0_POL polarity triggers a jump to vector # 0 when AUX_SCE sleeps."]
    #[inline]
    pub fn vec0_en(&self) -> VEC0_ENR {
        VEC0_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:4 - 4:0\\] Select vector 0 trigger source event."]
    #[inline]
    pub fn vec0_ev(&self) -> VEC0_EVR {
        VEC0_EVR::_from({
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
    #[doc = "Bit 14 - 14:14\\] Vector 1 trigger event polarity. To manually trigger vector 1 execution: - AUX_SCE must sleep. - Set VEC1_EV to a known static value. - Toggle VEC1_POL twice."]
    #[inline]
    pub fn vec1_pol(&mut self) -> _VEC1_POLW {
        _VEC1_POLW { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Vector 1 trigger enable. When enabled, VEC1_EV event with VEC1_POL polarity triggers a jump to vector # 1 when AUX_SCE sleeps. Lower vectors (0) have priority."]
    #[inline]
    pub fn vec1_en(&mut self) -> _VEC1_ENW {
        _VEC1_ENW { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\] Select vector 1 trigger source event."]
    #[inline]
    pub fn vec1_ev(&mut self) -> _VEC1_EVW {
        _VEC1_EVW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved7(&mut self) -> _RESERVED7W {
        _RESERVED7W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Vector 0 trigger event polarity. To manually trigger vector 0 execution: - AUX_SCE must sleep. - Set VEC0_EV to a known static value. - Toggle VEC0_POL twice."]
    #[inline]
    pub fn vec0_pol(&mut self) -> _VEC0_POLW {
        _VEC0_POLW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Vector 0 trigger enable. When enabled, VEC0_EV event with VEC0_POL polarity triggers a jump to vector # 0 when AUX_SCE sleeps."]
    #[inline]
    pub fn vec0_en(&mut self) -> _VEC0_ENW {
        _VEC0_ENW { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\] Select vector 0 trigger source event."]
    #[inline]
    pub fn vec0_ev(&mut self) -> _VEC0_EVW {
        _VEC0_EVW { w: self }
    }
}
