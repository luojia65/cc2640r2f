#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCTRL {
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
#[doc = "Possible values of the field `ACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKR {
    #[doc = "Enable acknowledge"]
    EN,
    #[doc = "Disable acknowledge"]
    DIS,
}
impl ACKR {
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
            ACKR::EN => true,
            ACKR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACKR {
        match value {
            true => ACKR::EN,
            false => ACKR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == ACKR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == ACKR::DIS
    }
}
#[doc = "Possible values of the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPR {
    #[doc = "Enable STOP"]
    EN,
    #[doc = "Disable STOP"]
    DIS,
}
impl STOPR {
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
            STOPR::EN => true,
            STOPR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPR {
        match value {
            true => STOPR::EN,
            false => STOPR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == STOPR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == STOPR::DIS
    }
}
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "Enable START"]
    EN,
    #[doc = "Disable START"]
    DIS,
}
impl STARTR {
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
            STARTR::EN => true,
            STARTR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTR {
        match value {
            true => STARTR::EN,
            false => STARTR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == STARTR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == STARTR::DIS
    }
}
#[doc = "Possible values of the field `RUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNR {
    #[doc = "Enable Master"]
    EN,
    #[doc = "Disable Master"]
    DIS,
}
impl RUNR {
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
            RUNR::EN => true,
            RUNR::DIS => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUNR {
        match value {
            true => RUNR::EN,
            false => RUNR::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == RUNR::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == RUNR::DIS
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
#[doc = "Values that can be written to the field `ACK`"]
pub enum ACKW {
    #[doc = "Enable acknowledge"]
    EN,
    #[doc = "Disable acknowledge"]
    DIS,
}
impl ACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACKW::EN => true,
            ACKW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _ACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable acknowledge"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(ACKW::EN)
    }
    #[doc = "Disable acknowledge"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACKW::DIS)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STOP`"]
pub enum STOPW {
    #[doc = "Enable STOP"]
    EN,
    #[doc = "Disable STOP"]
    DIS,
}
impl STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPW::EN => true,
            STOPW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable STOP"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(STOPW::EN)
    }
    #[doc = "Disable STOP"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(STOPW::DIS)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `START`"]
pub enum STARTW {
    #[doc = "Enable START"]
    EN,
    #[doc = "Disable START"]
    DIS,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTW::EN => true,
            STARTW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable START"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(STARTW::EN)
    }
    #[doc = "Disable START"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(STARTW::DIS)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RUN`"]
pub enum RUNW {
    #[doc = "Enable Master"]
    EN,
    #[doc = "Disable Master"]
    DIS,
}
impl RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RUNW::EN => true,
            RUNW::DIS => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable Master"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(RUNW::EN)
    }
    #[doc = "Disable Master"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(RUNW::DIS)
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
    #[doc = "Bit 3 - 3:3\\] Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline]
    pub fn ack(&self) -> ACKR {
        ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - 2:2\\] This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition."]
    #[inline]
    pub fn stop(&self) -> STOPR {
        STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1:1\\] This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition."]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - 0:0\\] I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data."]
    #[inline]
    pub fn run(&self) -> RUNR {
        RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 4:31 - 31:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] Data acknowledge enable 0: The received data byte is not acknowledged automatically by the master. 1: The received data byte is acknowledged automatically by the master. This bit-field must be cleared when the I2C bus controller requires no further data to be transmitted from the slave transmitter."]
    #[inline]
    pub fn ack(&mut self) -> _ACKW {
        _ACKW { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] This bit-field determines if the cycle stops at the end of the data cycle or continues on to a repeated START condition. 0: The controller does not generate the Stop condition. 1: The controller generates the Stop condition."]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] This bit-field generates the Start or Repeated Start condition. 0: The controller does not generate the Start condition. 1: The controller generates the Start condition."]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] I2C master enable 0: The master is disabled. 1: The master is enabled to transmit or receive data."]
    #[inline]
    pub fn run(&mut self) -> _RUNW {
        _RUNW { w: self }
    }
}
