#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RESETCTL {
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
pub struct SYSRESETR {
    bits: bool,
}
impl SYSRESETR {
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
pub struct RESERVED26R {
    bits: u8,
}
impl RESERVED26R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BOOT_DET_1_CLRR {
    bits: bool,
}
impl BOOT_DET_1_CLRR {
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
pub struct BOOT_DET_0_CLRR {
    bits: bool,
}
impl BOOT_DET_0_CLRR {
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
pub struct RESERVED18R {
    bits: u8,
}
impl RESERVED18R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BOOT_DET_1_SETR {
    bits: bool,
}
impl BOOT_DET_1_SETR {
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
pub struct BOOT_DET_0_SETR {
    bits: bool,
}
impl BOOT_DET_0_SETR {
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
pub struct WU_FROM_SDR {
    bits: bool,
}
impl WU_FROM_SDR {
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
pub struct GPIO_WU_FROM_SDR {
    bits: bool,
}
impl GPIO_WU_FROM_SDR {
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
pub struct BOOT_DET_1R {
    bits: bool,
}
impl BOOT_DET_1R {
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
pub struct BOOT_DET_0R {
    bits: bool,
}
impl BOOT_DET_0R {
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
pub struct VDDS_LOSS_EN_OVRR {
    bits: bool,
}
impl VDDS_LOSS_EN_OVRR {
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
pub struct VDDR_LOSS_EN_OVRR {
    bits: bool,
}
impl VDDR_LOSS_EN_OVRR {
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
pub struct VDD_LOSS_EN_OVRR {
    bits: bool,
}
impl VDD_LOSS_EN_OVRR {
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
pub struct RESERVED8R {
    bits: bool,
}
impl RESERVED8R {
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
pub struct VDDS_LOSS_ENR {
    bits: bool,
}
impl VDDS_LOSS_ENR {
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
pub struct VDDR_LOSS_ENR {
    bits: bool,
}
impl VDDR_LOSS_ENR {
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
pub struct VDD_LOSS_ENR {
    bits: bool,
}
impl VDD_LOSS_ENR {
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
pub struct CLK_LOSS_ENR {
    bits: bool,
}
impl CLK_LOSS_ENR {
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
#[doc = "Possible values of the field `RESET_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_SRCR {
    #[doc = "Software reset via PRCM warm reset request"]
    WARMRESET,
    #[doc = "Software reset via SYSRESET register"]
    SYSRESET,
    #[doc = "Clock loss detect"]
    CLK_LOSS,
    #[doc = "Brown out detect on VDDR"]
    VDDR_LOSS,
    #[doc = "Brown out detect on VDD"]
    VDD_LOSS,
    #[doc = "Brown out detect on VDDS"]
    VDDS_LOSS,
    #[doc = "Reset pin"]
    PIN_RESET,
    #[doc = "Power on reset"]
    PWR_ON,
}
impl RESET_SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESET_SRCR::WARMRESET => 7,
            RESET_SRCR::SYSRESET => 6,
            RESET_SRCR::CLK_LOSS => 5,
            RESET_SRCR::VDDR_LOSS => 4,
            RESET_SRCR::VDD_LOSS => 3,
            RESET_SRCR::VDDS_LOSS => 2,
            RESET_SRCR::PIN_RESET => 1,
            RESET_SRCR::PWR_ON => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESET_SRCR {
        match value {
            7 => RESET_SRCR::WARMRESET,
            6 => RESET_SRCR::SYSRESET,
            5 => RESET_SRCR::CLK_LOSS,
            4 => RESET_SRCR::VDDR_LOSS,
            3 => RESET_SRCR::VDD_LOSS,
            2 => RESET_SRCR::VDDS_LOSS,
            1 => RESET_SRCR::PIN_RESET,
            0 => RESET_SRCR::PWR_ON,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WARMRESET`"]
    #[inline]
    pub fn is_warmreset(&self) -> bool {
        *self == RESET_SRCR::WARMRESET
    }
    #[doc = "Checks if the value of the field is `SYSRESET`"]
    #[inline]
    pub fn is_sysreset(&self) -> bool {
        *self == RESET_SRCR::SYSRESET
    }
    #[doc = "Checks if the value of the field is `CLK_LOSS`"]
    #[inline]
    pub fn is_clk_loss(&self) -> bool {
        *self == RESET_SRCR::CLK_LOSS
    }
    #[doc = "Checks if the value of the field is `VDDR_LOSS`"]
    #[inline]
    pub fn is_vddr_loss(&self) -> bool {
        *self == RESET_SRCR::VDDR_LOSS
    }
    #[doc = "Checks if the value of the field is `VDD_LOSS`"]
    #[inline]
    pub fn is_vdd_loss(&self) -> bool {
        *self == RESET_SRCR::VDD_LOSS
    }
    #[doc = "Checks if the value of the field is `VDDS_LOSS`"]
    #[inline]
    pub fn is_vdds_loss(&self) -> bool {
        *self == RESET_SRCR::VDDS_LOSS
    }
    #[doc = "Checks if the value of the field is `PIN_RESET`"]
    #[inline]
    pub fn is_pin_reset(&self) -> bool {
        *self == RESET_SRCR::PIN_RESET
    }
    #[doc = "Checks if the value of the field is `PWR_ON`"]
    #[inline]
    pub fn is_pwr_on(&self) -> bool {
        *self == RESET_SRCR::PWR_ON
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED0R {
    bits: bool,
}
impl RESERVED0R {
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
pub struct _SYSRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSRESETW<'a> {
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
pub struct _RESERVED26W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED26W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_DET_1_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_DET_1_CLRW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_DET_0_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_DET_0_CLRW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED18W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED18W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_DET_1_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_DET_1_SETW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_DET_0_SETW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_DET_0_SETW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WU_FROM_SDW<'a> {
    w: &'a mut W,
}
impl<'a> _WU_FROM_SDW<'a> {
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
pub struct _GPIO_WU_FROM_SDW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_WU_FROM_SDW<'a> {
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
pub struct _BOOT_DET_1W<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_DET_1W<'a> {
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
pub struct _BOOT_DET_0W<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_DET_0W<'a> {
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
pub struct _VDDS_LOSS_EN_OVRW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDS_LOSS_EN_OVRW<'a> {
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
pub struct _VDDR_LOSS_EN_OVRW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDR_LOSS_EN_OVRW<'a> {
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
pub struct _VDD_LOSS_EN_OVRW<'a> {
    w: &'a mut W,
}
impl<'a> _VDD_LOSS_EN_OVRW<'a> {
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
pub struct _RESERVED8W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED8W<'a> {
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
pub struct _VDDS_LOSS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDS_LOSS_ENW<'a> {
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
pub struct _VDDR_LOSS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDR_LOSS_ENW<'a> {
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
pub struct _VDD_LOSS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _VDD_LOSS_ENW<'a> {
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
pub struct _CLK_LOSS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_LOSS_ENW<'a> {
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
#[doc = "Values that can be written to the field `RESET_SRC`"]
pub enum RESET_SRCW {
    #[doc = "Software reset via PRCM warm reset request"]
    WARMRESET,
    #[doc = "Software reset via SYSRESET register"]
    SYSRESET,
    #[doc = "Clock loss detect"]
    CLK_LOSS,
    #[doc = "Brown out detect on VDDR"]
    VDDR_LOSS,
    #[doc = "Brown out detect on VDD"]
    VDD_LOSS,
    #[doc = "Brown out detect on VDDS"]
    VDDS_LOSS,
    #[doc = "Reset pin"]
    PIN_RESET,
    #[doc = "Power on reset"]
    PWR_ON,
}
impl RESET_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESET_SRCW::WARMRESET => 7,
            RESET_SRCW::SYSRESET => 6,
            RESET_SRCW::CLK_LOSS => 5,
            RESET_SRCW::VDDR_LOSS => 4,
            RESET_SRCW::VDD_LOSS => 3,
            RESET_SRCW::VDDS_LOSS => 2,
            RESET_SRCW::PIN_RESET => 1,
            RESET_SRCW::PWR_ON => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESET_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESET_SRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Software reset via PRCM warm reset request"]
    #[inline]
    pub fn warmreset(self) -> &'a mut W {
        self.variant(RESET_SRCW::WARMRESET)
    }
    #[doc = "Software reset via SYSRESET register"]
    #[inline]
    pub fn sysreset(self) -> &'a mut W {
        self.variant(RESET_SRCW::SYSRESET)
    }
    #[doc = "Clock loss detect"]
    #[inline]
    pub fn clk_loss(self) -> &'a mut W {
        self.variant(RESET_SRCW::CLK_LOSS)
    }
    #[doc = "Brown out detect on VDDR"]
    #[inline]
    pub fn vddr_loss(self) -> &'a mut W {
        self.variant(RESET_SRCW::VDDR_LOSS)
    }
    #[doc = "Brown out detect on VDD"]
    #[inline]
    pub fn vdd_loss(self) -> &'a mut W {
        self.variant(RESET_SRCW::VDD_LOSS)
    }
    #[doc = "Brown out detect on VDDS"]
    #[inline]
    pub fn vdds_loss(self) -> &'a mut W {
        self.variant(RESET_SRCW::VDDS_LOSS)
    }
    #[doc = "Reset pin"]
    #[inline]
    pub fn pin_reset(self) -> &'a mut W {
        self.variant(RESET_SRCW::PIN_RESET)
    }
    #[doc = "Power on reset"]
    #[inline]
    pub fn pwr_on(self) -> &'a mut W {
        self.variant(RESET_SRCW::PWR_ON)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED0W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED0W<'a> {
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
    #[doc = "Bit 31 - 31:31\\] Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC."]
    #[inline]
    pub fn sysreset(&self) -> SYSRESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSRESETR { bits }
    }
    #[doc = "Bits 26:30 - 30:26\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved26(&self) -> RESERVED26R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED26R { bits }
    }
    #[doc = "Bit 25 - 25:25\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn boot_det_1_clr(&self) -> BOOT_DET_1_CLRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOOT_DET_1_CLRR { bits }
    }
    #[doc = "Bit 24 - 24:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn boot_det_0_clr(&self) -> BOOT_DET_0_CLRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOOT_DET_0_CLRR { bits }
    }
    #[doc = "Bits 18:23 - 23:18\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved18(&self) -> RESERVED18R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED18R { bits }
    }
    #[doc = "Bit 17 - 17:17\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn boot_det_1_set(&self) -> BOOT_DET_1_SETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOOT_DET_1_SETR { bits }
    }
    #[doc = "Bit 16 - 16:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn boot_det_0_set(&self) -> BOOT_DET_0_SETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOOT_DET_0_SETR { bits }
    }
    #[doc = "Bit 15 - 15:15\\] A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to \\[IOC:IOCFGn,.WU_CFG\\] for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset"]
    #[inline]
    pub fn wu_from_sd(&self) -> WU_FROM_SDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WU_FROM_SDR { bits }
    }
    #[doc = "Bit 14 - 14:14\\] A wakeup from SHUTDOWN on an IO event has occurred Please refer to \\[IOC:IOCFGn,.WU_CFG\\] for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset"]
    #[inline]
    pub fn gpio_wu_from_sd(&self) -> GPIO_WU_FROM_SDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GPIO_WU_FROM_SDR { bits }
    }
    #[doc = "Bit 13 - 13:13\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn boot_det_1(&self) -> BOOT_DET_1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOOT_DET_1R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn boot_det_0(&self) -> BOOT_DET_0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOOT_DET_0R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] Override of VDDS_LOSS_EN 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN=1 1: Brown out detect of VDDS generates system reset (regardless of VDDS_LOSS_EN) This bit can be locked"]
    #[inline]
    pub fn vdds_loss_en_ovr(&self) -> VDDS_LOSS_EN_OVRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDDS_LOSS_EN_OVRR { bits }
    }
    #[doc = "Bit 10 - 10:10\\] Override of VDDR_LOSS_EN 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN=1 1: Brown out detect of VDDR generates system reset (regardless of VDDR_LOSS_EN) This bit can be locked"]
    #[inline]
    pub fn vddr_loss_en_ovr(&self) -> VDDR_LOSS_EN_OVRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDDR_LOSS_EN_OVRR { bits }
    }
    #[doc = "Bit 9 - 9:9\\] Override of VDD_LOSS_EN 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN=1 1: Brown out detect of VDD generates system reset (regardless of VDD_LOSS_EN) This bit can be locked"]
    #[inline]
    pub fn vdd_loss_en_ovr(&self) -> VDD_LOSS_EN_OVRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDD_LOSS_EN_OVRR { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&self) -> RESERVED8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED8R { bits }
    }
    #[doc = "Bit 7 - 7:7\\] Controls reset generation in case VDDS is lost 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN_OVR=1 1: Brown out detect of VDDS generates system reset"]
    #[inline]
    pub fn vdds_loss_en(&self) -> VDDS_LOSS_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDDS_LOSS_ENR { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Controls reset generation in case VDDR is lost 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN_OVR=1 1: Brown out detect of VDDR generates system reset"]
    #[inline]
    pub fn vddr_loss_en(&self) -> VDDR_LOSS_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDDR_LOSS_ENR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Controls reset generation in case VDD is lost 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN_OVR=1 1: Brown out detect of VDD generates system reset"]
    #[inline]
    pub fn vdd_loss_en(&self) -> VDD_LOSS_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VDD_LOSS_ENR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Controls reset generation in case SCLK_LF is lost. (provided that clock loss detection is enabled by DDI_0_OSC:CTL0.CLK_LOSS_EN) Note: Clock loss reset generation must be disabled before SCLK_LF clock source is changed in DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL and remain disabled untill the change is confirmed in DDI_0_OSC:STAT0.SCLK_LF_SRC. Failure to do so may result in a spurious system reset. Clock loss reset generation can be disabled through this bitfield or by clearing DDI_0_OSC:CTL0.CLK_LOSS_EN 0: Clock loss is ignored 1: Clock loss generates system reset"]
    #[inline]
    pub fn clk_loss_en(&self) -> CLK_LOSS_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLK_LOSS_ENR { bits }
    }
    #[doc = "Bits 1:3 - 3:1\\] Shows the source of the last system reset: Occurrence of one of the reset sources may trigger several other reset sources as essential parts of the system are undergoing reset. This field will report the root cause of the reset (not the other resets that are consequence of the system reset). To support this feature the actual register is not captured before the reset source being released. If a new reset source is triggered, in a window of four 32 kHz periods after the previous has been released, this register may indicate Power on reset as source."]
    #[inline]
    pub fn reset_src(&self) -> RESET_SRCR {
        RESET_SRCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 0 - 0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED0R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 224 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - 31:31\\] Cold reset register. Writing 1 to this bitfield will reset the entire chip and cause boot code to run again. 0: No effect 1: Generate system reset. Appears as SYSRESET in RESET_SRC."]
    #[inline]
    pub fn sysreset(&mut self) -> _SYSRESETW {
        _SYSRESETW { w: self }
    }
    #[doc = "Bits 26:30 - 30:26\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved26(&mut self) -> _RESERVED26W {
        _RESERVED26W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn boot_det_1_clr(&mut self) -> _BOOT_DET_1_CLRW {
        _BOOT_DET_1_CLRW { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn boot_det_0_clr(&mut self) -> _BOOT_DET_0_CLRW {
        _BOOT_DET_0_CLRW { w: self }
    }
    #[doc = "Bits 18:23 - 23:18\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved18(&mut self) -> _RESERVED18W {
        _RESERVED18W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn boot_det_1_set(&mut self) -> _BOOT_DET_1_SETW {
        _BOOT_DET_1_SETW { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn boot_det_0_set(&mut self) -> _BOOT_DET_0_SETW {
        _BOOT_DET_0_SETW { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] A Wakeup from SHUTDOWN on an IO event has occurred, or a wakeup from SHUTDOWN has occurred as a result of the debugger being attached.. (TCK pin being forced low) Please refer to \\[IOC:IOCFGn,.WU_CFG\\] for configuring the IO's as wakeup sources. 0: Wakeup occurred from cold reset or brown out as seen in RESET_SRC 1: A wakeup has occurred from SHUTDOWN Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset"]
    #[inline]
    pub fn wu_from_sd(&mut self) -> _WU_FROM_SDW {
        _WU_FROM_SDW { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] A wakeup from SHUTDOWN on an IO event has occurred Please refer to \\[IOC:IOCFGn,.WU_CFG\\] for configuring the IO's as wakeup sources. 0: The wakeup did not occur from SHUTDOWN on an IO event 1: A wakeup from SHUTDOWN occurred from an IO event The case where WU_FROM_SD is asserted but this bitfield is not asserted will only occur in a debug session. The boot code will not proceed with wakeup from SHUTDOWN procedure until this bitfield is asserted as well. Note: This flag can not be cleared and will therefor remain valid untill poweroff/reset"]
    #[inline]
    pub fn gpio_wu_from_sd(&mut self) -> _GPIO_WU_FROM_SDW {
        _GPIO_WU_FROM_SDW { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn boot_det_1(&mut self) -> _BOOT_DET_1W {
        _BOOT_DET_1W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn boot_det_0(&mut self) -> _BOOT_DET_0W {
        _BOOT_DET_0W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Override of VDDS_LOSS_EN 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN=1 1: Brown out detect of VDDS generates system reset (regardless of VDDS_LOSS_EN) This bit can be locked"]
    #[inline]
    pub fn vdds_loss_en_ovr(&mut self) -> _VDDS_LOSS_EN_OVRW {
        _VDDS_LOSS_EN_OVRW { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Override of VDDR_LOSS_EN 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN=1 1: Brown out detect of VDDR generates system reset (regardless of VDDR_LOSS_EN) This bit can be locked"]
    #[inline]
    pub fn vddr_loss_en_ovr(&mut self) -> _VDDR_LOSS_EN_OVRW {
        _VDDR_LOSS_EN_OVRW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Override of VDD_LOSS_EN 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN=1 1: Brown out detect of VDD generates system reset (regardless of VDD_LOSS_EN) This bit can be locked"]
    #[inline]
    pub fn vdd_loss_en_ovr(&mut self) -> _VDD_LOSS_EN_OVRW {
        _VDD_LOSS_EN_OVRW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved8(&mut self) -> _RESERVED8W {
        _RESERVED8W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Controls reset generation in case VDDS is lost 0: Brown out detect of VDDS is ignored, unless VDDS_LOSS_EN_OVR=1 1: Brown out detect of VDDS generates system reset"]
    #[inline]
    pub fn vdds_loss_en(&mut self) -> _VDDS_LOSS_ENW {
        _VDDS_LOSS_ENW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Controls reset generation in case VDDR is lost 0: Brown out detect of VDDR is ignored, unless VDDR_LOSS_EN_OVR=1 1: Brown out detect of VDDR generates system reset"]
    #[inline]
    pub fn vddr_loss_en(&mut self) -> _VDDR_LOSS_ENW {
        _VDDR_LOSS_ENW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Controls reset generation in case VDD is lost 0: Brown out detect of VDD is ignored, unless VDD_LOSS_EN_OVR=1 1: Brown out detect of VDD generates system reset"]
    #[inline]
    pub fn vdd_loss_en(&mut self) -> _VDD_LOSS_ENW {
        _VDD_LOSS_ENW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Controls reset generation in case SCLK_LF is lost. (provided that clock loss detection is enabled by DDI_0_OSC:CTL0.CLK_LOSS_EN) Note: Clock loss reset generation must be disabled before SCLK_LF clock source is changed in DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL and remain disabled untill the change is confirmed in DDI_0_OSC:STAT0.SCLK_LF_SRC. Failure to do so may result in a spurious system reset. Clock loss reset generation can be disabled through this bitfield or by clearing DDI_0_OSC:CTL0.CLK_LOSS_EN 0: Clock loss is ignored 1: Clock loss generates system reset"]
    #[inline]
    pub fn clk_loss_en(&mut self) -> _CLK_LOSS_ENW {
        _CLK_LOSS_ENW { w: self }
    }
    #[doc = "Bits 1:3 - 3:1\\] Shows the source of the last system reset: Occurrence of one of the reset sources may trigger several other reset sources as essential parts of the system are undergoing reset. This field will report the root cause of the reset (not the other resets that are consequence of the system reset). To support this feature the actual register is not captured before the reset source being released. If a new reset source is triggered, in a window of four 32 kHz periods after the previous has been released, this register may indicate Power on reset as source."]
    #[inline]
    pub fn reset_src(&mut self) -> _RESET_SRCW {
        _RESET_SRCW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved0(&mut self) -> _RESERVED0W {
        _RESERVED0W { w: self }
    }
}
