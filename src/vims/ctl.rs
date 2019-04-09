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
pub struct STATS_CLRR {
    bits: bool,
}
impl STATS_CLRR {
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
pub struct STATS_ENR {
    bits: bool,
}
impl STATS_ENR {
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
pub struct DYN_CG_ENR {
    bits: bool,
}
impl DYN_CG_ENR {
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
    bits: u32,
}
impl RESERVED6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IDCODE_LB_DISR {
    bits: bool,
}
impl IDCODE_LB_DISR {
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
pub struct SYSBUS_LB_DISR {
    bits: bool,
}
impl SYSBUS_LB_DISR {
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
pub struct ARB_CFGR {
    bits: bool,
}
impl ARB_CFGR {
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
pub struct PREF_ENR {
    bits: bool,
}
impl PREF_ENR {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "VIMS Off mode"]
    OFF,
    #[doc = "VIMS Cache mode"]
    CACHE,
    #[doc = "VIMS GPRAM mode"]
    GPRAM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::OFF => 3,
            MODER::CACHE => 1,
            MODER::GPRAM => 0,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            3 => MODER::OFF,
            1 => MODER::CACHE,
            0 => MODER::GPRAM,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == MODER::OFF
    }
    #[doc = "Checks if the value of the field is `CACHE`"]
    #[inline]
    pub fn is_cache(&self) -> bool {
        *self == MODER::CACHE
    }
    #[doc = "Checks if the value of the field is `GPRAM`"]
    #[inline]
    pub fn is_gpram(&self) -> bool {
        *self == MODER::GPRAM
    }
}
#[doc = r" Proxy"]
pub struct _STATS_CLRW<'a> {
    w: &'a mut W,
}
impl<'a> _STATS_CLRW<'a> {
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
pub struct _STATS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _STATS_ENW<'a> {
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
pub struct _DYN_CG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DYN_CG_ENW<'a> {
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
pub struct _RESERVED6W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED6W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 8388607;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IDCODE_LB_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _IDCODE_LB_DISW<'a> {
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
pub struct _SYSBUS_LB_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSBUS_LB_DISW<'a> {
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
pub struct _ARB_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _ARB_CFGW<'a> {
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
pub struct _PREF_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PREF_ENW<'a> {
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "VIMS Off mode"]
    OFF,
    #[doc = "VIMS Cache mode"]
    CACHE,
    #[doc = "VIMS GPRAM mode"]
    GPRAM,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::OFF => 3,
            MODEW::CACHE => 1,
            MODEW::GPRAM => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "VIMS Off mode"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(MODEW::OFF)
    }
    #[doc = "VIMS Cache mode"]
    #[inline]
    pub fn cache(self) -> &'a mut W {
        self.variant(MODEW::CACHE)
    }
    #[doc = "VIMS GPRAM mode"]
    #[inline]
    pub fn gpram(self) -> &'a mut W {
        self.variant(MODEW::GPRAM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 31 - 31:31\\] Set this bit to clear statistic counters."]
    #[inline]
    pub fn stats_clr(&self) -> STATS_CLRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STATS_CLRR { bits }
    }
    #[doc = "Bit 30 - 30:30\\] Set this bit to enable statistic counters."]
    #[inline]
    pub fn stats_en(&self) -> STATS_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STATS_ENR { bits }
    }
    #[doc = "Bit 29 - 29:29\\] 0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
    #[inline]
    pub fn dyn_cg_en(&self) -> DYN_CG_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DYN_CG_ENR { bits }
    }
    #[doc = "Bits 6:28 - 28:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&self) -> RESERVED6R {
        let bits = {
            const MASK: u32 = 8388607;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED6R { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Icode/Dcode flash line buffer control 0: Enable 1: Disable"]
    #[inline]
    pub fn idcode_lb_dis(&self) -> IDCODE_LB_DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDCODE_LB_DISR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Sysbus flash line buffer control 0: Enable 1: Disable"]
    #[inline]
    pub fn sysbus_lb_dis(&self) -> SYSBUS_LB_DISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSBUS_LB_DISR { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration"]
    #[inline]
    pub fn arb_cfg(&self) -> ARB_CFGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ARB_CFGR { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Tag prefetch control 0: Disabled 1: Enabled"]
    #[inline]
    pub fn pref_en(&self) -> PREF_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PREF_ENR { bits }
    }
    #[doc = "Bits 0:1 - 1:0\\] VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1. Note: Transaction from CACHE mode to GPRAM mode should be done through OFF mode to minimize flash block delay."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bit 31 - 31:31\\] Set this bit to clear statistic counters."]
    #[inline]
    pub fn stats_clr(&mut self) -> _STATS_CLRW {
        _STATS_CLRW { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Set this bit to enable statistic counters."]
    #[inline]
    pub fn stats_en(&mut self) -> _STATS_ENW {
        _STATS_ENW { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] 0: The in-built clock gate functionality is bypassed. 1: The in-built clock gate functionality is enabled, automatically gating the clock when not needed."]
    #[inline]
    pub fn dyn_cg_en(&mut self) -> _DYN_CG_ENW {
        _DYN_CG_ENW { w: self }
    }
    #[doc = "Bits 6:28 - 28:6\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved6(&mut self) -> _RESERVED6W {
        _RESERVED6W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Icode/Dcode flash line buffer control 0: Enable 1: Disable"]
    #[inline]
    pub fn idcode_lb_dis(&mut self) -> _IDCODE_LB_DISW {
        _IDCODE_LB_DISW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Sysbus flash line buffer control 0: Enable 1: Disable"]
    #[inline]
    pub fn sysbus_lb_dis(&mut self) -> _SYSBUS_LB_DISW {
        _SYSBUS_LB_DISW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Icode/Dcode and sysbus arbitation scheme 0: Static arbitration (icode/docde > sysbus) 1: Round-robin arbitration"]
    #[inline]
    pub fn arb_cfg(&mut self) -> _ARB_CFGW {
        _ARB_CFGW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Tag prefetch control 0: Disabled 1: Enabled"]
    #[inline]
    pub fn pref_en(&mut self) -> _PREF_ENW {
        _PREF_ENW { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] VIMS mode request. Write accesses to this field will be blocked while STAT.MODE_CHANGING is set to 1. Note: Transaction from CACHE mode to GPRAM mode should be done through OFF mode to minimize flash block delay."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
