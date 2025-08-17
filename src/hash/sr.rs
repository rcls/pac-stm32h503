#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINIS_A {
    #[doc = "0: Less than 16 locations are free in the input buffer"]
    B_0x0 = 0,
    #[doc = "1: A new block can be entered into the input buffer. An interrupt is generated if the DINIE bit is set in the HASH_IMR register."]
    B_0x1 = 1,
}
impl From<DINIS_A> for bool {
    #[inline(always)]
    fn from(variant: DINIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINIS` reader - Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero."]
pub type DINIS_R = crate::BitReader<DINIS_A>;
impl DINIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DINIS_A {
        match self.bits {
            false => DINIS_A::B_0x0,
            true => DINIS_A::B_0x1,
        }
    }
    #[doc = "Less than 16 locations are free in the input buffer"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DINIS_A::B_0x0
    }
    #[doc = "A new block can be entered into the input buffer. An interrupt is generated if the DINIE bit is set in the HASH_IMR register."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DINIS_A::B_0x1
    }
}
#[doc = "Field `DINIS` writer - Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero."]
pub type DINIS_W<'a, REG> = crate::BitWriter<'a, REG, DINIS_A>;
impl<'a, REG> DINIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Less than 16 locations are free in the input buffer"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DINIS_A::B_0x0)
    }
    #[doc = "A new block can be entered into the input buffer. An interrupt is generated if the DINIE bit is set in the HASH_IMR register."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DINIS_A::B_0x1)
    }
}
#[doc = "Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCIS_A {
    #[doc = "0: No digest available in the HASH_HRx registers (zeros are returned)"]
    B_0x0 = 0,
    #[doc = "1: Digest calculation complete, a digest is available in the HASH_HRx registers. An interrupt is generated if the DCIE bit is set in the HASH_IMR register."]
    B_0x1 = 1,
}
impl From<DCIS_A> for bool {
    #[inline(always)]
    fn from(variant: DCIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCIS` reader - Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register."]
pub type DCIS_R = crate::BitReader<DCIS_A>;
impl DCIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCIS_A {
        match self.bits {
            false => DCIS_A::B_0x0,
            true => DCIS_A::B_0x1,
        }
    }
    #[doc = "No digest available in the HASH_HRx registers (zeros are returned)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DCIS_A::B_0x0
    }
    #[doc = "Digest calculation complete, a digest is available in the HASH_HRx registers. An interrupt is generated if the DCIE bit is set in the HASH_IMR register."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DCIS_A::B_0x1
    }
}
#[doc = "Field `DCIS` writer - Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register."]
pub type DCIS_W<'a, REG> = crate::BitWriter<'a, REG, DCIS_A>;
impl<'a, REG> DCIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No digest available in the HASH_HRx registers (zeros are returned)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DCIS_A::B_0x0)
    }
    #[doc = "Digest calculation complete, a digest is available in the HASH_HRx registers. An interrupt is generated if the DCIE bit is set in the HASH_IMR register."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DCIS_A::B_0x1)
    }
}
#[doc = "DMA Status This bit provides information on the DMA interface activity. It is set with DMAE and cleared when DMAE = 0 and no DMA transfer is ongoing. No interrupt is associated with this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAS_A {
    #[doc = "0: DMA interface is disabled (DMAE = 0) and no transfer is ongoing"]
    B_0x0 = 0,
    #[doc = "1: DMA interface is enabled (DMAE = 1) or a transfer is ongoing"]
    B_0x1 = 1,
}
impl From<DMAS_A> for bool {
    #[inline(always)]
    fn from(variant: DMAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAS` reader - DMA Status This bit provides information on the DMA interface activity. It is set with DMAE and cleared when DMAE = 0 and no DMA transfer is ongoing. No interrupt is associated with this bit."]
pub type DMAS_R = crate::BitReader<DMAS_A>;
impl DMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAS_A {
        match self.bits {
            false => DMAS_A::B_0x0,
            true => DMAS_A::B_0x1,
        }
    }
    #[doc = "DMA interface is disabled (DMAE = 0) and no transfer is ongoing"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DMAS_A::B_0x0
    }
    #[doc = "DMA interface is enabled (DMAE = 1) or a transfer is ongoing"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DMAS_A::B_0x1
    }
}
#[doc = "Busy bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: No block is currently being processed"]
    B_0x0 = 0,
    #[doc = "1: The hash core is processing a block of data"]
    B_0x1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy bit"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::B_0x0,
            true => BUSY_A::B_0x1,
        }
    }
    #[doc = "No block is currently being processed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BUSY_A::B_0x0
    }
    #[doc = "The hash core is processing a block of data"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BUSY_A::B_0x1
    }
}
#[doc = "Field `NBWP` reader - Number of words already pushed This bitfield is the exact number of words in the message that have already been pushed into the FIFO. NBWP is incremented by 1 when a write access is performed to the HASH_DIN register. When a digest calculation starts, NBWP is updated to NBWP- block size (in words), and NBWP goes to zero when the INIT bit is written to 1."]
pub type NBWP_R = crate::FieldReader;
#[doc = "DIN not empty This bit is set when the HASH_DIN register holds valid data (that is after being written at least once). It is cleared when either the INIT bit (initialization) or the DCAL bit (completion of the previous message processing) is written to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINNE_A {
    #[doc = "0: No data are present in the data input buffer"]
    B_0x0 = 0,
    #[doc = "1: The input buffer contains at least one word of data"]
    B_0x1 = 1,
}
impl From<DINNE_A> for bool {
    #[inline(always)]
    fn from(variant: DINNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINNE` reader - DIN not empty This bit is set when the HASH_DIN register holds valid data (that is after being written at least once). It is cleared when either the INIT bit (initialization) or the DCAL bit (completion of the previous message processing) is written to 1."]
pub type DINNE_R = crate::BitReader<DINNE_A>;
impl DINNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DINNE_A {
        match self.bits {
            false => DINNE_A::B_0x0,
            true => DINNE_A::B_0x1,
        }
    }
    #[doc = "No data are present in the data input buffer"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DINNE_A::B_0x0
    }
    #[doc = "The input buffer contains at least one word of data"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DINNE_A::B_0x1
    }
}
#[doc = "Field `NBWE` reader - Number of words expected This bitfield reflects the number of words in the message that must be pushed into the FIFO to trigger a partial computation. NBWE is decremented by 1 when a write access is performed to the HASH_DIN register. NBWE is set to the expected block size +1 in words (0x11) when INIT bit is set in HASH_CR. It is set to the expected block size (0x10) when the partial digest calculation ends."]
pub type NBWE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero."]
    #[inline(always)]
    pub fn DINIS(&self) -> DINIS_R {
        DINIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register."]
    #[inline(always)]
    pub fn DCIS(&self) -> DCIS_R {
        DCIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Status This bit provides information on the DMA interface activity. It is set with DMAE and cleared when DMAE = 0 and no DMA transfer is ongoing. No interrupt is associated with this bit."]
    #[inline(always)]
    pub fn DMAS(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Busy bit"]
    #[inline(always)]
    pub fn BUSY(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 9:13 - Number of words already pushed This bitfield is the exact number of words in the message that have already been pushed into the FIFO. NBWP is incremented by 1 when a write access is performed to the HASH_DIN register. When a digest calculation starts, NBWP is updated to NBWP- block size (in words), and NBWP goes to zero when the INIT bit is written to 1."]
    #[inline(always)]
    pub fn NBWP(&self) -> NBWP_R {
        NBWP_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - DIN not empty This bit is set when the HASH_DIN register holds valid data (that is after being written at least once). It is cleared when either the INIT bit (initialization) or the DCAL bit (completion of the previous message processing) is written to 1."]
    #[inline(always)]
    pub fn DINNE(&self) -> DINNE_R {
        DINNE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Number of words expected This bitfield reflects the number of words in the message that must be pushed into the FIFO to trigger a partial computation. NBWE is decremented by 1 when a write access is performed to the HASH_DIN register. NBWE is set to the expected block size +1 in words (0x11) when INIT bit is set in HASH_CR. It is set to the expected block size (0x10) when the partial digest calculation ends."]
    #[inline(always)]
    pub fn NBWE(&self) -> NBWE_R {
        NBWE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data input interrupt status This bit is set by hardware when the FIFO is ready to get a new block (16 locations are free). It is cleared by writing it to 0 or by writing the HASH_DIN register. When DINIS = 0, HASH_CSRx registers reads as zero."]
    #[inline(always)]
    pub fn DINIS(&mut self) -> DINIS_W<'_, SR_SPEC> {
        DINIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Digest calculation completion interrupt status This bit is set by hardware when a digest becomes ready (the whole message has been processed). It is cleared by writing it to 0 or by writing the INIT bit to 1 in the HASH_CR register."]
    #[inline(always)]
    pub fn DCIS(&mut self) -> DCIS_W<'_, SR_SPEC> {
        DCIS_W::new(self, 1)
    }
}
#[doc = "HASH status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SR to value 0x0011_0001"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0x0011_0001;
}
