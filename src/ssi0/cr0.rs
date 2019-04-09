#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR0 {
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
pub struct SCRR {
    bits: u8,
}
impl SCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SPH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPHR {
    #[doc = "Data is captured on the second clock edge transition."]
    _2ND_CLK_EDGE,
    #[doc = "Data is captured on the first clock edge transition."]
    _1ST_CLK_EDGE,
}
impl SPHR {
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
            SPHR::_2ND_CLK_EDGE => true,
            SPHR::_1ST_CLK_EDGE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPHR {
        match value {
            true => SPHR::_2ND_CLK_EDGE,
            false => SPHR::_1ST_CLK_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `_2ND_CLK_EDGE`"]
    #[inline]
    pub fn is_2nd_clk_edge(&self) -> bool {
        *self == SPHR::_2ND_CLK_EDGE
    }
    #[doc = "Checks if the value of the field is `_1ST_CLK_EDGE`"]
    #[inline]
    pub fn is_1st_clk_edge(&self) -> bool {
        *self == SPHR::_1ST_CLK_EDGE
    }
}
#[doc = "Possible values of the field `SPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOR {
    #[doc = "SSI produces a steady state HIGH value on the CLKOUT pin when data is not being transferred."]
    HIGH,
    #[doc = "SSI produces a steady state LOW value on the\nCLKOUT pin when data is not being transferred."]
    LOW,
}
impl SPOR {
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
            SPOR::HIGH => true,
            SPOR::LOW => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPOR {
        match value {
            true => SPOR::HIGH,
            false => SPOR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SPOR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SPOR::LOW
    }
}
#[doc = "Possible values of the field `FRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRFR {
    #[doc = "National Microwire frame format"]
    NATIONAL_MICROWIRE,
    #[doc = "TI synchronous serial frame format"]
    TI_SYNC_SERIAL,
    #[doc = "Motorola SPI frame format"]
    MOTOROLA_SPI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FRFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FRFR::NATIONAL_MICROWIRE => 2,
            FRFR::TI_SYNC_SERIAL => 1,
            FRFR::MOTOROLA_SPI => 0,
            FRFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FRFR {
        match value {
            2 => FRFR::NATIONAL_MICROWIRE,
            1 => FRFR::TI_SYNC_SERIAL,
            0 => FRFR::MOTOROLA_SPI,
            i => FRFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NATIONAL_MICROWIRE`"]
    #[inline]
    pub fn is_national_microwire(&self) -> bool {
        *self == FRFR::NATIONAL_MICROWIRE
    }
    #[doc = "Checks if the value of the field is `TI_SYNC_SERIAL`"]
    #[inline]
    pub fn is_ti_sync_serial(&self) -> bool {
        *self == FRFR::TI_SYNC_SERIAL
    }
    #[doc = "Checks if the value of the field is `MOTOROLA_SPI`"]
    #[inline]
    pub fn is_motorola_spi(&self) -> bool {
        *self == FRFR::MOTOROLA_SPI
    }
}
#[doc = "Possible values of the field `DSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSSR {
    #[doc = "16-bit data"]
    _16_BIT,
    #[doc = "15-bit data"]
    _15_BIT,
    #[doc = "14-bit data"]
    _14_BIT,
    #[doc = "13-bit data"]
    _13_BIT,
    #[doc = "12-bit data"]
    _12_BIT,
    #[doc = "11-bit data"]
    _11_BIT,
    #[doc = "10-bit data"]
    _10_BIT,
    #[doc = "9-bit data"]
    _9_BIT,
    #[doc = "8-bit data"]
    _8_BIT,
    #[doc = "7-bit data"]
    _7_BIT,
    #[doc = "6-bit data"]
    _6_BIT,
    #[doc = "5-bit data"]
    _5_BIT,
    #[doc = "4-bit data"]
    _4_BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSSR::_16_BIT => 15,
            DSSR::_15_BIT => 14,
            DSSR::_14_BIT => 13,
            DSSR::_13_BIT => 12,
            DSSR::_12_BIT => 11,
            DSSR::_11_BIT => 10,
            DSSR::_10_BIT => 9,
            DSSR::_9_BIT => 8,
            DSSR::_8_BIT => 7,
            DSSR::_7_BIT => 6,
            DSSR::_6_BIT => 5,
            DSSR::_5_BIT => 4,
            DSSR::_4_BIT => 3,
            DSSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSSR {
        match value {
            15 => DSSR::_16_BIT,
            14 => DSSR::_15_BIT,
            13 => DSSR::_14_BIT,
            12 => DSSR::_13_BIT,
            11 => DSSR::_12_BIT,
            10 => DSSR::_11_BIT,
            9 => DSSR::_10_BIT,
            8 => DSSR::_9_BIT,
            7 => DSSR::_8_BIT,
            6 => DSSR::_7_BIT,
            5 => DSSR::_6_BIT,
            4 => DSSR::_5_BIT,
            3 => DSSR::_4_BIT,
            i => DSSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline]
    pub fn is_16_bit(&self) -> bool {
        *self == DSSR::_16_BIT
    }
    #[doc = "Checks if the value of the field is `_15_BIT`"]
    #[inline]
    pub fn is_15_bit(&self) -> bool {
        *self == DSSR::_15_BIT
    }
    #[doc = "Checks if the value of the field is `_14_BIT`"]
    #[inline]
    pub fn is_14_bit(&self) -> bool {
        *self == DSSR::_14_BIT
    }
    #[doc = "Checks if the value of the field is `_13_BIT`"]
    #[inline]
    pub fn is_13_bit(&self) -> bool {
        *self == DSSR::_13_BIT
    }
    #[doc = "Checks if the value of the field is `_12_BIT`"]
    #[inline]
    pub fn is_12_bit(&self) -> bool {
        *self == DSSR::_12_BIT
    }
    #[doc = "Checks if the value of the field is `_11_BIT`"]
    #[inline]
    pub fn is_11_bit(&self) -> bool {
        *self == DSSR::_11_BIT
    }
    #[doc = "Checks if the value of the field is `_10_BIT`"]
    #[inline]
    pub fn is_10_bit(&self) -> bool {
        *self == DSSR::_10_BIT
    }
    #[doc = "Checks if the value of the field is `_9_BIT`"]
    #[inline]
    pub fn is_9_bit(&self) -> bool {
        *self == DSSR::_9_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == DSSR::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline]
    pub fn is_7_bit(&self) -> bool {
        *self == DSSR::_7_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline]
    pub fn is_6_bit(&self) -> bool {
        *self == DSSR::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline]
    pub fn is_5_bit(&self) -> bool {
        *self == DSSR::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_4_BIT`"]
    #[inline]
    pub fn is_4_bit(&self) -> bool {
        *self == DSSR::_4_BIT
    }
}
#[doc = r" Proxy"]
pub struct _SCRW<'a> {
    w: &'a mut W,
}
impl<'a> _SCRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPH`"]
pub enum SPHW {
    #[doc = "Data is captured on the second clock edge transition."]
    _2ND_CLK_EDGE,
    #[doc = "Data is captured on the first clock edge transition."]
    _1ST_CLK_EDGE,
}
impl SPHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPHW::_2ND_CLK_EDGE => true,
            SPHW::_1ST_CLK_EDGE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPHW<'a> {
    w: &'a mut W,
}
impl<'a> _SPHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is captured on the second clock edge transition."]
    #[inline]
    pub fn _2nd_clk_edge(self) -> &'a mut W {
        self.variant(SPHW::_2ND_CLK_EDGE)
    }
    #[doc = "Data is captured on the first clock edge transition."]
    #[inline]
    pub fn _1st_clk_edge(self) -> &'a mut W {
        self.variant(SPHW::_1ST_CLK_EDGE)
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
#[doc = "Values that can be written to the field `SPO`"]
pub enum SPOW {
    #[doc = "SSI produces a steady state HIGH value on the CLKOUT pin when data is not being transferred."]
    HIGH,
    #[doc = "SSI produces a steady state LOW value on the\nCLKOUT pin when data is not being transferred."]
    LOW,
}
impl SPOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPOW::HIGH => true,
            SPOW::LOW => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPOW<'a> {
    w: &'a mut W,
}
impl<'a> _SPOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSI produces a steady state HIGH value on the CLKOUT pin when data is not being transferred."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SPOW::HIGH)
    }
    #[doc = "SSI produces a steady state LOW value on the CLKOUT pin when data is not being transferred."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SPOW::LOW)
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
#[doc = "Values that can be written to the field `FRF`"]
pub enum FRFW {
    #[doc = "National Microwire frame format"]
    NATIONAL_MICROWIRE,
    #[doc = "TI synchronous serial frame format"]
    TI_SYNC_SERIAL,
    #[doc = "Motorola SPI frame format"]
    MOTOROLA_SPI,
}
impl FRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FRFW::NATIONAL_MICROWIRE => 2,
            FRFW::TI_SYNC_SERIAL => 1,
            FRFW::MOTOROLA_SPI => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRFW<'a> {
    w: &'a mut W,
}
impl<'a> _FRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "National Microwire frame format"]
    #[inline]
    pub fn national_microwire(self) -> &'a mut W {
        self.variant(FRFW::NATIONAL_MICROWIRE)
    }
    #[doc = "TI synchronous serial frame format"]
    #[inline]
    pub fn ti_sync_serial(self) -> &'a mut W {
        self.variant(FRFW::TI_SYNC_SERIAL)
    }
    #[doc = "Motorola SPI frame format"]
    #[inline]
    pub fn motorola_spi(self) -> &'a mut W {
        self.variant(FRFW::MOTOROLA_SPI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DSS`"]
pub enum DSSW {
    #[doc = "16-bit data"]
    _16_BIT,
    #[doc = "15-bit data"]
    _15_BIT,
    #[doc = "14-bit data"]
    _14_BIT,
    #[doc = "13-bit data"]
    _13_BIT,
    #[doc = "12-bit data"]
    _12_BIT,
    #[doc = "11-bit data"]
    _11_BIT,
    #[doc = "10-bit data"]
    _10_BIT,
    #[doc = "9-bit data"]
    _9_BIT,
    #[doc = "8-bit data"]
    _8_BIT,
    #[doc = "7-bit data"]
    _7_BIT,
    #[doc = "6-bit data"]
    _6_BIT,
    #[doc = "5-bit data"]
    _5_BIT,
    #[doc = "4-bit data"]
    _4_BIT,
}
impl DSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSSW::_16_BIT => 15,
            DSSW::_15_BIT => 14,
            DSSW::_14_BIT => 13,
            DSSW::_13_BIT => 12,
            DSSW::_12_BIT => 11,
            DSSW::_11_BIT => 10,
            DSSW::_10_BIT => 9,
            DSSW::_9_BIT => 8,
            DSSW::_8_BIT => 7,
            DSSW::_7_BIT => 6,
            DSSW::_6_BIT => 5,
            DSSW::_5_BIT => 4,
            DSSW::_4_BIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSSW<'a> {
    w: &'a mut W,
}
impl<'a> _DSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "16-bit data"]
    #[inline]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(DSSW::_16_BIT)
    }
    #[doc = "15-bit data"]
    #[inline]
    pub fn _15_bit(self) -> &'a mut W {
        self.variant(DSSW::_15_BIT)
    }
    #[doc = "14-bit data"]
    #[inline]
    pub fn _14_bit(self) -> &'a mut W {
        self.variant(DSSW::_14_BIT)
    }
    #[doc = "13-bit data"]
    #[inline]
    pub fn _13_bit(self) -> &'a mut W {
        self.variant(DSSW::_13_BIT)
    }
    #[doc = "12-bit data"]
    #[inline]
    pub fn _12_bit(self) -> &'a mut W {
        self.variant(DSSW::_12_BIT)
    }
    #[doc = "11-bit data"]
    #[inline]
    pub fn _11_bit(self) -> &'a mut W {
        self.variant(DSSW::_11_BIT)
    }
    #[doc = "10-bit data"]
    #[inline]
    pub fn _10_bit(self) -> &'a mut W {
        self.variant(DSSW::_10_BIT)
    }
    #[doc = "9-bit data"]
    #[inline]
    pub fn _9_bit(self) -> &'a mut W {
        self.variant(DSSW::_9_BIT)
    }
    #[doc = "8-bit data"]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(DSSW::_8_BIT)
    }
    #[doc = "7-bit data"]
    #[inline]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(DSSW::_7_BIT)
    }
    #[doc = "6-bit data"]
    #[inline]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(DSSW::_6_BIT)
    }
    #[doc = "5-bit data"]
    #[inline]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(DSSW::_5_BIT)
    }
    #[doc = "4-bit data"]
    #[inline]
    pub fn _4_bit(self) -> &'a mut W {
        self.variant(DSSW::_4_BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 8:15 - 15:8\\] Serial clock rate: This is used to generate the transmit and receive bit rate of the SSI. The bit rate is (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR). SCR is a value from 0-255."]
    #[inline]
    pub fn scr(&self) -> SCRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCRR { bits }
    }
    #[doc = "Bit 7 - 7:7\\] CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline]
    pub fn sph(&self) -> SPHR {
        SPHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - 6:6\\] CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline]
    pub fn spo(&self) -> SPOR {
        SPOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - 5:4\\] Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used."]
    #[inline]
    pub fn frf(&self) -> FRFR {
        FRFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:3 - 3:0\\] Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used."]
    #[inline]
    pub fn dss(&self) -> DSSR {
        DSSR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 8:15 - 15:8\\] Serial clock rate: This is used to generate the transmit and receive bit rate of the SSI. The bit rate is (SSI's clock frequency)/((SCR+1)*CPSR.CPSDVSR). SCR is a value from 0-255."]
    #[inline]
    pub fn scr(&mut self) -> _SCRW {
        _SCRW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline]
    pub fn sph(&mut self) -> _SPHW {
        _SPHW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline]
    pub fn spo(&mut self) -> _SPOW {
        _SPOW { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Frame format. The supported frame formats are Motorola SPI, TI synchronous serial and National Microwire. Value 0'b11 is reserved and shall not be used."]
    #[inline]
    pub fn frf(&mut self) -> _FRFW {
        _FRFW { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Data Size Select. Values 0b0000, 0b0001, 0b0010 are reserved and shall not be used."]
    #[inline]
    pub fn dss(&mut self) -> _DSSW {
        _DSSW { w: self }
    }
}
