#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AUXCTL {
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
pub struct RESET_REQR {
    bits: bool,
}
impl RESET_REQR {
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
pub struct RESERVED3R {
    bits: u32,
}
impl RESERVED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SCE_RUN_ENR {
    bits: bool,
}
impl SCE_RUN_ENR {
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
pub struct SWEVR {
    bits: bool,
}
impl SWEVR {
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
pub struct AUX_FORCE_ONR {
    bits: bool,
}
impl AUX_FORCE_ONR {
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
pub struct _RESET_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _RESET_REQW<'a> {
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
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCE_RUN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCE_RUN_ENW<'a> {
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
pub struct _SWEVW<'a> {
    w: &'a mut W,
}
impl<'a> _SWEVW<'a> {
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
pub struct _AUX_FORCE_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _AUX_FORCE_ONW<'a> {
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
    #[doc = "Bit 31 - 31:31\\] Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again. 0: AUX reset pin will be deasserted 1: AUX reset pin will be asserted"]
    #[inline]
    pub fn reset_req(&self) -> RESET_REQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESET_REQR { bits }
    }
    #[doc = "Bits 3:30 - 30:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u32 = 268435455;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set. Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. ( AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off) 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0 1: AUX_SCE execution is enabled."]
    #[inline]
    pub fn sce_run_en(&self) -> SCE_RUN_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SCE_RUN_ENR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW. This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register. Reading 0 means that there is no outstanding software event for AUX. Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX."]
    #[inline]
    pub fn swev(&self) -> SWEVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWEVR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL. Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode. MCU must set this bit in order to access the AUX peripherals. The AUX domain status can be read from PWRSTAT.AUX_PD_ON 0: AUX is allowed to Power Off, Power Down or Disconnect. 1: AUX Power OFF, Power Down or Disconnect requests will be overruled"]
    #[inline]
    pub fn aux_force_on(&self) -> AUX_FORCE_ONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUX_FORCE_ONR { bits }
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
    #[doc = "Bit 31 - 31:31\\] Reset request for AUX. Writing 1 to this register will assert reset to AUX. The reset will be held until the bit is cleared again. 0: AUX reset pin will be deasserted 1: AUX reset pin will be asserted"]
    #[inline]
    pub fn reset_req(&mut self) -> _RESET_REQW {
        _RESET_REQW { w: self }
    }
    #[doc = "Bits 3:30 - 30:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Enables (1) or disables (0) AUX_SCE execution. AUX_SCE execution will begin when AUX Domain is powered and either this or AUX_SCE:CTL.CLK_EN is set. Setting this bit will assure that AUX_SCE execution starts as soon as AUX power domain is woken up. ( AUX_SCE:CTL.CLK_EN will be reset to 0 if AUX power domain has been off) 0: AUX_SCE execution will be disabled if AUX_SCE:CTL.CLK_EN is 0 1: AUX_SCE execution is enabled."]
    #[inline]
    pub fn sce_run_en(&mut self) -> _SCE_RUN_ENW {
        _SCE_RUN_ENW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Writing 1 sets the software event to the AUX domain, which can be read through AUX_WUC:WUEVFLAGS.AON_SW. This event is normally cleared by AUX_SCE through the AUX_WUC:WUEVCLR.AON_SW. It can also be cleared by writing 0 to this register. Reading 0 means that there is no outstanding software event for AUX. Note that it can take up to 1,5 SCLK_LF clock cycles to clear the event from AUX."]
    #[inline]
    pub fn swev(&mut self) -> _SWEVW {
        _SWEVW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Forces the AUX domain into active mode, overriding the requests from AUX_WUC:PWROFFREQ, AUX_WUC:PWRDWNREQ and AUX_WUC:MCUBUSCTL. Note that an ongoing AUX_WUC:PWROFFREQ will complete before this bit will set the AUX domain into active mode. MCU must set this bit in order to access the AUX peripherals. The AUX domain status can be read from PWRSTAT.AUX_PD_ON 0: AUX is allowed to Power Off, Power Down or Disconnect. 1: AUX Power OFF, Power Down or Disconnect requests will be overruled"]
    #[inline]
    pub fn aux_force_on(&mut self) -> _AUX_FORCE_ONW {
        _AUX_FORCE_ONW { w: self }
    }
}
