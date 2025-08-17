#[doc = "Register `C1SR` reader"]
pub type R = crate::R<C1SR_SPEC>;
#[doc = "idle flag This idle flag is deasserted by hardware when the channel is enabled (GPDMA_CxCR.EN = 1) with a valid channel configuration (no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (in suspended or disabled state).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEF_A {
    #[doc = "0: channel not in idle state"]
    B_0x0 = 0,
    #[doc = "1: channel in idle state"]
    B_0x1 = 1,
}
impl From<IDLEF_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEF` reader - idle flag This idle flag is deasserted by hardware when the channel is enabled (GPDMA_CxCR.EN = 1) with a valid channel configuration (no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (in suspended or disabled state)."]
pub type IDLEF_R = crate::BitReader<IDLEF_A>;
impl IDLEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDLEF_A {
        match self.bits {
            false => IDLEF_A::B_0x0,
            true => IDLEF_A::B_0x1,
        }
    }
    #[doc = "channel not in idle state"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IDLEF_A::B_0x0
    }
    #[doc = "channel in idle state"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IDLEF_A::B_0x1
    }
}
#[doc = "transfer complete flag A transfer complete event is either a block transfer complete, a 2D/repeated block transfer complete, or a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\\[1:0\\]).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF_A {
    #[doc = "0: no transfer complete event"]
    B_0x0 = 0,
    #[doc = "1: a transfer complete event occurred"]
    B_0x1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF` reader - transfer complete flag A transfer complete event is either a block transfer complete, a 2D/repeated block transfer complete, or a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\\[1:0\\])."]
pub type TCF_R = crate::BitReader<TCF_A>;
impl TCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::B_0x0,
            true => TCF_A::B_0x1,
        }
    }
    #[doc = "no transfer complete event"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TCF_A::B_0x0
    }
    #[doc = "a transfer complete event occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TCF_A::B_0x1
    }
}
#[doc = "half transfer flag A half transfer event is either a half block transfer or a half 2D/repeated block transfer, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\\[1:0\\]). A half block transfer occurs when half of the bytes of the source block size (rounded up integer of GPDMA_CxBR1.BNDT\\[15:0\\]/2) has been transferred to the destination. A half 2D/repeated block transfer occurs when half of the repeated blocks (rounded up integer of (GPDMA_CxBR1.BRC\\[10:0\\]+1)/2)) has been transferred to the destination.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTF_A {
    #[doc = "0: no half transfer event"]
    B_0x0 = 0,
    #[doc = "1: a half transfer event occurred"]
    B_0x1 = 1,
}
impl From<HTF_A> for bool {
    #[inline(always)]
    fn from(variant: HTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTF` reader - half transfer flag A half transfer event is either a half block transfer or a half 2D/repeated block transfer, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\\[1:0\\]). A half block transfer occurs when half of the bytes of the source block size (rounded up integer of GPDMA_CxBR1.BNDT\\[15:0\\]/2) has been transferred to the destination. A half 2D/repeated block transfer occurs when half of the repeated blocks (rounded up integer of (GPDMA_CxBR1.BRC\\[10:0\\]+1)/2)) has been transferred to the destination."]
pub type HTF_R = crate::BitReader<HTF_A>;
impl HTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTF_A {
        match self.bits {
            false => HTF_A::B_0x0,
            true => HTF_A::B_0x1,
        }
    }
    #[doc = "no half transfer event"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HTF_A::B_0x0
    }
    #[doc = "a half transfer event occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HTF_A::B_0x1
    }
}
#[doc = "data transfer error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEF_A {
    #[doc = "0: no data transfer error event"]
    B_0x0 = 0,
    #[doc = "1: a master bus error event occurred on a data transfer"]
    B_0x1 = 1,
}
impl From<DTEF_A> for bool {
    #[inline(always)]
    fn from(variant: DTEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEF` reader - data transfer error flag"]
pub type DTEF_R = crate::BitReader<DTEF_A>;
impl DTEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTEF_A {
        match self.bits {
            false => DTEF_A::B_0x0,
            true => DTEF_A::B_0x1,
        }
    }
    #[doc = "no data transfer error event"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DTEF_A::B_0x0
    }
    #[doc = "a master bus error event occurred on a data transfer"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DTEF_A::B_0x1
    }
}
#[doc = "update link transfer error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULEF_A {
    #[doc = "0: no update link transfer error event"]
    B_0x0 = 0,
    #[doc = "1: a master bus error event occurred while updating a linked-list register from memory"]
    B_0x1 = 1,
}
impl From<ULEF_A> for bool {
    #[inline(always)]
    fn from(variant: ULEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULEF` reader - update link transfer error flag"]
pub type ULEF_R = crate::BitReader<ULEF_A>;
impl ULEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ULEF_A {
        match self.bits {
            false => ULEF_A::B_0x0,
            true => ULEF_A::B_0x1,
        }
    }
    #[doc = "no update link transfer error event"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ULEF_A::B_0x0
    }
    #[doc = "a master bus error event occurred while updating a linked-list register from memory"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ULEF_A::B_0x1
    }
}
#[doc = "user setting error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USEF_A {
    #[doc = "0: no user setting error event"]
    B_0x0 = 0,
    #[doc = "1: a user setting error event occurred"]
    B_0x1 = 1,
}
impl From<USEF_A> for bool {
    #[inline(always)]
    fn from(variant: USEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEF` reader - user setting error flag"]
pub type USEF_R = crate::BitReader<USEF_A>;
impl USEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USEF_A {
        match self.bits {
            false => USEF_A::B_0x0,
            true => USEF_A::B_0x1,
        }
    }
    #[doc = "no user setting error event"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USEF_A::B_0x0
    }
    #[doc = "a user setting error event occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USEF_A::B_0x1
    }
}
#[doc = "completed suspension flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPF_A {
    #[doc = "0: no completed suspension event"]
    B_0x0 = 0,
    #[doc = "1: a completed suspension event occurred"]
    B_0x1 = 1,
}
impl From<SUSPF_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPF` reader - completed suspension flag"]
pub type SUSPF_R = crate::BitReader<SUSPF_A>;
impl SUSPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSPF_A {
        match self.bits {
            false => SUSPF_A::B_0x0,
            true => SUSPF_A::B_0x1,
        }
    }
    #[doc = "no completed suspension event"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SUSPF_A::B_0x0
    }
    #[doc = "a completed suspension event occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SUSPF_A::B_0x1
    }
}
#[doc = "trigger overrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOF_A {
    #[doc = "0: no trigger overrun event"]
    B_0x0 = 0,
    #[doc = "1: a trigger overrun event occurred"]
    B_0x1 = 1,
}
impl From<TOF_A> for bool {
    #[inline(always)]
    fn from(variant: TOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOF` reader - trigger overrun flag"]
