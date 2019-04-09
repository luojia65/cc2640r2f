#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
pub struct RESERVED10R {
    bits: u32,
}
impl RESERVED10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct STKALIGNR {
    bits: bool,
}
impl STKALIGNR {
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
pub struct BFHFNMIGNR {
    bits: bool,
}
impl BFHFNMIGNR {
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
pub struct RESERVED5R {
    bits: u8,
}
impl RESERVED5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIV_0_TRPR {
    bits: bool,
}
impl DIV_0_TRPR {
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
pub struct UNALIGN_TRPR {
    bits: bool,
}
impl UNALIGN_TRPR {
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
pub struct RESERVED2R {
    bits: bool,
}
impl RESERVED2R {
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
pub struct USERSETMPENDR {
    bits: bool,
}
impl USERSETMPENDR {
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
pub struct NONBASETHREDENAR {
    bits: bool,
}
impl NONBASETHREDENAR {
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
pub struct _RESERVED10W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4194303;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STKALIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _STKALIGNW<'a> {
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
pub struct _BFHFNMIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _BFHFNMIGNW<'a> {
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
pub struct _RESERVED5W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED5W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIV_0_TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _DIV_0_TRPW<'a> {
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
#[doc = r" Proxy"]
pub struct _UNALIGN_TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _UNALIGN_TRPW<'a> {
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
pub struct _RESERVED2W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED2W<'a> {
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
pub struct _USERSETMPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _USERSETMPENDW<'a> {
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
pub struct _NONBASETHREDENAW<'a> {
    w: &'a mut W,
}
impl<'a> _NONBASETHREDENAW<'a> {
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
    #[doc = "Bits 10:31 - 31:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved10(&self) -> RESERVED10R {
        let bits = {
            const MASK: u32 = 4194303;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED10R { bits }
    }
    #[doc = "Bit 9 - 9:9\\] Stack alignment bit. 0: Only 4-byte alignment is guaranteed for the SP used prior to the exception on exception entry. 1: On exception entry, the SP used prior to the exception is adjusted to be 8-byte aligned and the context to restore it is saved. The SP is restored on the associated exception return."]
    #[inline]
    pub fn stkalign(&self) -> STKALIGNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STKALIGNR { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the HardFault, NMI, and FAULTMASK escalated handlers: 0: Data BusFaults caused by load and store instructions cause a lock-up 1: Data BusFaults caused by load and store instructions are ignored. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect problems."]
    #[inline]
    pub fn bfhfnmign(&self) -> BFHFNMIGNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BFHFNMIGNR { bits }
    }
    #[doc = "Bits 5:7 - 7:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved5(&self) -> RESERVED5R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED5R { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0: Do not trap divide by 0. In this mode, a divide by zero returns a quotient of 0. 1: Trap divide by 0. The relevant Usage Fault Status Register bit is CFSR.DIVBYZERO."]
    #[inline]
    pub fn div_0_trp(&self) -> DIV_0_TRPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIV_0_TRPR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Enables unaligned access traps: 0: Do not trap unaligned halfword and word accesses 1: Trap unaligned halfword and word accesses. The relevant Usage Fault Status Register bit is CFSR.UNALIGNED. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault regardless of the value in UNALIGN_TRP."]
    #[inline]
    pub fn unalign_trp(&self) -> UNALIGN_TRPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNALIGN_TRPR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&self) -> RESERVED2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Enables unprivileged software access to STIR: 0: User code is not allowed to write to the Software Trigger Interrupt register (STIR). 1: User code can write the Software Trigger Interrupt register (STIR) to trigger (pend) a Main exception, which is associated with the Main stack pointer."]
    #[inline]
    pub fn usersetmpend(&self) -> USERSETMPENDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USERSETMPENDR { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Indicates how the processor enters Thread mode: 0: Processor can enter Thread mode only when no exception is active. 1: Processor can enter Thread mode from any level using the appropriate return value (EXC_RETURN). Exception returns occur when one of the following instructions loads a value of 0xFXXXXXXX into the PC while in Handler mode: - POP/LDM which includes loading the PC. - LDR with PC as a destination. - BX with any register. The value written to the PC is intercepted and is referred to as the EXC_RETURN value."]
    #[inline]
    pub fn nonbasethredena(&self) -> NONBASETHREDENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NONBASETHREDENAR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 512 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 10:31 - 31:10\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved10(&mut self) -> _RESERVED10W {
        _RESERVED10W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Stack alignment bit. 0: Only 4-byte alignment is guaranteed for the SP used prior to the exception on exception entry. 1: On exception entry, the SP used prior to the exception is adjusted to be 8-byte aligned and the context to restore it is saved. The SP is restored on the associated exception return."]
    #[inline]
    pub fn stkalign(&mut self) -> _STKALIGNW {
        _STKALIGNW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions. This applies to the HardFault, NMI, and FAULTMASK escalated handlers: 0: Data BusFaults caused by load and store instructions cause a lock-up 1: Data BusFaults caused by load and store instructions are ignored. Set this bit to 1 only when the handler and its data are in absolutely safe memory. The normal use of this bit is to probe system devices and bridges to detect problems."]
    #[inline]
    pub fn bfhfnmign(&mut self) -> _BFHFNMIGNW {
        _BFHFNMIGNW { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved5(&mut self) -> _RESERVED5W {
        _RESERVED5W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0: 0: Do not trap divide by 0. In this mode, a divide by zero returns a quotient of 0. 1: Trap divide by 0. The relevant Usage Fault Status Register bit is CFSR.DIVBYZERO."]
    #[inline]
    pub fn div_0_trp(&mut self) -> _DIV_0_TRPW {
        _DIV_0_TRPW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Enables unaligned access traps: 0: Do not trap unaligned halfword and word accesses 1: Trap unaligned halfword and word accesses. The relevant Usage Fault Status Register bit is CFSR.UNALIGNED. If this bit is set to 1, an unaligned access generates a UsageFault. Unaligned LDM, STM, LDRD, and STRD instructions always fault regardless of the value in UNALIGN_TRP."]
    #[inline]
    pub fn unalign_trp(&mut self) -> _UNALIGN_TRPW {
        _UNALIGN_TRPW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved2(&mut self) -> _RESERVED2W {
        _RESERVED2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Enables unprivileged software access to STIR: 0: User code is not allowed to write to the Software Trigger Interrupt register (STIR). 1: User code can write the Software Trigger Interrupt register (STIR) to trigger (pend) a Main exception, which is associated with the Main stack pointer."]
    #[inline]
    pub fn usersetmpend(&mut self) -> _USERSETMPENDW {
        _USERSETMPENDW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Indicates how the processor enters Thread mode: 0: Processor can enter Thread mode only when no exception is active. 1: Processor can enter Thread mode from any level using the appropriate return value (EXC_RETURN). Exception returns occur when one of the following instructions loads a value of 0xFXXXXXXX into the PC while in Handler mode: - POP/LDM which includes loading the PC. - LDR with PC as a destination. - BX with any register. The value written to the PC is intercepted and is referred to as the EXC_RETURN value."]
    #[inline]
    pub fn nonbasethredena(&mut self) -> _NONBASETHREDENAW {
        _NONBASETHREDENAW { w: self }
    }
}
