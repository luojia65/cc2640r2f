#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VECFLAGS {
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
pub struct RESERVED4R {
    bits: u32,
}
impl RESERVED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VEC3R {
    bits: bool,
}
impl VEC3R {
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
pub struct VEC2R {
    bits: bool,
}
impl VEC2R {
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
pub struct VEC1R {
    bits: bool,
}
impl VEC1R {
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
pub struct VEC0R {
    bits: bool,
}
impl VEC0R {
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
pub struct _RESERVED4W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VEC3W<'a> {
    w: &'a mut W,
}
impl<'a> _VEC3W<'a> {
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
pub struct _VEC2W<'a> {
    w: &'a mut W,
}
impl<'a> _VEC2W<'a> {
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
pub struct _VEC1W<'a> {
    w: &'a mut W,
}
impl<'a> _VEC1W<'a> {
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
pub struct _VEC0W<'a> {
    w: &'a mut W,
}
impl<'a> _VEC0W<'a> {
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
    #[doc = "Bits 4:31 - 31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: u32 = 268435455;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED4R { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
    #[inline]
    pub fn vec3(&self) -> VEC3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VEC3R { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
    #[inline]
    pub fn vec2(&self) -> VEC2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VEC2R { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
    #[inline]
    pub fn vec1(&self) -> VEC1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VEC1R { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
    #[inline]
    pub fn vec0(&self) -> VEC0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VEC0R { bits }
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
    #[doc = "Bits 4:31 - 31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Vector flag 3. The vector flag is set if the edge selected VECCFG1.VEC3_POL occurs on the event selected in VECCFG1.VEC3_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC3."]
    #[inline]
    pub fn vec3(&mut self) -> _VEC3W {
        _VEC3W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Vector flag 2. The vector flag is set if the edge selected VECCFG1.VEC2_POL occurs on the event selected in VECCFG1.VEC2_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC2."]
    #[inline]
    pub fn vec2(&mut self) -> _VEC2W {
        _VEC2W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Vector flag 1. The vector flag is set if the edge selected VECCFG0.VEC1_POL occurs on the event selected in VECCFG0.VEC1_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC1."]
    #[inline]
    pub fn vec1(&mut self) -> _VEC1W {
        _VEC1W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Vector flag 0. The vector flag is set if the edge selected VECCFG0.VEC0_POL occurs on the event selected in VECCFG0.VEC0_EV. The flag is cleared by writing a 0 to this bit, or (preferably) a 1 to VECFLAGSCLR.VEC0."]
    #[inline]
    pub fn vec0(&mut self) -> _VEC0W {
        _VEC0W { w: self }
    }
}
