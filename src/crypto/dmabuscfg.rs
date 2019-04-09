#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMABUSCFG {
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
pub struct RESERVED16R {
    bits: u16,
}
impl RESERVED16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `AHB_MST1_BURST_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_BURST_SIZER {
    #[doc = "64 bytes "]
    _64_BYTE,
    #[doc = "32 bytes "]
    _32_BYTE,
    #[doc = "16 bytes "]
    _16_BYTE,
    #[doc = "8 bytes "]
    _8_BYTE,
    #[doc = "4 bytes"]
    _4_BYTE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AHB_MST1_BURST_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AHB_MST1_BURST_SIZER::_64_BYTE => 6,
            AHB_MST1_BURST_SIZER::_32_BYTE => 5,
            AHB_MST1_BURST_SIZER::_16_BYTE => 4,
            AHB_MST1_BURST_SIZER::_8_BYTE => 3,
            AHB_MST1_BURST_SIZER::_4_BYTE => 2,
            AHB_MST1_BURST_SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AHB_MST1_BURST_SIZER {
        match value {
            6 => AHB_MST1_BURST_SIZER::_64_BYTE,
            5 => AHB_MST1_BURST_SIZER::_32_BYTE,
            4 => AHB_MST1_BURST_SIZER::_16_BYTE,
            3 => AHB_MST1_BURST_SIZER::_8_BYTE,
            2 => AHB_MST1_BURST_SIZER::_4_BYTE,
            i => AHB_MST1_BURST_SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline]
    pub fn is_64_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZER::_64_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline]
    pub fn is_32_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZER::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline]
    pub fn is_16_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZER::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline]
    pub fn is_8_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZER::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_4_BYTE`"]
    #[inline]
    pub fn is_4_byte(&self) -> bool {
        *self == AHB_MST1_BURST_SIZER::_4_BYTE
    }
}
#[doc = "Possible values of the field `AHB_MST1_IDLE_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_IDLE_ENR {
    #[doc = "Idle transfer insertion enabled"]
    IDLE,
    #[doc = "Do not insert idle transfers."]
    NO_IDLE,
}
impl AHB_MST1_IDLE_ENR {
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
            AHB_MST1_IDLE_ENR::IDLE => true,
            AHB_MST1_IDLE_ENR::NO_IDLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHB_MST1_IDLE_ENR {
        match value {
            true => AHB_MST1_IDLE_ENR::IDLE,
            false => AHB_MST1_IDLE_ENR::NO_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline]
    pub fn is_idle(&self) -> bool {
        *self == AHB_MST1_IDLE_ENR::IDLE
    }
    #[doc = "Checks if the value of the field is `NO_IDLE`"]
    #[inline]
    pub fn is_no_idle(&self) -> bool {
        *self == AHB_MST1_IDLE_ENR::NO_IDLE
    }
}
#[doc = "Possible values of the field `AHB_MST1_INCR_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_INCR_ENR {
    #[doc = "Fixed length bursts or single transfers"]
    SPECIFIED,
    #[doc = "Unspecified length burst transfers"]
    UNSPECIFIED,
}
impl AHB_MST1_INCR_ENR {
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
            AHB_MST1_INCR_ENR::SPECIFIED => true,
            AHB_MST1_INCR_ENR::UNSPECIFIED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHB_MST1_INCR_ENR {
        match value {
            true => AHB_MST1_INCR_ENR::SPECIFIED,
            false => AHB_MST1_INCR_ENR::UNSPECIFIED,
        }
    }
    #[doc = "Checks if the value of the field is `SPECIFIED`"]
    #[inline]
    pub fn is_specified(&self) -> bool {
        *self == AHB_MST1_INCR_ENR::SPECIFIED
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline]
    pub fn is_unspecified(&self) -> bool {
        *self == AHB_MST1_INCR_ENR::UNSPECIFIED
    }
}
#[doc = "Possible values of the field `AHB_MST1_LOCK_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_LOCK_ENR {
    #[doc = "Transfers are locked"]
    LOCKED,
    #[doc = "Transfers are not locked"]
    NOT_LOCKED,
}
impl AHB_MST1_LOCK_ENR {
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
            AHB_MST1_LOCK_ENR::LOCKED => true,
            AHB_MST1_LOCK_ENR::NOT_LOCKED => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHB_MST1_LOCK_ENR {
        match value {
            true => AHB_MST1_LOCK_ENR::LOCKED,
            false => AHB_MST1_LOCK_ENR::NOT_LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline]
    pub fn is_locked(&self) -> bool {
        *self == AHB_MST1_LOCK_ENR::LOCKED
    }
    #[doc = "Checks if the value of the field is `NOT_LOCKED`"]
    #[inline]
    pub fn is_not_locked(&self) -> bool {
        *self == AHB_MST1_LOCK_ENR::NOT_LOCKED
    }
}
#[doc = "Possible values of the field `AHB_MST1_BIGEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_MST1_BIGENDR {
    #[doc = "Big Endian"]
    BIG_ENDIAN,
    #[doc = "Little Endian"]
    LITTLE_ENDIAN,
}
impl AHB_MST1_BIGENDR {
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
            AHB_MST1_BIGENDR::BIG_ENDIAN => true,
            AHB_MST1_BIGENDR::LITTLE_ENDIAN => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHB_MST1_BIGENDR {
        match value {
            true => AHB_MST1_BIGENDR::BIG_ENDIAN,
            false => AHB_MST1_BIGENDR::LITTLE_ENDIAN,
        }
    }
    #[doc = "Checks if the value of the field is `BIG_ENDIAN`"]
    #[inline]
    pub fn is_big_endian(&self) -> bool {
        *self == AHB_MST1_BIGENDR::BIG_ENDIAN
    }
    #[doc = "Checks if the value of the field is `LITTLE_ENDIAN`"]
    #[inline]
    pub fn is_little_endian(&self) -> bool {
        *self == AHB_MST1_BIGENDR::LITTLE_ENDIAN
    }
}
#[doc = r" Value of the field"]
pub struct RESERVED0R {
    bits: u8,
}
impl RESERVED0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RESERVED16W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED16W<'a> {
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
#[doc = "Values that can be written to the field `AHB_MST1_BURST_SIZE`"]
pub enum AHB_MST1_BURST_SIZEW {
    #[doc = "64 bytes "]
    _64_BYTE,
    #[doc = "32 bytes "]
    _32_BYTE,
    #[doc = "16 bytes "]
    _16_BYTE,
    #[doc = "8 bytes "]
    _8_BYTE,
    #[doc = "4 bytes"]
    _4_BYTE,
}
impl AHB_MST1_BURST_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AHB_MST1_BURST_SIZEW::_64_BYTE => 6,
            AHB_MST1_BURST_SIZEW::_32_BYTE => 5,
            AHB_MST1_BURST_SIZEW::_16_BYTE => 4,
            AHB_MST1_BURST_SIZEW::_8_BYTE => 3,
            AHB_MST1_BURST_SIZEW::_4_BYTE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHB_MST1_BURST_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_MST1_BURST_SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHB_MST1_BURST_SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "64 bytes"]
    #[inline]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZEW::_64_BYTE)
    }
    #[doc = "32 bytes"]
    #[inline]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZEW::_32_BYTE)
    }
    #[doc = "16 bytes"]
    #[inline]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZEW::_16_BYTE)
    }
    #[doc = "8 bytes"]
    #[inline]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZEW::_8_BYTE)
    }
    #[doc = "4 bytes"]
    #[inline]
    pub fn _4_byte(self) -> &'a mut W {
        self.variant(AHB_MST1_BURST_SIZEW::_4_BYTE)
    }
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
#[doc = "Values that can be written to the field `AHB_MST1_IDLE_EN`"]
pub enum AHB_MST1_IDLE_ENW {
    #[doc = "Idle transfer insertion enabled"]
    IDLE,
    #[doc = "Do not insert idle transfers."]
    NO_IDLE,
}
impl AHB_MST1_IDLE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHB_MST1_IDLE_ENW::IDLE => true,
            AHB_MST1_IDLE_ENW::NO_IDLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHB_MST1_IDLE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_MST1_IDLE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHB_MST1_IDLE_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Idle transfer insertion enabled"]
    #[inline]
    pub fn idle(self) -> &'a mut W {
        self.variant(AHB_MST1_IDLE_ENW::IDLE)
    }
    #[doc = "Do not insert idle transfers."]
    #[inline]
    pub fn no_idle(self) -> &'a mut W {
        self.variant(AHB_MST1_IDLE_ENW::NO_IDLE)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AHB_MST1_INCR_EN`"]
pub enum AHB_MST1_INCR_ENW {
    #[doc = "Fixed length bursts or single transfers"]
    SPECIFIED,
    #[doc = "Unspecified length burst transfers"]
    UNSPECIFIED,
}
impl AHB_MST1_INCR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHB_MST1_INCR_ENW::SPECIFIED => true,
            AHB_MST1_INCR_ENW::UNSPECIFIED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHB_MST1_INCR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_MST1_INCR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHB_MST1_INCR_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fixed length bursts or single transfers"]
    #[inline]
    pub fn specified(self) -> &'a mut W {
        self.variant(AHB_MST1_INCR_ENW::SPECIFIED)
    }
    #[doc = "Unspecified length burst transfers"]
    #[inline]
    pub fn unspecified(self) -> &'a mut W {
        self.variant(AHB_MST1_INCR_ENW::UNSPECIFIED)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AHB_MST1_LOCK_EN`"]
