#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOCFG30 {
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
pub struct RESERVED31R {
    bits: bool,
}
impl RESERVED31R {
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
pub struct HYST_ENR {
    bits: bool,
}
impl HYST_ENR {
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
pub struct IER {
    bits: bool,
}
impl IER {
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
pub struct WU_CFGR {
    bits: u8,
}
impl WU_CFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `IOMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOMODER {
    #[doc = "Open Source\nInverted input / output\n\n"]
    OPENSRC_INV,
    #[doc = "Open Source\nNormal input / output\n\n"]
    OPENSRC,
    #[doc = "Open Drain\nInverted input / output\n\n"]
    OPENDR_INV,
    #[doc = "Open Drain, \nNormal input / output\n\n"]
    OPENDR,
    #[doc = "Inverted input / ouput\n\n"]
    INV,
    #[doc = "Normal input / output\n\n"]
    NORMAL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IOMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IOMODER::OPENSRC_INV => 7,
            IOMODER::OPENSRC => 6,
            IOMODER::OPENDR_INV => 5,
            IOMODER::OPENDR => 4,
            IOMODER::INV => 1,
            IOMODER::NORMAL => 0,
            IOMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IOMODER {
        match value {
            7 => IOMODER::OPENSRC_INV,
            6 => IOMODER::OPENSRC,
            5 => IOMODER::OPENDR_INV,
            4 => IOMODER::OPENDR,
            1 => IOMODER::INV,
            0 => IOMODER::NORMAL,
            i => IOMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OPENSRC_INV`"]
    #[inline]
    pub fn is_opensrc_inv(&self) -> bool {
        *self == IOMODER::OPENSRC_INV
    }
    #[doc = "Checks if the value of the field is `OPENSRC`"]
    #[inline]
    pub fn is_opensrc(&self) -> bool {
        *self == IOMODER::OPENSRC
    }
    #[doc = "Checks if the value of the field is `OPENDR_INV`"]
    #[inline]
    pub fn is_opendr_inv(&self) -> bool {
        *self == IOMODER::OPENDR_INV
    }
    #[doc = "Checks if the value of the field is `OPENDR`"]
    #[inline]
    pub fn is_opendr(&self) -> bool {
        *self == IOMODER::OPENDR
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline]
    pub fn is_inv(&self) -> bool {
        *self == IOMODER::INV
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == IOMODER::NORMAL
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED19R {
    bits: u8,
}
impl RESERVED19R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EDGE_IRQ_ENR {
    bits: bool,
}
impl EDGE_IRQ_ENR {
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
#[doc = "Possible values of the field `EDGE_DET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGE_DETR {
    #[doc = "Positive and negative edge detection"]
    BOTH,
    #[doc = "Positive edge detection"]
    POS,
    #[doc = "Negative edge detection"]
    NEG,
    #[doc = "No edge detection"]
    NONE,
}
impl EDGE_DETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EDGE_DETR::BOTH => 3,
            EDGE_DETR::POS => 2,
            EDGE_DETR::NEG => 1,
            EDGE_DETR::NONE => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EDGE_DETR {
        match value {
            3 => EDGE_DETR::BOTH,
            2 => EDGE_DETR::POS,
            1 => EDGE_DETR::NEG,
            0 => EDGE_DETR::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == EDGE_DETR::BOTH
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline]
    pub fn is_pos(&self) -> bool {
        *self == EDGE_DETR::POS
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline]
    pub fn is_neg(&self) -> bool {
        *self == EDGE_DETR::NEG
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == EDGE_DETR::NONE
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED15R {
    bits: bool,
}
impl RESERVED15R {
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
#[doc = "Possible values of the field `PULL_CTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULL_CTLR {
    #[doc = "No pull"]
    DIS,
    #[doc = "Pull up"]
    UP,
    #[doc = "Pull down"]
    DWN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PULL_CTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PULL_CTLR::DIS => 3,
            PULL_CTLR::UP => 2,
            PULL_CTLR::DWN => 1,
            PULL_CTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PULL_CTLR {
        match value {
            3 => PULL_CTLR::DIS,
            2 => PULL_CTLR::UP,
            1 => PULL_CTLR::DWN,
            i => PULL_CTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == PULL_CTLR::DIS
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline]
    pub fn is_up(&self) -> bool {
        *self == PULL_CTLR::UP
    }
    #[doc = "Checks if the value of the field is `DWN`"]
    #[inline]
    pub fn is_dwn(&self) -> bool {
        *self == PULL_CTLR::DWN
    }
}
#[doc = r" Value of the field"]
pub struct SLEW_REDR {
    bits: bool,
}
impl SLEW_REDR {
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
#[doc = "Possible values of the field `IOCURR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCURRR {
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO"]
    _4_8MA,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO"]
    _4MA,
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO"]
    _2MA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IOCURRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IOCURRR::_4_8MA => 2,
            IOCURRR::_4MA => 1,
            IOCURRR::_2MA => 0,
            IOCURRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IOCURRR {
        match value {
            2 => IOCURRR::_4_8MA,
            1 => IOCURRR::_4MA,
            0 => IOCURRR::_2MA,
            i => IOCURRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4_8MA`"]
    #[inline]
    pub fn is_4_8ma(&self) -> bool {
        *self == IOCURRR::_4_8MA
    }
    #[doc = "Checks if the value of the field is `_4MA`"]
    #[inline]
    pub fn is_4ma(&self) -> bool {
        *self == IOCURRR::_4MA
    }
    #[doc = "Checks if the value of the field is `_2MA`"]
    #[inline]
    pub fn is_2ma(&self) -> bool {
        *self == IOCURRR::_2MA
    }
}
#[doc = "Possible values of the field `IOSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOSTRR {
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)"]
    MAX,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)"]
    MED,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)"]
    MIN,
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)"]
    AUTO,
}
impl IOSTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IOSTRR::MAX => 3,
            IOSTRR::MED => 2,
            IOSTRR::MIN => 1,
            IOSTRR::AUTO => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IOSTRR {
        match value {
            3 => IOSTRR::MAX,
            2 => IOSTRR::MED,
            1 => IOSTRR::MIN,
            0 => IOSTRR::AUTO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline]
    pub fn is_max(&self) -> bool {
        *self == IOSTRR::MAX
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline]
    pub fn is_med(&self) -> bool {
        *self == IOSTRR::MED
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline]
    pub fn is_min(&self) -> bool {
        *self == IOSTRR::MIN
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline]
    pub fn is_auto(&self) -> bool {
        *self == IOSTRR::AUTO
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
#[doc = "Possible values of the field `PORT_ID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT_IDR {
    #[doc = "RF Core SMI Command Link In"]
    RFC_SMI_CL_IN,
    #[doc = "RF Core SMI Command Link Out"]
    RFC_SMI_CL_OUT,
    #[doc = "RF Core SMI Data Link In"]
    RFC_SMI_DL_IN,
    #[doc = "RF Core SMI Data Link Out"]
    RFC_SMI_DL_OUT,
    #[doc = "RF Core Data In 1"]
    RFC_GPI1,
    #[doc = "RF Core Data In 0"]
    RFC_GPI0,
    #[doc = "RF Core Data Out 3"]
    RFC_GPO3,
    #[doc = "RF Core Data Out 2"]
    RFC_GPO2,
    #[doc = "RF Core Data Out 1"]
    RFC_GPO1,
    #[doc = "RF Core Data Out 0"]
    RFC_GPO0,
    #[doc = "RF Core Trace"]
    RFC_TRC,
    #[doc = "I2S MCLK "]
    I2S_MCLK,
    #[doc = "I2S BCLK "]
    I2S_BCLK,
    #[doc = "I2S WCLK "]
    I2S_WCLK,
    #[doc = "I2S Data 1"]
    I2S_AD1,
    #[doc = "I2S Data 0"]
    I2S_AD0,
    #[doc = "SSI1 CLK"]
    SSI1_CLK,
    #[doc = "SSI1 FSS "]
    SSI1_FSS,
    #[doc = "SSI1 TX "]
    SSI1_TX,
    #[doc = "SSI1 RX "]
    SSI1_RX,
    #[doc = "CPU SWV "]
    CPU_SWV,
    #[doc = "PORT EVENT 7\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT7,
    #[doc = "PORT EVENT 6\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT6,
    #[doc = "PORT EVENT 5\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT5,
    #[doc = "PORT EVENT 4\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT4,
    #[doc = "PORT EVENT 3\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT3,
    #[doc = "PORT EVENT 2\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT2,
    #[doc = "PORT EVENT 1\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT1,
    #[doc = "PORT EVENT 0\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT0,
    #[doc = "UART0 RTS "]
    UART0_RTS,
    #[doc = "UART0 CTS "]
    UART0_CTS,
    #[doc = "UART0 TX "]
    UART0_TX,
    #[doc = "UART0 RX "]
    UART0_RX,
    #[doc = "I2C Clock"]
    I2C_MSSCL,
    #[doc = "I2C Data"]
    I2C_MSSDA,
    #[doc = "SSI0 CLK"]
    SSI0_CLK,
    #[doc = "SSI0 FSS "]
    SSI0_FSS,
    #[doc = "SSI0 TX "]
    SSI0_TX,
    #[doc = "SSI0 RX "]
    SSI0_RX,
    #[doc = "AUX IO "]
    AUX_IO,
    #[doc = "AON 32 KHz clock (SCLK_LF)"]
    AON_CLK32K,
    #[doc = "General Purpose IO "]
    GPIO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PORT_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PORT_IDR::RFC_SMI_CL_IN => 56,
            PORT_IDR::RFC_SMI_CL_OUT => 55,
            PORT_IDR::RFC_SMI_DL_IN => 54,
            PORT_IDR::RFC_SMI_DL_OUT => 53,
            PORT_IDR::RFC_GPI1 => 52,
            PORT_IDR::RFC_GPI0 => 51,
            PORT_IDR::RFC_GPO3 => 50,
            PORT_IDR::RFC_GPO2 => 49,
            PORT_IDR::RFC_GPO1 => 48,
            PORT_IDR::RFC_GPO0 => 47,
            PORT_IDR::RFC_TRC => 46,
            PORT_IDR::I2S_MCLK => 41,
            PORT_IDR::I2S_BCLK => 40,
            PORT_IDR::I2S_WCLK => 39,
            PORT_IDR::I2S_AD1 => 38,
            PORT_IDR::I2S_AD0 => 37,
            PORT_IDR::SSI1_CLK => 36,
            PORT_IDR::SSI1_FSS => 35,
            PORT_IDR::SSI1_TX => 34,
            PORT_IDR::SSI1_RX => 33,
            PORT_IDR::CPU_SWV => 32,
            PORT_IDR::PORT_EVENT7 => 30,
            PORT_IDR::PORT_EVENT6 => 29,
            PORT_IDR::PORT_EVENT5 => 28,
            PORT_IDR::PORT_EVENT4 => 27,
            PORT_IDR::PORT_EVENT3 => 26,
            PORT_IDR::PORT_EVENT2 => 25,
            PORT_IDR::PORT_EVENT1 => 24,
            PORT_IDR::PORT_EVENT0 => 23,
            PORT_IDR::UART0_RTS => 18,
            PORT_IDR::UART0_CTS => 17,
            PORT_IDR::UART0_TX => 16,
            PORT_IDR::UART0_RX => 15,
            PORT_IDR::I2C_MSSCL => 14,
            PORT_IDR::I2C_MSSDA => 13,
            PORT_IDR::SSI0_CLK => 12,
            PORT_IDR::SSI0_FSS => 11,
            PORT_IDR::SSI0_TX => 10,
            PORT_IDR::SSI0_RX => 9,
            PORT_IDR::AUX_IO => 8,
            PORT_IDR::AON_CLK32K => 7,
            PORT_IDR::GPIO => 0,
            PORT_IDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PORT_IDR {
        match value {
            56 => PORT_IDR::RFC_SMI_CL_IN,
            55 => PORT_IDR::RFC_SMI_CL_OUT,
            54 => PORT_IDR::RFC_SMI_DL_IN,
            53 => PORT_IDR::RFC_SMI_DL_OUT,
            52 => PORT_IDR::RFC_GPI1,
            51 => PORT_IDR::RFC_GPI0,
            50 => PORT_IDR::RFC_GPO3,
            49 => PORT_IDR::RFC_GPO2,
            48 => PORT_IDR::RFC_GPO1,
            47 => PORT_IDR::RFC_GPO0,
            46 => PORT_IDR::RFC_TRC,
            41 => PORT_IDR::I2S_MCLK,
            40 => PORT_IDR::I2S_BCLK,
            39 => PORT_IDR::I2S_WCLK,
            38 => PORT_IDR::I2S_AD1,
            37 => PORT_IDR::I2S_AD0,
            36 => PORT_IDR::SSI1_CLK,
            35 => PORT_IDR::SSI1_FSS,
            34 => PORT_IDR::SSI1_TX,
            33 => PORT_IDR::SSI1_RX,
            32 => PORT_IDR::CPU_SWV,
            30 => PORT_IDR::PORT_EVENT7,
            29 => PORT_IDR::PORT_EVENT6,
            28 => PORT_IDR::PORT_EVENT5,
            27 => PORT_IDR::PORT_EVENT4,
            26 => PORT_IDR::PORT_EVENT3,
            25 => PORT_IDR::PORT_EVENT2,
            24 => PORT_IDR::PORT_EVENT1,
            23 => PORT_IDR::PORT_EVENT0,
            18 => PORT_IDR::UART0_RTS,
            17 => PORT_IDR::UART0_CTS,
            16 => PORT_IDR::UART0_TX,
            15 => PORT_IDR::UART0_RX,
            14 => PORT_IDR::I2C_MSSCL,
            13 => PORT_IDR::I2C_MSSDA,
            12 => PORT_IDR::SSI0_CLK,
            11 => PORT_IDR::SSI0_FSS,
            10 => PORT_IDR::SSI0_TX,
            9 => PORT_IDR::SSI0_RX,
            8 => PORT_IDR::AUX_IO,
            7 => PORT_IDR::AON_CLK32K,
            0 => PORT_IDR::GPIO,
            i => PORT_IDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RFC_SMI_CL_IN`"]
    #[inline]
    pub fn is_rfc_smi_cl_in(&self) -> bool {
        *self == PORT_IDR::RFC_SMI_CL_IN
    }
    #[doc = "Checks if the value of the field is `RFC_SMI_CL_OUT`"]
    #[inline]
    pub fn is_rfc_smi_cl_out(&self) -> bool {
        *self == PORT_IDR::RFC_SMI_CL_OUT
    }
    #[doc = "Checks if the value of the field is `RFC_SMI_DL_IN`"]
    #[inline]
    pub fn is_rfc_smi_dl_in(&self) -> bool {
        *self == PORT_IDR::RFC_SMI_DL_IN
    }
    #[doc = "Checks if the value of the field is `RFC_SMI_DL_OUT`"]
    #[inline]
    pub fn is_rfc_smi_dl_out(&self) -> bool {
        *self == PORT_IDR::RFC_SMI_DL_OUT
    }
    #[doc = "Checks if the value of the field is `RFC_GPI1`"]
    #[inline]
    pub fn is_rfc_gpi1(&self) -> bool {
        *self == PORT_IDR::RFC_GPI1
    }
    #[doc = "Checks if the value of the field is `RFC_GPI0`"]
    #[inline]
    pub fn is_rfc_gpi0(&self) -> bool {
        *self == PORT_IDR::RFC_GPI0
    }
    #[doc = "Checks if the value of the field is `RFC_GPO3`"]
    #[inline]
    pub fn is_rfc_gpo3(&self) -> bool {
        *self == PORT_IDR::RFC_GPO3
    }
    #[doc = "Checks if the value of the field is `RFC_GPO2`"]
    #[inline]
    pub fn is_rfc_gpo2(&self) -> bool {
        *self == PORT_IDR::RFC_GPO2
    }
    #[doc = "Checks if the value of the field is `RFC_GPO1`"]
    #[inline]
    pub fn is_rfc_gpo1(&self) -> bool {
        *self == PORT_IDR::RFC_GPO1
    }
    #[doc = "Checks if the value of the field is `RFC_GPO0`"]
    #[inline]
    pub fn is_rfc_gpo0(&self) -> bool {
        *self == PORT_IDR::RFC_GPO0
    }
    #[doc = "Checks if the value of the field is `RFC_TRC`"]
    #[inline]
    pub fn is_rfc_trc(&self) -> bool {
        *self == PORT_IDR::RFC_TRC
    }
    #[doc = "Checks if the value of the field is `I2S_MCLK`"]
    #[inline]
    pub fn is_i2s_mclk(&self) -> bool {
        *self == PORT_IDR::I2S_MCLK
    }
    #[doc = "Checks if the value of the field is `I2S_BCLK`"]
    #[inline]
    pub fn is_i2s_bclk(&self) -> bool {
        *self == PORT_IDR::I2S_BCLK
    }
    #[doc = "Checks if the value of the field is `I2S_WCLK`"]
    #[inline]
    pub fn is_i2s_wclk(&self) -> bool {
        *self == PORT_IDR::I2S_WCLK
    }
    #[doc = "Checks if the value of the field is `I2S_AD1`"]
    #[inline]
    pub fn is_i2s_ad1(&self) -> bool {
        *self == PORT_IDR::I2S_AD1
    }
    #[doc = "Checks if the value of the field is `I2S_AD0`"]
    #[inline]
    pub fn is_i2s_ad0(&self) -> bool {
        *self == PORT_IDR::I2S_AD0
    }
    #[doc = "Checks if the value of the field is `SSI1_CLK`"]
    #[inline]
    pub fn is_ssi1_clk(&self) -> bool {
        *self == PORT_IDR::SSI1_CLK
    }
    #[doc = "Checks if the value of the field is `SSI1_FSS`"]
    #[inline]
    pub fn is_ssi1_fss(&self) -> bool {
        *self == PORT_IDR::SSI1_FSS
    }
    #[doc = "Checks if the value of the field is `SSI1_TX`"]
    #[inline]
    pub fn is_ssi1_tx(&self) -> bool {
        *self == PORT_IDR::SSI1_TX
    }
    #[doc = "Checks if the value of the field is `SSI1_RX`"]
    #[inline]
    pub fn is_ssi1_rx(&self) -> bool {
        *self == PORT_IDR::SSI1_RX
    }
    #[doc = "Checks if the value of the field is `CPU_SWV`"]
    #[inline]
    pub fn is_cpu_swv(&self) -> bool {
        *self == PORT_IDR::CPU_SWV
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT7`"]
    #[inline]
    pub fn is_port_event7(&self) -> bool {
        *self == PORT_IDR::PORT_EVENT7
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT6`"]
    #[inline]
    pub fn is_port_event6(&self) -> bool {
        *self == PORT_IDR::PORT_EVENT6
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT5`"]
    #[inline]
    pub fn is_port_event5(&self) -> bool {
        *self == PORT_IDR::PORT_EVENT5
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT4`"]
    #[inline]
    pub fn is_port_event4(&self) -> bool {
        *self == PORT_IDR::PORT_EVENT4
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT3`"]
    #[inline]
    pub fn is_port_event3(&self) -> bool {
        *self == PORT_IDR::PORT_EVENT3
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT2`"]
    #[inline]
    pub fn is_port_event2(&self) -> bool {
        *self == PORT_IDR::PORT_EVENT2
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT1`"]
    #[inline]
    pub fn is_port_event1(&self) -> bool {
        *self == PORT_IDR::PORT_EVENT1
    }
    #[doc = "Checks if the value of the field is `PORT_EVENT0`"]
    #[inline]
    pub fn is_port_event0(&self) -> bool {
        *self == PORT_IDR::PORT_EVENT0
    }
    #[doc = "Checks if the value of the field is `UART0_RTS`"]
    #[inline]
    pub fn is_uart0_rts(&self) -> bool {
        *self == PORT_IDR::UART0_RTS
    }
    #[doc = "Checks if the value of the field is `UART0_CTS`"]
    #[inline]
    pub fn is_uart0_cts(&self) -> bool {
        *self == PORT_IDR::UART0_CTS
    }
    #[doc = "Checks if the value of the field is `UART0_TX`"]
    #[inline]
    pub fn is_uart0_tx(&self) -> bool {
        *self == PORT_IDR::UART0_TX
    }
    #[doc = "Checks if the value of the field is `UART0_RX`"]
    #[inline]
    pub fn is_uart0_rx(&self) -> bool {
        *self == PORT_IDR::UART0_RX
    }
    #[doc = "Checks if the value of the field is `I2C_MSSCL`"]
    #[inline]
    pub fn is_i2c_msscl(&self) -> bool {
        *self == PORT_IDR::I2C_MSSCL
    }
    #[doc = "Checks if the value of the field is `I2C_MSSDA`"]
    #[inline]
    pub fn is_i2c_mssda(&self) -> bool {
        *self == PORT_IDR::I2C_MSSDA
    }
    #[doc = "Checks if the value of the field is `SSI0_CLK`"]
    #[inline]
    pub fn is_ssi0_clk(&self) -> bool {
        *self == PORT_IDR::SSI0_CLK
    }
    #[doc = "Checks if the value of the field is `SSI0_FSS`"]
    #[inline]
    pub fn is_ssi0_fss(&self) -> bool {
        *self == PORT_IDR::SSI0_FSS
    }
    #[doc = "Checks if the value of the field is `SSI0_TX`"]
    #[inline]
    pub fn is_ssi0_tx(&self) -> bool {
        *self == PORT_IDR::SSI0_TX
    }
    #[doc = "Checks if the value of the field is `SSI0_RX`"]
    #[inline]
    pub fn is_ssi0_rx(&self) -> bool {
        *self == PORT_IDR::SSI0_RX
    }
    #[doc = "Checks if the value of the field is `AUX_IO`"]
    #[inline]
    pub fn is_aux_io(&self) -> bool {
        *self == PORT_IDR::AUX_IO
    }
    #[doc = "Checks if the value of the field is `AON_CLK32K`"]
    #[inline]
    pub fn is_aon_clk32k(&self) -> bool {
        *self == PORT_IDR::AON_CLK32K
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline]
    pub fn is_gpio(&self) -> bool {
        *self == PORT_IDR::GPIO
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED31W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED31W<'a> {
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HYST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _HYST_ENW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IEW<'a> {
    w: &'a mut W,
}
impl<'a> _IEW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WU_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _WU_CFGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IOMODE`"]
pub enum IOMODEW {
    #[doc = "Open Source\nInverted input / output\n\n"]
    OPENSRC_INV,
    #[doc = "Open Source\nNormal input / output\n\n"]
    OPENSRC,
    #[doc = "Open Drain\nInverted input / output\n\n"]
    OPENDR_INV,
    #[doc = "Open Drain, \nNormal input / output\n\n"]
    OPENDR,
    #[doc = "Inverted input / ouput\n\n"]
    INV,
    #[doc = "Normal input / output\n\n"]
    NORMAL,
}
impl IOMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IOMODEW::OPENSRC_INV => 7,
            IOMODEW::OPENSRC => 6,
            IOMODEW::OPENDR_INV => 5,
            IOMODEW::OPENDR => 4,
            IOMODEW::INV => 1,
            IOMODEW::NORMAL => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _IOMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Open Source Inverted input / output"]
    #[inline]
    pub fn opensrc_inv(self) -> &'a mut W {
        self.variant(IOMODEW::OPENSRC_INV)
    }
    #[doc = "Open Source Normal input / output"]
    #[inline]
    pub fn opensrc(self) -> &'a mut W {
        self.variant(IOMODEW::OPENSRC)
    }
    #[doc = "Open Drain Inverted input / output"]
    #[inline]
    pub fn opendr_inv(self) -> &'a mut W {
        self.variant(IOMODEW::OPENDR_INV)
    }
    #[doc = "Open Drain, Normal input / output"]
    #[inline]
    pub fn opendr(self) -> &'a mut W {
        self.variant(IOMODEW::OPENDR)
    }
    #[doc = "Inverted input / ouput"]
    #[inline]
    pub fn inv(self) -> &'a mut W {
        self.variant(IOMODEW::INV)
    }
    #[doc = "Normal input / output"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(IOMODEW::NORMAL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED19W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED19W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EDGE_IRQ_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGE_IRQ_ENW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EDGE_DET`"]
pub enum EDGE_DETW {
    #[doc = "Positive and negative edge detection"]
    BOTH,
    #[doc = "Positive edge detection"]
    POS,
    #[doc = "Negative edge detection"]
    NEG,
    #[doc = "No edge detection"]
    NONE,
}
impl EDGE_DETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGE_DETW::BOTH => 3,
            EDGE_DETW::POS => 2,
            EDGE_DETW::NEG => 1,
            EDGE_DETW::NONE => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDGE_DETW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGE_DETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDGE_DETW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Positive and negative edge detection"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(EDGE_DETW::BOTH)
    }
    #[doc = "Positive edge detection"]
    #[inline]
    pub fn pos(self) -> &'a mut W {
        self.variant(EDGE_DETW::POS)
    }
    #[doc = "Negative edge detection"]
    #[inline]
    pub fn neg(self) -> &'a mut W {
        self.variant(EDGE_DETW::NEG)
    }
    #[doc = "No edge detection"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(EDGE_DETW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED15W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED15W<'a> {
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
#[doc = "Values that can be written to the field `PULL_CTL`"]
pub enum PULL_CTLW {
    #[doc = "No pull"]
    DIS,
    #[doc = "Pull up"]
    UP,
    #[doc = "Pull down"]
    DWN,
}
impl PULL_CTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PULL_CTLW::DIS => 3,
            PULL_CTLW::UP => 2,
            PULL_CTLW::DWN => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PULL_CTLW<'a> {
    w: &'a mut W,
}
impl<'a> _PULL_CTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PULL_CTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(PULL_CTLW::DIS)
    }
    #[doc = "Pull up"]
    #[inline]
    pub fn up(self) -> &'a mut W {
        self.variant(PULL_CTLW::UP)
    }
    #[doc = "Pull down"]
    #[inline]
    pub fn dwn(self) -> &'a mut W {
        self.variant(PULL_CTLW::DWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SLEW_REDW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEW_REDW<'a> {
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
#[doc = "Values that can be written to the field `IOCURR`"]
pub enum IOCURRW {
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO"]
    _4_8MA,
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO"]
    _4MA,
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO"]
    _2MA,
}
impl IOCURRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IOCURRW::_4_8MA => 2,
            IOCURRW::_4MA => 1,
            IOCURRW::_2MA => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOCURRW<'a> {
    w: &'a mut W,
}
impl<'a> _IOCURRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOCURRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Extended-Current (EC) mode: Min 8 mA for double drive strength IOs (min 4 mA for normal IOs) when IOSTR is set to AUTO"]
    #[inline]
    pub fn _4_8ma(self) -> &'a mut W {
        self.variant(IOCURRW::_4_8MA)
    }
    #[doc = "High-Current (HC) mode: Min 4 mA when IOSTR is set to AUTO"]
    #[inline]
    pub fn _4ma(self) -> &'a mut W {
        self.variant(IOCURRW::_4MA)
    }
    #[doc = "Low-Current (LC) mode: Min 2 mA when IOSTR is set to AUTO"]
    #[inline]
    pub fn _2ma(self) -> &'a mut W {
        self.variant(IOCURRW::_2MA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IOSTR`"]
pub enum IOSTRW {
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)"]
    MAX,
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)"]
    MED,
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)"]
    MIN,
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)"]
    AUTO,
}
impl IOSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IOSTRW::MAX => 3,
            IOSTRW::MED => 2,
            IOSTRW::MIN => 1,
            IOSTRW::AUTO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _IOSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOSTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Maximum drive strength, controlled by AON_IOC:IOSTRMAX (min 2 mA @1.8V with default values)"]
    #[inline]
    pub fn max(self) -> &'a mut W {
        self.variant(IOSTRW::MAX)
    }
    #[doc = "Medium drive strength, controlled by AON_IOC:IOSTRMED (min 2 mA @2.5V with default values)"]
    #[inline]
    pub fn med(self) -> &'a mut W {
        self.variant(IOSTRW::MED)
    }
    #[doc = "Minimum drive strength, controlled by AON_IOC:IOSTRMIN (min 2 mA @3.3V with default values)"]
    #[inline]
    pub fn min(self) -> &'a mut W {
        self.variant(IOSTRW::MIN)
    }
    #[doc = "Automatic drive strength, controlled by AON BATMON based on battery voltage. (min 2 mA @VDDS)"]
    #[inline]
    pub fn auto(self) -> &'a mut W {
        self.variant(IOSTRW::AUTO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
#[doc = "Values that can be written to the field `PORT_ID`"]
pub enum PORT_IDW {
    #[doc = "RF Core SMI Command Link In"]
    RFC_SMI_CL_IN,
    #[doc = "RF Core SMI Command Link Out"]
    RFC_SMI_CL_OUT,
    #[doc = "RF Core SMI Data Link In"]
    RFC_SMI_DL_IN,
    #[doc = "RF Core SMI Data Link Out"]
    RFC_SMI_DL_OUT,
    #[doc = "RF Core Data In 1"]
    RFC_GPI1,
    #[doc = "RF Core Data In 0"]
    RFC_GPI0,
    #[doc = "RF Core Data Out 3"]
    RFC_GPO3,
    #[doc = "RF Core Data Out 2"]
    RFC_GPO2,
    #[doc = "RF Core Data Out 1"]
    RFC_GPO1,
    #[doc = "RF Core Data Out 0"]
    RFC_GPO0,
    #[doc = "RF Core Trace"]
    RFC_TRC,
    #[doc = "I2S MCLK "]
    I2S_MCLK,
    #[doc = "I2S BCLK "]
    I2S_BCLK,
    #[doc = "I2S WCLK "]
    I2S_WCLK,
    #[doc = "I2S Data 1"]
    I2S_AD1,
    #[doc = "I2S Data 0"]
    I2S_AD0,
    #[doc = "SSI1 CLK"]
    SSI1_CLK,
    #[doc = "SSI1 FSS "]
    SSI1_FSS,
    #[doc = "SSI1 TX "]
    SSI1_TX,
    #[doc = "SSI1 RX "]
    SSI1_RX,
    #[doc = "CPU SWV "]
    CPU_SWV,
    #[doc = "PORT EVENT 7\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT7,
    #[doc = "PORT EVENT 6\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT6,
    #[doc = "PORT EVENT 5\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT5,
    #[doc = "PORT EVENT 4\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT4,
    #[doc = "PORT EVENT 3\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT3,
    #[doc = "PORT EVENT 2\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT2,
    #[doc = "PORT EVENT 1\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT1,
    #[doc = "PORT EVENT 0\nCan be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    PORT_EVENT0,
    #[doc = "UART0 RTS "]
    UART0_RTS,
    #[doc = "UART0 CTS "]
    UART0_CTS,
    #[doc = "UART0 TX "]
    UART0_TX,
    #[doc = "UART0 RX "]
    UART0_RX,
    #[doc = "I2C Clock"]
    I2C_MSSCL,
    #[doc = "I2C Data"]
    I2C_MSSDA,
    #[doc = "SSI0 CLK"]
    SSI0_CLK,
    #[doc = "SSI0 FSS "]
    SSI0_FSS,
    #[doc = "SSI0 TX "]
    SSI0_TX,
    #[doc = "SSI0 RX "]
    SSI0_RX,
    #[doc = "AUX IO "]
    AUX_IO,
    #[doc = "AON 32 KHz clock (SCLK_LF)"]
    AON_CLK32K,
    #[doc = "General Purpose IO "]
    GPIO,
}
impl PORT_IDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PORT_IDW::RFC_SMI_CL_IN => 56,
            PORT_IDW::RFC_SMI_CL_OUT => 55,
            PORT_IDW::RFC_SMI_DL_IN => 54,
            PORT_IDW::RFC_SMI_DL_OUT => 53,
            PORT_IDW::RFC_GPI1 => 52,
            PORT_IDW::RFC_GPI0 => 51,
            PORT_IDW::RFC_GPO3 => 50,
            PORT_IDW::RFC_GPO2 => 49,
            PORT_IDW::RFC_GPO1 => 48,
            PORT_IDW::RFC_GPO0 => 47,
            PORT_IDW::RFC_TRC => 46,
            PORT_IDW::I2S_MCLK => 41,
            PORT_IDW::I2S_BCLK => 40,
            PORT_IDW::I2S_WCLK => 39,
            PORT_IDW::I2S_AD1 => 38,
            PORT_IDW::I2S_AD0 => 37,
            PORT_IDW::SSI1_CLK => 36,
            PORT_IDW::SSI1_FSS => 35,
            PORT_IDW::SSI1_TX => 34,
            PORT_IDW::SSI1_RX => 33,
            PORT_IDW::CPU_SWV => 32,
            PORT_IDW::PORT_EVENT7 => 30,
            PORT_IDW::PORT_EVENT6 => 29,
            PORT_IDW::PORT_EVENT5 => 28,
            PORT_IDW::PORT_EVENT4 => 27,
            PORT_IDW::PORT_EVENT3 => 26,
            PORT_IDW::PORT_EVENT2 => 25,
            PORT_IDW::PORT_EVENT1 => 24,
            PORT_IDW::PORT_EVENT0 => 23,
            PORT_IDW::UART0_RTS => 18,
            PORT_IDW::UART0_CTS => 17,
            PORT_IDW::UART0_TX => 16,
            PORT_IDW::UART0_RX => 15,
            PORT_IDW::I2C_MSSCL => 14,
            PORT_IDW::I2C_MSSDA => 13,
            PORT_IDW::SSI0_CLK => 12,
            PORT_IDW::SSI0_FSS => 11,
            PORT_IDW::SSI0_TX => 10,
            PORT_IDW::SSI0_RX => 9,
            PORT_IDW::AUX_IO => 8,
            PORT_IDW::AON_CLK32K => 7,
            PORT_IDW::GPIO => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PORT_IDW<'a> {
    w: &'a mut W,
}
impl<'a> _PORT_IDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PORT_IDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "RF Core SMI Command Link In"]
    #[inline]
    pub fn rfc_smi_cl_in(self) -> &'a mut W {
        self.variant(PORT_IDW::RFC_SMI_CL_IN)
    }
    #[doc = "RF Core SMI Command Link Out"]
    #[inline]
    pub fn rfc_smi_cl_out(self) -> &'a mut W {
        self.variant(PORT_IDW::RFC_SMI_CL_OUT)
    }
    #[doc = "RF Core SMI Data Link In"]
    #[inline]
    pub fn rfc_smi_dl_in(self) -> &'a mut W {
        self.variant(PORT_IDW::RFC_SMI_DL_IN)
    }
    #[doc = "RF Core SMI Data Link Out"]
    #[inline]
    pub fn rfc_smi_dl_out(self) -> &'a mut W {
        self.variant(PORT_IDW::RFC_SMI_DL_OUT)
    }
    #[doc = "RF Core Data In 1"]
    #[inline]
    pub fn rfc_gpi1(self) -> &'a mut W {
        self.variant(PORT_IDW::RFC_GPI1)
    }
    #[doc = "RF Core Data In 0"]
    #[inline]
    pub fn rfc_gpi0(self) -> &'a mut W {
        self.variant(PORT_IDW::RFC_GPI0)
    }
    #[doc = "RF Core Data Out 3"]
    #[inline]
    pub fn rfc_gpo3(self) -> &'a mut W {
        self.variant(PORT_IDW::RFC_GPO3)
    }
    #[doc = "RF Core Data Out 2"]
    #[inline]
    pub fn rfc_gpo2(self) -> &'a mut W {
        self.variant(PORT_IDW::RFC_GPO2)
    }
    #[doc = "RF Core Data Out 1"]
    #[inline]
    pub fn rfc_gpo1(self) -> &'a mut W {
        self.variant(PORT_IDW::RFC_GPO1)
    }
    #[doc = "RF Core Data Out 0"]
    #[inline]
    pub fn rfc_gpo0(self) -> &'a mut W {
        self.variant(PORT_IDW::RFC_GPO0)
    }
    #[doc = "RF Core Trace"]
    #[inline]
    pub fn rfc_trc(self) -> &'a mut W {
        self.variant(PORT_IDW::RFC_TRC)
    }
    #[doc = "I2S MCLK"]
    #[inline]
    pub fn i2s_mclk(self) -> &'a mut W {
        self.variant(PORT_IDW::I2S_MCLK)
    }
    #[doc = "I2S BCLK"]
    #[inline]
    pub fn i2s_bclk(self) -> &'a mut W {
        self.variant(PORT_IDW::I2S_BCLK)
    }
    #[doc = "I2S WCLK"]
    #[inline]
    pub fn i2s_wclk(self) -> &'a mut W {
        self.variant(PORT_IDW::I2S_WCLK)
    }
    #[doc = "I2S Data 1"]
    #[inline]
    pub fn i2s_ad1(self) -> &'a mut W {
        self.variant(PORT_IDW::I2S_AD1)
    }
    #[doc = "I2S Data 0"]
    #[inline]
    pub fn i2s_ad0(self) -> &'a mut W {
        self.variant(PORT_IDW::I2S_AD0)
    }
    #[doc = "SSI1 CLK"]
    #[inline]
    pub fn ssi1_clk(self) -> &'a mut W {
        self.variant(PORT_IDW::SSI1_CLK)
    }
    #[doc = "SSI1 FSS"]
    #[inline]
    pub fn ssi1_fss(self) -> &'a mut W {
        self.variant(PORT_IDW::SSI1_FSS)
    }
    #[doc = "SSI1 TX"]
    #[inline]
    pub fn ssi1_tx(self) -> &'a mut W {
        self.variant(PORT_IDW::SSI1_TX)
    }
    #[doc = "SSI1 RX"]
    #[inline]
    pub fn ssi1_rx(self) -> &'a mut W {
        self.variant(PORT_IDW::SSI1_RX)
    }
    #[doc = "CPU SWV"]
    #[inline]
    pub fn cpu_swv(self) -> &'a mut W {
        self.variant(PORT_IDW::CPU_SWV)
    }
    #[doc = "PORT EVENT 7 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline]
    pub fn port_event7(self) -> &'a mut W {
        self.variant(PORT_IDW::PORT_EVENT7)
    }
    #[doc = "PORT EVENT 6 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline]
    pub fn port_event6(self) -> &'a mut W {
        self.variant(PORT_IDW::PORT_EVENT6)
    }
    #[doc = "PORT EVENT 5 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline]
    pub fn port_event5(self) -> &'a mut W {
        self.variant(PORT_IDW::PORT_EVENT5)
    }
    #[doc = "PORT EVENT 4 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline]
    pub fn port_event4(self) -> &'a mut W {
        self.variant(PORT_IDW::PORT_EVENT4)
    }
    #[doc = "PORT EVENT 3 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline]
    pub fn port_event3(self) -> &'a mut W {
        self.variant(PORT_IDW::PORT_EVENT3)
    }
    #[doc = "PORT EVENT 2 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline]
    pub fn port_event2(self) -> &'a mut W {
        self.variant(PORT_IDW::PORT_EVENT2)
    }
    #[doc = "PORT EVENT 1 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline]
    pub fn port_event1(self) -> &'a mut W {
        self.variant(PORT_IDW::PORT_EVENT1)
    }
    #[doc = "PORT EVENT 0 Can be used as a general purpose IO event by selecting it via registers in the EVENT module, e.g. EVENT:GPT0ACAPTSEL.EV, EVENT:UDMACH14BSEL.EV, etc"]
    #[inline]
    pub fn port_event0(self) -> &'a mut W {
        self.variant(PORT_IDW::PORT_EVENT0)
    }
    #[doc = "UART0 RTS"]
    #[inline]
    pub fn uart0_rts(self) -> &'a mut W {
        self.variant(PORT_IDW::UART0_RTS)
    }
    #[doc = "UART0 CTS"]
    #[inline]
    pub fn uart0_cts(self) -> &'a mut W {
        self.variant(PORT_IDW::UART0_CTS)
    }
    #[doc = "UART0 TX"]
    #[inline]
    pub fn uart0_tx(self) -> &'a mut W {
        self.variant(PORT_IDW::UART0_TX)
    }
    #[doc = "UART0 RX"]
    #[inline]
    pub fn uart0_rx(self) -> &'a mut W {
        self.variant(PORT_IDW::UART0_RX)
    }
    #[doc = "I2C Clock"]
    #[inline]
    pub fn i2c_msscl(self) -> &'a mut W {
        self.variant(PORT_IDW::I2C_MSSCL)
    }
    #[doc = "I2C Data"]
    #[inline]
    pub fn i2c_mssda(self) -> &'a mut W {
        self.variant(PORT_IDW::I2C_MSSDA)
    }
    #[doc = "SSI0 CLK"]
    #[inline]
    pub fn ssi0_clk(self) -> &'a mut W {
        self.variant(PORT_IDW::SSI0_CLK)
    }
    #[doc = "SSI0 FSS"]
    #[inline]
    pub fn ssi0_fss(self) -> &'a mut W {
        self.variant(PORT_IDW::SSI0_FSS)
    }
    #[doc = "SSI0 TX"]
    #[inline]
    pub fn ssi0_tx(self) -> &'a mut W {
        self.variant(PORT_IDW::SSI0_TX)
    }
    #[doc = "SSI0 RX"]
    #[inline]
    pub fn ssi0_rx(self) -> &'a mut W {
        self.variant(PORT_IDW::SSI0_RX)
    }
    #[doc = "AUX IO"]
    #[inline]
    pub fn aux_io(self) -> &'a mut W {
        self.variant(PORT_IDW::AUX_IO)
    }
    #[doc = "AON 32 KHz clock (SCLK_LF)"]
    #[inline]
    pub fn aon_clk32k(self) -> &'a mut W {
        self.variant(PORT_IDW::AON_CLK32K)
    }
    #[doc = "General Purpose IO"]
    #[inline]
    pub fn gpio(self) -> &'a mut W {
        self.variant(PORT_IDW::GPIO)
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
    #[doc = "Bit 31 - 31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved31(&self) -> RESERVED31R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED31R { bits }
    }
    #[doc = "Bit 30 - 30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable"]
    #[inline]
    pub fn hyst_en(&self) -> HYST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HYST_ENR { bits }
    }
    #[doc = "Bit 29 - 29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline]
    pub fn ie(&self) -> IER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IER { bits }
    }
    #[doc = "Bits 27:28 - 28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline]
    pub fn wu_cfg(&self) -> WU_CFGR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WU_CFGR { bits }
    }
    #[doc = "Bits 24:26 - 26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline]
    pub fn iomode(&self) -> IOMODER {
        IOMODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 19:23 - 23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved19(&self) -> RESERVED19R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED19R { bits }
    }
    #[doc = "Bit 18 - 18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)"]
    #[inline]
    pub fn edge_irq_en(&self) -> EDGE_IRQ_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EDGE_IRQ_ENR { bits }
    }
    #[doc = "Bits 16:17 - 17:16\\] Enable generation of edge detection events on this IO"]
    #[inline]
    pub fn edge_det(&self) -> EDGE_DETR {
        EDGE_DETR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - 15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved15(&self) -> RESERVED15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED15R { bits }
    }
    #[doc = "Bits 13:14 - 14:13\\] Pull control"]
    #[inline]
    pub fn pull_ctl(&self) -> PULL_CTLR {
        PULL_CTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - 12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline]
    pub fn slew_red(&self) -> SLEW_REDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLEW_REDR { bits }
    }
    #[doc = "Bits 10:11 - 11:10\\] Selects IO current mode of this IO."]
    #[inline]
    pub fn iocurr(&self) -> IOCURRR {
        IOCURRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - 9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR"]
    #[inline]
    pub fn iostr(&self) -> IOSTRR {
        IOSTRR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bits 0:5 - 5:0\\] Selects usage for DIO30"]
    #[inline]
    pub fn port_id(&self) -> PORT_IDR {
        PORT_IDR::_from({
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
        W { bits: 24576 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - 31:31\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved31(&mut self) -> _RESERVED31W {
        _RESERVED31W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] 0: Input hysteresis disable 1: Input hysteresis enable"]
    #[inline]
    pub fn hyst_en(&mut self) -> _HYST_ENW {
        _HYST_ENW { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] 0: Input disabled 1: Input enabled Note: If IO is configured for AUX ie. PORT_ID = 0x08, the enable will be ignored."]
    #[inline]
    pub fn ie(&mut self) -> _IEW {
        _IEW { w: self }
    }
    #[doc = "Bits 27:28 - 28:27\\] If DIO is configured GPIO or non-AON peripheral signals, i.e. PORT_ID 0x00 or >0x08: 00: No wake-up 01: No wake-up 10: Wakes up from shutdown if this pad is going low. 11: Wakes up from shutdown if this pad is going high. If IO is configured for AON peripheral signals or AUX ie. PORT_ID 0x01-0x08, this register only sets wakeup enable or not. 00, 01: Wakeup disabled 10, 11: Wakeup enabled Polarity is controlled from AON registers. Note:When the MSB is set, the IOC will deactivate the output enable for the DIO."]
    #[inline]
    pub fn wu_cfg(&mut self) -> _WU_CFGW {
        _WU_CFGW { w: self }
    }
    #[doc = "Bits 24:26 - 26:24\\] IO Mode N/A for IO configured for AON periph. signals and AUX ie. PORT_ID 0x01-0x08 AUX has its own open_source/drain configuration. 0x2: Reserved. Undefined behavior. 0x3: Reserved. Undefined behavior."]
    #[inline]
    pub fn iomode(&mut self) -> _IOMODEW {
        _IOMODEW { w: self }
    }
    #[doc = "Bits 19:23 - 23:19\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved19(&mut self) -> _RESERVED19W {
        _RESERVED19W { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] 0: No interrupt generation 1: Enable interrupt generation for this IO (Only effective if EDGE_DET is enabled)"]
    #[inline]
    pub fn edge_irq_en(&mut self) -> _EDGE_IRQ_ENW {
        _EDGE_IRQ_ENW { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\] Enable generation of edge detection events on this IO"]
    #[inline]
    pub fn edge_det(&mut self) -> _EDGE_DETW {
        _EDGE_DETW { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved15(&mut self) -> _RESERVED15W {
        _RESERVED15W { w: self }
    }
    #[doc = "Bits 13:14 - 14:13\\] Pull control"]
    #[inline]
    pub fn pull_ctl(&mut self) -> _PULL_CTLW {
        _PULL_CTLW { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] 0: Normal slew rate 1: Enables reduced slew rate in output driver."]
    #[inline]
    pub fn slew_red(&mut self) -> _SLEW_REDW {
        _SLEW_REDW { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\] Selects IO current mode of this IO."]
    #[inline]
    pub fn iocurr(&mut self) -> _IOCURRW {
        _IOCURRW { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\] Select source for drive strength control of this IO. This setting controls the drive strength of the Low-Current (LC) mode. Higher drive strength can be selected in IOCURR"]
    #[inline]
    pub fn iostr(&mut self) -> _IOSTRW {
        _IOSTRW { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\] Selects usage for DIO30"]
    #[inline]
    pub fn port_id(&mut self) -> _PORT_IDW {
        _PORT_IDW { w: self }
    }
}
