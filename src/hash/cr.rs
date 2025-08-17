#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `INIT` reader - Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0."]
pub type INIT_R = crate::BitReader;
#[doc = "Field `INIT` writer - Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0."]
pub type INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAE_A {
    #[doc = "0: DMA transfers disabled"]
    B_0x0 = 0,
    #[doc = "1: DMA transfers enabled. A DMA request is sent as soon as the hash core is ready to receive data."]
    B_0x1 = 1,
}
impl From<DMAE_A> for bool {
    #[inline(always)]
    fn from(variant: DMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAE` reader - DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit."]
pub type DMAE_R = crate::BitReader<DMAE_A>;
impl DMAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAE_A {
        match self.bits {
            false => DMAE_A::B_0x0,
            true => DMAE_A::B_0x1,
        }
    }
    #[doc = "DMA transfers disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMAE_A::B_0x0
    }
    #[doc = "DMA transfers enabled. A DMA request is sent as soon as the hash core is ready to receive data."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMAE_A::B_0x1
    }
}
#[doc = "Field `DMAE` writer - DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit."]
pub type DMAE_W<'a, REG> = crate::BitWriter<'a, REG, DMAE_A>;
impl<'a, REG> DMAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transfers disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAE_A::B_0x0)
    }
    #[doc = "DMA transfers enabled. A DMA request is sent as soon as the hash core is ready to receive data."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAE_A::B_0x1)
    }
}
#[doc = "Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATATYPE_A {
    #[doc = "0: 32-bit data. The data written into HASH_DIN are directly used by the HASH processing, without reordering."]
    B_0x0 = 0,
    #[doc = "1: 16-bit data or half-word. The data written into HASH_DIN are considered as two half-words, and are swapped before being used by the HASH processing."]
    B_0x1 = 1,
    #[doc = "2: 8-bit data or bytes. The data written into HASH_DIN are considered as four bytes, and are swapped before being used by the HASH processing."]
    B_0x2 = 2,
    #[doc = "3: bit data or bit string. The data written into HASH_DIN are considered as 32 bits (1st bit of the string at position 0), and are swapped before being used by the HASH processing (1st bit of the string at position 31)."]
    B_0x3 = 3,
}
impl From<DATATYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: DATATYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATATYPE_A {
    type Ux = u8;
}
impl crate::IsEnum for DATATYPE_A {}
#[doc = "Field `DATATYPE` reader - Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:"]
pub type DATATYPE_R = crate::FieldReader<DATATYPE_A>;
impl DATATYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATATYPE_A {
        match self.bits {
            0 => DATATYPE_A::B_0x0,
            1 => DATATYPE_A::B_0x1,
            2 => DATATYPE_A::B_0x2,
            3 => DATATYPE_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "32-bit data. The data written into HASH_DIN are directly used by the HASH processing, without reordering."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DATATYPE_A::B_0x0
    }
    #[doc = "16-bit data or half-word. The data written into HASH_DIN are considered as two half-words, and are swapped before being used by the HASH processing."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DATATYPE_A::B_0x1
    }
    #[doc = "8-bit data or bytes. The data written into HASH_DIN are considered as four bytes, and are swapped before being used by the HASH processing."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == DATATYPE_A::B_0x2
    }
    #[doc = "bit data or bit string. The data written into HASH_DIN are considered as 32 bits (1st bit of the string at position 0), and are swapped before being used by the HASH processing (1st bit of the string at position 31)."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == DATATYPE_A::B_0x3
    }
}
#[doc = "Field `DATATYPE` writer - Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:"]
pub type DATATYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DATATYPE_A, crate::Safe>;
impl<'a, REG> DATATYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-bit data. The data written into HASH_DIN are directly used by the HASH processing, without reordering."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE_A::B_0x0)
    }
    #[doc = "16-bit data or half-word. The data written into HASH_DIN are considered as two half-words, and are swapped before being used by the HASH processing."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE_A::B_0x1)
    }
    #[doc = "8-bit data or bytes. The data written into HASH_DIN are considered as four bytes, and are swapped before being used by the HASH processing."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE_A::B_0x2)
    }
    #[doc = "bit data or bit string. The data written into HASH_DIN are considered as 32 bits (1st bit of the string at position 0), and are swapped before being used by the HASH processing (1st bit of the string at position 31)."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(DATATYPE_A::B_0x3)
    }
}
#[doc = "Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: Hash mode selected"]
    B_0x0 = 0,
    #[doc = "1: HMAC mode selected. LKEY bit must be set if the key being used is longer than 64 bytes."]
    B_0x1 = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect."]
pub type MODE_R = crate::BitReader<MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::B_0x0,
            true => MODE_A::B_0x1,
        }
    }
    #[doc = "Hash mode selected"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MODE_A::B_0x0
    }
    #[doc = "HMAC mode selected. LKEY bit must be set if the key being used is longer than 64 bytes."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MODE_A::B_0x1
    }
}
#[doc = "Field `MODE` writer - Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect."]
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG, MODE_A>;
impl<'a, REG> MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hash mode selected"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::B_0x0)
    }
    #[doc = "HMAC mode selected. LKEY bit must be set if the key being used is longer than 64 bytes."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::B_0x1)
    }
}
#[doc = "Field `NBW` reader - Number of words already pushed Refer to NBWP\\[3:0\\] bitfield of HASH_SR for a description of NBW\\[3:0\\] bitfield. This bit is read-only."]
pub type NBW_R = crate::FieldReader;
#[doc = "Field `DINNE` reader - DIN not empty Refer to DINNE bit of HASH_SR for a description of DINNE bit. This bit is read-only."]
pub type DINNE_R = crate::BitReader;
#[doc = "Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDMAT_A {
    #[doc = "0: DCAL is automatically set at the end of a DMA transfer."]
    B_0x0 = 0,
    #[doc = "1: DCAL is not automatically set at the end of a DMA transfer."]
    B_0x1 = 1,
}
impl From<MDMAT_A> for bool {
    #[inline(always)]
    fn from(variant: MDMAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDMAT` reader - Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed."]
pub type MDMAT_R = crate::BitReader<MDMAT_A>;
impl MDMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MDMAT_A {
        match self.bits {
            false => MDMAT_A::B_0x0,
            true => MDMAT_A::B_0x1,
        }
    }
    #[doc = "DCAL is automatically set at the end of a DMA transfer."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MDMAT_A::B_0x0
    }
    #[doc = "DCAL is not automatically set at the end of a DMA transfer."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MDMAT_A::B_0x1
    }
}
#[doc = "Field `MDMAT` writer - Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed."]
pub type MDMAT_W<'a, REG> = crate::BitWriter<'a, REG, MDMAT_A>;
impl<'a, REG> MDMAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DCAL is automatically set at the end of a DMA transfer."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MDMAT_A::B_0x0)
    }
    #[doc = "DCAL is not automatically set at the end of a DMA transfer."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MDMAT_A::B_0x1)
    }
}
#[doc = "Long key selection The application must set this bit if the HMAC key is greater than the block size (64 bytes) This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LKEY_A {
    #[doc = "0: HMAC key is shorter or equal to the block size (short key). The actual key value written in HASH_DIN is used during the HMAC computation."]
    B_0x0 = 0,
    #[doc = "1: HMAC key is longer than the block size (long key). The hash of the key is used instead of the real key during the HMAC computation."]
    B_0x1 = 1,
}
impl From<LKEY_A> for bool {
    #[inline(always)]
    fn from(variant: LKEY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LKEY` reader - Long key selection The application must set this bit if the HMAC key is greater than the block size (64 bytes) This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect."]
pub type LKEY_R = crate::BitReader<LKEY_A>;
impl LKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LKEY_A {
        match self.bits {
            false => LKEY_A::B_0x0,
            true => LKEY_A::B_0x1,
        }
    }
    #[doc = "HMAC key is shorter or equal to the block size (short key). The actual key value written in HASH_DIN is used during the HMAC computation."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LKEY_A::B_0x0
    }
    #[doc = "HMAC key is longer than the block size (long key). The hash of the key is used instead of the real key during the HMAC computation."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LKEY_A::B_0x1
    }
}
#[doc = "Field `LKEY` writer - Long key selection The application must set this bit if the HMAC key is greater than the block size (64 bytes) This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect."]
pub type LKEY_W<'a, REG> = crate::BitWriter<'a, REG, LKEY_A>;
impl<'a, REG> LKEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HMAC key is shorter or equal to the block size (short key). The actual key value written in HASH_DIN is used during the HMAC computation."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LKEY_A::B_0x0)
    }
    #[doc = "HMAC key is longer than the block size (long key). The hash of the key is used instead of the real key during the HMAC computation."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LKEY_A::B_0x1)
    }
}
#[doc = "Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALGO_A {
    #[doc = "0: SHA-1"]
    B_0x0 = 0,
    #[doc = "2: SHA-224"]
    B_0x2 = 2,
    #[doc = "3: SHA-256"]
    B_0x3 = 3,
}
impl From<ALGO_A> for u8 {
    #[inline(always)]
    fn from(variant: ALGO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ALGO_A {
    type Ux = u8;
}
impl crate::IsEnum for ALGO_A {}
#[doc = "Field `ALGO` reader - Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11."]
pub type ALGO_R = crate::FieldReader<ALGO_A>;
impl ALGO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ALGO_A> {
        match self.bits {
            0 => Some(ALGO_A::B_0x0),
            2 => Some(ALGO_A::B_0x2),
            3 => Some(ALGO_A::B_0x3),
            _ => None,
        }
    }
    #[doc = "SHA-1"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ALGO_A::B_0x0
    }
    #[doc = "SHA-224"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ALGO_A::B_0x2
    }
    #[doc = "SHA-256"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ALGO_A::B_0x3
    }
}
#[doc = "Field `ALGO` writer - Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11."]
pub type ALGO_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ALGO_A>;
impl<'a, REG> ALGO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SHA-1"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALGO_A::B_0x0)
    }
    #[doc = "SHA-224"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ALGO_A::B_0x2)
    }
    #[doc = "SHA-256"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ALGO_A::B_0x3)
    }
}
impl R {
    #[doc = "Bit 2 - Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0."]
    #[inline(always)]
    pub fn INIT(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit."]
    #[inline(always)]
    pub fn DMAE(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:"]
    #[inline(always)]
    pub fn DATATYPE(&self) -> DATATYPE_R {
        DATATYPE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect."]
    #[inline(always)]
    pub fn MODE(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of words already pushed Refer to NBWP\\[3:0\\] bitfield of HASH_SR for a description of NBW\\[3:0\\] bitfield. This bit is read-only."]
    #[inline(always)]
    pub fn NBW(&self) -> NBW_R {
        NBW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DIN not empty Refer to DINNE bit of HASH_SR for a description of DINNE bit. This bit is read-only."]
    #[inline(always)]
    pub fn DINNE(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed."]
    #[inline(always)]
    pub fn MDMAT(&self) -> MDMAT_R {
        MDMAT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Long key selection The application must set this bit if the HMAC key is greater than the block size (64 bytes) This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect."]
    #[inline(always)]
    pub fn LKEY(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11."]
    #[inline(always)]
    pub fn ALGO(&self) -> ALGO_R {
        ALGO_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Initialize message digest calculation Writing this bit to 1 resets the hash processor core, so that the HASH is ready to compute the message digest of a new message. Writing this bit to 0 has no effect. Reading this bit always returns 0."]
    #[inline(always)]
    pub fn INIT(&mut self) -> INIT_W<'_, CR_SPEC> {
        INIT_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA enable After this bit is set, it is cleared by hardware while the last data of the message is written into the hash processor. Setting this bit to 0 while a DMA transfer is ongoing does not abort the current transfer. Instead, the DMA interface of the HASH remains internally enabled until the transfer is completed or INIT is written to 1. Setting INIT bit to 1 does not clear DMAE bit."]
    #[inline(always)]
    pub fn DMAE(&mut self) -> DMAE_W<'_, CR_SPEC> {
        DMAE_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Data type selection This bitfield defines the format of the data entered into the HASH_DIN register:"]
    #[inline(always)]
    pub fn DATATYPE(&mut self) -> DATATYPE_W<'_, CR_SPEC> {
        DATATYPE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Mode selection This bit selects the normal or the keyed HMAC mode for the selected algorithm: This selection is only taken into account when the INIT bit is set. Changing this bit during a computation has no effect."]
    #[inline(always)]
    pub fn MODE(&mut self) -> MODE_W<'_, CR_SPEC> {
        MODE_W::new(self, 6)
    }
    #[doc = "Bit 13 - Multiple DMA transfers This bit is set when hashing large files when multiple DMA transfers are needed."]
    #[inline(always)]
    pub fn MDMAT(&mut self) -> MDMAT_W<'_, CR_SPEC> {
        MDMAT_W::new(self, 13)
    }
    #[doc = "Bit 16 - Long key selection The application must set this bit if the HMAC key is greater than the block size (64 bytes) This selection is only taken into account when the INIT and MODE bits are set (HMAC mode selected). Changing this bit during a computation has no effect."]
    #[inline(always)]
    pub fn LKEY(&mut self) -> LKEY_W<'_, CR_SPEC> {
        LKEY_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Algorithm selection These bits select the hash algorithm: This selection is only taken into account when the INIT bit is set. Changing this bitfield during a computation has no effect. When the ALGO bitfield is updated and INIT bit is set, NBWE in HASH_SR is automatically updated to 0x11."]
    #[inline(always)]
    pub fn ALGO(&mut self) -> ALGO_W<'_, CR_SPEC> {
        ALGO_W::new(self, 17)
    }
}
#[doc = "HASH control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {}
