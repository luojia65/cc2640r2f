#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FUNCTION1 {
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
pub struct RESERVED25R {
    bits: u8,
}
impl RESERVED25R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MATCHEDR {
    bits: bool,
}
impl MATCHEDR {
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
pub struct RESERVED20R {
    bits: u8,
}
impl RESERVED20R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATAVADDR1R {
    bits: u8,
}
impl DATAVADDR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATAVADDR0R {
    bits: u8,
}
impl DATAVADDR0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATAVSIZER {
    bits: u8,
}
impl DATAVSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LNK1ENAR {
    bits: bool,
}
impl LNK1ENAR {
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
pub struct DATAVMATCHR {
    bits: bool,
}
impl DATAVMATCHR {
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
#[doc = r" Value of the field"]
pub struct EMITRANGER {
    bits: bool,
}
impl EMITRANGER {
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
pub struct RESERVED4R {
    bits: bool,
}
impl RESERVED4R {
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
pub struct FUNCTIONR {
    bits: u8,
}
impl FUNCTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED25W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED25W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MATCHEDW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCHEDW<'a> {
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
pub struct _RESERVED20W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED20W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATAVADDR1W<'a> {
    w: &'a mut W,
}
impl<'a> _DATAVADDR1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATAVADDR0W<'a> {
    w: &'a mut W,
}
impl<'a> _DATAVADDR0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATAVSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAVSIZEW<'a> {
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
#[doc = r" Proxy"]
pub struct _LNK1ENAW<'a> {
    w: &'a mut W,
}
impl<'a> _LNK1ENAW<'a> {
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
pub struct _DATAVMATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAVMATCHW<'a> {
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
#[doc = r" Proxy"]
pub struct _EMITRANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMITRANGEW<'a> {
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
pub struct _RESERVED4W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED4W<'a> {
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
pub struct _FUNCTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _FUNCTIONW<'a> {
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
    #[doc = "Bits 25:31 - 31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved25(&self) -> RESERVED25R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED25R { bits }
    }
    #[doc = "Bit 24 - 24:24\\] This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline]
    pub fn matched(&self) -> MATCHEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MATCHEDR { bits }
    }
    #[doc = "Bits 20:23 - 23:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved20(&self) -> RESERVED20R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED20R { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
    #[inline]
    pub fn datavaddr1(&self) -> DATAVADDR1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATAVADDR1R { bits }
    }
    #[doc = "Bits 12:15 - 15:12\\] Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
    #[inline]
    pub fn datavaddr0(&self) -> DATAVADDR0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATAVADDR0R { bits }
    }
    #[doc = "Bits 10:11 - 11:10\\] Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
    #[inline]
    pub fn datavsize(&self) -> DATAVSIZER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATAVSIZER { bits }
    }
    #[doc = "Bit 9 - 9:9\\] Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)"]
    #[inline]
    pub fn lnk1ena(&self) -> LNK1ENAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LNK1ENAR { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
    #[inline]
    pub fn datavmatch(&self) -> DATAVMATCHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATAVMATCHR { bits }
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
    #[doc = "Bit 5 - 5:5\\] Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline]
    pub fn emitrange(&self) -> EMITRANGER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EMITRANGER { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESERVED4R { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline]
    pub fn function(&self) -> FUNCTIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FUNCTIONR { bits }
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
    #[doc = "Bits 25:31 - 31:25\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved25(&mut self) -> _RESERVED25W {
        _RESERVED25W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] This bit is set when the comparator matches, and indicates that the operation defined by FUNCTION has occurred since this bit was last read. This bit is cleared on read."]
    #[inline]
    pub fn matched(&mut self) -> _MATCHEDW {
        _MATCHEDW { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved20(&mut self) -> _RESERVED20W {
        _RESERVED20W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Identity of a second linked address comparator for data value matching when DATAVMATCH == 1 and LNK1ENA == 1."]
    #[inline]
    pub fn datavaddr1(&mut self) -> _DATAVADDR1W {
        _DATAVADDR1W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] Identity of a linked address comparator for data value matching when DATAVMATCH == 1."]
    #[inline]
    pub fn datavaddr0(&mut self) -> _DATAVADDR0W {
        _DATAVADDR0W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\] Defines the size of the data in the COMP1 register that is to be matched: 0x0: Byte 0x1: Halfword 0x2: Word 0x3: Unpredictable."]
    #[inline]
    pub fn datavsize(&mut self) -> _DATAVSIZEW {
        _DATAVSIZEW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Read only bit-field only supported in comparator 1. 0: DATAVADDR1 not supported 1: DATAVADDR1 supported (enabled)"]
    #[inline]
    pub fn lnk1ena(&mut self) -> _LNK1ENAW {
        _LNK1ENAW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Data match feature: 0: Perform address comparison 1: Perform data value compare. The comparators given by DATAVADDR0 and DATAVADDR1 provide the address for the data comparison. The FUNCTION setting for the comparators given by DATAVADDR0 and DATAVADDR1 are overridden and those comparators only provide the address match for the data comparison. This bit is only available in comparator 1."]
    #[inline]
    pub fn datavmatch(&mut self) -> _DATAVMATCHW {
        _DATAVMATCHW { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Emit range field. This bit permits emitting offset when range match occurs. PC sampling is not supported when emit range is enabled. This field only applies for: FUNCTION = 1, 2, 3, 12, 13, 14, and 15."]
    #[inline]
    pub fn emitrange(&mut self) -> _EMITRANGEW {
        _EMITRANGEW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Function settings: 0x0: Disabled 0x1: EMITRANGE = 0, sample and emit PC through ITM. EMITRANGE = 1, emit address offset through ITM 0x2: EMITRANGE = 0, emit data through ITM on read and write. EMITRANGE = 1, emit data and address offset through ITM on read or write. 0x3: EMITRANGE = 0, sample PC and data value through ITM on read or write. EMITRANGE = 1, emit address offset and data value through ITM on read or write. 0x4: Watchpoint on PC match. 0x5: Watchpoint on read. 0x6: Watchpoint on write. 0x7: Watchpoint on read or write. 0x8: ETM trigger on PC match 0x9: ETM trigger on read 0xA: ETM trigger on write 0xB: ETM trigger on read or write 0xC: EMITRANGE = 0, sample data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for read transfers 0xD: EMITRANGE = 0, sample data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) for write transfers 0xE: EMITRANGE = 0, sample PC + data for read transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for read transfers 0xF: EMITRANGE = 0, sample PC + data for write transfers. EMITRANGE = 1, sample Daddr (lower 16 bits) + data for write transfers Note 1: If the ETM is not fitted, then ETM trigger is not possible. Note 2: Data value is only sampled for accesses that do not fault (MPU or bus fault). The PC is sampled irrespective of any faults. The PC is only sampled for the first address of a burst. Note 3: FUNCTION is overridden for comparators given by DATAVADDR0 and DATAVADDR1 if DATAVMATCH is also set. The comparators given by DATAVADDR0 and DATAVADDR1 can then only perform address comparator matches for comparator 1 data matches. Note 4: If the data matching functionality is not included during implementation it is not possible to set DATAVADDR0, DATAVADDR1, or DATAVMATCH. This means that the data matching functionality is not available in the implementation. Test the availability of data matching by writing and reading DATAVMATCH. If it is not settable then data matching is unavailable. Note 5: PC match is not recommended for watchpoints because it stops after the instruction. It mainly guards and triggers the ETM."]
    #[inline]
    pub fn function(&mut self) -> _FUNCTIONW {
        _FUNCTIONW { w: self }
    }
}
