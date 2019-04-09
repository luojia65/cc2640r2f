#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SIZE_AND_DIS_FLAGS {
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
pub struct SIZE_OF_CCFGR {
    bits: u16,
}
impl SIZE_OF_CCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DISABLE_FLAGSR {
    bits: u16,
}
impl DISABLE_FLAGSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIS_TCXOR {
    bits: bool,
}
impl DIS_TCXOR {
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
pub struct DIS_GPRAMR {
    bits: bool,
}
impl DIS_GPRAMR {
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
pub struct DIS_ALT_DCDC_SETTINGR {
    bits: bool,
}
impl DIS_ALT_DCDC_SETTINGR {
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
pub struct DIS_XOSC_OVRR {
    bits: bool,
}
impl DIS_XOSC_OVRR {
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
pub struct _SIZE_OF_CCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _SIZE_OF_CCFGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_FLAGSW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_FLAGSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIS_TCXOW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_TCXOW<'a> {
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
pub struct _DIS_GPRAMW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_GPRAMW<'a> {
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
pub struct _DIS_ALT_DCDC_SETTINGW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_ALT_DCDC_SETTINGW<'a> {
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
pub struct _DIS_XOSC_OVRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIS_XOSC_OVRW<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\] Total size of CCFG in bytes."]
    #[inline]
    pub fn size_of_ccfg(&self) -> SIZE_OF_CCFGR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SIZE_OF_CCFGR { bits }
    }
    #[doc = "Bits 4:15 - 15:4\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline]
    pub fn disable_flags(&self) -> DISABLE_FLAGSR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DISABLE_FLAGSR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
    #[inline]
    pub fn dis_tcxo(&self) -> DIS_TCXOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_TCXOR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
    #[inline]
    pub fn dis_gpram(&self) -> DIS_GPRAMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_GPRAMR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline]
    pub fn dis_alt_dcdc_setting(&self) -> DIS_ALT_DCDC_SETTINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_ALT_DCDC_SETTINGR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
    #[inline]
    pub fn dis_xosc_ovr(&self) -> DIS_XOSC_OVRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIS_XOSC_OVRR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:31 - 31:16\\] Total size of CCFG in bytes."]
    #[inline]
    pub fn size_of_ccfg(&mut self) -> _SIZE_OF_CCFGW {
        _SIZE_OF_CCFGW { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\] Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline]
    pub fn disable_flags(&mut self) -> _DISABLE_FLAGSW {
        _DISABLE_FLAGSW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
    #[inline]
    pub fn dis_tcxo(&mut self) -> _DIS_TCXOW {
        _DIS_TCXOW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
    #[inline]
    pub fn dis_gpram(&mut self) -> _DIS_GPRAMW {
        _DIS_GPRAMW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline]
    pub fn dis_alt_dcdc_setting(&mut self) -> _DIS_ALT_DCDC_SETTINGW {
        _DIS_ALT_DCDC_SETTINGW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
    #[inline]
    pub fn dis_xosc_ovr(&mut self) -> _DIS_XOSC_OVRW {
        _DIS_XOSC_OVRW { w: self }
    }
}
