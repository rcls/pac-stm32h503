#[doc = "Register `C0TR1` reader"]
pub type R = crate::R<C0TR1_SPEC>;
#[doc = "Register `C0TR1` writer"]
pub type W = crate::W<C0TR1_SPEC>;
#[doc = "binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\] versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\] versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDW_LOG2_A {
    #[doc = "0: byte"]
    B_0x0 = 0,
    #[doc = "1: half-word (2 bytes)"]
    B_0x1 = 1,
    #[doc = "2: word (4 bytes)"]
    B_0x2 = 2,
    #[doc = "3: user setting error reported and no transfer issued"]
    B_0x3 = 3,
}
impl From<SDW_LOG2_A> for u8 {
    #[inline(always)]
    fn from(variant: SDW_LOG2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDW_LOG2_A {
    type Ux = u8;
}
impl crate::IsEnum for SDW_LOG2_A {}
#[doc = "Field `SDW_LOG2` reader - binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\] versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\] versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
pub type SDW_LOG2_R = crate::FieldReader<SDW_LOG2_A>;
impl SDW_LOG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDW_LOG2_A {
        match self.bits {
            0 => SDW_LOG2_A::B_0x0,
            1 => SDW_LOG2_A::B_0x1,
            2 => SDW_LOG2_A::B_0x2,
            3 => SDW_LOG2_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "byte"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SDW_LOG2_A::B_0x0
    }
    #[doc = "half-word (2 bytes)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SDW_LOG2_A::B_0x1
    }
    #[doc = "word (4 bytes)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == SDW_LOG2_A::B_0x2
    }
    #[doc = "user setting error reported and no transfer issued"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == SDW_LOG2_A::B_0x3
    }
}
#[doc = "Field `SDW_LOG2` writer - binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\] versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\] versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
pub type SDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SDW_LOG2_A, crate::Safe>;
impl<'a, REG> SDW_LOG2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "byte"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SDW_LOG2_A::B_0x0)
    }
    #[doc = "half-word (2 bytes)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SDW_LOG2_A::B_0x1)
    }
    #[doc = "word (4 bytes)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SDW_LOG2_A::B_0x2)
    }
    #[doc = "user setting error reported and no transfer issued"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SDW_LOG2_A::B_0x3)
    }
}
#[doc = "source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINC_A {
    #[doc = "0: fixed burst"]
    B_0x0 = 0,
    #[doc = "1: contiguously incremented burst"]
    B_0x1 = 1,
}
impl From<SINC_A> for bool {
    #[inline(always)]
    fn from(variant: SINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINC` reader - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
pub type SINC_R = crate::BitReader<SINC_A>;
impl SINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SINC_A {
        match self.bits {
            false => SINC_A::B_0x0,
            true => SINC_A::B_0x1,
        }
    }
    #[doc = "fixed burst"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SINC_A::B_0x0
    }
    #[doc = "contiguously incremented burst"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SINC_A::B_0x1
    }
}
#[doc = "Field `SINC` writer - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
pub type SINC_W<'a, REG> = crate::BitWriter<'a, REG, SINC_A>;
impl<'a, REG> SINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fixed burst"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SINC_A::B_0x0)
    }
    #[doc = "contiguously incremented burst"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SINC_A::B_0x1)
    }
}
#[doc = "Field `SBL_1` reader - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
pub type SBL_1_R = crate::FieldReader;
#[doc = "Field `SBL_1` writer - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
pub type SBL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "padding/alignment mode If DDW_LOG2\\[1:0\\] = SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width the source data width, packing is not supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PAM_A {
    #[doc = "0: source data is transferred as right aligned, padded with 0s up to the destination data width"]
    B_0x0_PAM_1 = 0,
    #[doc = "1: source data is transferred as right aligned, sign extended up to the destination data width"]
    B_0x1_PAM_1 = 1,
}
impl From<PAM_A> for u8 {
    #[inline(always)]
    fn from(variant: PAM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PAM_A {
    type Ux = u8;
}
impl crate::IsEnum for PAM_A {}
#[doc = "Field `PAM` reader - padding/alignment mode If DDW_LOG2\\[1:0\\] = SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width the source data width, packing is not supported."]
pub type PAM_R = crate::FieldReader<PAM_A>;
impl PAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PAM_A> {
        match self.bits {
            0 => Some(PAM_A::B_0x0_PAM_1),
            1 => Some(PAM_A::B_0x1_PAM_1),
            _ => None,
        }
    }
    #[doc = "source data is transferred as right aligned, padded with 0s up to the destination data width"]
    #[inline(always)]
    pub fn is_B_0x0_PAM_1(&self) -> bool {
        *self == PAM_A::B_0x0_PAM_1
    }
    #[doc = "source data is transferred as right aligned, sign extended up to the destination data width"]
    #[inline(always)]
    pub fn is_B_0x1_PAM_1(&self) -> bool {
        *self == PAM_A::B_0x1_PAM_1
    }
}
#[doc = "Field `PAM` writer - padding/alignment mode If DDW_LOG2\\[1:0\\] = SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width the source data width, packing is not supported."]
pub type PAM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PAM_A>;
impl<'a, REG> PAM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "source data is transferred as right aligned, padded with 0s up to the destination data width"]
    #[inline(always)]
    pub fn B_0x0_PAM_1(self) -> &'a mut crate::W<REG> {
        self.variant(PAM_A::B_0x0_PAM_1)
    }
    #[doc = "source data is transferred as right aligned, sign extended up to the destination data width"]
    #[inline(always)]
    pub fn B_0x1_PAM_1(self) -> &'a mut crate::W<REG> {
        self.variant(PAM_A::B_0x1_PAM_1)
    }
}
#[doc = "source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBX_A {
    #[doc = "0: no byte-based exchange within the unaligned half-word of each source word"]
    B_0x0 = 0,
    #[doc = "1: the two consecutive bytes within the unaligned half-word of each source word are exchanged."]
    B_0x1 = 1,
}
impl From<SBX_A> for bool {
    #[inline(always)]
    fn from(variant: SBX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBX` reader - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:"]
pub type SBX_R = crate::BitReader<SBX_A>;
impl SBX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBX_A {
        match self.bits {
            false => SBX_A::B_0x0,
            true => SBX_A::B_0x1,
        }
    }
    #[doc = "no byte-based exchange within the unaligned half-word of each source word"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SBX_A::B_0x0
    }
    #[doc = "the two consecutive bytes within the unaligned half-word of each source word are exchanged."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SBX_A::B_0x1
    }
}
#[doc = "Field `SBX` writer - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:"]
pub type SBX_W<'a, REG> = crate::BitWriter<'a, REG, SBX_A>;
impl<'a, REG> SBX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no byte-based exchange within the unaligned half-word of each source word"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SBX_A::B_0x0)
    }
    #[doc = "the two consecutive bytes within the unaligned half-word of each source word are exchanged."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SBX_A::B_0x1)
    }
}
#[doc = "source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAP_A {
    #[doc = "0: port 0 (AHB) allocated"]
    B_0x0 = 0,
    #[doc = "1: port 1 (AHB) allocated"]
    B_0x1 = 1,
}
impl From<SAP_A> for bool {
    #[inline(always)]
    fn from(variant: SAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAP` reader - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type SAP_R = crate::BitReader<SAP_A>;
impl SAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAP_A {
        match self.bits {
            false => SAP_A::B_0x0,
            true => SAP_A::B_0x1,
        }
    }
    #[doc = "port 0 (AHB) allocated"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SAP_A::B_0x0
    }
    #[doc = "port 1 (AHB) allocated"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SAP_A::B_0x1
    }
}
#[doc = "Field `SAP` writer - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type SAP_W<'a, REG> = crate::BitWriter<'a, REG, SAP_A>;
impl<'a, REG> SAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "port 0 (AHB) allocated"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SAP_A::B_0x0)
    }
    #[doc = "port 1 (AHB) allocated"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SAP_A::B_0x1)
    }
}
#[doc = "binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\] and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DDW_LOG2_A {
    #[doc = "0: byte"]
    B_0x0 = 0,
    #[doc = "1: half-word (2 bytes)"]
    B_0x1 = 1,
    #[doc = "2: word (4 bytes)"]
    B_0x2 = 2,
    #[doc = "3: user setting error reported and no transfer issued"]
    B_0x3 = 3,
}
impl From<DDW_LOG2_A> for u8 {
    #[inline(always)]
    fn from(variant: DDW_LOG2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DDW_LOG2_A {
    type Ux = u8;
}
impl crate::IsEnum for DDW_LOG2_A {}
#[doc = "Field `DDW_LOG2` reader - binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\] and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
pub type DDW_LOG2_R = crate::FieldReader<DDW_LOG2_A>;
impl DDW_LOG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DDW_LOG2_A {
        match self.bits {
            0 => DDW_LOG2_A::B_0x0,
            1 => DDW_LOG2_A::B_0x1,
            2 => DDW_LOG2_A::B_0x2,
            3 => DDW_LOG2_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "byte"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DDW_LOG2_A::B_0x0
    }
    #[doc = "half-word (2 bytes)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DDW_LOG2_A::B_0x1
    }
    #[doc = "word (4 bytes)"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == DDW_LOG2_A::B_0x2
    }
    #[doc = "user setting error reported and no transfer issued"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == DDW_LOG2_A::B_0x3
    }
}
#[doc = "Field `DDW_LOG2` writer - binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\] and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
pub type DDW_LOG2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DDW_LOG2_A, crate::Safe>;
impl<'a, REG> DDW_LOG2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "byte"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DDW_LOG2_A::B_0x0)
    }
    #[doc = "half-word (2 bytes)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DDW_LOG2_A::B_0x1)
    }
    #[doc = "word (4 bytes)"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DDW_LOG2_A::B_0x2)
    }
    #[doc = "user setting error reported and no transfer issued"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(DDW_LOG2_A::B_0x3)
    }
}
#[doc = "destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINC_A {
    #[doc = "0: fixed burst"]
    B_0x0 = 0,
    #[doc = "1: contiguously incremented burst"]
    B_0x1 = 1,
}
impl From<DINC_A> for bool {
    #[inline(always)]
    fn from(variant: DINC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINC` reader - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
pub type DINC_R = crate::BitReader<DINC_A>;
impl DINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DINC_A {
        match self.bits {
            false => DINC_A::B_0x0,
            true => DINC_A::B_0x1,
        }
    }
    #[doc = "fixed burst"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DINC_A::B_0x0
    }
    #[doc = "contiguously incremented burst"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DINC_A::B_0x1
    }
}
#[doc = "Field `DINC` writer - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
pub type DINC_W<'a, REG> = crate::BitWriter<'a, REG, DINC_A>;
impl<'a, REG> DINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fixed burst"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DINC_A::B_0x0)
    }
    #[doc = "contiguously incremented burst"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DINC_A::B_0x1)
    }
}
#[doc = "Field `DBL_1` reader - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
pub type DBL_1_R = crate::FieldReader;
#[doc = "Field `DBL_1` writer - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
pub type DBL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBX_A {
    #[doc = "0: no byte-based exchange within half-word"]
    B_0x0 = 0,
    #[doc = "1: the two consecutive (post PAM) bytes are exchanged in each destination half-word."]
    B_0x1 = 1,
}
impl From<DBX_A> for bool {
    #[inline(always)]
    fn from(variant: DBX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBX` reader - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:"]
pub type DBX_R = crate::BitReader<DBX_A>;
impl DBX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBX_A {
        match self.bits {
            false => DBX_A::B_0x0,
            true => DBX_A::B_0x1,
        }
    }
    #[doc = "no byte-based exchange within half-word"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBX_A::B_0x0
    }
    #[doc = "the two consecutive (post PAM) bytes are exchanged in each destination half-word."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBX_A::B_0x1
    }
}
#[doc = "Field `DBX` writer - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:"]
pub type DBX_W<'a, REG> = crate::BitWriter<'a, REG, DBX_A>;
impl<'a, REG> DBX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no byte-based exchange within half-word"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBX_A::B_0x0)
    }
    #[doc = "the two consecutive (post PAM) bytes are exchanged in each destination half-word."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBX_A::B_0x1)
    }
}
#[doc = "destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DHX_A {
    #[doc = "0: no halfword-based exchanged within word"]
    B_0x0 = 0,
    #[doc = "1: the two consecutive (post PAM) half-words are exchanged in each destination word."]
    B_0x1 = 1,
}
impl From<DHX_A> for bool {
    #[inline(always)]
    fn from(variant: DHX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DHX` reader - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:"]
pub type DHX_R = crate::BitReader<DHX_A>;
impl DHX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DHX_A {
        match self.bits {
            false => DHX_A::B_0x0,
            true => DHX_A::B_0x1,
        }
    }
    #[doc = "no halfword-based exchanged within word"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DHX_A::B_0x0
    }
    #[doc = "the two consecutive (post PAM) half-words are exchanged in each destination word."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DHX_A::B_0x1
    }
}
#[doc = "Field `DHX` writer - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:"]
pub type DHX_W<'a, REG> = crate::BitWriter<'a, REG, DHX_A>;
impl<'a, REG> DHX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no halfword-based exchanged within word"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DHX_A::B_0x0)
    }
    #[doc = "the two consecutive (post PAM) half-words are exchanged in each destination word."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DHX_A::B_0x1)
    }
}
#[doc = "destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAP_A {
    #[doc = "0: port 0 (AHB) allocated"]
    B_0x0 = 0,
    #[doc = "1: port 1 (AHB) allocated"]
    B_0x1 = 1,
}
impl From<DAP_A> for bool {
    #[inline(always)]
    fn from(variant: DAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAP` reader - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type DAP_R = crate::BitReader<DAP_A>;
impl DAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAP_A {
        match self.bits {
            false => DAP_A::B_0x0,
            true => DAP_A::B_0x1,
        }
    }
    #[doc = "port 0 (AHB) allocated"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DAP_A::B_0x0
    }
    #[doc = "port 1 (AHB) allocated"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DAP_A::B_0x1
    }
}
#[doc = "Field `DAP` writer - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type DAP_W<'a, REG> = crate::BitWriter<'a, REG, DAP_A>;
impl<'a, REG> DAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "port 0 (AHB) allocated"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DAP_A::B_0x0)
    }
    #[doc = "port 1 (AHB) allocated"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DAP_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:1 - binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\] versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\] versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
    #[inline(always)]
    pub fn SDW_LOG2(&self) -> SDW_LOG2_R {
        SDW_LOG2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
    #[inline(always)]
    pub fn SINC(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
    #[inline(always)]
    pub fn SBL_1(&self) -> SBL_1_R {
        SBL_1_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 11:12 - padding/alignment mode If DDW_LOG2\\[1:0\\] = SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width the source data width, packing is not supported."]
    #[inline(always)]
    pub fn PAM(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:"]
    #[inline(always)]
    pub fn SBX(&self) -> SBX_R {
        SBX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn SAP(&self) -> SAP_R {
        SAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\] and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    pub fn DDW_LOG2(&self) -> DDW_LOG2_R {
        DDW_LOG2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
    #[inline(always)]
    pub fn DINC(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:25 - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
    #[inline(always)]
    pub fn DBL_1(&self) -> DBL_1_R {
        DBL_1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:"]
    #[inline(always)]
    pub fn DBX(&self) -> DBX_R {
        DBX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:"]
    #[inline(always)]
    pub fn DHX(&self) -> DHX_R {
        DHX_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn DAP(&self) -> DAP_R {
        DAP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - binary logarithm of the source data width of a burst in bytes Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (GPDMA_CxBR1.BNDT\\[2:0\\] versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. Note: A source burst transfer must have an aligned address with its data width (start address GPDMA_CxSAR\\[2:0\\] versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
    #[inline(always)]
    pub fn SDW_LOG2(&mut self) -> SDW_LOG2_W<'_, C0TR1_SPEC> {
        SDW_LOG2_W::new(self, 0)
    }
    #[doc = "Bit 3 - source incrementing burst The source address, pointed by GPDMA_CxSAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
    #[inline(always)]
    pub fn SINC(&mut self) -> SINC_W<'_, C0TR1_SPEC> {
        SINC_W::new(self, 3)
    }
    #[doc = "Bits 4:9 - source burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
    #[inline(always)]
    pub fn SBL_1(&mut self) -> SBL_1_W<'_, C0TR1_SPEC> {
        SBL_1_W::new(self, 4)
    }
    #[doc = "Bits 11:12 - padding/alignment mode If DDW_LOG2\\[1:0\\] = SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else, in the following enumerated values, the condition PAM_1 is when destination data width is higher that source data width, and the condition PAM_2 is when destination data width is higher than source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination Note: If the transfer from the source peripheral is configured with peripheral flow-control mode (SWREQ = 0 and PFREQ = 1 and DREQ = 0), and if the destination data width the source data width, packing is not supported."]
    #[inline(always)]
    pub fn PAM(&mut self) -> PAM_W<'_, C0TR1_SPEC> {
        PAM_W::new(self, 11)
    }
    #[doc = "Bit 13 - source byte exchange within the unaligned half-word of each source word If the source data width is shorter than a word, this bit is ignored. If the source data width is a word:"]
    #[inline(always)]
    pub fn SBX(&mut self) -> SBX_W<'_, C0TR1_SPEC> {
        SBX_W::new(self, 13)
    }
    #[doc = "Bit 14 - source allocated port This bit is used to allocate the master port for the source transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn SAP(&mut self) -> SAP_W<'_, C0TR1_SPEC> {
        SAP_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - binary logarithm of the destination data width of a burst, in bytes Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. Note: A destination burst transfer must have an aligned address with its data width (start address GPDMA_CxDAR\\[2:0\\] and address offset GPDMA_CxTR3.DAO\\[2:0\\], versus DDW_LOG2\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
    #[inline(always)]
    pub fn DDW_LOG2(&mut self) -> DDW_LOG2_W<'_, C0TR1_SPEC> {
        DDW_LOG2_W::new(self, 16)
    }
    #[doc = "Bit 19 - destination incrementing burst The destination address, pointed by GPDMA_CxDAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
    #[inline(always)]
    pub fn DINC(&mut self) -> DINC_W<'_, C0TR1_SPEC> {
        DINC_W::new(self, 19)
    }
    #[doc = "Bits 20:25 - destination burst length minus 1, between 0 and 63 The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\] =0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. Note: If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower singles/bursts, but the data integrity is guaranteed."]
    #[inline(always)]
    pub fn DBL_1(&mut self) -> DBL_1_W<'_, C0TR1_SPEC> {
        DBL_1_W::new(self, 20)
    }
    #[doc = "Bit 26 - destination byte exchange If the destination data size is a byte, this bit is ignored. If the destination data size is not a byte:"]
    #[inline(always)]
    pub fn DBX(&mut self) -> DBX_W<'_, C0TR1_SPEC> {
        DBX_W::new(self, 26)
    }
    #[doc = "Bit 27 - destination half-word exchange If the destination data size is shorter than a word, this bit is ignored. If the destination data size is a word:"]
    #[inline(always)]
    pub fn DHX(&mut self) -> DHX_W<'_, C0TR1_SPEC> {
        DHX_W::new(self, 27)
    }
    #[doc = "Bit 30 - destination allocated port This bit is used to allocate the master port for the destination transfer Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn DAP(&mut self) -> DAP_W<'_, C0TR1_SPEC> {
        DAP_W::new(self, 30)
    }
}
#[doc = "GPDMA channel 0 transfer register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c0tr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0tr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C0TR1_SPEC;
impl crate::RegisterSpec for C0TR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c0tr1::R`](R) reader structure"]
impl crate::Readable for C0TR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c0tr1::W`](W) writer structure"]
impl crate::Writable for C0TR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets C0TR1 to value 0"]
impl crate::Resettable for C0TR1_SPEC {}
