#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IOMODE {
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
#[doc = "Possible values of the field `IO7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO7R {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\] is driven low.  \n\nWhen GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 7 is 0: AUXIO\\[8i+7\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 7 is 1: AUXIO\\[8i+7\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 7 drives AUXIO\\[8i+7\\]."]
    OUT,
}
impl IO7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IO7R::OPEN_SOURCE => 3,
            IO7R::OPEN_DRAIN => 2,
            IO7R::IN => 1,
            IO7R::OUT => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IO7R {
        match value {
            3 => IO7R::OPEN_SOURCE,
            2 => IO7R::OPEN_DRAIN,
            1 => IO7R::IN,
            0 => IO7R::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline]
    pub fn is_open_source(&self) -> bool {
        *self == IO7R::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == IO7R::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == IO7R::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == IO7R::OUT
    }
}
#[doc = "Possible values of the field `IO6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO6R {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\] is driven low.  \n\nWhen GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 6 is 0: AUXIO\\[8i+6\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 6 is 1: AUXIO\\[8i+6\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 6 drives AUXIO\\[8i+6\\]."]
    OUT,
}
impl IO6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IO6R::OPEN_SOURCE => 3,
            IO6R::OPEN_DRAIN => 2,
            IO6R::IN => 1,
            IO6R::OUT => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IO6R {
        match value {
            3 => IO6R::OPEN_SOURCE,
            2 => IO6R::OPEN_DRAIN,
            1 => IO6R::IN,
            0 => IO6R::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline]
    pub fn is_open_source(&self) -> bool {
        *self == IO6R::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == IO6R::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == IO6R::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == IO6R::OUT
    }
}
#[doc = "Possible values of the field `IO5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO5R {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\] is driven low.  \n\nWhen GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 5 is 0: AUXIO\\[8i+5\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 5 is 1: AUXIO\\[8i+5\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 5 drives AUXIO\\[8i+5\\]."]
    OUT,
}
impl IO5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IO5R::OPEN_SOURCE => 3,
            IO5R::OPEN_DRAIN => 2,
            IO5R::IN => 1,
            IO5R::OUT => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IO5R {
        match value {
            3 => IO5R::OPEN_SOURCE,
            2 => IO5R::OPEN_DRAIN,
            1 => IO5R::IN,
            0 => IO5R::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline]
    pub fn is_open_source(&self) -> bool {
        *self == IO5R::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == IO5R::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == IO5R::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == IO5R::OUT
    }
}
#[doc = "Possible values of the field `IO4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO4R {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\] is driven low.  \n\nWhen GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 4 is 0: AUXIO\\[8i+4\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 4 is 1: AUXIO\\[8i+4\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 4 drives AUXIO\\[8i+4\\]."]
    OUT,
}
impl IO4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IO4R::OPEN_SOURCE => 3,
            IO4R::OPEN_DRAIN => 2,
            IO4R::IN => 1,
            IO4R::OUT => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IO4R {
        match value {
            3 => IO4R::OPEN_SOURCE,
            2 => IO4R::OPEN_DRAIN,
            1 => IO4R::IN,
            0 => IO4R::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline]
    pub fn is_open_source(&self) -> bool {
        *self == IO4R::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == IO4R::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == IO4R::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == IO4R::OUT
    }
}
#[doc = "Possible values of the field `IO3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO3R {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\] is driven low.  \n\nWhen GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 3 is 0: AUXIO\\[8i+3\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 3 is 1: AUXIO\\[8i+3\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 3 drives AUXIO\\[8i+3\\]."]
    OUT,
}
impl IO3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IO3R::OPEN_SOURCE => 3,
            IO3R::OPEN_DRAIN => 2,
            IO3R::IN => 1,
            IO3R::OUT => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IO3R {
        match value {
            3 => IO3R::OPEN_SOURCE,
            2 => IO3R::OPEN_DRAIN,
            1 => IO3R::IN,
            0 => IO3R::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline]
    pub fn is_open_source(&self) -> bool {
        *self == IO3R::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == IO3R::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == IO3R::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == IO3R::OUT
    }
}
#[doc = "Possible values of the field `IO2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO2R {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\] is driven low.  \n\nWhen GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 2 is 0: AUXIO\\[8i+2\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 2 is 1: AUXIO\\[8i+2\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 2 drives AUXIO\\[8i+2\\]."]
    OUT,
}
impl IO2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IO2R::OPEN_SOURCE => 3,
            IO2R::OPEN_DRAIN => 2,
            IO2R::IN => 1,
            IO2R::OUT => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IO2R {
        match value {
            3 => IO2R::OPEN_SOURCE,
            2 => IO2R::OPEN_DRAIN,
            1 => IO2R::IN,
            0 => IO2R::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline]
    pub fn is_open_source(&self) -> bool {
        *self == IO2R::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == IO2R::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == IO2R::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == IO2R::OUT
    }
}
#[doc = "Possible values of the field `IO1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO1R {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\] is driven low.  \n\nWhen GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 1 is 0: AUXIO\\[8i+1\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 1 is 1: AUXIO\\[8i+1\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 1 drives AUXIO\\[8i+1\\]."]
    OUT,
}
impl IO1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IO1R::OPEN_SOURCE => 3,
            IO1R::OPEN_DRAIN => 2,
            IO1R::IN => 1,
            IO1R::OUT => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IO1R {
        match value {
            3 => IO1R::OPEN_SOURCE,
            2 => IO1R::OPEN_DRAIN,
            1 => IO1R::IN,
            0 => IO1R::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline]
    pub fn is_open_source(&self) -> bool {
        *self == IO1R::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == IO1R::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == IO1R::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == IO1R::OUT
    }
}
#[doc = "Possible values of the field `IO0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO0R {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\] is driven low.  \n\nWhen GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 0 is 0: AUXIO\\[8i+0\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 0 is 1: AUXIO\\[8i+0\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 0 drives AUXIO\\[8i+0\\]."]
    OUT,
}
impl IO0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IO0R::OPEN_SOURCE => 3,
            IO0R::OPEN_DRAIN => 2,
            IO0R::IN => 1,
            IO0R::OUT => 0,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IO0R {
        match value {
            3 => IO0R::OPEN_SOURCE,
            2 => IO0R::OPEN_DRAIN,
            1 => IO0R::IN,
            0 => IO0R::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_SOURCE`"]
    #[inline]
    pub fn is_open_source(&self) -> bool {
        *self == IO0R::OPEN_SOURCE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline]
    pub fn is_open_drain(&self) -> bool {
        *self == IO0R::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == IO0R::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == IO0R::OUT
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
#[doc = "Values that can be written to the field `IO7`"]
pub enum IO7W {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\] is driven low.  \n\nWhen GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 7 is 0: AUXIO\\[8i+7\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 7 is 1: AUXIO\\[8i+7\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 7 drives AUXIO\\[8i+7\\]."]
    OUT,
}
impl IO7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IO7W::OPEN_SOURCE => 3,
            IO7W::OPEN_DRAIN => 2,
            IO7W::IN => 1,
            IO7W::OUT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IO7W<'a> {
    w: &'a mut W,
}
impl<'a> _IO7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IO7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\] is driven high."]
    #[inline]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO7W::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 7 is 0: AUXIO\\[8i+7\\] is driven low. When GPIODOUT bit 7 is 1: AUXIO\\[8i+7\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO7W::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 7 is 0: AUXIO\\[8i+7\\] is enabled for analog signal transfer. When GPIODIE bit 7 is 1: AUXIO\\[8i+7\\] is enabled for digital input."]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO7W::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 7 drives AUXIO\\[8i+7\\]."]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(IO7W::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IO6`"]
pub enum IO6W {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\] is driven low.  \n\nWhen GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 6 is 0: AUXIO\\[8i+6\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 6 is 1: AUXIO\\[8i+6\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 6 drives AUXIO\\[8i+6\\]."]
    OUT,
}
impl IO6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IO6W::OPEN_SOURCE => 3,
            IO6W::OPEN_DRAIN => 2,
            IO6W::IN => 1,
            IO6W::OUT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IO6W<'a> {
    w: &'a mut W,
}
impl<'a> _IO6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IO6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\] is driven high."]
    #[inline]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO6W::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 6 is 0: AUXIO\\[8i+6\\] is driven low. When GPIODOUT bit 6 is 1: AUXIO\\[8i+6\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO6W::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 6 is 0: AUXIO\\[8i+6\\] is enabled for analog signal transfer. When GPIODIE bit 6 is 1: AUXIO\\[8i+6\\] is enabled for digital input."]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO6W::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 6 drives AUXIO\\[8i+6\\]."]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(IO6W::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IO5`"]
pub enum IO5W {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\] is driven low.  \n\nWhen GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 5 is 0: AUXIO\\[8i+5\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 5 is 1: AUXIO\\[8i+5\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 5 drives AUXIO\\[8i+5\\]."]
    OUT,
}
impl IO5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IO5W::OPEN_SOURCE => 3,
            IO5W::OPEN_DRAIN => 2,
            IO5W::IN => 1,
            IO5W::OUT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IO5W<'a> {
    w: &'a mut W,
}
impl<'a> _IO5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IO5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\] is driven high."]
    #[inline]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO5W::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 5 is 0: AUXIO\\[8i+5\\] is driven low. When GPIODOUT bit 5 is 1: AUXIO\\[8i+5\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO5W::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 5 is 0: AUXIO\\[8i+5\\] is enabled for analog signal transfer. When GPIODIE bit 5 is 1: AUXIO\\[8i+5\\] is enabled for digital input."]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO5W::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 5 drives AUXIO\\[8i+5\\]."]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(IO5W::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IO4`"]
pub enum IO4W {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\] is driven low.  \n\nWhen GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 4 is 0: AUXIO\\[8i+4\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 4 is 1: AUXIO\\[8i+4\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 4 drives AUXIO\\[8i+4\\]."]
    OUT,
}
impl IO4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IO4W::OPEN_SOURCE => 3,
            IO4W::OPEN_DRAIN => 2,
            IO4W::IN => 1,
            IO4W::OUT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IO4W<'a> {
    w: &'a mut W,
}
impl<'a> _IO4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IO4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\] is driven high."]
    #[inline]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO4W::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 4 is 0: AUXIO\\[8i+4\\] is driven low. When GPIODOUT bit 4 is 1: AUXIO\\[8i+4\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO4W::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 4 is 0: AUXIO\\[8i+4\\] is enabled for analog signal transfer. When GPIODIE bit 4 is 1: AUXIO\\[8i+4\\] is enabled for digital input."]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO4W::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 4 drives AUXIO\\[8i+4\\]."]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(IO4W::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IO3`"]
pub enum IO3W {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\] is driven low.  \n\nWhen GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 3 is 0: AUXIO\\[8i+3\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 3 is 1: AUXIO\\[8i+3\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 3 drives AUXIO\\[8i+3\\]."]
    OUT,
}
impl IO3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IO3W::OPEN_SOURCE => 3,
            IO3W::OPEN_DRAIN => 2,
            IO3W::IN => 1,
            IO3W::OUT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IO3W<'a> {
    w: &'a mut W,
}
impl<'a> _IO3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IO3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\] is driven high."]
    #[inline]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO3W::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 3 is 0: AUXIO\\[8i+3\\] is driven low. When GPIODOUT bit 3 is 1: AUXIO\\[8i+3\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO3W::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 3 is 0: AUXIO\\[8i+3\\] is enabled for analog signal transfer. When GPIODIE bit 3 is 1: AUXIO\\[8i+3\\] is enabled for digital input."]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO3W::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 3 drives AUXIO\\[8i+3\\]."]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(IO3W::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IO2`"]
pub enum IO2W {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\] is driven low.  \n\nWhen GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 2 is 0: AUXIO\\[8i+2\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 2 is 1: AUXIO\\[8i+2\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 2 drives AUXIO\\[8i+2\\]."]
    OUT,
}
impl IO2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IO2W::OPEN_SOURCE => 3,
            IO2W::OPEN_DRAIN => 2,
            IO2W::IN => 1,
            IO2W::OUT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IO2W<'a> {
    w: &'a mut W,
}
impl<'a> _IO2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IO2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\] is driven high."]
    #[inline]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO2W::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 2 is 0: AUXIO\\[8i+2\\] is driven low. When GPIODOUT bit 2 is 1: AUXIO\\[8i+2\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO2W::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 2 is 0: AUXIO\\[8i+2\\] is enabled for analog signal transfer. When GPIODIE bit 2 is 1: AUXIO\\[8i+2\\] is enabled for digital input."]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO2W::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 2 drives AUXIO\\[8i+2\\]."]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(IO2W::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IO1`"]
pub enum IO1W {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\] is driven low.  \n\nWhen GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 1 is 0: AUXIO\\[8i+1\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 1 is 1: AUXIO\\[8i+1\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 1 drives AUXIO\\[8i+1\\]."]
    OUT,
}
impl IO1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IO1W::OPEN_SOURCE => 3,
            IO1W::OPEN_DRAIN => 2,
            IO1W::IN => 1,
            IO1W::OUT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IO1W<'a> {
    w: &'a mut W,
}
impl<'a> _IO1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IO1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\] is driven high."]
    #[inline]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO1W::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 1 is 0: AUXIO\\[8i+1\\] is driven low. When GPIODOUT bit 1 is 1: AUXIO\\[8i+1\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO1W::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 1 is 0: AUXIO\\[8i+1\\] is enabled for analog signal transfer. When GPIODIE bit 1 is 1: AUXIO\\[8i+1\\] is enabled for digital input."]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO1W::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 1 drives AUXIO\\[8i+1\\]."]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(IO1W::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IO0`"]
pub enum IO0W {
    #[doc = "Open-Source Mode: \n\nWhen GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n\nWhen GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\] is driven high.\n"]
    OPEN_SOURCE,
    #[doc = "Open-Drain Mode: \n\nWhen GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\] is driven low.  \n\nWhen GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL.\n"]
    OPEN_DRAIN,
    #[doc = "Input Mode:\n\nWhen GPIODIE bit 0 is 0: AUXIO\\[8i+0\\] is enabled for analog signal transfer.\n\nWhen GPIODIE bit 0 is 1: AUXIO\\[8i+0\\] is enabled for digital input.\n"]
    IN,
    #[doc = "Output Mode:\n\nGPIODOUT bit 0 drives AUXIO\\[8i+0\\]."]
    OUT,
}
impl IO0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IO0W::OPEN_SOURCE => 3,
            IO0W::OPEN_DRAIN => 2,
            IO0W::IN => 1,
            IO0W::OUT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IO0W<'a> {
    w: &'a mut W,
}
impl<'a> _IO0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IO0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Open-Source Mode: When GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL. When GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\] is driven high."]
    #[inline]
    pub fn open_source(self) -> &'a mut W {
        self.variant(IO0W::OPEN_SOURCE)
    }
    #[doc = "Open-Drain Mode: When GPIODOUT bit 0 is 0: AUXIO\\[8i+0\\] is driven low. When GPIODOUT bit 0 is 1: AUXIO\\[8i+0\\] is tri-stated or pulled. This depends on IOC:IOCFGn.PULL_CTL."]
    #[inline]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(IO0W::OPEN_DRAIN)
    }
    #[doc = "Input Mode: When GPIODIE bit 0 is 0: AUXIO\\[8i+0\\] is enabled for analog signal transfer. When GPIODIE bit 0 is 1: AUXIO\\[8i+0\\] is enabled for digital input."]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO0W::IN)
    }
    #[doc = "Output Mode: GPIODOUT bit 0 drives AUXIO\\[8i+0\\]."]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(IO0W::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 14:15 - 15:14\\] Select mode for AUXIO\\[8i+7\\]."]
    #[inline]
    pub fn io7(&self) -> IO7R {
        IO7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - 13:12\\] Select mode for AUXIO\\[8i+6\\]."]
    #[inline]
    pub fn io6(&self) -> IO6R {
        IO6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - 11:10\\] Select mode for AUXIO\\[8i+5\\]."]
    #[inline]
    pub fn io5(&self) -> IO5R {
        IO5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - 9:8\\] Select mode for AUXIO\\[8i+4\\]."]
    #[inline]
    pub fn io4(&self) -> IO4R {
        IO4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - 7:6\\] Select mode for AUXIO\\[8i+3\\]."]
    #[inline]
    pub fn io3(&self) -> IO3R {
        IO3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - 5:4\\] Select mode for AUXIO\\[8i+2\\]."]
    #[inline]
    pub fn io2(&self) -> IO2R {
        IO2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - 3:2\\] Select mode for AUXIO\\[8i+1\\]."]
    #[inline]
    pub fn io1(&self) -> IO1R {
        IO1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1 - 1:0\\] Select mode for AUXIO\\[8i+0\\]."]
    #[inline]
    pub fn io0(&self) -> IO0R {
        IO0R::_from({
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
    #[doc = "Bits 16:31 - 31:16\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved16(&mut self) -> _RESERVED16W {
        _RESERVED16W { w: self }
    }
    #[doc = "Bits 14:15 - 15:14\\] Select mode for AUXIO\\[8i+7\\]."]
    #[inline]
    pub fn io7(&mut self) -> _IO7W {
        _IO7W { w: self }
    }
    #[doc = "Bits 12:13 - 13:12\\] Select mode for AUXIO\\[8i+6\\]."]
    #[inline]
    pub fn io6(&mut self) -> _IO6W {
        _IO6W { w: self }
    }
    #[doc = "Bits 10:11 - 11:10\\] Select mode for AUXIO\\[8i+5\\]."]
    #[inline]
    pub fn io5(&mut self) -> _IO5W {
        _IO5W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\] Select mode for AUXIO\\[8i+4\\]."]
    #[inline]
    pub fn io4(&mut self) -> _IO4W {
        _IO4W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\] Select mode for AUXIO\\[8i+3\\]."]
    #[inline]
    pub fn io3(&mut self) -> _IO3W {
        _IO3W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\] Select mode for AUXIO\\[8i+2\\]."]
    #[inline]
    pub fn io2(&mut self) -> _IO2W {
        _IO2W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\] Select mode for AUXIO\\[8i+1\\]."]
    #[inline]
    pub fn io1(&mut self) -> _IO1W {
        _IO1W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] Select mode for AUXIO\\[8i+0\\]."]
    #[inline]
    pub fn io0(&mut self) -> _IO0W {
        _IO0W { w: self }
    }
}
