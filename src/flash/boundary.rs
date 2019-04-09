#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BOUNDARY {
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
pub struct DISROW0R {
    bits: bool,
}
impl DISROW0R {
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
pub struct SPARER {
    bits: bool,
}
impl SPARER {
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
pub struct EFC_SELF_TEST_ERRORR {
    bits: bool,
}
impl EFC_SELF_TEST_ERRORR {
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
pub struct EFC_INSTRUCTION_INFOR {
    bits: bool,
}
impl EFC_INSTRUCTION_INFOR {
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
pub struct EFC_INSTRUCTION_ERRORR {
    bits: bool,
}
impl EFC_INSTRUCTION_ERRORR {
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
pub struct EFC_AUTOLOAD_ERRORR {
    bits: bool,
}
impl EFC_AUTOLOAD_ERRORR {
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
pub struct OUTPUTENABLER {
    bits: u8,
}
impl OUTPUTENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_ECC_SELF_TEST_ENR {
    bits: bool,
}
impl SYS_ECC_SELF_TEST_ENR {
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
pub struct SYS_ECC_OVERRIDE_ENR {
    bits: bool,
}
impl SYS_ECC_OVERRIDE_ENR {
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
pub struct EFC_FDIR {
    bits: bool,
}
impl EFC_FDIR {
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
pub struct SYS_DIEID_AUTOLOAD_ENR {
    bits: bool,
}
impl SYS_DIEID_AUTOLOAD_ENR {
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
pub struct SYS_REPAIR_ENR {
    bits: u8,
}
impl SYS_REPAIR_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SYS_WS_READ_STATESR {
    bits: u8,
}
impl SYS_WS_READ_STATESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INPUTENABLER {
    bits: u8,
}
impl INPUTENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISROW0W<'a> {
    w: &'a mut W,
}
impl<'a> _DISROW0W<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _SPAREW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EFC_SELF_TEST_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _EFC_SELF_TEST_ERRORW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EFC_INSTRUCTION_INFOW<'a> {
    w: &'a mut W,
}
impl<'a> _EFC_INSTRUCTION_INFOW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EFC_INSTRUCTION_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _EFC_INSTRUCTION_ERRORW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EFC_AUTOLOAD_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _EFC_AUTOLOAD_ERRORW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OUTPUTENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTPUTENABLEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_ECC_SELF_TEST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_ECC_SELF_TEST_ENW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_ECC_OVERRIDE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_ECC_OVERRIDE_ENW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EFC_FDIW<'a> {
    w: &'a mut W,
}
impl<'a> _EFC_FDIW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_DIEID_AUTOLOAD_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_DIEID_AUTOLOAD_ENW<'a> {
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
pub struct _SYS_REPAIR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_REPAIR_ENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SYS_WS_READ_STATESW<'a> {
    w: &'a mut W,
}
impl<'a> _SYS_WS_READ_STATESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INPUTENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUTENABLEW<'a> {
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
    #[doc = "Bits 24:31 - 31:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved24(&self) -> RESERVED24R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED24R { bits }
    }
    #[doc = "Bit 23 - 23:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn disrow0(&self) -> DISROW0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISROW0R { bits }
    }
    #[doc = "Bit 22 - 22:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn spare(&self) -> SPARER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPARER { bits }
    }
    #[doc = "Bit 21 - 21:21\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_self_test_error(&self) -> EFC_SELF_TEST_ERRORR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFC_SELF_TEST_ERRORR { bits }
    }
    #[doc = "Bit 20 - 20:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_instruction_info(&self) -> EFC_INSTRUCTION_INFOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFC_INSTRUCTION_INFOR { bits }
    }
    #[doc = "Bit 19 - 19:19\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_instruction_error(&self) -> EFC_INSTRUCTION_ERRORR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFC_INSTRUCTION_ERRORR { bits }
    }
    #[doc = "Bit 18 - 18:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_autoload_error(&self) -> EFC_AUTOLOAD_ERRORR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFC_AUTOLOAD_ERRORR { bits }
    }
    #[doc = "Bits 14:17 - 17:14\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn outputenable(&self) -> OUTPUTENABLER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OUTPUTENABLER { bits }
    }
    #[doc = "Bit 13 - 13:13\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_ecc_self_test_en(&self) -> SYS_ECC_SELF_TEST_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_ECC_SELF_TEST_ENR { bits }
    }
    #[doc = "Bit 12 - 12:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_ecc_override_en(&self) -> SYS_ECC_OVERRIDE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_ECC_OVERRIDE_ENR { bits }
    }
    #[doc = "Bit 11 - 11:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_fdi(&self) -> EFC_FDIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFC_FDIR { bits }
    }
    #[doc = "Bit 10 - 10:10\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_dieid_autoload_en(&self) -> SYS_DIEID_AUTOLOAD_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYS_DIEID_AUTOLOAD_ENR { bits }
    }
    #[doc = "Bits 8:9 - 9:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_repair_en(&self) -> SYS_REPAIR_ENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_REPAIR_ENR { bits }
    }
    #[doc = "Bits 4:7 - 7:4\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_ws_read_states(&self) -> SYS_WS_READ_STATESR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SYS_WS_READ_STATESR { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn inputenable(&self) -> INPUTENABLER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INPUTENABLER { bits }
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
    #[doc = "Bits 24:31 - 31:24\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn reserved24(&mut self) -> _RESERVED24W {
        _RESERVED24W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn disrow0(&mut self) -> _DISROW0W {
        _DISROW0W { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn spare(&mut self) -> _SPAREW {
        _SPAREW { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_self_test_error(&mut self) -> _EFC_SELF_TEST_ERRORW {
        _EFC_SELF_TEST_ERRORW { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_instruction_info(&mut self) -> _EFC_INSTRUCTION_INFOW {
        _EFC_INSTRUCTION_INFOW { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_instruction_error(&mut self) -> _EFC_INSTRUCTION_ERRORW {
        _EFC_INSTRUCTION_ERRORW { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_autoload_error(&mut self) -> _EFC_AUTOLOAD_ERRORW {
        _EFC_AUTOLOAD_ERRORW { w: self }
    }
    #[doc = "Bits 14:17 - 17:14\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn outputenable(&mut self) -> _OUTPUTENABLEW {
        _OUTPUTENABLEW { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_ecc_self_test_en(&mut self) -> _SYS_ECC_SELF_TEST_ENW {
        _SYS_ECC_SELF_TEST_ENW { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_ecc_override_en(&mut self) -> _SYS_ECC_OVERRIDE_ENW {
        _SYS_ECC_OVERRIDE_ENW { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn efc_fdi(&mut self) -> _EFC_FDIW {
        _EFC_FDIW { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_dieid_autoload_en(&mut self) -> _SYS_DIEID_AUTOLOAD_ENW {
        _SYS_DIEID_AUTOLOAD_ENW { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_repair_en(&mut self) -> _SYS_REPAIR_ENW {
        _SYS_REPAIR_ENW { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn sys_ws_read_states(&mut self) -> _SYS_WS_READ_STATESW {
        _SYS_WS_READ_STATESW { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Internal. Only to be used through TI provided API."]
    #[inline]
    pub fn inputenable(&mut self) -> _INPUTENABLEW {
        _INPUTENABLEW { w: self }
    }
}
