#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::KEYREADAREA {
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
pub struct BUSYR {
    bits: bool,
}
impl BUSYR {
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
    bits: u32,
}
impl RESERVED4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `RAM_AREA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_AREAR {
    #[doc = "No RAM"]
    NO_RAM,
    #[doc = "RAM Area 7"]
    RAM_AREA7,
    #[doc = "RAM Area 6"]
    RAM_AREA6,
    #[doc = "RAM Area 5"]
    RAM_AREA5,
    #[doc = "RAM Area 4"]
    RAM_AREA4,
    #[doc = "RAM Area 3"]
    RAM_AREA3,
    #[doc = "RAM Area 2"]
    RAM_AREA2,
    #[doc = "RAM Area 1"]
    RAM_AREA1,
    #[doc = "RAM Area 0"]
    RAM_AREA0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAM_AREAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAM_AREAR::NO_RAM => 8,
            RAM_AREAR::RAM_AREA7 => 7,
            RAM_AREAR::RAM_AREA6 => 6,
            RAM_AREAR::RAM_AREA5 => 5,
            RAM_AREAR::RAM_AREA4 => 4,
            RAM_AREAR::RAM_AREA3 => 3,
            RAM_AREAR::RAM_AREA2 => 2,
            RAM_AREAR::RAM_AREA1 => 1,
            RAM_AREAR::RAM_AREA0 => 0,
            RAM_AREAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAM_AREAR {
        match value {
            8 => RAM_AREAR::NO_RAM,
            7 => RAM_AREAR::RAM_AREA7,
            6 => RAM_AREAR::RAM_AREA6,
            5 => RAM_AREAR::RAM_AREA5,
            4 => RAM_AREAR::RAM_AREA4,
            3 => RAM_AREAR::RAM_AREA3,
            2 => RAM_AREAR::RAM_AREA2,
            1 => RAM_AREAR::RAM_AREA1,
            0 => RAM_AREAR::RAM_AREA0,
            i => RAM_AREAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_RAM`"]
    #[inline]
    pub fn is_no_ram(&self) -> bool {
        *self == RAM_AREAR::NO_RAM
    }
    #[doc = "Checks if the value of the field is `RAM_AREA7`"]
    #[inline]
    pub fn is_ram_area7(&self) -> bool {
        *self == RAM_AREAR::RAM_AREA7
    }
    #[doc = "Checks if the value of the field is `RAM_AREA6`"]
    #[inline]
    pub fn is_ram_area6(&self) -> bool {
        *self == RAM_AREAR::RAM_AREA6
    }
    #[doc = "Checks if the value of the field is `RAM_AREA5`"]
    #[inline]
    pub fn is_ram_area5(&self) -> bool {
        *self == RAM_AREAR::RAM_AREA5
    }
    #[doc = "Checks if the value of the field is `RAM_AREA4`"]
    #[inline]
    pub fn is_ram_area4(&self) -> bool {
        *self == RAM_AREAR::RAM_AREA4
    }
    #[doc = "Checks if the value of the field is `RAM_AREA3`"]
    #[inline]
    pub fn is_ram_area3(&self) -> bool {
        *self == RAM_AREAR::RAM_AREA3
    }
    #[doc = "Checks if the value of the field is `RAM_AREA2`"]
    #[inline]
    pub fn is_ram_area2(&self) -> bool {
        *self == RAM_AREAR::RAM_AREA2
    }
    #[doc = "Checks if the value of the field is `RAM_AREA1`"]
    #[inline]
    pub fn is_ram_area1(&self) -> bool {
        *self == RAM_AREAR::RAM_AREA1
    }
    #[doc = "Checks if the value of the field is `RAM_AREA0`"]
    #[inline]
    pub fn is_ram_area0(&self) -> bool {
        *self == RAM_AREAR::RAM_AREA0
    }
}
#[doc = r" Proxy"]
pub struct _BUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _BUSYW<'a> {
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
pub struct _RESERVED4W<'a> {
    w: &'a mut W,
}
impl<'a> _RESERVED4W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 134217727;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAM_AREA`"]
pub enum RAM_AREAW {
    #[doc = "No RAM"]
    NO_RAM,
    #[doc = "RAM Area 7"]
    RAM_AREA7,
    #[doc = "RAM Area 6"]
    RAM_AREA6,
    #[doc = "RAM Area 5"]
    RAM_AREA5,
    #[doc = "RAM Area 4"]
    RAM_AREA4,
    #[doc = "RAM Area 3"]
    RAM_AREA3,
    #[doc = "RAM Area 2"]
    RAM_AREA2,
    #[doc = "RAM Area 1"]
    RAM_AREA1,
    #[doc = "RAM Area 0"]
    RAM_AREA0,
}
impl RAM_AREAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAM_AREAW::NO_RAM => 8,
            RAM_AREAW::RAM_AREA7 => 7,
            RAM_AREAW::RAM_AREA6 => 6,
            RAM_AREAW::RAM_AREA5 => 5,
            RAM_AREAW::RAM_AREA4 => 4,
            RAM_AREAW::RAM_AREA3 => 3,
            RAM_AREAW::RAM_AREA2 => 2,
            RAM_AREAW::RAM_AREA1 => 1,
            RAM_AREAW::RAM_AREA0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM_AREAW<'a> {
    w: &'a mut W,
}
impl<'a> _RAM_AREAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM_AREAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No RAM"]
    #[inline]
    pub fn no_ram(self) -> &'a mut W {
        self.variant(RAM_AREAW::NO_RAM)
    }
    #[doc = "RAM Area 7"]
    #[inline]
    pub fn ram_area7(self) -> &'a mut W {
        self.variant(RAM_AREAW::RAM_AREA7)
    }
    #[doc = "RAM Area 6"]
    #[inline]
    pub fn ram_area6(self) -> &'a mut W {
        self.variant(RAM_AREAW::RAM_AREA6)
    }
    #[doc = "RAM Area 5"]
    #[inline]
    pub fn ram_area5(self) -> &'a mut W {
        self.variant(RAM_AREAW::RAM_AREA5)
    }
    #[doc = "RAM Area 4"]
    #[inline]
    pub fn ram_area4(self) -> &'a mut W {
        self.variant(RAM_AREAW::RAM_AREA4)
    }
    #[doc = "RAM Area 3"]
    #[inline]
    pub fn ram_area3(self) -> &'a mut W {
        self.variant(RAM_AREAW::RAM_AREA3)
    }
    #[doc = "RAM Area 2"]
    #[inline]
    pub fn ram_area2(self) -> &'a mut W {
        self.variant(RAM_AREAW::RAM_AREA2)
    }
    #[doc = "RAM Area 1"]
    #[inline]
    pub fn ram_area1(self) -> &'a mut W {
        self.variant(RAM_AREAW::RAM_AREA1)
    }
    #[doc = "RAM Area 0"]
    #[inline]
    pub fn ram_area0(self) -> &'a mut W {
        self.variant(RAM_AREAW::RAM_AREA0)
    }
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
    #[doc = "Bit 31 - 31:31\\] Key store operation busy status flag (read only) 0: operation is completed. 1: operation is not completed and the key store is busy."]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUSYR { bits }
    }
    #[doc = "Bits 4:30 - 30:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&self) -> RESERVED4R {
        let bits = {
            const MASK: u32 = 134217727;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        RESERVED4R { bits }
    }
    #[doc = "Bits 0:3 - 3:0\\] Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected."]
    #[inline]
    pub fn ram_area(&self) -> RAM_AREAR {
        RAM_AREAR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 31 - 31:31\\] Key store operation busy status flag (read only) 0: operation is completed. 1: operation is not completed and the key store is busy."]
    #[inline]
    pub fn busy(&mut self) -> _BUSYW {
        _BUSYW { w: self }
    }
    #[doc = "Bits 4:30 - 30:4\\] Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline]
    pub fn reserved4(&mut self) -> _RESERVED4W {
        _RESERVED4W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\] Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected."]
    #[inline]
    pub fn ram_area(&mut self) -> _RAM_AREAW {
        _RAM_AREAW { w: self }
    }
}
