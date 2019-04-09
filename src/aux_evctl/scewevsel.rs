#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCEWEVSEL {
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
pub struct RESERVED5R {
    bits: u32,
}
impl RESERVED5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `WEV7_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WEV7_EVR {
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
impl WEV7_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WEV7_EVR::ADC_IRQ => 31,
            WEV7_EVR::MCU_EV => 30,
            WEV7_EVR::ACLK_REF => 29,
            WEV7_EVR::AUXIO15 => 28,
            WEV7_EVR::AUXIO14 => 27,
            WEV7_EVR::AUXIO13 => 26,
            WEV7_EVR::AUXIO12 => 25,
            WEV7_EVR::AUXIO11 => 24,
            WEV7_EVR::AUXIO10 => 23,
            WEV7_EVR::AUXIO9 => 22,
            WEV7_EVR::AUXIO8 => 21,
            WEV7_EVR::AUXIO7 => 20,
            WEV7_EVR::AUXIO6 => 19,
            WEV7_EVR::AUXIO5 => 18,
            WEV7_EVR::AUXIO4 => 17,
            WEV7_EVR::AUXIO3 => 16,
            WEV7_EVR::AUXIO2 => 15,
            WEV7_EVR::AUXIO1 => 14,
            WEV7_EVR::AUXIO0 => 13,
            WEV7_EVR::AON_PROG_WU => 12,
            WEV7_EVR::AON_SW => 11,
            WEV7_EVR::OBSMUX1 => 10,
            WEV7_EVR::OBSMUX0 => 9,
            WEV7_EVR::ADC_FIFO_ALMOST_FULL => 8,
            WEV7_EVR::ADC_DONE => 7,
            WEV7_EVR::SMPH_AUTOTAKE_DONE => 6,
            WEV7_EVR::TIMER1_EV => 5,
            WEV7_EVR::TIMER0_EV => 4,
            WEV7_EVR::TDC_DONE => 3,
            WEV7_EVR::AUX_COMPB => 2,
            WEV7_EVR::AUX_COMPA => 1,
            WEV7_EVR::AON_RTC_CH2 => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WEV7_EVR {
        match value {
            31 => WEV7_EVR::ADC_IRQ,
            30 => WEV7_EVR::MCU_EV,
            29 => WEV7_EVR::ACLK_REF,
            28 => WEV7_EVR::AUXIO15,
            27 => WEV7_EVR::AUXIO14,
            26 => WEV7_EVR::AUXIO13,
            25 => WEV7_EVR::AUXIO12,
            24 => WEV7_EVR::AUXIO11,
            23 => WEV7_EVR::AUXIO10,
            22 => WEV7_EVR::AUXIO9,
            21 => WEV7_EVR::AUXIO8,
            20 => WEV7_EVR::AUXIO7,
            19 => WEV7_EVR::AUXIO6,
            18 => WEV7_EVR::AUXIO5,
            17 => WEV7_EVR::AUXIO4,
            16 => WEV7_EVR::AUXIO3,
            15 => WEV7_EVR::AUXIO2,
            14 => WEV7_EVR::AUXIO1,
            13 => WEV7_EVR::AUXIO0,
            12 => WEV7_EVR::AON_PROG_WU,
            11 => WEV7_EVR::AON_SW,
            10 => WEV7_EVR::OBSMUX1,
            9 => WEV7_EVR::OBSMUX0,
            8 => WEV7_EVR::ADC_FIFO_ALMOST_FULL,
            7 => WEV7_EVR::ADC_DONE,
            6 => WEV7_EVR::SMPH_AUTOTAKE_DONE,
            5 => WEV7_EVR::TIMER1_EV,
            4 => WEV7_EVR::TIMER0_EV,
            3 => WEV7_EVR::TDC_DONE,
            2 => WEV7_EVR::AUX_COMPB,
            1 => WEV7_EVR::AUX_COMPA,
            0 => WEV7_EVR::AON_RTC_CH2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_IRQ`"]
    #[inline]
    pub fn is_adc_irq(&self) -> bool {
        *self == WEV7_EVR::ADC_IRQ
    }
    #[doc = "Checks if the value of the field is `MCU_EV`"]
    #[inline]
    pub fn is_mcu_ev(&self) -> bool {
        *self == WEV7_EVR::MCU_EV
    }
    #[doc = "Checks if the value of the field is `ACLK_REF`"]
    #[inline]
    pub fn is_aclk_ref(&self) -> bool {
        *self == WEV7_EVR::ACLK_REF
    }
    #[doc = "Checks if the value of the field is `AUXIO15`"]
    #[inline]
    pub fn is_auxio15(&self) -> bool {
        *self == WEV7_EVR::AUXIO15
    }
    #[doc = "Checks if the value of the field is `AUXIO14`"]
    #[inline]
    pub fn is_auxio14(&self) -> bool {
        *self == WEV7_EVR::AUXIO14
    }
    #[doc = "Checks if the value of the field is `AUXIO13`"]
    #[inline]
    pub fn is_auxio13(&self) -> bool {
        *self == WEV7_EVR::AUXIO13
    }
    #[doc = "Checks if the value of the field is `AUXIO12`"]
    #[inline]
    pub fn is_auxio12(&self) -> bool {
        *self == WEV7_EVR::AUXIO12
    }
    #[doc = "Checks if the value of the field is `AUXIO11`"]
    #[inline]
    pub fn is_auxio11(&self) -> bool {
        *self == WEV7_EVR::AUXIO11
    }
    #[doc = "Checks if the value of the field is `AUXIO10`"]
    #[inline]
    pub fn is_auxio10(&self) -> bool {
        *self == WEV7_EVR::AUXIO10
    }
    #[doc = "Checks if the value of the field is `AUXIO9`"]
    #[inline]
    pub fn is_auxio9(&self) -> bool {
        *self == WEV7_EVR::AUXIO9
    }
    #[doc = "Checks if the value of the field is `AUXIO8`"]
    #[inline]
    pub fn is_auxio8(&self) -> bool {
        *self == WEV7_EVR::AUXIO8
    }
    #[doc = "Checks if the value of the field is `AUXIO7`"]
    #[inline]
    pub fn is_auxio7(&self) -> bool {
        *self == WEV7_EVR::AUXIO7
    }
    #[doc = "Checks if the value of the field is `AUXIO6`"]
    #[inline]
    pub fn is_auxio6(&self) -> bool {
        *self == WEV7_EVR::AUXIO6
    }
    #[doc = "Checks if the value of the field is `AUXIO5`"]
    #[inline]
    pub fn is_auxio5(&self) -> bool {
        *self == WEV7_EVR::AUXIO5
    }
    #[doc = "Checks if the value of the field is `AUXIO4`"]
    #[inline]
    pub fn is_auxio4(&self) -> bool {
        *self == WEV7_EVR::AUXIO4
    }
    #[doc = "Checks if the value of the field is `AUXIO3`"]
    #[inline]
    pub fn is_auxio3(&self) -> bool {
        *self == WEV7_EVR::AUXIO3
    }
    #[doc = "Checks if the value of the field is `AUXIO2`"]
    #[inline]
    pub fn is_auxio2(&self) -> bool {
        *self == WEV7_EVR::AUXIO2
    }
    #[doc = "Checks if the value of the field is `AUXIO1`"]
    #[inline]
    pub fn is_auxio1(&self) -> bool {
        *self == WEV7_EVR::AUXIO1
    }
    #[doc = "Checks if the value of the field is `AUXIO0`"]
    #[inline]
    pub fn is_auxio0(&self) -> bool {
        *self == WEV7_EVR::AUXIO0
    }
    #[doc = "Checks if the value of the field is `AON_PROG_WU`"]
    #[inline]
    pub fn is_aon_prog_wu(&self) -> bool {
        *self == WEV7_EVR::AON_PROG_WU
    }
    #[doc = "Checks if the value of the field is `AON_SW`"]
    #[inline]
    pub fn is_aon_sw(&self) -> bool {
        *self == WEV7_EVR::AON_SW
    }
    #[doc = "Checks if the value of the field is `OBSMUX1`"]
    #[inline]
    pub fn is_obsmux1(&self) -> bool {
        *self == WEV7_EVR::OBSMUX1
    }
    #[doc = "Checks if the value of the field is `OBSMUX0`"]
    #[inline]
    pub fn is_obsmux0(&self) -> bool {
        *self == WEV7_EVR::OBSMUX0
    }
    #[doc = "Checks if the value of the field is `ADC_FIFO_ALMOST_FULL`"]
    #[inline]
    pub fn is_adc_fifo_almost_full(&self) -> bool {
        *self == WEV7_EVR::ADC_FIFO_ALMOST_FULL
    }
    #[doc = "Checks if the value of the field is `ADC_DONE`"]
    #[inline]
    pub fn is_adc_done(&self) -> bool {
        *self == WEV7_EVR::ADC_DONE
    }
    #[doc = "Checks if the value of the field is `SMPH_AUTOTAKE_DONE`"]
    #[inline]
    pub fn is_smph_autotake_done(&self) -> bool {
        *self == WEV7_EVR::SMPH_AUTOTAKE_DONE
    }
    #[doc = "Checks if the value of the field is `TIMER1_EV`"]
    #[inline]
    pub fn is_timer1_ev(&self) -> bool {
        *self == WEV7_EVR::TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `TIMER0_EV`"]
    #[inline]
    pub fn is_timer0_ev(&self) -> bool {
        *self == WEV7_EVR::TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `TDC_DONE`"]
    #[inline]
    pub fn is_tdc_done(&self) -> bool {
        *self == WEV7_EVR::TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == WEV7_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == WEV7_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AON_RTC_CH2`"]
    #[inline]
    pub fn is_aon_rtc_ch2(&self) -> bool {
        *self == WEV7_EVR::AON_RTC_CH2
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED5W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 134217727;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WEV7_EV`"]
pub enum WEV7_EVW {
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
impl WEV7_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WEV7_EVW::ADC_IRQ => 31,
            WEV7_EVW::MCU_EV => 30,
            WEV7_EVW::ACLK_REF => 29,
            WEV7_EVW::AUXIO15 => 28,
            WEV7_EVW::AUXIO14 => 27,
            WEV7_EVW::AUXIO13 => 26,
            WEV7_EVW::AUXIO12 => 25,
            WEV7_EVW::AUXIO11 => 24,
            WEV7_EVW::AUXIO10 => 23,
            WEV7_EVW::AUXIO9 => 22,
            WEV7_EVW::AUXIO8 => 21,
            WEV7_EVW::AUXIO7 => 20,
            WEV7_EVW::AUXIO6 => 19,
            WEV7_EVW::AUXIO5 => 18,
            WEV7_EVW::AUXIO4 => 17,
            WEV7_EVW::AUXIO3 => 16,
            WEV7_EVW::AUXIO2 => 15,
            WEV7_EVW::AUXIO1 => 14,
            WEV7_EVW::AUXIO0 => 13,
            WEV7_EVW::AON_PROG_WU => 12,
            WEV7_EVW::AON_SW => 11,
            WEV7_EVW::OBSMUX1 => 10,
            WEV7_EVW::OBSMUX0 => 9,
            WEV7_EVW::ADC_FIFO_ALMOST_FULL => 8,
            WEV7_EVW::ADC_DONE => 7,
            WEV7_EVW::SMPH_AUTOTAKE_DONE => 6,
            WEV7_EVW::TIMER1_EV => 5,
            WEV7_EVW::TIMER0_EV => 4,
            WEV7_EVW::TDC_DONE => 3,
            WEV7_EVW::AUX_COMPB => 2,
            WEV7_EVW::AUX_COMPA => 1,
            WEV7_EVW::AON_RTC_CH2 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WEV7_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _WEV7_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WEV7_EVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EVSTAT1.ADC_IRQ"]
    #[inline]
    pub fn adc_irq(self) -> &'a mut W {
        self.variant(WEV7_EVW::ADC_IRQ)
    }
    #[doc = "EVSTAT1.MCU_EV"]
    #[inline]
    pub fn mcu_ev(self) -> &'a mut W {
        self.variant(WEV7_EVW::MCU_EV)
    }
    #[doc = "EVSTAT1.ACLK_REF"]
    #[inline]
    pub fn aclk_ref(self) -> &'a mut W {
        self.variant(WEV7_EVW::ACLK_REF)
    }
    #[doc = "EVSTAT1.AUXIO15"]
    #[inline]
    pub fn auxio15(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO15)
    }
    #[doc = "EVSTAT1.AUXIO14"]
    #[inline]
    pub fn auxio14(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO14)
    }
    #[doc = "EVSTAT1.AUXIO13"]
    #[inline]
    pub fn auxio13(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO13)
    }
    #[doc = "EVSTAT1.AUXIO12"]
    #[inline]
    pub fn auxio12(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO12)
    }
    #[doc = "EVSTAT1.AUXIO11"]
    #[inline]
    pub fn auxio11(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO11)
    }
    #[doc = "EVSTAT1.AUXIO10"]
    #[inline]
    pub fn auxio10(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO10)
    }
    #[doc = "EVSTAT1.AUXIO9"]
    #[inline]
    pub fn auxio9(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO9)
    }
    #[doc = "EVSTAT1.AUXIO8"]
    #[inline]
    pub fn auxio8(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO8)
    }
    #[doc = "EVSTAT1.AUXIO7"]
    #[inline]
    pub fn auxio7(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO7)
    }
    #[doc = "EVSTAT1.AUXIO6"]
    #[inline]
    pub fn auxio6(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO6)
    }
    #[doc = "EVSTAT1.AUXIO5"]
    #[inline]
    pub fn auxio5(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO5)
    }
    #[doc = "EVSTAT1.AUXIO4"]
    #[inline]
    pub fn auxio4(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO4)
    }
    #[doc = "EVSTAT1.AUXIO3"]
    #[inline]
    pub fn auxio3(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO3)
    }
    #[doc = "EVSTAT0.AUXIO2"]
    #[inline]
    pub fn auxio2(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO2)
    }
    #[doc = "EVSTAT0.AUXIO1"]
    #[inline]
    pub fn auxio1(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO1)
    }
    #[doc = "EVSTAT0.AUXIO0"]
    #[inline]
    pub fn auxio0(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUXIO0)
    }
    #[doc = "EVSTAT0.AON_PROG_WU"]
    #[inline]
    pub fn aon_prog_wu(self) -> &'a mut W {
        self.variant(WEV7_EVW::AON_PROG_WU)
    }
    #[doc = "EVSTAT0.AON_SW"]
    #[inline]
    pub fn aon_sw(self) -> &'a mut W {
        self.variant(WEV7_EVW::AON_SW)
    }
    #[doc = "EVSTAT0.OBSMUX1"]
    #[inline]
    pub fn obsmux1(self) -> &'a mut W {
        self.variant(WEV7_EVW::OBSMUX1)
    }
    #[doc = "EVSTAT0.OBSMUX0"]
    #[inline]
    pub fn obsmux0(self) -> &'a mut W {
        self.variant(WEV7_EVW::OBSMUX0)
    }
    #[doc = "EVSTAT0.ADC_FIFO_ALMOST_FULL"]
    #[inline]
    pub fn adc_fifo_almost_full(self) -> &'a mut W {
        self.variant(WEV7_EVW::ADC_FIFO_ALMOST_FULL)
    }
    #[doc = "EVSTAT0.ADC_DONE"]
    #[inline]
    pub fn adc_done(self) -> &'a mut W {
        self.variant(WEV7_EVW::ADC_DONE)
    }
    #[doc = "EVSTAT0.SMPH_AUTOTAKE_DONE"]
    #[inline]
    pub fn smph_autotake_done(self) -> &'a mut W {
        self.variant(WEV7_EVW::SMPH_AUTOTAKE_DONE)
    }
    #[doc = "EVSTAT0.TIMER1_EV"]
    #[inline]
    pub fn timer1_ev(self) -> &'a mut W {
        self.variant(WEV7_EVW::TIMER1_EV)
    }
    #[doc = "EVSTAT0.TIMER0_EV"]
    #[inline]
    pub fn timer0_ev(self) -> &'a mut W {
        self.variant(WEV7_EVW::TIMER0_EV)
    }
    #[doc = "EVSTAT0.TDC_DONE"]
    #[inline]
    pub fn tdc_done(self) -> &'a mut W {
        self.variant(WEV7_EVW::TDC_DONE)
    }
    #[doc = "EVSTAT0.AUX_COMPB"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUX_COMPB)
    }
    #[doc = "EVSTAT0.AUX_COMPA"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WEV7_EVW::AUX_COMPA)
    }
    #[doc = "EVSTAT0.AON_RTC_CH2"]
    #[inline]
    pub fn aon_rtc_ch2(self) -> &'a mut W {
        self.variant(WEV7_EVW::AON_RTC_CH2)
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
    #[doc = "Bits 5:31 - 31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved5(&self) -> RESERVED5R {
        let bits = {
            const MASK: u32 = 134217727;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED5R { bits }
    }
    #[doc = "Bits 0:4 - 4:0\\] Select event source to connect to AUX_SCE:WUSTAT.EV_SIGNALS bit 7."]
    #[inline]
    pub fn wev7_ev(&self) -> WEV7_EVR {
        WEV7_EVR::_from({
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
    #[doc = "Bits 5:31 - 31:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved5(&mut self) -> _RESERVED5W {
        _RESERVED5W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\] Select event source to connect to AUX_SCE:WUSTAT.EV_SIGNALS bit 7."]
    #[inline]
    pub fn wev7_ev(&mut self) -> _WEV7_EVW {
        _WEV7_EVW { w: self }
    }
}