pub type TOF_R = crate::BitReader<TOF_A>;
impl TOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOF_A {
        match self.bits {
            false => TOF_A::B_0x0,
            true => TOF_A::B_0x1,
        }
    }
    #[doc = "no trigger overrun event"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TOF_A::B_0x0
    }
    #[doc = "a trigger overrun event occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TOF_A::B_0x1
    }
}
#[doc = "Field `FIFOL` reader - monitored FIFO level Number of available write beats in the FIFO, in units of the programmed destination data width (see GPDMA_CxTR1.DDW_LOG2\\[1:0\\], in units of bytes, half-words, or words). Note: After having suspended an active transfer, the user may need to read FIFOL\\[7:0\\], additionally to GPDMA_CxBR1.BDNT\\[15:0\\] and GPDMA_CxBR1.BRC\\[10:0\\], to know how many data have been transferred to the destination. Before reading, the user may wait for the transfer to be suspended (GPDMA_CxSR.SUSPF = 1)."]
pub type FIFOL_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - idle flag This idle flag is deasserted by hardware when the channel is enabled (GPDMA_CxCR.EN = 1) with a valid channel configuration (no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (in suspended or disabled state)."]
    #[inline(always)]
    pub fn IDLEF(&self) -> IDLEF_R {
        IDLEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - transfer complete flag A transfer complete event is either a block transfer complete, a 2D/repeated block transfer complete, or a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\\[1:0\\])."]
    #[inline(always)]
    pub fn TCF(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - half transfer flag A half transfer event is either a half block transfer or a half 2D/repeated block transfer, depending on the transfer complete event mode (GPDMA_CxTR2.TCEM\\[1:0\\]). A half block transfer occurs when half of the bytes of the source block size (rounded up integer of GPDMA_CxBR1.BNDT\\[15:0\\]/2) has been transferred to the destination. A half 2D/repeated block transfer occurs when half of the repeated blocks (rounded up integer of (GPDMA_CxBR1.BRC\\[10:0\\]+1)/2)) has been transferred to the destination."]
    #[inline(always)]
    pub fn HTF(&self) -> HTF_R {
        HTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - data transfer error flag"]
    #[inline(always)]
    pub fn DTEF(&self) -> DTEF_R {
        DTEF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - update link transfer error flag"]
    #[inline(always)]
    pub fn ULEF(&self) -> ULEF_R {
        ULEF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - user setting error flag"]
    #[inline(always)]
    pub fn USEF(&self) -> USEF_R {
        USEF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - completed suspension flag"]
    #[inline(always)]
    pub fn SUSPF(&self) -> SUSPF_R {
        SUSPF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - trigger overrun flag"]
    #[inline(always)]
    pub fn TOF(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - monitored FIFO level Number of available write beats in the FIFO, in units of the programmed destination data width (see GPDMA_CxTR1.DDW_LOG2\\[1:0\\], in units of bytes, half-words, or words). Note: After having suspended an active transfer, the user may need to read FIFOL\\[7:0\\], additionally to GPDMA_CxBR1.BDNT\\[15:0\\] and GPDMA_CxBR1.BRC\\[10:0\\], to know how many data have been transferred to the destination. Before reading, the user may wait for the transfer to be suspended (GPDMA_CxSR.SUSPF = 1)."]
    #[inline(always)]
    pub fn FIFOL(&self) -> FIFOL_R {
        FIFOL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "GPDMA channel 1 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1SR_SPEC;
impl crate::RegisterSpec for C1SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1sr::R`](R) reader structure"]
impl crate::Readable for C1SR_SPEC {}
#[doc = "`reset()` method sets C1SR to value 0x01"]
impl crate::Resettable for C1SR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
