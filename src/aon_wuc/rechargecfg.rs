#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RECHARGECFG {
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
pub struct ADAPTIVE_ENR {
    bits: bool,
}
impl ADAPTIVE_ENR {
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
pub struct RESERVED24R {
    bits: u8,
}
impl RESERVED24R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct C2R {
    bits: u8,
}
impl C2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct C1R {
    bits: u8,
}
impl C1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAX_PER_MR {
    bits: u8,
}
impl MAX_PER_MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAX_PER_ER {
    bits: u8,
}
impl MAX_PER_ER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PER_MR {
    bits: u8,
}
impl PER_MR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PER_ER {
    bits: u8,
}
impl PER_ER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ADAPTIVE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADAPTIVE_ENW<'a> {
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
pub struct _RESERVED24W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED24W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C2W<'a> {
    w: &'a mut W,
}
impl<'a> _C2W<'a> {
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
pub struct _C1W<'a> {
    w: &'a mut W,
}
impl<'a> _C1W<'a> {
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
pub struct _MAX_PER_MW<'a> {
    w: &'a mut W,
}
impl<'a> _MAX_PER_MW<'a> {
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
pub struct _MAX_PER_EW<'a> {
    w: &'a mut W,
}
impl<'a> _MAX_PER_EW<'a> {
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
pub struct _PER_MW<'a> {
    w: &'a mut W,
}
impl<'a> _PER_MW<'a> {
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
pub struct _PER_EW<'a> {
    w: &'a mut W,
}
impl<'a> _PER_EW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 31 - 31:31\\] Enable adaptive recharge Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0"]
    #[inline]
    pub fn adaptive_en(&self) -> ADAPTIVE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADAPTIVE_ENR { bits }
    }
    #[doc = "Bits 24:30 - 30:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&self) -> RESERVED24R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED24R { bits }
    }
    #[doc = "Bits 20:23 - 23:20\\] Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C2 is 2 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
    #[inline]
    pub fn c2(&self) -> C2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        C2R { bits }
    }
    #[doc = "Bits 16:19 - 19:16\\] Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C1 is 1 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
    #[inline]
    pub fn c1(&self) -> C1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        C1R { bits }
    }
    #[doc = "Bits 11:15 - 15:11\\] This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the mantissa of MAXCYCLES"]
    #[inline]
    pub fn max_per_m(&self) -> MAX_PER_MR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAX_PER_MR { bits }
    }
    #[doc = "Bits 8:10 - 10:8\\] This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the exponent MAXCYCLES"]
    #[inline]
    pub fn max_per_e(&self) -> MAX_PER_ER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAX_PER_ER { bits }
    }
    #[doc = "Bits 3:7 - 7:3\\] Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Mantissa of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
    #[inline]
    pub fn per_m(&self) -> PER_MR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PER_MR { bits }
    }
    #[doc = "Bits 0:2 - 2:0\\] Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Exponent of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
    #[inline]
    pub fn per_e(&self) -> PER_ER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PER_ER { bits }
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
    #[doc = "Bit 31 - 31:31\\] Enable adaptive recharge Note: Recharge can be turned completely of by setting MAX_PER_E=7 and MAX_PER_M=31 and this bitfield to 0"]
    #[inline]
    pub fn adaptive_en(&mut self) -> _ADAPTIVE_ENW {
        _ADAPTIVE_ENW { w: self }
    }
    #[doc = "Bits 24:30 - 30:24\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved24(&mut self) -> _RESERVED24W {
        _RESERVED24W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\] Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C2 is 2 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
    #[inline]
    pub fn c2(&mut self) -> _C2W {
        _C2W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\] Gain factor for adaptive recharge algorithm period_new=period * ( 1+/-(2^-C1+2^-C2) ) Valid values for C1 is 1 to 10 Note: Rounding may cause adaptive recharge not to start for very small values of both Gain and Initial period. Criteria for algorithm to start is MAX(PERIOD*2^-C1,PERIOD*2^-C2) >= 1"]
    #[inline]
    pub fn c1(&mut self) -> _C1W {
        _C1W { w: self }
    }
    #[doc = "Bits 11:15 - 15:11\\] This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the mantissa of MAXCYCLES"]
    #[inline]
    pub fn max_per_m(&mut self) -> _MAX_PER_MW {
        _MAX_PER_MW { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\] This register defines the maximum period that the recharge algorithm can take, i.e. it defines the maximum number of cycles between 2 recharges. The maximum number of cycles is specified with a 5 bit mantissa and 3 bit exponent: MAXCYCLES=(MAX_PER_M*16+15)*2^MAX_PER_E This field sets the exponent MAXCYCLES"]
    #[inline]
    pub fn max_per_e(&mut self) -> _MAX_PER_EW {
        _MAX_PER_EW { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\] Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Mantissa of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
    #[inline]
    pub fn per_m(&mut self) -> _PER_MW {
        _PER_MW { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\] Number of 32 KHz clocks between activation of recharge controller For recharge algorithm, PERIOD is the initial period when entering powerdown mode. The adaptive recharge algorithm will not change this register PERIOD will effectively be a 16 bit value coded in a 5 bit mantissa and 3 bit exponent: This field sets the Exponent of the Period. PERIOD=(PER_M*16+15)*2^PER_E"]
    #[inline]
    pub fn per_e(&mut self) -> _PER_EW {
        _PER_EW { w: self }
    }
}
