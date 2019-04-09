#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RFCPEIFG {
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
pub struct INTERNAL_ERRORR {
    bits: bool,
}
impl INTERNAL_ERRORR {
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
pub struct BOOT_DONER {
    bits: bool,
}
impl BOOT_DONER {
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
pub struct MODULES_UNLOCKEDR {
    bits: bool,
}
impl MODULES_UNLOCKEDR {
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
pub struct SYNTH_NO_LOCKR {
    bits: bool,
}
impl SYNTH_NO_LOCKR {
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
pub struct IRQ27R {
    bits: bool,
}
impl IRQ27R {
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
pub struct RX_ABORTEDR {
    bits: bool,
}
impl RX_ABORTEDR {
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
pub struct RX_N_DATA_WRITTENR {
    bits: bool,
}
impl RX_N_DATA_WRITTENR {
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
pub struct RX_DATA_WRITTENR {
    bits: bool,
}
impl RX_DATA_WRITTENR {
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
pub struct RX_ENTRY_DONER {
    bits: bool,
}
impl RX_ENTRY_DONER {
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
pub struct RX_BUF_FULLR {
    bits: bool,
}
impl RX_BUF_FULLR {
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
pub struct RX_CTRL_ACKR {
    bits: bool,
}
impl RX_CTRL_ACKR {
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
pub struct RX_CTRLR {
    bits: bool,
}
impl RX_CTRLR {
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
pub struct RX_EMPTYR {
    bits: bool,
}
impl RX_EMPTYR {
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
pub struct RX_IGNOREDR {
    bits: bool,
}
impl RX_IGNOREDR {
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
pub struct RX_NOKR {
    bits: bool,
}
impl RX_NOKR {
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
pub struct RX_OKR {
    bits: bool,
}
impl RX_OKR {
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
pub struct IRQ15R {
    bits: bool,
}
impl IRQ15R {
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
pub struct IRQ14R {
    bits: bool,
}
impl IRQ14R {
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
pub struct IRQ13R {
    bits: bool,
}
impl IRQ13R {
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
pub struct IRQ12R {
    bits: bool,
}
impl IRQ12R {
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
pub struct TX_BUFFER_CHANGEDR {
    bits: bool,
}
impl TX_BUFFER_CHANGEDR {
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
pub struct TX_ENTRY_DONER {
    bits: bool,
}
impl TX_ENTRY_DONER {
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
pub struct TX_RETRANSR {
    bits: bool,
}
impl TX_RETRANSR {
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
pub struct TX_CTRL_ACK_ACKR {
    bits: bool,
}
impl TX_CTRL_ACK_ACKR {
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
pub struct TX_CTRL_ACKR {
    bits: bool,
}
impl TX_CTRL_ACKR {
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
pub struct TX_CTRLR {
    bits: bool,
}
impl TX_CTRLR {
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
pub struct TX_ACKR {
    bits: bool,
}
impl TX_ACKR {
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
pub struct TX_DONER {
    bits: bool,
}
impl TX_DONER {
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
pub struct LAST_FG_COMMAND_DONER {
    bits: bool,
}
impl LAST_FG_COMMAND_DONER {
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
pub struct FG_COMMAND_DONER {
    bits: bool,
}
impl FG_COMMAND_DONER {
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
pub struct LAST_COMMAND_DONER {
    bits: bool,
}
impl LAST_COMMAND_DONER {
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
pub struct COMMAND_DONER {
    bits: bool,
}
impl COMMAND_DONER {
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
pub struct _INTERNAL_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _INTERNAL_ERRORW<'a> {
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
pub struct _BOOT_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_DONEW<'a> {
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
pub struct _MODULES_UNLOCKEDW<'a> {
    w: &'a mut W,
}
impl<'a> _MODULES_UNLOCKEDW<'a> {
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
pub struct _SYNTH_NO_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNTH_NO_LOCKW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IRQ27W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ27W<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_ABORTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ABORTEDW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_N_DATA_WRITTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_N_DATA_WRITTENW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_DATA_WRITTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DATA_WRITTENW<'a> {
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
pub struct _RX_ENTRY_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_ENTRY_DONEW<'a> {
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
pub struct _RX_BUF_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_BUF_FULLW<'a> {
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
pub struct _RX_CTRL_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_CTRL_ACKW<'a> {
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
pub struct _RX_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_CTRLW<'a> {
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
pub struct _RX_EMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_EMPTYW<'a> {
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
pub struct _RX_IGNOREDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_IGNOREDW<'a> {
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
pub struct _RX_NOKW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_NOKW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_OKW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_OKW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IRQ15W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ15W<'a> {
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
pub struct _IRQ14W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ14W<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IRQ13W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ13W<'a> {
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
pub struct _IRQ12W<'a> {
    w: &'a mut W,
}
impl<'a> _IRQ12W<'a> {
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
pub struct _TX_BUFFER_CHANGEDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_BUFFER_CHANGEDW<'a> {
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
pub struct _TX_ENTRY_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_ENTRY_DONEW<'a> {
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
pub struct _TX_RETRANSW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_RETRANSW<'a> {
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
pub struct _TX_CTRL_ACK_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_CTRL_ACK_ACKW<'a> {
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
pub struct _TX_CTRL_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_CTRL_ACKW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_CTRLW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_ACKW<'a> {
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
pub struct _TX_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DONEW<'a> {
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
pub struct _LAST_FG_COMMAND_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _LAST_FG_COMMAND_DONEW<'a> {
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
pub struct _FG_COMMAND_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _FG_COMMAND_DONEW<'a> {
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
pub struct _LAST_COMMAND_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _LAST_COMMAND_DONEW<'a> {
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
pub struct _COMMAND_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _COMMAND_DONEW<'a> {
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
    #[doc = "Bit 31 - 31:31\\] Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn internal_error(&self) -> INTERNAL_ERRORR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTERNAL_ERRORR { bits }
    }
    #[doc = "Bit 30 - 30:30\\] Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn boot_done(&self) -> BOOT_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOOT_DONER { bits }
    }
    #[doc = "Bit 29 - 29:29\\] Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn modules_unlocked(&self) -> MODULES_UNLOCKEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MODULES_UNLOCKEDR { bits }
    }
    #[doc = "Bit 28 - 28:28\\] Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn synth_no_lock(&self) -> SYNTH_NO_LOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNTH_NO_LOCKR { bits }
    }
    #[doc = "Bit 27 - 27:27\\] Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn irq27(&self) -> IRQ27R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQ27R { bits }
    }
    #[doc = "Bit 26 - 26:26\\] Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_aborted(&self) -> RX_ABORTEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_ABORTEDR { bits }
    }
    #[doc = "Bit 25 - 25:25\\] Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_n_data_written(&self) -> RX_N_DATA_WRITTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_N_DATA_WRITTENR { bits }
    }
    #[doc = "Bit 24 - 24:24\\] Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_data_written(&self) -> RX_DATA_WRITTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_DATA_WRITTENR { bits }
    }
    #[doc = "Bit 23 - 23:23\\] Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_entry_done(&self) -> RX_ENTRY_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_ENTRY_DONER { bits }
    }
    #[doc = "Bit 22 - 22:22\\] Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_buf_full(&self) -> RX_BUF_FULLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_BUF_FULLR { bits }
    }
    #[doc = "Bit 21 - 21:21\\] Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_ctrl_ack(&self) -> RX_CTRL_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_CTRL_ACKR { bits }
    }
    #[doc = "Bit 20 - 20:20\\] Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_ctrl(&self) -> RX_CTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_CTRLR { bits }
    }
    #[doc = "Bit 19 - 19:19\\] Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_empty(&self) -> RX_EMPTYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_EMPTYR { bits }
    }
    #[doc = "Bit 18 - 18:18\\] Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_ignored(&self) -> RX_IGNOREDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_IGNOREDR { bits }
    }
    #[doc = "Bit 17 - 17:17\\] Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_nok(&self) -> RX_NOKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_NOKR { bits }
    }
    #[doc = "Bit 16 - 16:16\\] Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_ok(&self) -> RX_OKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_OKR { bits }
    }
    #[doc = "Bit 15 - 15:15\\] Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn irq15(&self) -> IRQ15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQ15R { bits }
    }
    #[doc = "Bit 14 - 14:14\\] Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn irq14(&self) -> IRQ14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQ14R { bits }
    }
    #[doc = "Bit 13 - 13:13\\] Interrupt flag 13. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn irq13(&self) -> IRQ13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQ13R { bits }
    }
    #[doc = "Bit 12 - 12:12\\] Interrupt flag 12. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn irq12(&self) -> IRQ12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRQ12R { bits }
    }
    #[doc = "Bit 11 - 11:11\\] Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_buffer_changed(&self) -> TX_BUFFER_CHANGEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_BUFFER_CHANGEDR { bits }
    }
    #[doc = "Bit 10 - 10:10\\] Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_entry_done(&self) -> TX_ENTRY_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_ENTRY_DONER { bits }
    }
    #[doc = "Bit 9 - 9:9\\] Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_retrans(&self) -> TX_RETRANSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_RETRANSR { bits }
    }
    #[doc = "Bit 8 - 8:8\\] Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_ctrl_ack_ack(&self) -> TX_CTRL_ACK_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_CTRL_ACK_ACKR { bits }
    }
    #[doc = "Bit 7 - 7:7\\] Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_ctrl_ack(&self) -> TX_CTRL_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_CTRL_ACKR { bits }
    }
    #[doc = "Bit 6 - 6:6\\] Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_ctrl(&self) -> TX_CTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_CTRLR { bits }
    }
    #[doc = "Bit 5 - 5:5\\] Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_ack(&self) -> TX_ACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_ACKR { bits }
    }
    #[doc = "Bit 4 - 4:4\\] Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_done(&self) -> TX_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_DONER { bits }
    }
    #[doc = "Bit 3 - 3:3\\] Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn last_fg_command_done(&self) -> LAST_FG_COMMAND_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LAST_FG_COMMAND_DONER { bits }
    }
    #[doc = "Bit 2 - 2:2\\] Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn fg_command_done(&self) -> FG_COMMAND_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FG_COMMAND_DONER { bits }
    }
    #[doc = "Bit 1 - 1:1\\] Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn last_command_done(&self) -> LAST_COMMAND_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LAST_COMMAND_DONER { bits }
    }
    #[doc = "Bit 0 - 0:0\\] Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn command_done(&self) -> COMMAND_DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMMAND_DONER { bits }
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
    #[doc = "Bit 31 - 31:31\\] Interrupt flag 31. The command and packet engine (CPE) has observed an unexpected error. A reset of the CPE is needed. This can be done by switching the RF Core power domain off and on in PRCM:PDCTL1RFC. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn internal_error(&mut self) -> _INTERNAL_ERRORW {
        _INTERNAL_ERRORW { w: self }
    }
    #[doc = "Bit 30 - 30:30\\] Interrupt flag 30. The command and packet engine (CPE) boot is finished. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn boot_done(&mut self) -> _BOOT_DONEW {
        _BOOT_DONEW { w: self }
    }
    #[doc = "Bit 29 - 29:29\\] Interrupt flag 29. As part of command and packet engine (CPE) boot process, it has opened access to RF Core modules and memories. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn modules_unlocked(&mut self) -> _MODULES_UNLOCKEDW {
        _MODULES_UNLOCKEDW { w: self }
    }
    #[doc = "Bit 28 - 28:28\\] Interrupt flag 28. The phase-locked loop in frequency synthesizer has reported loss of lock. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn synth_no_lock(&mut self) -> _SYNTH_NO_LOCKW {
        _SYNTH_NO_LOCKW { w: self }
    }
    #[doc = "Bit 27 - 27:27\\] Interrupt flag 27. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn irq27(&mut self) -> _IRQ27W {
        _IRQ27W { w: self }
    }
    #[doc = "Bit 26 - 26:26\\] Interrupt flag 26. Packet reception stopped before packet was done. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_aborted(&mut self) -> _RX_ABORTEDW {
        _RX_ABORTEDW { w: self }
    }
    #[doc = "Bit 25 - 25:25\\] Interrupt flag 25. Specified number of bytes written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_n_data_written(&mut self) -> _RX_N_DATA_WRITTENW {
        _RX_N_DATA_WRITTENW { w: self }
    }
    #[doc = "Bit 24 - 24:24\\] Interrupt flag 24. Data written to partial read Rx buffer. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_data_written(&mut self) -> _RX_DATA_WRITTENW {
        _RX_DATA_WRITTENW { w: self }
    }
    #[doc = "Bit 23 - 23:23\\] Interrupt flag 23. Rx queue data entry changing state to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_entry_done(&mut self) -> _RX_ENTRY_DONEW {
        _RX_ENTRY_DONEW { w: self }
    }
    #[doc = "Bit 22 - 22:22\\] Interrupt flag 22. Packet received that did not fit in Rx queue. BLE mode: Packet received that did not fit in the Rx queue. IEEE 802.15.4 mode: Frame received that did not fit in the Rx queue. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_buf_full(&mut self) -> _RX_BUF_FULLW {
        _RX_BUF_FULLW { w: self }
    }
    #[doc = "Bit 21 - 21:21\\] Interrupt flag 21. BLE mode only: LL control packet received with CRC OK, not to be ignored, then acknowledgement sent. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_ctrl_ack(&mut self) -> _RX_CTRL_ACKW {
        _RX_CTRL_ACKW { w: self }
    }
    #[doc = "Bit 20 - 20:20\\] Interrupt flag 20. BLE mode only: LL control packet received with CRC OK, not to be ignored. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_ctrl(&mut self) -> _RX_CTRLW {
        _RX_CTRLW { w: self }
    }
    #[doc = "Bit 19 - 19:19\\] Interrupt flag 19. BLE mode only: Packet received with CRC OK, not to be ignored, no payload. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_empty(&mut self) -> _RX_EMPTYW {
        _RX_EMPTYW { w: self }
    }
    #[doc = "Bit 18 - 18:18\\] Interrupt flag 18. Packet received, but can be ignored. BLE mode: Packet received with CRC OK, but to be ignored. IEEE 802.15.4 mode: Frame received with ignore flag set. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_ignored(&mut self) -> _RX_IGNOREDW {
        _RX_IGNOREDW { w: self }
    }
    #[doc = "Bit 17 - 17:17\\] Interrupt flag 17. Packet received with CRC error. BLE mode: Packet received with CRC error. IEEE 802.15.4 mode: Frame received with CRC error. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_nok(&mut self) -> _RX_NOKW {
        _RX_NOKW { w: self }
    }
    #[doc = "Bit 16 - 16:16\\] Interrupt flag 16. Packet received correctly. BLE mode: Packet received with CRC OK, payload, and not to be ignored. IEEE 802.15.4 mode: Frame received with CRC OK. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn rx_ok(&mut self) -> _RX_OKW {
        _RX_OKW { w: self }
    }
    #[doc = "Bit 15 - 15:15\\] Interrupt flag 15. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn irq15(&mut self) -> _IRQ15W {
        _IRQ15W { w: self }
    }
    #[doc = "Bit 14 - 14:14\\] Interrupt flag 14. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn irq14(&mut self) -> _IRQ14W {
        _IRQ14W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\] Interrupt flag 13. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn irq13(&mut self) -> _IRQ13W {
        _IRQ13W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\] Interrupt flag 12. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn irq12(&mut self) -> _IRQ12W {
        _IRQ12W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Interrupt flag 11. BLE mode only: A buffer change is complete after CMD_BLE_ADV_PAYLOAD. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_buffer_changed(&mut self) -> _TX_BUFFER_CHANGEDW {
        _TX_BUFFER_CHANGEDW { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Interrupt flag 10. Tx queue data entry state changed to finished. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_entry_done(&mut self) -> _TX_ENTRY_DONEW {
        _TX_ENTRY_DONEW { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] Interrupt flag 9. BLE mode only: Packet retransmitted. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_retrans(&mut self) -> _TX_RETRANSW {
        _TX_RETRANSW { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] Interrupt flag 8. BLE mode only: Acknowledgement received on a transmitted LL control packet, and acknowledgement transmitted for that packet. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_ctrl_ack_ack(&mut self) -> _TX_CTRL_ACK_ACKW {
        _TX_CTRL_ACK_ACKW { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] Interrupt flag 7. BLE mode: Acknowledgement received on a transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_ctrl_ack(&mut self) -> _TX_CTRL_ACKW {
        _TX_CTRL_ACKW { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] Interrupt flag 6. BLE mode: Transmitted LL control packet. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_ctrl(&mut self) -> _TX_CTRLW {
        _TX_CTRLW { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] Interrupt flag 5. BLE mode: Acknowledgement received on a transmitted packet. IEEE 802.15.4 mode: Transmitted automatic ACK frame. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_ack(&mut self) -> _TX_ACKW {
        _TX_ACKW { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] Interrupt flag 4. Packet transmitted. (BLE mode: A packet has been transmitted.) (IEEE 802.15.4 mode: A frame has been transmitted). Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn tx_done(&mut self) -> _TX_DONEW {
        _TX_DONEW { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Interrupt flag 3. IEEE 802.15.4 mode only: The last foreground radio operation command in a chain of commands has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn last_fg_command_done(&mut self) -> _LAST_FG_COMMAND_DONEW {
        _LAST_FG_COMMAND_DONEW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] Interrupt flag 2. IEEE 802.15.4 mode only: A foreground radio operation command has finished. Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn fg_command_done(&mut self) -> _FG_COMMAND_DONEW {
        _FG_COMMAND_DONEW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] Interrupt flag 1. The last radio operation command in a chain of commands has finished. (IEEE 802.15.4 mode: The last background level radio operation command in a chain of commands has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn last_command_done(&mut self) -> _LAST_COMMAND_DONEW {
        _LAST_COMMAND_DONEW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] Interrupt flag 0. A radio operation has finished. (IEEE 802.15.4 mode: A background level radio operation command has finished.) Write zero to clear flag. Write to one has no effect."]
    #[inline]
    pub fn command_done(&mut self) -> _COMMAND_DONEW {
        _COMMAND_DONEW { w: self }
    }
}
