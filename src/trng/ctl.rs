#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
pub struct STARTUP_CYCLESR {
    bits: u16,
}
impl STARTUP_CYCLESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
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
pub struct TRNG_ENR {
    bits: bool,
}
impl TRNG_ENR {
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
pub struct NO_LFSR_FBR {
    bits: bool,
}
impl NO_LFSR_FBR {
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
pub struct TEST_MODER {
    bits: bool,
}
impl TEST_MODER {
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
pub struct _STARTUP_CYCLESW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTUP_CYCLESW<'a> {
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
pub struct _RESERVED11W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRNG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TRNG_ENW<'a> {
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
pub struct _RESERVED3W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NO_LFSR_FBW<'a> {
    w: &'a mut W,
}
impl<'a> _NO_LFSR_FBW<'a> {
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
pub struct _TEST_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEST_MODEW<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\] This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored."]
    #[inline]
    pub fn startup_cycles(&self) -> STARTUP_CYCLESR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        STARTUP_CYCLESR { bits }
    }
    #[doc = "Bits 11:15 - 15:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved11(&self) -> RESERVED11R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED11R { bits }
    }
    #[doc = "Bit 10 - 10:10\\] 0: Forces all TRNG logic back into the idle state immediately. 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES."]
    #[inline]
    pub fn trng_en(&self) -> TRNG_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRNG_ENR { bits }
    }
    #[doc = "Bits 3:9 - 9:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&self) -> RESERVED3R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] 1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously. This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes"]
    #[inline]
    pub fn no_lfsr_fb(&self) -> NO_LFSR_FBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NO_LFSR_FBR { bits }
    }
    #[doc = "Bit 1 - 1:1\\] 1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access) and keeps IRQFLAGSTAT.NEED_CLOCK at '1'. This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control."]
    #[inline]
    pub fn test_mode(&self) -> TEST_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEST_MODER { bits }
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:31 - 31:16\\] This field determines the number of samples (between 2^8 and 2^24) taken to gather entropy from the FROs during startup. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while TRNG_EN is 0. If 1 an update will be ignored."]
    #[inline]
    pub fn startup_cycles(&mut self) -> _STARTUP_CYCLESW {
        _STARTUP_CYCLESW { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved11(&mut self) -> _RESERVED11W {
        _RESERVED11W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] 0: Forces all TRNG logic back into the idle state immediately. 1: Starts TRNG, gathering entropy from the FROs for the number of samples determined by STARTUP_CYCLES."]
    #[inline]
    pub fn trng_en(&mut self) -> _TRNG_ENW {
        _TRNG_ENW { w: self }
    }
    #[doc = "Bits 3:9 - 9:3\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved3(&mut self) -> _RESERVED3W {
        _RESERVED3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] 1: Remove XNOR feedback from the main LFSR, converting it into a normal shift register for the XOR-ed outputs of the FROs (shifting data in on the LSB side). A '1' also forces the LFSR to sample continuously. This bit can only be set to '1' when TEST_MODE is also set to '1' and should not be used for other than test purposes"]
    #[inline]
    pub fn no_lfsr_fb(&mut self) -> _NO_LFSR_FBW {
        _NO_LFSR_FBW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] 1: Enables access to the TESTCNT and LFSR0/LFSR1/LFSR2 registers (the latter are automatically cleared before enabling access) and keeps IRQFLAGSTAT.NEED_CLOCK at '1'. This bit shall not be used unless you need to change the LFSR seed prior to creating a new random value. All other testing is done external to register control."]
    #[inline]
    pub fn test_mode(&mut self) -> _TEST_MODEW {
        _TEST_MODEW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved0(&mut self) -> _RESERVED0W {
        _RESERVED0W { w: self }
    }
}