pub enum AHB_MST1_LOCK_ENW {
    #[doc = "Transfers are locked"]
    LOCKED,
    #[doc = "Transfers are not locked"]
    NOT_LOCKED,
}
impl AHB_MST1_LOCK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHB_MST1_LOCK_ENW::LOCKED => true,
            AHB_MST1_LOCK_ENW::NOT_LOCKED => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHB_MST1_LOCK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_MST1_LOCK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHB_MST1_LOCK_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfers are locked"]
    #[inline]
    pub fn locked(self) -> &'a mut W {
        self.variant(AHB_MST1_LOCK_ENW::LOCKED)
    }
    #[doc = "Transfers are not locked"]
    #[inline]
    pub fn not_locked(self) -> &'a mut W {
        self.variant(AHB_MST1_LOCK_ENW::NOT_LOCKED)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AHB_MST1_BIGEND`"]
pub enum AHB_MST1_BIGENDW {
    #[doc = "Big Endian"]
    BIG_ENDIAN,
    #[doc = "Little Endian"]
    LITTLE_ENDIAN,
}
impl AHB_MST1_BIGENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHB_MST1_BIGENDW::BIG_ENDIAN => true,
            AHB_MST1_BIGENDW::LITTLE_ENDIAN => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHB_MST1_BIGENDW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_MST1_BIGENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHB_MST1_BIGENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Big Endian"]
    #[inline]
    pub fn big_endian(self) -> &'a mut W {
        self.variant(AHB_MST1_BIGENDW::BIG_ENDIAN)
    }
    #[doc = "Little Endian"]
    #[inline]
    pub fn little_endian(self) -> &'a mut W {
        self.variant(AHB_MST1_BIGENDW::LITTLE_ENDIAN)
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
        const OFFSET: u8 = 8;
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
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&self) -> RESERVED16R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RESERVED16R { bits }
    }
    #[doc = "Bits 12:15 - 15:12\\] Maximum burst size that can be performed on the AHB bus"]
    #[inline]
    pub fn ahb_mst1_burst_size(&self) -> AHB_MST1_BURST_SIZER {
        AHB_MST1_BURST_SIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - 11:11\\] Idle transfer insertion between consecutive burst transfers on AHB"]
    #[inline]
    pub fn ahb_mst1_idle_en(&self) -> AHB_MST1_IDLE_ENR {
        AHB_MST1_IDLE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - 10:10\\] Burst length type of AHB transfer"]
    #[inline]
    pub fn ahb_mst1_incr_en(&self) -> AHB_MST1_INCR_ENR {
        AHB_MST1_INCR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - 9:9\\] Locked transform on AHB"]
    #[inline]
    pub fn ahb_mst1_lock_en(&self) -> AHB_MST1_LOCK_ENR {
        AHB_MST1_LOCK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - 8:8\\] Endianess for the AHB master"]
    #[inline]
    pub fn ahb_mst1_bigend(&self) -> AHB_MST1_BIGENDR {
        AHB_MST1_BIGENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:7 - 7:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved0(&self) -> RESERVED0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RESERVED0R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 9216 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&mut self) -> _RESERVED16W {
        _RESERVED16W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\] Maximum burst size that can be performed on the AHB bus"]
    #[inline]
    pub fn ahb_mst1_burst_size(&mut self) -> _AHB_MST1_BURST_SIZEW {
        _AHB_MST1_BURST_SIZEW { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Idle transfer insertion between consecutive burst transfers on AHB"]
    #[inline]
    pub fn ahb_mst1_idle_en(&mut self) -> _AHB_MST1_IDLE_ENW {
        _AHB_MST1_IDLE_ENW { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Burst length type of AHB transfer"]
    #[inline]
    pub fn ahb_mst1_incr_en(&mut self) -> _AHB_MST1_INCR_ENW {
        _AHB_MST1_INCR_ENW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Locked transform on AHB"]
    #[inline]
    pub fn ahb_mst1_lock_en(&mut self) -> _AHB_MST1_LOCK_ENW {
        _AHB_MST1_LOCK_ENW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Endianess for the AHB master"]
    #[inline]
    pub fn ahb_mst1_bigend(&mut self) -> _AHB_MST1_BIGENDW {
        _AHB_MST1_BIGENDW { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved0(&mut self) -> _RESERVED0W {
        _RESERVED0W { w: self }
    }
}
