#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AIRCR {
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
pub struct VECTKEYR {
    bits: u16,
}
impl VECTKEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `ENDIANESS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIANESSR {
    #[doc = "Big endian"]
    BIG,
    #[doc = "Little endian"]
    LITTLE,
}
impl ENDIANESSR {
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
            ENDIANESSR::BIG => true,
            ENDIANESSR::LITTLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDIANESSR {
        match value {
            true => ENDIANESSR::BIG,
            false => ENDIANESSR::LITTLE,
        }
    }
    #[doc = "Checks if the value of the field is `BIG`"]
    #[inline]
    pub fn is_big(&self) -> bool {
        *self == ENDIANESSR::BIG
    }
    #[doc = "Checks if the value of the field is `LITTLE`"]
    #[inline]
    pub fn is_little(&self) -> bool {
        *self == ENDIANESSR::LITTLE
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED11R {
    bits: u8,
}
impl RESERVED11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRIGROUPR {
    bits: u8,
}
impl PRIGROUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED3R {
    bits: u8,
}
impl RESERVED3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYSRESETREQR {
    bits: bool,
}
impl SYSRESETREQR {
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
pub struct VECTCLRACTIVER {
    bits: bool,
}
impl VECTCLRACTIVER {
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
pub struct VECTRESETR {
    bits: bool,
}
impl VECTRESETR {
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
pub struct _VECTKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTKEYW<'a> {
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
#[doc = "Values that can be written to the field `ENDIANESS`"]
pub enum ENDIANESSW {
    #[doc = "Big endian"]
    BIG,
    #[doc = "Little endian"]
    LITTLE,
}
impl ENDIANESSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDIANESSW::BIG => true,
            ENDIANESSW::LITTLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDIANESSW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDIANESSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDIANESSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Big endian"]
    #[inline]
    pub fn big(self) -> &'a mut W {
        self.variant(ENDIANESSW::BIG)
    }
    #[doc = "Little endian"]
    #[inline]
    pub fn little(self) -> &'a mut W {
        self.variant(ENDIANESSW::LITTLE)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED11W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRIGROUPW<'a> {
    w: &'a mut W,
}
impl<'a> _PRIGROUPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
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
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYSRESETREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSRESETREQW<'a> {
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
pub struct _VECTCLRACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTCLRACTIVEW<'a> {
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
pub struct _VECTRESETW<'a> {
    w: &'a mut W,
}
impl<'a> _VECTRESETW<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\] Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
    #[inline]
    pub fn vectkey(&self) -> VECTKEYR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VECTKEYR { bits }
    }
    #[doc = "Bit 15 - 15:15\\] Data endianness bit"]
    #[inline]
    pub fn endianess(&self) -> ENDIANESSR {
        ENDIANESSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:14 - 14:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved11(&self) -> RESERVED11R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED11R { bits }
    }
    #[doc = "Bits 8:10 - 10:8\\] Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
    #[inline]
    pub fn prigroup(&self) -> PRIGROUPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRIGROUPR { bits }
    }
    #[doc = "Bits 3:7 - 7:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
    #[inline]
    pub fn sysresetreq(&self) -> SYSRESETREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSRESETREQR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
    #[inline]
    pub fn vectclractive(&self) -> VECTCLRACTIVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VECTCLRACTIVER { bits }
    }
    #[doc = "Bit 0 - 0:0\\] System Reset bit. Resets the system, with the exception of debug components. This bit is reserved for debug use and can be written to 1 only when the core is halted. The bit self-clears. Writing this bit to 1 while core is not halted may result in unpredictable behavior."]
    #[inline]
    pub fn vectreset(&self) -> VECTRESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VECTRESETR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4194631680 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:31 - 31:16\\] Register key. Writing to this register (AIRCR) requires 0x05FA in VECTKEY. Otherwise the write value is ignored. Read always returns 0xFA05."]
    #[inline]
    pub fn vectkey(&mut self) -> _VECTKEYW {
        _VECTKEYW { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Data endianness bit"]
    #[inline]
    pub fn endianess(&mut self) -> _ENDIANESSW {
        _ENDIANESSW { w: self }
    }
    #[doc = "Bits 11:14 - 14:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved11(&mut self) -> _RESERVED11W {
        _RESERVED11W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\] Interrupt priority grouping field. This field is a binary point position indicator for creating subpriorities for exceptions that share the same pre-emption level. It divides the PRI_n field in the Interrupt Priority Registers (NVIC_IPR0, NVIC_IPR1,..., and NVIC_IPR8) into a pre-emption level and a subpriority level. The binary point is a left-of value. This means that the PRIGROUP value represents a point starting at the left of the Least Significant Bit (LSB). The lowest value might not be 0 depending on the number of bits allocated for priorities, and implementation choices."]
    #[inline]
    pub fn prigroup(&mut self) -> _PRIGROUPW {
        _PRIGROUPW { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Requests a warm reset. Setting this bit does not prevent Halting Debug from running."]
    #[inline]
    pub fn sysresetreq(&mut self) -> _SYSRESETREQW {
        _SYSRESETREQW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Clears all active state information for active NMI, fault, and interrupts. It is the responsibility of the application to reinitialize the stack. This bit is for returning to a known state during debug. The bit self-clears. IPSR is not cleared by this operation. So, if used by an application, it must only be used at the base level of activation, or within a system handler whose active bit can be set."]
    #[inline]
    pub fn vectclractive(&mut self) -> _VECTCLRACTIVEW {
        _VECTCLRACTIVEW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] System Reset bit. Resets the system, with the exception of debug components. This bit is reserved for debug use and can be written to 1 only when the core is halted. The bit self-clears. Writing this bit to 1 while core is not halted may result in unpredictable behavior."]
    #[inline]
    pub fn vectreset(&mut self) -> _VECTRESETW {
        _VECTRESETW { w: self }
    }
}
