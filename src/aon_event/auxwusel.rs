#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUXWUSEL {
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
pub struct RESERVED22R {
    bits: u16,
}
impl RESERVED22R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `WU2_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU2_EVR {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Edge detect on PAD31"]
    PAD31,
    #[doc = "Edge detect on PAD30"]
    PAD30,
    #[doc = "Edge detect on PAD29"]
    PAD29,
    #[doc = "Edge detect on PAD28"]
    PAD28,
    #[doc = "Edge detect on PAD27"]
    PAD27,
    #[doc = "Edge detect on PAD26"]
    PAD26,
    #[doc = "Edge detect on PAD25"]
    PAD25,
    #[doc = "Edge detect on PAD24"]
    PAD24,
    #[doc = "Edge detect on PAD23"]
    PAD23,
    #[doc = "Edge detect on PAD22"]
    PAD22,
    #[doc = "Edge detect on PAD21"]
    PAD21,
    #[doc = "Edge detect on PAD20"]
    PAD20,
    #[doc = "Edge detect on PAD19"]
    PAD19,
    #[doc = "Edge detect on PAD18"]
    PAD18,
    #[doc = "Edge detect on PAD17"]
    PAD17,
    #[doc = "Edge detect on PAD16"]
    PAD16,
    #[doc = "Edge detect on PAD15"]
    PAD15,
    #[doc = "Edge detect on PAD14"]
    PAD14,
    #[doc = "Edge detect on PAD13"]
    PAD13,
    #[doc = "Edge detect on PAD12"]
    PAD12,
    #[doc = "Edge detect on PAD11"]
    PAD11,
    #[doc = "Edge detect on PAD10"]
    PAD10,
    #[doc = "Edge detect on PAD9"]
    PAD9,
    #[doc = "Edge detect on PAD8"]
    PAD8,
    #[doc = "Edge detect on PAD7"]
    PAD7,
    #[doc = "Edge detect on PAD6"]
    PAD6,
    #[doc = "Edge detect on PAD5"]
    PAD5,
    #[doc = "Edge detect on PAD4"]
    PAD4,
    #[doc = "Edge detect on PAD3"]
    PAD3,
    #[doc = "Edge detect on PAD2"]
    PAD2,
    #[doc = "Edge detect on PAD1"]
    PAD1,
    #[doc = "Edge detect on PAD0"]
    PAD0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WU2_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WU2_EVR::NONE => 63,
            WU2_EVR::AUX_COMPB_ASYNC_N => 56,
            WU2_EVR::AUX_COMPB_ASYNC => 55,
            WU2_EVR::BATMON_VOLT => 54,
            WU2_EVR::BATMON_TEMP => 53,
            WU2_EVR::AUX_TIMER1_EV => 52,
            WU2_EVR::AUX_TIMER0_EV => 51,
            WU2_EVR::AUX_TDC_DONE => 50,
            WU2_EVR::AUX_ADC_DONE => 49,
            WU2_EVR::AUX_COMPB => 48,
            WU2_EVR::AUX_COMPA => 47,
            WU2_EVR::AUX_SWEV2 => 46,
            WU2_EVR::AUX_SWEV1 => 45,
            WU2_EVR::AUX_SWEV0 => 44,
            WU2_EVR::JTAG => 43,
            WU2_EVR::RTC_UPD => 42,
            WU2_EVR::RTC_COMB_DLY => 41,
            WU2_EVR::RTC_CH2_DLY => 40,
            WU2_EVR::RTC_CH1_DLY => 39,
            WU2_EVR::RTC_CH0_DLY => 38,
            WU2_EVR::RTC_CH2 => 37,
            WU2_EVR::RTC_CH1 => 36,
            WU2_EVR::RTC_CH0 => 35,
            WU2_EVR::PAD => 32,
            WU2_EVR::PAD31 => 31,
            WU2_EVR::PAD30 => 30,
            WU2_EVR::PAD29 => 29,
            WU2_EVR::PAD28 => 28,
            WU2_EVR::PAD27 => 27,
            WU2_EVR::PAD26 => 26,
            WU2_EVR::PAD25 => 25,
            WU2_EVR::PAD24 => 24,
            WU2_EVR::PAD23 => 23,
            WU2_EVR::PAD22 => 22,
            WU2_EVR::PAD21 => 21,
            WU2_EVR::PAD20 => 20,
            WU2_EVR::PAD19 => 19,
            WU2_EVR::PAD18 => 18,
            WU2_EVR::PAD17 => 17,
            WU2_EVR::PAD16 => 16,
            WU2_EVR::PAD15 => 15,
            WU2_EVR::PAD14 => 14,
            WU2_EVR::PAD13 => 13,
            WU2_EVR::PAD12 => 12,
            WU2_EVR::PAD11 => 11,
            WU2_EVR::PAD10 => 10,
            WU2_EVR::PAD9 => 9,
            WU2_EVR::PAD8 => 8,
            WU2_EVR::PAD7 => 7,
            WU2_EVR::PAD6 => 6,
            WU2_EVR::PAD5 => 5,
            WU2_EVR::PAD4 => 4,
            WU2_EVR::PAD3 => 3,
            WU2_EVR::PAD2 => 2,
            WU2_EVR::PAD1 => 1,
            WU2_EVR::PAD0 => 0,
            WU2_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WU2_EVR {
        match value {
            63 => WU2_EVR::NONE,
            56 => WU2_EVR::AUX_COMPB_ASYNC_N,
            55 => WU2_EVR::AUX_COMPB_ASYNC,
            54 => WU2_EVR::BATMON_VOLT,
            53 => WU2_EVR::BATMON_TEMP,
            52 => WU2_EVR::AUX_TIMER1_EV,
            51 => WU2_EVR::AUX_TIMER0_EV,
            50 => WU2_EVR::AUX_TDC_DONE,
            49 => WU2_EVR::AUX_ADC_DONE,
            48 => WU2_EVR::AUX_COMPB,
            47 => WU2_EVR::AUX_COMPA,
            46 => WU2_EVR::AUX_SWEV2,
            45 => WU2_EVR::AUX_SWEV1,
            44 => WU2_EVR::AUX_SWEV0,
            43 => WU2_EVR::JTAG,
            42 => WU2_EVR::RTC_UPD,
            41 => WU2_EVR::RTC_COMB_DLY,
            40 => WU2_EVR::RTC_CH2_DLY,
            39 => WU2_EVR::RTC_CH1_DLY,
            38 => WU2_EVR::RTC_CH0_DLY,
            37 => WU2_EVR::RTC_CH2,
            36 => WU2_EVR::RTC_CH1,
            35 => WU2_EVR::RTC_CH0,
            32 => WU2_EVR::PAD,
            31 => WU2_EVR::PAD31,
            30 => WU2_EVR::PAD30,
            29 => WU2_EVR::PAD29,
            28 => WU2_EVR::PAD28,
            27 => WU2_EVR::PAD27,
            26 => WU2_EVR::PAD26,
            25 => WU2_EVR::PAD25,
            24 => WU2_EVR::PAD24,
            23 => WU2_EVR::PAD23,
            22 => WU2_EVR::PAD22,
            21 => WU2_EVR::PAD21,
            20 => WU2_EVR::PAD20,
            19 => WU2_EVR::PAD19,
            18 => WU2_EVR::PAD18,
            17 => WU2_EVR::PAD17,
            16 => WU2_EVR::PAD16,
            15 => WU2_EVR::PAD15,
            14 => WU2_EVR::PAD14,
            13 => WU2_EVR::PAD13,
            12 => WU2_EVR::PAD12,
            11 => WU2_EVR::PAD11,
            10 => WU2_EVR::PAD10,
            9 => WU2_EVR::PAD9,
            8 => WU2_EVR::PAD8,
            7 => WU2_EVR::PAD7,
            6 => WU2_EVR::PAD6,
            5 => WU2_EVR::PAD5,
            4 => WU2_EVR::PAD4,
            3 => WU2_EVR::PAD3,
            2 => WU2_EVR::PAD2,
            1 => WU2_EVR::PAD1,
            0 => WU2_EVR::PAD0,
            i => WU2_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WU2_EVR::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU2_EVR::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU2_EVR::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU2_EVR::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU2_EVR::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU2_EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU2_EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU2_EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU2_EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU2_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU2_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU2_EVR::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU2_EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU2_EVR::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline]
    pub fn is_jtag(&self) -> bool {
        *self == WU2_EVR::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU2_EVR::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU2_EVR::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU2_EVR::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU2_EVR::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU2_EVR::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU2_EVR::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU2_EVR::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU2_EVR::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline]
    pub fn is_pad(&self) -> bool {
        *self == WU2_EVR::PAD
    }
    #[doc = "Checks if the value of the field is `PAD31`"]
    #[inline]
    pub fn is_pad31(&self) -> bool {
        *self == WU2_EVR::PAD31
    }
    #[doc = "Checks if the value of the field is `PAD30`"]
    #[inline]
    pub fn is_pad30(&self) -> bool {
        *self == WU2_EVR::PAD30
    }
    #[doc = "Checks if the value of the field is `PAD29`"]
    #[inline]
    pub fn is_pad29(&self) -> bool {
        *self == WU2_EVR::PAD29
    }
    #[doc = "Checks if the value of the field is `PAD28`"]
    #[inline]
    pub fn is_pad28(&self) -> bool {
        *self == WU2_EVR::PAD28
    }
    #[doc = "Checks if the value of the field is `PAD27`"]
    #[inline]
    pub fn is_pad27(&self) -> bool {
        *self == WU2_EVR::PAD27
    }
    #[doc = "Checks if the value of the field is `PAD26`"]
    #[inline]
    pub fn is_pad26(&self) -> bool {
        *self == WU2_EVR::PAD26
    }
    #[doc = "Checks if the value of the field is `PAD25`"]
    #[inline]
    pub fn is_pad25(&self) -> bool {
        *self == WU2_EVR::PAD25
    }
    #[doc = "Checks if the value of the field is `PAD24`"]
    #[inline]
    pub fn is_pad24(&self) -> bool {
        *self == WU2_EVR::PAD24
    }
    #[doc = "Checks if the value of the field is `PAD23`"]
    #[inline]
    pub fn is_pad23(&self) -> bool {
        *self == WU2_EVR::PAD23
    }
    #[doc = "Checks if the value of the field is `PAD22`"]
    #[inline]
    pub fn is_pad22(&self) -> bool {
        *self == WU2_EVR::PAD22
    }
    #[doc = "Checks if the value of the field is `PAD21`"]
    #[inline]
    pub fn is_pad21(&self) -> bool {
        *self == WU2_EVR::PAD21
    }
    #[doc = "Checks if the value of the field is `PAD20`"]
    #[inline]
    pub fn is_pad20(&self) -> bool {
        *self == WU2_EVR::PAD20
    }
    #[doc = "Checks if the value of the field is `PAD19`"]
    #[inline]
    pub fn is_pad19(&self) -> bool {
        *self == WU2_EVR::PAD19
    }
    #[doc = "Checks if the value of the field is `PAD18`"]
    #[inline]
    pub fn is_pad18(&self) -> bool {
        *self == WU2_EVR::PAD18
    }
    #[doc = "Checks if the value of the field is `PAD17`"]
    #[inline]
    pub fn is_pad17(&self) -> bool {
        *self == WU2_EVR::PAD17
    }
    #[doc = "Checks if the value of the field is `PAD16`"]
    #[inline]
    pub fn is_pad16(&self) -> bool {
        *self == WU2_EVR::PAD16
    }
    #[doc = "Checks if the value of the field is `PAD15`"]
    #[inline]
    pub fn is_pad15(&self) -> bool {
        *self == WU2_EVR::PAD15
    }
    #[doc = "Checks if the value of the field is `PAD14`"]
    #[inline]
    pub fn is_pad14(&self) -> bool {
        *self == WU2_EVR::PAD14
    }
    #[doc = "Checks if the value of the field is `PAD13`"]
    #[inline]
    pub fn is_pad13(&self) -> bool {
        *self == WU2_EVR::PAD13
    }
    #[doc = "Checks if the value of the field is `PAD12`"]
    #[inline]
    pub fn is_pad12(&self) -> bool {
        *self == WU2_EVR::PAD12
    }
    #[doc = "Checks if the value of the field is `PAD11`"]
    #[inline]
    pub fn is_pad11(&self) -> bool {
        *self == WU2_EVR::PAD11
    }
    #[doc = "Checks if the value of the field is `PAD10`"]
    #[inline]
    pub fn is_pad10(&self) -> bool {
        *self == WU2_EVR::PAD10
    }
    #[doc = "Checks if the value of the field is `PAD9`"]
    #[inline]
    pub fn is_pad9(&self) -> bool {
        *self == WU2_EVR::PAD9
    }
    #[doc = "Checks if the value of the field is `PAD8`"]
    #[inline]
    pub fn is_pad8(&self) -> bool {
        *self == WU2_EVR::PAD8
    }
    #[doc = "Checks if the value of the field is `PAD7`"]
    #[inline]
    pub fn is_pad7(&self) -> bool {
        *self == WU2_EVR::PAD7
    }
    #[doc = "Checks if the value of the field is `PAD6`"]
    #[inline]
    pub fn is_pad6(&self) -> bool {
        *self == WU2_EVR::PAD6
    }
    #[doc = "Checks if the value of the field is `PAD5`"]
    #[inline]
    pub fn is_pad5(&self) -> bool {
        *self == WU2_EVR::PAD5
    }
    #[doc = "Checks if the value of the field is `PAD4`"]
    #[inline]
    pub fn is_pad4(&self) -> bool {
        *self == WU2_EVR::PAD4
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline]
    pub fn is_pad3(&self) -> bool {
        *self == WU2_EVR::PAD3
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline]
    pub fn is_pad2(&self) -> bool {
        *self == WU2_EVR::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline]
    pub fn is_pad1(&self) -> bool {
        *self == WU2_EVR::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline]
    pub fn is_pad0(&self) -> bool {
        *self == WU2_EVR::PAD0
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED14R {
    bits: u8,
}
impl RESERVED14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WU1_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU1_EVR {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Edge detect on PAD31"]
    PAD31,
    #[doc = "Edge detect on PAD30"]
    PAD30,
    #[doc = "Edge detect on PAD29"]
    PAD29,
    #[doc = "Edge detect on PAD28"]
    PAD28,
    #[doc = "Edge detect on PAD27"]
    PAD27,
    #[doc = "Edge detect on PAD26"]
    PAD26,
    #[doc = "Edge detect on PAD25"]
    PAD25,
    #[doc = "Edge detect on PAD24"]
    PAD24,
    #[doc = "Edge detect on PAD23"]
    PAD23,
    #[doc = "Edge detect on PAD22"]
    PAD22,
    #[doc = "Edge detect on PAD21"]
    PAD21,
    #[doc = "Edge detect on PAD20"]
    PAD20,
    #[doc = "Edge detect on PAD19"]
    PAD19,
    #[doc = "Edge detect on PAD18"]
    PAD18,
    #[doc = "Edge detect on PAD17"]
    PAD17,
    #[doc = "Edge detect on PAD16"]
    PAD16,
    #[doc = "Edge detect on PAD15"]
    PAD15,
    #[doc = "Edge detect on PAD14"]
    PAD14,
    #[doc = "Edge detect on PAD13"]
    PAD13,
    #[doc = "Edge detect on PAD12"]
    PAD12,
    #[doc = "Edge detect on PAD11"]
    PAD11,
    #[doc = "Edge detect on PAD10"]
    PAD10,
    #[doc = "Edge detect on PAD9"]
    PAD9,
    #[doc = "Edge detect on PAD8"]
    PAD8,
    #[doc = "Edge detect on PAD7"]
    PAD7,
    #[doc = "Edge detect on PAD6"]
    PAD6,
    #[doc = "Edge detect on PAD5"]
    PAD5,
    #[doc = "Edge detect on PAD4"]
    PAD4,
    #[doc = "Edge detect on PAD3"]
    PAD3,
    #[doc = "Edge detect on PAD2"]
    PAD2,
    #[doc = "Edge detect on PAD1"]
    PAD1,
    #[doc = "Edge detect on PAD0"]
    PAD0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WU1_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WU1_EVR::NONE => 63,
            WU1_EVR::AUX_COMPB_ASYNC_N => 56,
            WU1_EVR::AUX_COMPB_ASYNC => 55,
            WU1_EVR::BATMON_VOLT => 54,
            WU1_EVR::BATMON_TEMP => 53,
            WU1_EVR::AUX_TIMER1_EV => 52,
            WU1_EVR::AUX_TIMER0_EV => 51,
            WU1_EVR::AUX_TDC_DONE => 50,
            WU1_EVR::AUX_ADC_DONE => 49,
            WU1_EVR::AUX_COMPB => 48,
            WU1_EVR::AUX_COMPA => 47,
            WU1_EVR::AUX_SWEV2 => 46,
            WU1_EVR::AUX_SWEV1 => 45,
            WU1_EVR::AUX_SWEV0 => 44,
            WU1_EVR::JTAG => 43,
            WU1_EVR::RTC_UPD => 42,
            WU1_EVR::RTC_COMB_DLY => 41,
            WU1_EVR::RTC_CH2_DLY => 40,
            WU1_EVR::RTC_CH1_DLY => 39,
            WU1_EVR::RTC_CH0_DLY => 38,
            WU1_EVR::RTC_CH2 => 37,
            WU1_EVR::RTC_CH1 => 36,
            WU1_EVR::RTC_CH0 => 35,
            WU1_EVR::PAD => 32,
            WU1_EVR::PAD31 => 31,
            WU1_EVR::PAD30 => 30,
            WU1_EVR::PAD29 => 29,
            WU1_EVR::PAD28 => 28,
            WU1_EVR::PAD27 => 27,
            WU1_EVR::PAD26 => 26,
            WU1_EVR::PAD25 => 25,
            WU1_EVR::PAD24 => 24,
            WU1_EVR::PAD23 => 23,
            WU1_EVR::PAD22 => 22,
            WU1_EVR::PAD21 => 21,
            WU1_EVR::PAD20 => 20,
            WU1_EVR::PAD19 => 19,
            WU1_EVR::PAD18 => 18,
            WU1_EVR::PAD17 => 17,
            WU1_EVR::PAD16 => 16,
            WU1_EVR::PAD15 => 15,
            WU1_EVR::PAD14 => 14,
            WU1_EVR::PAD13 => 13,
            WU1_EVR::PAD12 => 12,
            WU1_EVR::PAD11 => 11,
            WU1_EVR::PAD10 => 10,
            WU1_EVR::PAD9 => 9,
            WU1_EVR::PAD8 => 8,
            WU1_EVR::PAD7 => 7,
            WU1_EVR::PAD6 => 6,
            WU1_EVR::PAD5 => 5,
            WU1_EVR::PAD4 => 4,
            WU1_EVR::PAD3 => 3,
            WU1_EVR::PAD2 => 2,
            WU1_EVR::PAD1 => 1,
            WU1_EVR::PAD0 => 0,
            WU1_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WU1_EVR {
        match value {
            63 => WU1_EVR::NONE,
            56 => WU1_EVR::AUX_COMPB_ASYNC_N,
            55 => WU1_EVR::AUX_COMPB_ASYNC,
            54 => WU1_EVR::BATMON_VOLT,
            53 => WU1_EVR::BATMON_TEMP,
            52 => WU1_EVR::AUX_TIMER1_EV,
            51 => WU1_EVR::AUX_TIMER0_EV,
            50 => WU1_EVR::AUX_TDC_DONE,
            49 => WU1_EVR::AUX_ADC_DONE,
            48 => WU1_EVR::AUX_COMPB,
            47 => WU1_EVR::AUX_COMPA,
            46 => WU1_EVR::AUX_SWEV2,
            45 => WU1_EVR::AUX_SWEV1,
            44 => WU1_EVR::AUX_SWEV0,
            43 => WU1_EVR::JTAG,
            42 => WU1_EVR::RTC_UPD,
            41 => WU1_EVR::RTC_COMB_DLY,
            40 => WU1_EVR::RTC_CH2_DLY,
            39 => WU1_EVR::RTC_CH1_DLY,
            38 => WU1_EVR::RTC_CH0_DLY,
            37 => WU1_EVR::RTC_CH2,
            36 => WU1_EVR::RTC_CH1,
            35 => WU1_EVR::RTC_CH0,
            32 => WU1_EVR::PAD,
            31 => WU1_EVR::PAD31,
            30 => WU1_EVR::PAD30,
            29 => WU1_EVR::PAD29,
            28 => WU1_EVR::PAD28,
            27 => WU1_EVR::PAD27,
            26 => WU1_EVR::PAD26,
            25 => WU1_EVR::PAD25,
            24 => WU1_EVR::PAD24,
            23 => WU1_EVR::PAD23,
            22 => WU1_EVR::PAD22,
            21 => WU1_EVR::PAD21,
            20 => WU1_EVR::PAD20,
            19 => WU1_EVR::PAD19,
            18 => WU1_EVR::PAD18,
            17 => WU1_EVR::PAD17,
            16 => WU1_EVR::PAD16,
            15 => WU1_EVR::PAD15,
            14 => WU1_EVR::PAD14,
            13 => WU1_EVR::PAD13,
            12 => WU1_EVR::PAD12,
            11 => WU1_EVR::PAD11,
            10 => WU1_EVR::PAD10,
            9 => WU1_EVR::PAD9,
            8 => WU1_EVR::PAD8,
            7 => WU1_EVR::PAD7,
            6 => WU1_EVR::PAD6,
            5 => WU1_EVR::PAD5,
            4 => WU1_EVR::PAD4,
            3 => WU1_EVR::PAD3,
            2 => WU1_EVR::PAD2,
            1 => WU1_EVR::PAD1,
            0 => WU1_EVR::PAD0,
            i => WU1_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WU1_EVR::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU1_EVR::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU1_EVR::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU1_EVR::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU1_EVR::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU1_EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU1_EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU1_EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU1_EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU1_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU1_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU1_EVR::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU1_EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU1_EVR::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline]
    pub fn is_jtag(&self) -> bool {
        *self == WU1_EVR::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU1_EVR::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU1_EVR::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU1_EVR::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU1_EVR::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU1_EVR::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU1_EVR::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU1_EVR::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU1_EVR::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline]
    pub fn is_pad(&self) -> bool {
        *self == WU1_EVR::PAD
    }
    #[doc = "Checks if the value of the field is `PAD31`"]
    #[inline]
    pub fn is_pad31(&self) -> bool {
        *self == WU1_EVR::PAD31
    }
    #[doc = "Checks if the value of the field is `PAD30`"]
    #[inline]
    pub fn is_pad30(&self) -> bool {
        *self == WU1_EVR::PAD30
    }
    #[doc = "Checks if the value of the field is `PAD29`"]
    #[inline]
    pub fn is_pad29(&self) -> bool {
        *self == WU1_EVR::PAD29
    }
    #[doc = "Checks if the value of the field is `PAD28`"]
    #[inline]
    pub fn is_pad28(&self) -> bool {
        *self == WU1_EVR::PAD28
    }
    #[doc = "Checks if the value of the field is `PAD27`"]
    #[inline]
    pub fn is_pad27(&self) -> bool {
        *self == WU1_EVR::PAD27
    }
    #[doc = "Checks if the value of the field is `PAD26`"]
    #[inline]
    pub fn is_pad26(&self) -> bool {
        *self == WU1_EVR::PAD26
    }
    #[doc = "Checks if the value of the field is `PAD25`"]
    #[inline]
    pub fn is_pad25(&self) -> bool {
        *self == WU1_EVR::PAD25
    }
    #[doc = "Checks if the value of the field is `PAD24`"]
    #[inline]
    pub fn is_pad24(&self) -> bool {
        *self == WU1_EVR::PAD24
    }
    #[doc = "Checks if the value of the field is `PAD23`"]
    #[inline]
    pub fn is_pad23(&self) -> bool {
        *self == WU1_EVR::PAD23
    }
    #[doc = "Checks if the value of the field is `PAD22`"]
    #[inline]
    pub fn is_pad22(&self) -> bool {
        *self == WU1_EVR::PAD22
    }
    #[doc = "Checks if the value of the field is `PAD21`"]
    #[inline]
    pub fn is_pad21(&self) -> bool {
        *self == WU1_EVR::PAD21
    }
    #[doc = "Checks if the value of the field is `PAD20`"]
    #[inline]
    pub fn is_pad20(&self) -> bool {
        *self == WU1_EVR::PAD20
    }
    #[doc = "Checks if the value of the field is `PAD19`"]
    #[inline]
    pub fn is_pad19(&self) -> bool {
        *self == WU1_EVR::PAD19
    }
    #[doc = "Checks if the value of the field is `PAD18`"]
    #[inline]
    pub fn is_pad18(&self) -> bool {
        *self == WU1_EVR::PAD18
    }
    #[doc = "Checks if the value of the field is `PAD17`"]
    #[inline]
    pub fn is_pad17(&self) -> bool {
        *self == WU1_EVR::PAD17
    }
    #[doc = "Checks if the value of the field is `PAD16`"]
    #[inline]
    pub fn is_pad16(&self) -> bool {
        *self == WU1_EVR::PAD16
    }
    #[doc = "Checks if the value of the field is `PAD15`"]
    #[inline]
    pub fn is_pad15(&self) -> bool {
        *self == WU1_EVR::PAD15
    }
    #[doc = "Checks if the value of the field is `PAD14`"]
    #[inline]
    pub fn is_pad14(&self) -> bool {
        *self == WU1_EVR::PAD14
    }
    #[doc = "Checks if the value of the field is `PAD13`"]
    #[inline]
    pub fn is_pad13(&self) -> bool {
        *self == WU1_EVR::PAD13
    }
    #[doc = "Checks if the value of the field is `PAD12`"]
    #[inline]
    pub fn is_pad12(&self) -> bool {
        *self == WU1_EVR::PAD12
    }
    #[doc = "Checks if the value of the field is `PAD11`"]
    #[inline]
    pub fn is_pad11(&self) -> bool {
        *self == WU1_EVR::PAD11
    }
    #[doc = "Checks if the value of the field is `PAD10`"]
    #[inline]
    pub fn is_pad10(&self) -> bool {
        *self == WU1_EVR::PAD10
    }
    #[doc = "Checks if the value of the field is `PAD9`"]
    #[inline]
    pub fn is_pad9(&self) -> bool {
        *self == WU1_EVR::PAD9
    }
    #[doc = "Checks if the value of the field is `PAD8`"]
    #[inline]
    pub fn is_pad8(&self) -> bool {
        *self == WU1_EVR::PAD8
    }
    #[doc = "Checks if the value of the field is `PAD7`"]
    #[inline]
    pub fn is_pad7(&self) -> bool {
        *self == WU1_EVR::PAD7
    }
    #[doc = "Checks if the value of the field is `PAD6`"]
    #[inline]
    pub fn is_pad6(&self) -> bool {
        *self == WU1_EVR::PAD6
    }
    #[doc = "Checks if the value of the field is `PAD5`"]
    #[inline]
    pub fn is_pad5(&self) -> bool {
        *self == WU1_EVR::PAD5
    }
    #[doc = "Checks if the value of the field is `PAD4`"]
    #[inline]
    pub fn is_pad4(&self) -> bool {
        *self == WU1_EVR::PAD4
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline]
    pub fn is_pad3(&self) -> bool {
        *self == WU1_EVR::PAD3
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline]
    pub fn is_pad2(&self) -> bool {
        *self == WU1_EVR::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline]
    pub fn is_pad1(&self) -> bool {
        *self == WU1_EVR::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline]
    pub fn is_pad0(&self) -> bool {
        *self == WU1_EVR::PAD0
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED6R {
    bits: u8,
}
impl RESERVED6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WU0_EV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WU0_EVR {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Edge detect on PAD31"]
    PAD31,
    #[doc = "Edge detect on PAD30"]
    PAD30,
    #[doc = "Edge detect on PAD29"]
    PAD29,
    #[doc = "Edge detect on PAD28"]
    PAD28,
    #[doc = "Edge detect on PAD27"]
    PAD27,
    #[doc = "Edge detect on PAD26"]
    PAD26,
    #[doc = "Edge detect on PAD25"]
    PAD25,
    #[doc = "Edge detect on PAD24"]
    PAD24,
    #[doc = "Edge detect on PAD23"]
    PAD23,
    #[doc = "Edge detect on PAD22"]
    PAD22,
    #[doc = "Edge detect on PAD21"]
    PAD21,
    #[doc = "Edge detect on PAD20"]
    PAD20,
    #[doc = "Edge detect on PAD19"]
    PAD19,
    #[doc = "Edge detect on PAD18"]
    PAD18,
    #[doc = "Edge detect on PAD17"]
    PAD17,
    #[doc = "Edge detect on PAD16"]
    PAD16,
    #[doc = "Edge detect on PAD15"]
    PAD15,
    #[doc = "Edge detect on PAD14"]
    PAD14,
    #[doc = "Edge detect on PAD13"]
    PAD13,
    #[doc = "Edge detect on PAD12"]
    PAD12,
    #[doc = "Edge detect on PAD11"]
    PAD11,
    #[doc = "Edge detect on PAD10"]
    PAD10,
    #[doc = "Edge detect on PAD9"]
    PAD9,
    #[doc = "Edge detect on PAD8"]
    PAD8,
    #[doc = "Edge detect on PAD7"]
    PAD7,
    #[doc = "Edge detect on PAD6"]
    PAD6,
    #[doc = "Edge detect on PAD5"]
    PAD5,
    #[doc = "Edge detect on PAD4"]
    PAD4,
    #[doc = "Edge detect on PAD3"]
    PAD3,
    #[doc = "Edge detect on PAD2"]
    PAD2,
    #[doc = "Edge detect on PAD1"]
    PAD1,
    #[doc = "Edge detect on PAD0"]
    PAD0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WU0_EVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WU0_EVR::NONE => 63,
            WU0_EVR::AUX_COMPB_ASYNC_N => 56,
            WU0_EVR::AUX_COMPB_ASYNC => 55,
            WU0_EVR::BATMON_VOLT => 54,
            WU0_EVR::BATMON_TEMP => 53,
            WU0_EVR::AUX_TIMER1_EV => 52,
            WU0_EVR::AUX_TIMER0_EV => 51,
            WU0_EVR::AUX_TDC_DONE => 50,
            WU0_EVR::AUX_ADC_DONE => 49,
            WU0_EVR::AUX_COMPB => 48,
            WU0_EVR::AUX_COMPA => 47,
            WU0_EVR::AUX_SWEV2 => 46,
            WU0_EVR::AUX_SWEV1 => 45,
            WU0_EVR::AUX_SWEV0 => 44,
            WU0_EVR::JTAG => 43,
            WU0_EVR::RTC_UPD => 42,
            WU0_EVR::RTC_COMB_DLY => 41,
            WU0_EVR::RTC_CH2_DLY => 40,
            WU0_EVR::RTC_CH1_DLY => 39,
            WU0_EVR::RTC_CH0_DLY => 38,
            WU0_EVR::RTC_CH2 => 37,
            WU0_EVR::RTC_CH1 => 36,
            WU0_EVR::RTC_CH0 => 35,
            WU0_EVR::PAD => 32,
            WU0_EVR::PAD31 => 31,
            WU0_EVR::PAD30 => 30,
            WU0_EVR::PAD29 => 29,
            WU0_EVR::PAD28 => 28,
            WU0_EVR::PAD27 => 27,
            WU0_EVR::PAD26 => 26,
            WU0_EVR::PAD25 => 25,
            WU0_EVR::PAD24 => 24,
            WU0_EVR::PAD23 => 23,
            WU0_EVR::PAD22 => 22,
            WU0_EVR::PAD21 => 21,
            WU0_EVR::PAD20 => 20,
            WU0_EVR::PAD19 => 19,
            WU0_EVR::PAD18 => 18,
            WU0_EVR::PAD17 => 17,
            WU0_EVR::PAD16 => 16,
            WU0_EVR::PAD15 => 15,
            WU0_EVR::PAD14 => 14,
            WU0_EVR::PAD13 => 13,
            WU0_EVR::PAD12 => 12,
            WU0_EVR::PAD11 => 11,
            WU0_EVR::PAD10 => 10,
            WU0_EVR::PAD9 => 9,
            WU0_EVR::PAD8 => 8,
            WU0_EVR::PAD7 => 7,
            WU0_EVR::PAD6 => 6,
            WU0_EVR::PAD5 => 5,
            WU0_EVR::PAD4 => 4,
            WU0_EVR::PAD3 => 3,
            WU0_EVR::PAD2 => 2,
            WU0_EVR::PAD1 => 1,
            WU0_EVR::PAD0 => 0,
            WU0_EVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WU0_EVR {
        match value {
            63 => WU0_EVR::NONE,
            56 => WU0_EVR::AUX_COMPB_ASYNC_N,
            55 => WU0_EVR::AUX_COMPB_ASYNC,
            54 => WU0_EVR::BATMON_VOLT,
            53 => WU0_EVR::BATMON_TEMP,
            52 => WU0_EVR::AUX_TIMER1_EV,
            51 => WU0_EVR::AUX_TIMER0_EV,
            50 => WU0_EVR::AUX_TDC_DONE,
            49 => WU0_EVR::AUX_ADC_DONE,
            48 => WU0_EVR::AUX_COMPB,
            47 => WU0_EVR::AUX_COMPA,
            46 => WU0_EVR::AUX_SWEV2,
            45 => WU0_EVR::AUX_SWEV1,
            44 => WU0_EVR::AUX_SWEV0,
            43 => WU0_EVR::JTAG,
            42 => WU0_EVR::RTC_UPD,
            41 => WU0_EVR::RTC_COMB_DLY,
            40 => WU0_EVR::RTC_CH2_DLY,
            39 => WU0_EVR::RTC_CH1_DLY,
            38 => WU0_EVR::RTC_CH0_DLY,
            37 => WU0_EVR::RTC_CH2,
            36 => WU0_EVR::RTC_CH1,
            35 => WU0_EVR::RTC_CH0,
            32 => WU0_EVR::PAD,
            31 => WU0_EVR::PAD31,
            30 => WU0_EVR::PAD30,
            29 => WU0_EVR::PAD29,
            28 => WU0_EVR::PAD28,
            27 => WU0_EVR::PAD27,
            26 => WU0_EVR::PAD26,
            25 => WU0_EVR::PAD25,
            24 => WU0_EVR::PAD24,
            23 => WU0_EVR::PAD23,
            22 => WU0_EVR::PAD22,
            21 => WU0_EVR::PAD21,
            20 => WU0_EVR::PAD20,
            19 => WU0_EVR::PAD19,
            18 => WU0_EVR::PAD18,
            17 => WU0_EVR::PAD17,
            16 => WU0_EVR::PAD16,
            15 => WU0_EVR::PAD15,
            14 => WU0_EVR::PAD14,
            13 => WU0_EVR::PAD13,
            12 => WU0_EVR::PAD12,
            11 => WU0_EVR::PAD11,
            10 => WU0_EVR::PAD10,
            9 => WU0_EVR::PAD9,
            8 => WU0_EVR::PAD8,
            7 => WU0_EVR::PAD7,
            6 => WU0_EVR::PAD6,
            5 => WU0_EVR::PAD5,
            4 => WU0_EVR::PAD4,
            3 => WU0_EVR::PAD3,
            2 => WU0_EVR::PAD2,
            1 => WU0_EVR::PAD1,
            0 => WU0_EVR::PAD0,
            i => WU0_EVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WU0_EVR::NONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC_N`"]
    #[inline]
    pub fn is_aux_compb_async_n(&self) -> bool {
        *self == WU0_EVR::AUX_COMPB_ASYNC_N
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB_ASYNC`"]
    #[inline]
    pub fn is_aux_compb_async(&self) -> bool {
        *self == WU0_EVR::AUX_COMPB_ASYNC
    }
    #[doc = "Checks if the value of the field is `BATMON_VOLT`"]
    #[inline]
    pub fn is_batmon_volt(&self) -> bool {
        *self == WU0_EVR::BATMON_VOLT
    }
    #[doc = "Checks if the value of the field is `BATMON_TEMP`"]
    #[inline]
    pub fn is_batmon_temp(&self) -> bool {
        *self == WU0_EVR::BATMON_TEMP
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV`"]
    #[inline]
    pub fn is_aux_timer1_ev(&self) -> bool {
        *self == WU0_EVR::AUX_TIMER1_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV`"]
    #[inline]
    pub fn is_aux_timer0_ev(&self) -> bool {
        *self == WU0_EVR::AUX_TIMER0_EV
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == WU0_EVR::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_DONE`"]
    #[inline]
    pub fn is_aux_adc_done(&self) -> bool {
        *self == WU0_EVR::AUX_ADC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline]
    pub fn is_aux_compb(&self) -> bool {
        *self == WU0_EVR::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline]
    pub fn is_aux_compa(&self) -> bool {
        *self == WU0_EVR::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV2`"]
    #[inline]
    pub fn is_aux_swev2(&self) -> bool {
        *self == WU0_EVR::AUX_SWEV2
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV1`"]
    #[inline]
    pub fn is_aux_swev1(&self) -> bool {
        *self == WU0_EVR::AUX_SWEV1
    }
    #[doc = "Checks if the value of the field is `AUX_SWEV0`"]
    #[inline]
    pub fn is_aux_swev0(&self) -> bool {
        *self == WU0_EVR::AUX_SWEV0
    }
    #[doc = "Checks if the value of the field is `JTAG`"]
    #[inline]
    pub fn is_jtag(&self) -> bool {
        *self == WU0_EVR::JTAG
    }
    #[doc = "Checks if the value of the field is `RTC_UPD`"]
    #[inline]
    pub fn is_rtc_upd(&self) -> bool {
        *self == WU0_EVR::RTC_UPD
    }
    #[doc = "Checks if the value of the field is `RTC_COMB_DLY`"]
    #[inline]
    pub fn is_rtc_comb_dly(&self) -> bool {
        *self == WU0_EVR::RTC_COMB_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2_DLY`"]
    #[inline]
    pub fn is_rtc_ch2_dly(&self) -> bool {
        *self == WU0_EVR::RTC_CH2_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH1_DLY`"]
    #[inline]
    pub fn is_rtc_ch1_dly(&self) -> bool {
        *self == WU0_EVR::RTC_CH1_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH0_DLY`"]
    #[inline]
    pub fn is_rtc_ch0_dly(&self) -> bool {
        *self == WU0_EVR::RTC_CH0_DLY
    }
    #[doc = "Checks if the value of the field is `RTC_CH2`"]
    #[inline]
    pub fn is_rtc_ch2(&self) -> bool {
        *self == WU0_EVR::RTC_CH2
    }
    #[doc = "Checks if the value of the field is `RTC_CH1`"]
    #[inline]
    pub fn is_rtc_ch1(&self) -> bool {
        *self == WU0_EVR::RTC_CH1
    }
    #[doc = "Checks if the value of the field is `RTC_CH0`"]
    #[inline]
    pub fn is_rtc_ch0(&self) -> bool {
        *self == WU0_EVR::RTC_CH0
    }
    #[doc = "Checks if the value of the field is `PAD`"]
    #[inline]
    pub fn is_pad(&self) -> bool {
        *self == WU0_EVR::PAD
    }
    #[doc = "Checks if the value of the field is `PAD31`"]
    #[inline]
    pub fn is_pad31(&self) -> bool {
        *self == WU0_EVR::PAD31
    }
    #[doc = "Checks if the value of the field is `PAD30`"]
    #[inline]
    pub fn is_pad30(&self) -> bool {
        *self == WU0_EVR::PAD30
    }
    #[doc = "Checks if the value of the field is `PAD29`"]
    #[inline]
    pub fn is_pad29(&self) -> bool {
        *self == WU0_EVR::PAD29
    }
    #[doc = "Checks if the value of the field is `PAD28`"]
    #[inline]
    pub fn is_pad28(&self) -> bool {
        *self == WU0_EVR::PAD28
    }
    #[doc = "Checks if the value of the field is `PAD27`"]
    #[inline]
    pub fn is_pad27(&self) -> bool {
        *self == WU0_EVR::PAD27
    }
    #[doc = "Checks if the value of the field is `PAD26`"]
    #[inline]
    pub fn is_pad26(&self) -> bool {
        *self == WU0_EVR::PAD26
    }
    #[doc = "Checks if the value of the field is `PAD25`"]
    #[inline]
    pub fn is_pad25(&self) -> bool {
        *self == WU0_EVR::PAD25
    }
    #[doc = "Checks if the value of the field is `PAD24`"]
    #[inline]
    pub fn is_pad24(&self) -> bool {
        *self == WU0_EVR::PAD24
    }
    #[doc = "Checks if the value of the field is `PAD23`"]
    #[inline]
    pub fn is_pad23(&self) -> bool {
        *self == WU0_EVR::PAD23
    }
    #[doc = "Checks if the value of the field is `PAD22`"]
    #[inline]
    pub fn is_pad22(&self) -> bool {
        *self == WU0_EVR::PAD22
    }
    #[doc = "Checks if the value of the field is `PAD21`"]
    #[inline]
    pub fn is_pad21(&self) -> bool {
        *self == WU0_EVR::PAD21
    }
    #[doc = "Checks if the value of the field is `PAD20`"]
    #[inline]
    pub fn is_pad20(&self) -> bool {
        *self == WU0_EVR::PAD20
    }
    #[doc = "Checks if the value of the field is `PAD19`"]
    #[inline]
    pub fn is_pad19(&self) -> bool {
        *self == WU0_EVR::PAD19
    }
    #[doc = "Checks if the value of the field is `PAD18`"]
    #[inline]
    pub fn is_pad18(&self) -> bool {
        *self == WU0_EVR::PAD18
    }
    #[doc = "Checks if the value of the field is `PAD17`"]
    #[inline]
    pub fn is_pad17(&self) -> bool {
        *self == WU0_EVR::PAD17
    }
    #[doc = "Checks if the value of the field is `PAD16`"]
    #[inline]
    pub fn is_pad16(&self) -> bool {
        *self == WU0_EVR::PAD16
    }
    #[doc = "Checks if the value of the field is `PAD15`"]
    #[inline]
    pub fn is_pad15(&self) -> bool {
        *self == WU0_EVR::PAD15
    }
    #[doc = "Checks if the value of the field is `PAD14`"]
    #[inline]
    pub fn is_pad14(&self) -> bool {
        *self == WU0_EVR::PAD14
    }
    #[doc = "Checks if the value of the field is `PAD13`"]
    #[inline]
    pub fn is_pad13(&self) -> bool {
        *self == WU0_EVR::PAD13
    }
    #[doc = "Checks if the value of the field is `PAD12`"]
    #[inline]
    pub fn is_pad12(&self) -> bool {
        *self == WU0_EVR::PAD12
    }
    #[doc = "Checks if the value of the field is `PAD11`"]
    #[inline]
    pub fn is_pad11(&self) -> bool {
        *self == WU0_EVR::PAD11
    }
    #[doc = "Checks if the value of the field is `PAD10`"]
    #[inline]
    pub fn is_pad10(&self) -> bool {
        *self == WU0_EVR::PAD10
    }
    #[doc = "Checks if the value of the field is `PAD9`"]
    #[inline]
    pub fn is_pad9(&self) -> bool {
        *self == WU0_EVR::PAD9
    }
    #[doc = "Checks if the value of the field is `PAD8`"]
    #[inline]
    pub fn is_pad8(&self) -> bool {
        *self == WU0_EVR::PAD8
    }
    #[doc = "Checks if the value of the field is `PAD7`"]
    #[inline]
    pub fn is_pad7(&self) -> bool {
        *self == WU0_EVR::PAD7
    }
    #[doc = "Checks if the value of the field is `PAD6`"]
    #[inline]
    pub fn is_pad6(&self) -> bool {
        *self == WU0_EVR::PAD6
    }
    #[doc = "Checks if the value of the field is `PAD5`"]
    #[inline]
    pub fn is_pad5(&self) -> bool {
        *self == WU0_EVR::PAD5
    }
    #[doc = "Checks if the value of the field is `PAD4`"]
    #[inline]
    pub fn is_pad4(&self) -> bool {
        *self == WU0_EVR::PAD4
    }
    #[doc = "Checks if the value of the field is `PAD3`"]
    #[inline]
    pub fn is_pad3(&self) -> bool {
        *self == WU0_EVR::PAD3
    }
    #[doc = "Checks if the value of the field is `PAD2`"]
    #[inline]
    pub fn is_pad2(&self) -> bool {
        *self == WU0_EVR::PAD2
    }
    #[doc = "Checks if the value of the field is `PAD1`"]
    #[inline]
    pub fn is_pad1(&self) -> bool {
        *self == WU0_EVR::PAD1
    }
    #[doc = "Checks if the value of the field is `PAD0`"]
    #[inline]
    pub fn is_pad0(&self) -> bool {
        *self == WU0_EVR::PAD0
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED22W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED22W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WU2_EV`"]
pub enum WU2_EVW {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Edge detect on PAD31"]
    PAD31,
    #[doc = "Edge detect on PAD30"]
    PAD30,
    #[doc = "Edge detect on PAD29"]
    PAD29,
    #[doc = "Edge detect on PAD28"]
    PAD28,
    #[doc = "Edge detect on PAD27"]
    PAD27,
    #[doc = "Edge detect on PAD26"]
    PAD26,
    #[doc = "Edge detect on PAD25"]
    PAD25,
    #[doc = "Edge detect on PAD24"]
    PAD24,
    #[doc = "Edge detect on PAD23"]
    PAD23,
    #[doc = "Edge detect on PAD22"]
    PAD22,
    #[doc = "Edge detect on PAD21"]
    PAD21,
    #[doc = "Edge detect on PAD20"]
    PAD20,
    #[doc = "Edge detect on PAD19"]
    PAD19,
    #[doc = "Edge detect on PAD18"]
    PAD18,
    #[doc = "Edge detect on PAD17"]
    PAD17,
    #[doc = "Edge detect on PAD16"]
    PAD16,
    #[doc = "Edge detect on PAD15"]
    PAD15,
    #[doc = "Edge detect on PAD14"]
    PAD14,
    #[doc = "Edge detect on PAD13"]
    PAD13,
    #[doc = "Edge detect on PAD12"]
    PAD12,
    #[doc = "Edge detect on PAD11"]
    PAD11,
    #[doc = "Edge detect on PAD10"]
    PAD10,
    #[doc = "Edge detect on PAD9"]
    PAD9,
    #[doc = "Edge detect on PAD8"]
    PAD8,
    #[doc = "Edge detect on PAD7"]
    PAD7,
    #[doc = "Edge detect on PAD6"]
    PAD6,
    #[doc = "Edge detect on PAD5"]
    PAD5,
    #[doc = "Edge detect on PAD4"]
    PAD4,
    #[doc = "Edge detect on PAD3"]
    PAD3,
    #[doc = "Edge detect on PAD2"]
    PAD2,
    #[doc = "Edge detect on PAD1"]
    PAD1,
    #[doc = "Edge detect on PAD0"]
    PAD0,
}
impl WU2_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WU2_EVW::NONE => 63,
            WU2_EVW::AUX_COMPB_ASYNC_N => 56,
            WU2_EVW::AUX_COMPB_ASYNC => 55,
            WU2_EVW::BATMON_VOLT => 54,
            WU2_EVW::BATMON_TEMP => 53,
            WU2_EVW::AUX_TIMER1_EV => 52,
            WU2_EVW::AUX_TIMER0_EV => 51,
            WU2_EVW::AUX_TDC_DONE => 50,
            WU2_EVW::AUX_ADC_DONE => 49,
            WU2_EVW::AUX_COMPB => 48,
            WU2_EVW::AUX_COMPA => 47,
            WU2_EVW::AUX_SWEV2 => 46,
            WU2_EVW::AUX_SWEV1 => 45,
            WU2_EVW::AUX_SWEV0 => 44,
            WU2_EVW::JTAG => 43,
            WU2_EVW::RTC_UPD => 42,
            WU2_EVW::RTC_COMB_DLY => 41,
            WU2_EVW::RTC_CH2_DLY => 40,
            WU2_EVW::RTC_CH1_DLY => 39,
            WU2_EVW::RTC_CH0_DLY => 38,
            WU2_EVW::RTC_CH2 => 37,
            WU2_EVW::RTC_CH1 => 36,
            WU2_EVW::RTC_CH0 => 35,
            WU2_EVW::PAD => 32,
            WU2_EVW::PAD31 => 31,
            WU2_EVW::PAD30 => 30,
            WU2_EVW::PAD29 => 29,
            WU2_EVW::PAD28 => 28,
            WU2_EVW::PAD27 => 27,
            WU2_EVW::PAD26 => 26,
            WU2_EVW::PAD25 => 25,
            WU2_EVW::PAD24 => 24,
            WU2_EVW::PAD23 => 23,
            WU2_EVW::PAD22 => 22,
            WU2_EVW::PAD21 => 21,
            WU2_EVW::PAD20 => 20,
            WU2_EVW::PAD19 => 19,
            WU2_EVW::PAD18 => 18,
            WU2_EVW::PAD17 => 17,
            WU2_EVW::PAD16 => 16,
            WU2_EVW::PAD15 => 15,
            WU2_EVW::PAD14 => 14,
            WU2_EVW::PAD13 => 13,
            WU2_EVW::PAD12 => 12,
            WU2_EVW::PAD11 => 11,
            WU2_EVW::PAD10 => 10,
            WU2_EVW::PAD9 => 9,
            WU2_EVW::PAD8 => 8,
            WU2_EVW::PAD7 => 7,
            WU2_EVW::PAD6 => 6,
            WU2_EVW::PAD5 => 5,
            WU2_EVW::PAD4 => 4,
            WU2_EVW::PAD3 => 3,
            WU2_EVW::PAD2 => 2,
            WU2_EVW::PAD1 => 1,
            WU2_EVW::PAD0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU2_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _WU2_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU2_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event, always low"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(WU2_EVW::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU2_EVW::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU2_EVW::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU2_EVW::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU2_EVW::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU2_EVW::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD)
    }
    #[doc = "Edge detect on PAD31"]
    #[inline]
    pub fn pad31(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD31)
    }
    #[doc = "Edge detect on PAD30"]
    #[inline]
    pub fn pad30(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD30)
    }
    #[doc = "Edge detect on PAD29"]
    #[inline]
    pub fn pad29(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD29)
    }
    #[doc = "Edge detect on PAD28"]
    #[inline]
    pub fn pad28(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD28)
    }
    #[doc = "Edge detect on PAD27"]
    #[inline]
    pub fn pad27(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD27)
    }
    #[doc = "Edge detect on PAD26"]
    #[inline]
    pub fn pad26(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD26)
    }
    #[doc = "Edge detect on PAD25"]
    #[inline]
    pub fn pad25(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD25)
    }
    #[doc = "Edge detect on PAD24"]
    #[inline]
    pub fn pad24(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD24)
    }
    #[doc = "Edge detect on PAD23"]
    #[inline]
    pub fn pad23(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD23)
    }
    #[doc = "Edge detect on PAD22"]
    #[inline]
    pub fn pad22(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD22)
    }
    #[doc = "Edge detect on PAD21"]
    #[inline]
    pub fn pad21(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD21)
    }
    #[doc = "Edge detect on PAD20"]
    #[inline]
    pub fn pad20(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD20)
    }
    #[doc = "Edge detect on PAD19"]
    #[inline]
    pub fn pad19(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD19)
    }
    #[doc = "Edge detect on PAD18"]
    #[inline]
    pub fn pad18(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD18)
    }
    #[doc = "Edge detect on PAD17"]
    #[inline]
    pub fn pad17(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD17)
    }
    #[doc = "Edge detect on PAD16"]
    #[inline]
    pub fn pad16(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD16)
    }
    #[doc = "Edge detect on PAD15"]
    #[inline]
    pub fn pad15(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD15)
    }
    #[doc = "Edge detect on PAD14"]
    #[inline]
    pub fn pad14(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD14)
    }
    #[doc = "Edge detect on PAD13"]
    #[inline]
    pub fn pad13(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD13)
    }
    #[doc = "Edge detect on PAD12"]
    #[inline]
    pub fn pad12(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD12)
    }
    #[doc = "Edge detect on PAD11"]
    #[inline]
    pub fn pad11(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD11)
    }
    #[doc = "Edge detect on PAD10"]
    #[inline]
    pub fn pad10(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD10)
    }
    #[doc = "Edge detect on PAD9"]
    #[inline]
    pub fn pad9(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD9)
    }
    #[doc = "Edge detect on PAD8"]
    #[inline]
    pub fn pad8(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD8)
    }
    #[doc = "Edge detect on PAD7"]
    #[inline]
    pub fn pad7(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD7)
    }
    #[doc = "Edge detect on PAD6"]
    #[inline]
    pub fn pad6(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD6)
    }
    #[doc = "Edge detect on PAD5"]
    #[inline]
    pub fn pad5(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD5)
    }
    #[doc = "Edge detect on PAD4"]
    #[inline]
    pub fn pad4(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD4)
    }
    #[doc = "Edge detect on PAD3"]
    #[inline]
    pub fn pad3(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD3)
    }
    #[doc = "Edge detect on PAD2"]
    #[inline]
    pub fn pad2(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD2)
    }
    #[doc = "Edge detect on PAD1"]
    #[inline]
    pub fn pad1(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD1)
    }
    #[doc = "Edge detect on PAD0"]
    #[inline]
    pub fn pad0(self) -> &'a mut W {
        self.variant(WU2_EVW::PAD0)
    }
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
pub struct _RESERVED14W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WU1_EV`"]
pub enum WU1_EVW {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Edge detect on PAD31"]
    PAD31,
    #[doc = "Edge detect on PAD30"]
    PAD30,
    #[doc = "Edge detect on PAD29"]
    PAD29,
    #[doc = "Edge detect on PAD28"]
    PAD28,
    #[doc = "Edge detect on PAD27"]
    PAD27,
    #[doc = "Edge detect on PAD26"]
    PAD26,
    #[doc = "Edge detect on PAD25"]
    PAD25,
    #[doc = "Edge detect on PAD24"]
    PAD24,
    #[doc = "Edge detect on PAD23"]
    PAD23,
    #[doc = "Edge detect on PAD22"]
    PAD22,
    #[doc = "Edge detect on PAD21"]
    PAD21,
    #[doc = "Edge detect on PAD20"]
    PAD20,
    #[doc = "Edge detect on PAD19"]
    PAD19,
    #[doc = "Edge detect on PAD18"]
    PAD18,
    #[doc = "Edge detect on PAD17"]
    PAD17,
    #[doc = "Edge detect on PAD16"]
    PAD16,
    #[doc = "Edge detect on PAD15"]
    PAD15,
    #[doc = "Edge detect on PAD14"]
    PAD14,
    #[doc = "Edge detect on PAD13"]
    PAD13,
    #[doc = "Edge detect on PAD12"]
    PAD12,
    #[doc = "Edge detect on PAD11"]
    PAD11,
    #[doc = "Edge detect on PAD10"]
    PAD10,
    #[doc = "Edge detect on PAD9"]
    PAD9,
    #[doc = "Edge detect on PAD8"]
    PAD8,
    #[doc = "Edge detect on PAD7"]
    PAD7,
    #[doc = "Edge detect on PAD6"]
    PAD6,
    #[doc = "Edge detect on PAD5"]
    PAD5,
    #[doc = "Edge detect on PAD4"]
    PAD4,
    #[doc = "Edge detect on PAD3"]
    PAD3,
    #[doc = "Edge detect on PAD2"]
    PAD2,
    #[doc = "Edge detect on PAD1"]
    PAD1,
    #[doc = "Edge detect on PAD0"]
    PAD0,
}
impl WU1_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WU1_EVW::NONE => 63,
            WU1_EVW::AUX_COMPB_ASYNC_N => 56,
            WU1_EVW::AUX_COMPB_ASYNC => 55,
            WU1_EVW::BATMON_VOLT => 54,
            WU1_EVW::BATMON_TEMP => 53,
            WU1_EVW::AUX_TIMER1_EV => 52,
            WU1_EVW::AUX_TIMER0_EV => 51,
            WU1_EVW::AUX_TDC_DONE => 50,
            WU1_EVW::AUX_ADC_DONE => 49,
            WU1_EVW::AUX_COMPB => 48,
            WU1_EVW::AUX_COMPA => 47,
            WU1_EVW::AUX_SWEV2 => 46,
            WU1_EVW::AUX_SWEV1 => 45,
            WU1_EVW::AUX_SWEV0 => 44,
            WU1_EVW::JTAG => 43,
            WU1_EVW::RTC_UPD => 42,
            WU1_EVW::RTC_COMB_DLY => 41,
            WU1_EVW::RTC_CH2_DLY => 40,
            WU1_EVW::RTC_CH1_DLY => 39,
            WU1_EVW::RTC_CH0_DLY => 38,
            WU1_EVW::RTC_CH2 => 37,
            WU1_EVW::RTC_CH1 => 36,
            WU1_EVW::RTC_CH0 => 35,
            WU1_EVW::PAD => 32,
            WU1_EVW::PAD31 => 31,
            WU1_EVW::PAD30 => 30,
            WU1_EVW::PAD29 => 29,
            WU1_EVW::PAD28 => 28,
            WU1_EVW::PAD27 => 27,
            WU1_EVW::PAD26 => 26,
            WU1_EVW::PAD25 => 25,
            WU1_EVW::PAD24 => 24,
            WU1_EVW::PAD23 => 23,
            WU1_EVW::PAD22 => 22,
            WU1_EVW::PAD21 => 21,
            WU1_EVW::PAD20 => 20,
            WU1_EVW::PAD19 => 19,
            WU1_EVW::PAD18 => 18,
            WU1_EVW::PAD17 => 17,
            WU1_EVW::PAD16 => 16,
            WU1_EVW::PAD15 => 15,
            WU1_EVW::PAD14 => 14,
            WU1_EVW::PAD13 => 13,
            WU1_EVW::PAD12 => 12,
            WU1_EVW::PAD11 => 11,
            WU1_EVW::PAD10 => 10,
            WU1_EVW::PAD9 => 9,
            WU1_EVW::PAD8 => 8,
            WU1_EVW::PAD7 => 7,
            WU1_EVW::PAD6 => 6,
            WU1_EVW::PAD5 => 5,
            WU1_EVW::PAD4 => 4,
            WU1_EVW::PAD3 => 3,
            WU1_EVW::PAD2 => 2,
            WU1_EVW::PAD1 => 1,
            WU1_EVW::PAD0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU1_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _WU1_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU1_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event, always low"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(WU1_EVW::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU1_EVW::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU1_EVW::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU1_EVW::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU1_EVW::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU1_EVW::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD)
    }
    #[doc = "Edge detect on PAD31"]
    #[inline]
    pub fn pad31(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD31)
    }
    #[doc = "Edge detect on PAD30"]
    #[inline]
    pub fn pad30(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD30)
    }
    #[doc = "Edge detect on PAD29"]
    #[inline]
    pub fn pad29(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD29)
    }
    #[doc = "Edge detect on PAD28"]
    #[inline]
    pub fn pad28(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD28)
    }
    #[doc = "Edge detect on PAD27"]
    #[inline]
    pub fn pad27(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD27)
    }
    #[doc = "Edge detect on PAD26"]
    #[inline]
    pub fn pad26(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD26)
    }
    #[doc = "Edge detect on PAD25"]
    #[inline]
    pub fn pad25(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD25)
    }
    #[doc = "Edge detect on PAD24"]
    #[inline]
    pub fn pad24(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD24)
    }
    #[doc = "Edge detect on PAD23"]
    #[inline]
    pub fn pad23(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD23)
    }
    #[doc = "Edge detect on PAD22"]
    #[inline]
    pub fn pad22(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD22)
    }
    #[doc = "Edge detect on PAD21"]
    #[inline]
    pub fn pad21(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD21)
    }
    #[doc = "Edge detect on PAD20"]
    #[inline]
    pub fn pad20(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD20)
    }
    #[doc = "Edge detect on PAD19"]
    #[inline]
    pub fn pad19(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD19)
    }
    #[doc = "Edge detect on PAD18"]
    #[inline]
    pub fn pad18(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD18)
    }
    #[doc = "Edge detect on PAD17"]
    #[inline]
    pub fn pad17(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD17)
    }
    #[doc = "Edge detect on PAD16"]
    #[inline]
    pub fn pad16(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD16)
    }
    #[doc = "Edge detect on PAD15"]
    #[inline]
    pub fn pad15(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD15)
    }
    #[doc = "Edge detect on PAD14"]
    #[inline]
    pub fn pad14(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD14)
    }
    #[doc = "Edge detect on PAD13"]
    #[inline]
    pub fn pad13(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD13)
    }
    #[doc = "Edge detect on PAD12"]
    #[inline]
    pub fn pad12(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD12)
    }
    #[doc = "Edge detect on PAD11"]
    #[inline]
    pub fn pad11(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD11)
    }
    #[doc = "Edge detect on PAD10"]
    #[inline]
    pub fn pad10(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD10)
    }
    #[doc = "Edge detect on PAD9"]
    #[inline]
    pub fn pad9(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD9)
    }
    #[doc = "Edge detect on PAD8"]
    #[inline]
    pub fn pad8(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD8)
    }
    #[doc = "Edge detect on PAD7"]
    #[inline]
    pub fn pad7(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD7)
    }
    #[doc = "Edge detect on PAD6"]
    #[inline]
    pub fn pad6(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD6)
    }
    #[doc = "Edge detect on PAD5"]
    #[inline]
    pub fn pad5(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD5)
    }
    #[doc = "Edge detect on PAD4"]
    #[inline]
    pub fn pad4(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD4)
    }
    #[doc = "Edge detect on PAD3"]
    #[inline]
    pub fn pad3(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD3)
    }
    #[doc = "Edge detect on PAD2"]
    #[inline]
    pub fn pad2(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD2)
    }
    #[doc = "Edge detect on PAD1"]
    #[inline]
    pub fn pad1(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD1)
    }
    #[doc = "Edge detect on PAD0"]
    #[inline]
    pub fn pad0(self) -> &'a mut W {
        self.variant(WU1_EVW::PAD0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED6W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WU0_EV`"]
