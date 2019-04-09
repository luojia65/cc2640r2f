#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WARMRESET {
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
pub struct WR_TO_PINRESETR {
    bits: bool,
}
impl WR_TO_PINRESETR {
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
pub struct LOCKUP_STATR {
    bits: bool,
}
impl LOCKUP_STATR {
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
pub struct WDT_STATR {
    bits: bool,
}
impl WDT_STATR {
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
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WR_TO_PINRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _WR_TO_PINRESETW<'a> {
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
pub struct _LOCKUP_STATW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKUP_STATW<'a> {
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
pub struct _WDT_STATW<'a> {
    w: &'a mut W,
}
impl<'a> _WDT_STATW<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] 0: No action 1: A warm system reset event triggered by the below listed sources will result in an emulated pin reset. Warm reset sources included: ICEPick sysreset System CPU reset request, CPU_SCS:AIRCR.SYSRESETREQ System CPU Lockup WDT timeout An active ICEPick block system reset will gate all sources except ICEPick sysreset SW can read AON_SYSCTL:RESETCTL.RESET_SRC to find the source of the last reset resulting in a full power up sequence. WARMRESET in this register is set in the scenario that WR_TO_PINRESET=1 and one of the above listed sources is triggered."]
    #[inline]
    pub fn wr_to_pinreset(&self) -> WR_TO_PINRESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WR_TO_PINRESETR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] 0: No registred event 1: A system CPU LOCKUP event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline]
    pub fn lockup_stat(&self) -> LOCKUP_STATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LOCKUP_STATR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] 0: No registered event 1: A WDT event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline]
    pub fn wdt_stat(&self) -> WDT_STATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDT_STATR { bits }
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
    #[doc = "Bits 3:31 - 31:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 0: No action 1: A warm system reset event triggered by the below listed sources will result in an emulated pin reset. Warm reset sources included: ICEPick sysreset System CPU reset request, CPU_SCS:AIRCR.SYSRESETREQ System CPU Lockup WDT timeout An active ICEPick block system reset will gate all sources except ICEPick sysreset SW can read AON_SYSCTL:RESETCTL.RESET_SRC to find the source of the last reset resulting in a full power up sequence. WARMRESET in this register is set in the scenario that WR_TO_PINRESET=1 and one of the above listed sources is triggered."]
    #[inline]
    pub fn wr_to_pinreset(&mut self) -> _WR_TO_PINRESETW {
        _WR_TO_PINRESETW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 0: No registred event 1: A system CPU LOCKUP event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline]
    pub fn lockup_stat(&mut self) -> _LOCKUP_STATW {
        _LOCKUP_STATW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] 0: No registered event 1: A WDT event has occured since last SW clear of the register. A read of this register clears both WDT_STAT and LOCKUP_STAT."]
    #[inline]
    pub fn wdt_stat(&mut self) -> _WDT_STATW {
        _WDT_STATW { w: self }
    }
}
