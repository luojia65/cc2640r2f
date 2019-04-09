#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AESDATALEN1 {
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
pub struct LEN_MSWR {
    bits: u32,
}
impl LEN_MSWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LEN_MSWW<'a> {
    w: &'a mut W,
}
impl<'a> _LEN_MSWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
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
    #[doc = "Bits 0:28 - 28:0\\] Bits \\[60:32\\] of the combined data length. Bits \\[60:0\\] of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed. For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32. Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
    #[inline]
    pub fn len_msw(&self) -> LEN_MSWR {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        LEN_MSWR { bits }
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
    #[doc = "Bits 0:28 - 28:0\\] Bits \\[60:32\\] of the combined data length. Bits \\[60:0\\] of the crypto length registers AESDATALEN1 and AESDATALEN0 store the cryptographic data length in bytes for all modes. Once processing with this context is started, this length decrements to zero. Data lengths up to (2^61 - 1) bytes are allowed. For GCM, any value up to 2^36 - 32 bytes can be used. This is because a 32-bit counter mode is used; the maximum number of 128-bit blocks is 2^32 - 2, resulting in a maximum number of bytes of 2^36 - 32. Writing to this register triggers the engine to start using this context. This is valid for all modes except GCM and CCM. Note: For the combined modes (GCM and CCM), this length does not include the authentication only data; the authentication length is specified in the AESAUTHLEN.LEN. All modes must have a length > 0. For the combined modes, it is allowed to have one of the lengths equal to zero. For the basic encryption modes (ECB/CBC/CTR) it is allowed to program zero to the length field; in that case the length is assumed infinite. All data must be byte (8-bit) aligned for stream cipher modes; bit aligned data streams are not supported by the Crypto peripheral. For block cipher modes, the data length must be programmed in multiples of the block cipher size, 16 bytes."]
    #[inline]
    pub fn len_msw(&mut self) -> _LEN_MSWW {
        _LEN_MSWW { w: self }
    }
}