pub enum WU0_EVW {
    #[doc = "No event, always low"]
    NONE,
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC_N,
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    AUX_COMPB_ASYNC,
    #[doc = "BATMON voltage update event"]
    BATMON_VOLT,
    #[doc = "BATMON temperature update event"]
    BATMON_TEMP,
    #[doc = "AUX Timer 1 Event"]
    AUX_TIMER1_EV,
    #[doc = "AUX Timer 0 Event"]
    AUX_TIMER0_EV,
    #[doc = "TDC completed or timed out"]
    AUX_TDC_DONE,
    #[doc = "ADC conversion completed"]
    AUX_ADC_DONE,
    #[doc = "Comparator B triggered"]
    AUX_COMPB,
    #[doc = "Comparator A triggered"]
    AUX_COMPA,
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    AUX_SWEV2,
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    AUX_SWEV1,
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    AUX_SWEV0,
    #[doc = "JTAG generated event"]
    JTAG,
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    RTC_UPD,
    #[doc = "RTC combined delayed event"]
    RTC_COMB_DLY,
    #[doc = "RTC channel 2 - delayed event"]
    RTC_CH2_DLY,
    #[doc = "RTC channel 1 - delayed event"]
    RTC_CH1_DLY,
    #[doc = "RTC channel 0 - delayed event"]
    RTC_CH0_DLY,
    #[doc = "RTC channel 2 event"]
    RTC_CH2,
    #[doc = "RTC channel 1 event"]
    RTC_CH1,
    #[doc = "RTC channel 0 event"]
    RTC_CH0,
    #[doc = "Edge detect on any PAD"]
    PAD,
    #[doc = "Edge detect on PAD31"]
    PAD31,
    #[doc = "Edge detect on PAD30"]
    PAD30,
    #[doc = "Edge detect on PAD29"]
    PAD29,
    #[doc = "Edge detect on PAD28"]
    PAD28,
    #[doc = "Edge detect on PAD27"]
    PAD27,
    #[doc = "Edge detect on PAD26"]
    PAD26,
    #[doc = "Edge detect on PAD25"]
    PAD25,
    #[doc = "Edge detect on PAD24"]
    PAD24,
    #[doc = "Edge detect on PAD23"]
    PAD23,
    #[doc = "Edge detect on PAD22"]
    PAD22,
    #[doc = "Edge detect on PAD21"]
    PAD21,
    #[doc = "Edge detect on PAD20"]
    PAD20,
    #[doc = "Edge detect on PAD19"]
    PAD19,
    #[doc = "Edge detect on PAD18"]
    PAD18,
    #[doc = "Edge detect on PAD17"]
    PAD17,
    #[doc = "Edge detect on PAD16"]
    PAD16,
    #[doc = "Edge detect on PAD15"]
    PAD15,
    #[doc = "Edge detect on PAD14"]
    PAD14,
    #[doc = "Edge detect on PAD13"]
    PAD13,
    #[doc = "Edge detect on PAD12"]
    PAD12,
    #[doc = "Edge detect on PAD11"]
    PAD11,
    #[doc = "Edge detect on PAD10"]
    PAD10,
    #[doc = "Edge detect on PAD9"]
    PAD9,
    #[doc = "Edge detect on PAD8"]
    PAD8,
    #[doc = "Edge detect on PAD7"]
    PAD7,
    #[doc = "Edge detect on PAD6"]
    PAD6,
    #[doc = "Edge detect on PAD5"]
    PAD5,
    #[doc = "Edge detect on PAD4"]
    PAD4,
    #[doc = "Edge detect on PAD3"]
    PAD3,
    #[doc = "Edge detect on PAD2"]
    PAD2,
    #[doc = "Edge detect on PAD1"]
    PAD1,
    #[doc = "Edge detect on PAD0"]
    PAD0,
}
impl WU0_EVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WU0_EVW::NONE => 63,
            WU0_EVW::AUX_COMPB_ASYNC_N => 56,
            WU0_EVW::AUX_COMPB_ASYNC => 55,
            WU0_EVW::BATMON_VOLT => 54,
            WU0_EVW::BATMON_TEMP => 53,
            WU0_EVW::AUX_TIMER1_EV => 52,
            WU0_EVW::AUX_TIMER0_EV => 51,
            WU0_EVW::AUX_TDC_DONE => 50,
            WU0_EVW::AUX_ADC_DONE => 49,
            WU0_EVW::AUX_COMPB => 48,
            WU0_EVW::AUX_COMPA => 47,
            WU0_EVW::AUX_SWEV2 => 46,
            WU0_EVW::AUX_SWEV1 => 45,
            WU0_EVW::AUX_SWEV0 => 44,
            WU0_EVW::JTAG => 43,
            WU0_EVW::RTC_UPD => 42,
            WU0_EVW::RTC_COMB_DLY => 41,
            WU0_EVW::RTC_CH2_DLY => 40,
            WU0_EVW::RTC_CH1_DLY => 39,
            WU0_EVW::RTC_CH0_DLY => 38,
            WU0_EVW::RTC_CH2 => 37,
            WU0_EVW::RTC_CH1 => 36,
            WU0_EVW::RTC_CH0 => 35,
            WU0_EVW::PAD => 32,
            WU0_EVW::PAD31 => 31,
            WU0_EVW::PAD30 => 30,
            WU0_EVW::PAD29 => 29,
            WU0_EVW::PAD28 => 28,
            WU0_EVW::PAD27 => 27,
            WU0_EVW::PAD26 => 26,
            WU0_EVW::PAD25 => 25,
            WU0_EVW::PAD24 => 24,
            WU0_EVW::PAD23 => 23,
            WU0_EVW::PAD22 => 22,
            WU0_EVW::PAD21 => 21,
            WU0_EVW::PAD20 => 20,
            WU0_EVW::PAD19 => 19,
            WU0_EVW::PAD18 => 18,
            WU0_EVW::PAD17 => 17,
            WU0_EVW::PAD16 => 16,
            WU0_EVW::PAD15 => 15,
            WU0_EVW::PAD14 => 14,
            WU0_EVW::PAD13 => 13,
            WU0_EVW::PAD12 => 12,
            WU0_EVW::PAD11 => 11,
            WU0_EVW::PAD10 => 10,
            WU0_EVW::PAD9 => 9,
            WU0_EVW::PAD8 => 8,
            WU0_EVW::PAD7 => 7,
            WU0_EVW::PAD6 => 6,
            WU0_EVW::PAD5 => 5,
            WU0_EVW::PAD4 => 4,
            WU0_EVW::PAD3 => 3,
            WU0_EVW::PAD2 => 2,
            WU0_EVW::PAD1 => 1,
            WU0_EVW::PAD0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WU0_EVW<'a> {
    w: &'a mut W,
}
impl<'a> _WU0_EVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WU0_EVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No event, always low"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(WU0_EVW::NONE)
    }
    #[doc = "Comparator B not triggered. Asynchronous signal directly from AUX Comparator B (inverted) as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async_n(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_COMPB_ASYNC_N)
    }
    #[doc = "Comparator B triggered. Asynchronous signal directly from the AUX Comparator B as opposed to AUX_COMPB which is synchronized in AUX"]
    #[inline]
    pub fn aux_compb_async(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_COMPB_ASYNC)
    }
    #[doc = "BATMON voltage update event"]
    #[inline]
    pub fn batmon_volt(self) -> &'a mut W {
        self.variant(WU0_EVW::BATMON_VOLT)
    }
    #[doc = "BATMON temperature update event"]
    #[inline]
    pub fn batmon_temp(self) -> &'a mut W {
        self.variant(WU0_EVW::BATMON_TEMP)
    }
    #[doc = "AUX Timer 1 Event"]
    #[inline]
    pub fn aux_timer1_ev(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_TIMER1_EV)
    }
    #[doc = "AUX Timer 0 Event"]
    #[inline]
    pub fn aux_timer0_ev(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_TIMER0_EV)
    }
    #[doc = "TDC completed or timed out"]
    #[inline]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_TDC_DONE)
    }
    #[doc = "ADC conversion completed"]
    #[inline]
    pub fn aux_adc_done(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_ADC_DONE)
    }
    #[doc = "Comparator B triggered"]
    #[inline]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_COMPB)
    }
    #[doc = "Comparator A triggered"]
    #[inline]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_COMPA)
    }
    #[doc = "AUX Software triggered event #2. Triggered by AUX_EVCTL:SWEVSET.SWEV2"]
    #[inline]
    pub fn aux_swev2(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_SWEV2)
    }
    #[doc = "AUX Software triggered event #1. Triggered by AUX_EVCTL:SWEVSET.SWEV1"]
    #[inline]
    pub fn aux_swev1(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_SWEV1)
    }
    #[doc = "AUX Software triggered event #0. Triggered by AUX_EVCTL:SWEVSET.SWEV0"]
    #[inline]
    pub fn aux_swev0(self) -> &'a mut W {
        self.variant(WU0_EVW::AUX_SWEV0)
    }
    #[doc = "JTAG generated event"]
    #[inline]
    pub fn jtag(self) -> &'a mut W {
        self.variant(WU0_EVW::JTAG)
    }
    #[doc = "RTC Update Tick (16 kHz signal, i.e. event line toggles value every 32 kHz clock period)"]
    #[inline]
    pub fn rtc_upd(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_UPD)
    }
    #[doc = "RTC combined delayed event"]
    #[inline]
    pub fn rtc_comb_dly(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_COMB_DLY)
    }
    #[doc = "RTC channel 2 - delayed event"]
    #[inline]
    pub fn rtc_ch2_dly(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_CH2_DLY)
    }
    #[doc = "RTC channel 1 - delayed event"]
    #[inline]
    pub fn rtc_ch1_dly(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_CH1_DLY)
    }
    #[doc = "RTC channel 0 - delayed event"]
    #[inline]
    pub fn rtc_ch0_dly(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_CH0_DLY)
    }
    #[doc = "RTC channel 2 event"]
    #[inline]
    pub fn rtc_ch2(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_CH2)
    }
    #[doc = "RTC channel 1 event"]
    #[inline]
    pub fn rtc_ch1(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_CH1)
    }
    #[doc = "RTC channel 0 event"]
    #[inline]
    pub fn rtc_ch0(self) -> &'a mut W {
        self.variant(WU0_EVW::RTC_CH0)
    }
    #[doc = "Edge detect on any PAD"]
    #[inline]
    pub fn pad(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD)
    }
    #[doc = "Edge detect on PAD31"]
    #[inline]
    pub fn pad31(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD31)
    }
    #[doc = "Edge detect on PAD30"]
    #[inline]
    pub fn pad30(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD30)
    }
    #[doc = "Edge detect on PAD29"]
    #[inline]
    pub fn pad29(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD29)
    }
    #[doc = "Edge detect on PAD28"]
    #[inline]
    pub fn pad28(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD28)
    }
    #[doc = "Edge detect on PAD27"]
    #[inline]
    pub fn pad27(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD27)
    }
    #[doc = "Edge detect on PAD26"]
    #[inline]
    pub fn pad26(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD26)
    }
    #[doc = "Edge detect on PAD25"]
    #[inline]
    pub fn pad25(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD25)
    }
    #[doc = "Edge detect on PAD24"]
    #[inline]
    pub fn pad24(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD24)
    }
    #[doc = "Edge detect on PAD23"]
    #[inline]
    pub fn pad23(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD23)
    }
    #[doc = "Edge detect on PAD22"]
    #[inline]
    pub fn pad22(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD22)
    }
    #[doc = "Edge detect on PAD21"]
    #[inline]
    pub fn pad21(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD21)
    }
    #[doc = "Edge detect on PAD20"]
    #[inline]
    pub fn pad20(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD20)
    }
    #[doc = "Edge detect on PAD19"]
    #[inline]
    pub fn pad19(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD19)
    }
    #[doc = "Edge detect on PAD18"]
    #[inline]
    pub fn pad18(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD18)
    }
    #[doc = "Edge detect on PAD17"]
    #[inline]
    pub fn pad17(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD17)
    }
    #[doc = "Edge detect on PAD16"]
    #[inline]
    pub fn pad16(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD16)
    }
    #[doc = "Edge detect on PAD15"]
    #[inline]
    pub fn pad15(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD15)
    }
    #[doc = "Edge detect on PAD14"]
    #[inline]
    pub fn pad14(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD14)
    }
    #[doc = "Edge detect on PAD13"]
    #[inline]
    pub fn pad13(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD13)
    }
    #[doc = "Edge detect on PAD12"]
    #[inline]
    pub fn pad12(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD12)
    }
    #[doc = "Edge detect on PAD11"]
    #[inline]
    pub fn pad11(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD11)
    }
    #[doc = "Edge detect on PAD10"]
    #[inline]
    pub fn pad10(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD10)
    }
    #[doc = "Edge detect on PAD9"]
    #[inline]
    pub fn pad9(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD9)
    }
    #[doc = "Edge detect on PAD8"]
    #[inline]
    pub fn pad8(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD8)
    }
    #[doc = "Edge detect on PAD7"]
    #[inline]
    pub fn pad7(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD7)
    }
    #[doc = "Edge detect on PAD6"]
    #[inline]
    pub fn pad6(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD6)
    }
    #[doc = "Edge detect on PAD5"]
    #[inline]
    pub fn pad5(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD5)
    }
    #[doc = "Edge detect on PAD4"]
    #[inline]
    pub fn pad4(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD4)
    }
    #[doc = "Edge detect on PAD3"]
    #[inline]
    pub fn pad3(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD3)
    }
    #[doc = "Edge detect on PAD2"]
    #[inline]
    pub fn pad2(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD2)
    }
    #[doc = "Edge detect on PAD1"]
    #[inline]
    pub fn pad1(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD1)
    }
    #[doc = "Edge detect on PAD0"]
    #[inline]
    pub fn pad0(self) -> &'a mut W {
        self.variant(WU0_EVW::PAD0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bits 22:31 - 31:22\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved22(&self) -> RESERVED22R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED22R { bits }
    }
    #[doc = "Bits 16:21 - 21:16\\] AUX Wakeup Source #2 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu2_ev(&self) -> WU2_EVR {
        WU2_EVR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - 15:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved14(&self) -> RESERVED14R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED14R { bits }
    }
    #[doc = "Bits 8:13 - 13:8\\] AUX Wakeup Source #1 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu1_ev(&self) -> WU1_EVR {
        WU1_EVR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - 7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&self) -> RESERVED6R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED6R { bits }
    }
    #[doc = "Bits 0:5 - 5:0\\] AUX Wakeup Source #0 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu0_ev(&self) -> WU0_EVR {
        WU0_EVR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4144959 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 22:31 - 31:22\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved22(&mut self) -> _RESERVED22W {
        _RESERVED22W { w: self }
    }
    #[doc = "Bits 16:21 - 21:16\\] AUX Wakeup Source #2 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu2_ev(&mut self) -> _WU2_EVW {
        _WU2_EVW { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved14(&mut self) -> _RESERVED14W {
        _RESERVED14W { w: self }
    }
    #[doc = "Bits 8:13 - 13:8\\] AUX Wakeup Source #1 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu1_ev(&mut self) -> _WU1_EVW {
        _WU1_EVW { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] AUX Wakeup Source #0 AON Event Source selecting 1 of 3 events routed to AON_WUC for waking up the AUX domain from Power Off or Power Down. Note:"]
    #[inline]
    pub fn wu0_ev(&mut self) -> _WU0_EVW {
        _WU0_EVW { w: self }
    }
}
