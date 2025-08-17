#[doc = "Register `C0TR2` reader"]
pub type R = crate::R<C0TR2_SPEC>;
#[doc = "Register `C0TR2` writer"]
pub type W = crate::W<C0TR2_SPEC>;
#[doc = "Field `REQSEL` reader - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 14.3.4. The user must not assign a same input hardware request (same REQSEL\\[7:0\\] value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
pub type REQSEL_R = crate::FieldReader;
#[doc = "Field `REQSEL` writer - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 14.3.4. The user must not assign a same input hardware request (same REQSEL\\[7:0\\] value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
pub type REQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWREQ_A {
    #[doc = "0: no software request. The selected hardware request REQSEL\\[6:0\\] is taken into account."]
    B_0x0 = 0,
    #[doc = "1: software request for a memory-to-memory transfer. The default selected hardware request as per REQSEL\\[6:0\\] is ignored."]
    B_0x1 = 1,
}
impl From<SWREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SWREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ` reader - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted."]
pub type SWREQ_R = crate::BitReader<SWREQ_A>;
impl SWREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWREQ_A {
        match self.bits {
            false => SWREQ_A::B_0x0,
            true => SWREQ_A::B_0x1,
        }
    }
    #[doc = "no software request. The selected hardware request REQSEL\\[6:0\\] is taken into account."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWREQ_A::B_0x0
    }
    #[doc = "software request for a memory-to-memory transfer. The default selected hardware request as per REQSEL\\[6:0\\] is ignored."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWREQ_A::B_0x1
    }
}
#[doc = "Field `SWREQ` writer - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted."]
pub type SWREQ_W<'a, REG> = crate::BitWriter<'a, REG, SWREQ_A>;
impl<'a, REG> SWREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no software request. The selected hardware request REQSEL\\[6:0\\] is taken into account."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWREQ_A::B_0x0)
    }
    #[doc = "software request for a memory-to-memory transfer. The default selected hardware request as per REQSEL\\[6:0\\] is ignored."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWREQ_A::B_0x1)
    }
}
#[doc = "destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DREQ_A {
    #[doc = "0: selected hardware request driven by a source peripheral (request signal taken into account by the GPDMA transfer scheduler over the source/read port)"]
    B_0x0 = 0,
    #[doc = "1: selected hardware request driven by a destination peripheral (request signal taken into account by the GPDMA transfer scheduler over the destination/write port)"]
    B_0x1 = 1,
}
impl From<DREQ_A> for bool {
    #[inline(always)]
    fn from(variant: DREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DREQ` reader - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported."]
pub type DREQ_R = crate::BitReader<DREQ_A>;
impl DREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DREQ_A {
        match self.bits {
            false => DREQ_A::B_0x0,
            true => DREQ_A::B_0x1,
        }
    }
    #[doc = "selected hardware request driven by a source peripheral (request signal taken into account by the GPDMA transfer scheduler over the source/read port)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DREQ_A::B_0x0
    }
    #[doc = "selected hardware request driven by a destination peripheral (request signal taken into account by the GPDMA transfer scheduler over the destination/write port)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DREQ_A::B_0x1
    }
}
#[doc = "Field `DREQ` writer - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported."]
pub type DREQ_W<'a, REG> = crate::BitWriter<'a, REG, DREQ_A>;
impl<'a, REG> DREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "selected hardware request driven by a source peripheral (request signal taken into account by the GPDMA transfer scheduler over the source/read port)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DREQ_A::B_0x0)
    }
    #[doc = "selected hardware request driven by a destination peripheral (request signal taken into account by the GPDMA transfer scheduler over the destination/write port)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DREQ_A::B_0x1)
    }
}
#[doc = "Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREQ_A {
    #[doc = "0: the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a burst level."]
    B_0x0 = 0,
    #[doc = "1: the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a block level (see Section 14.3.4)."]
    B_0x1 = 1,
}
impl From<BREQ_A> for bool {
    #[inline(always)]
    fn from(variant: BREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREQ` reader - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
pub type BREQ_R = crate::BitReader<BREQ_A>;
impl BREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BREQ_A {
        match self.bits {
            false => BREQ_A::B_0x0,
            true => BREQ_A::B_0x1,
        }
    }
    #[doc = "the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a burst level."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BREQ_A::B_0x0
    }
    #[doc = "the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a block level (see Section 14.3.4)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BREQ_A::B_0x1
    }
}
#[doc = "Field `BREQ` writer - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
pub type BREQ_W<'a, REG> = crate::BitWriter<'a, REG, BREQ_A>;
impl<'a, REG> BREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a burst level."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BREQ_A::B_0x0)
    }
    #[doc = "the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a block level (see Section 14.3.4)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BREQ_A::B_0x1)
    }
}
#[doc = "Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 14.3.2 for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\] must be set to 0) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width source data width, GPDMA_CxTR1.PAM\\[1\\] must be set to 0). Note: - GPDMA_CxBR1.BNDT\\[15:0\\] must be programmed as a multiple of the source (peripheral) burst size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFREQ_A {
    #[doc = "0: the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in GPDMA control mode. The GPDMA is programmed with GPDMA_CxCTR1.BNDT\\[15:0\\] and this is internally used by the hardware for the block transfer completion."]
    B_0x0 = 0,
    #[doc = "1: the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in peripheral control mode. The GPDMA block transfer can be early completed by the peripheral itself (see Section 14.3.6 for more details)."]
    B_0x1 = 1,
}
impl From<PFREQ_A> for bool {
    #[inline(always)]
    fn from(variant: PFREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFREQ` reader - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 14.3.2 for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\] must be set to 0) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width source data width, GPDMA_CxTR1.PAM\\[1\\] must be set to 0). Note: - GPDMA_CxBR1.BNDT\\[15:0\\] must be programmed as a multiple of the source (peripheral) burst size."]
pub type PFREQ_R = crate::BitReader<PFREQ_A>;
impl PFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PFREQ_A {
        match self.bits {
            false => PFREQ_A::B_0x0,
            true => PFREQ_A::B_0x1,
        }
    }
    #[doc = "the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in GPDMA control mode. The GPDMA is programmed with GPDMA_CxCTR1.BNDT\\[15:0\\] and this is internally used by the hardware for the block transfer completion."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PFREQ_A::B_0x0
    }
    #[doc = "the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in peripheral control mode. The GPDMA block transfer can be early completed by the peripheral itself (see Section 14.3.6 for more details)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PFREQ_A::B_0x1
    }
}
#[doc = "Field `PFREQ` writer - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 14.3.2 for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\] must be set to 0) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width source data width, GPDMA_CxTR1.PAM\\[1\\] must be set to 0). Note: - GPDMA_CxBR1.BNDT\\[15:0\\] must be programmed as a multiple of the source (peripheral) burst size."]
pub type PFREQ_W<'a, REG> = crate::BitWriter<'a, REG, PFREQ_A>;
impl<'a, REG> PFREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in GPDMA control mode. The GPDMA is programmed with GPDMA_CxCTR1.BNDT\\[15:0\\] and this is internally used by the hardware for the block transfer completion."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PFREQ_A::B_0x0)
    }
    #[doc = "the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in peripheral control mode. The GPDMA block transfer can be early completed by the peripheral itself (see Section 14.3.6 for more details)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PFREQ_A::B_0x1)
    }
}
#[doc = "trigger mode These bits define the transfer granularity for its conditioning by the trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGM_A {
    #[doc = "0: at block level: the first burst read of each block transfer is conditioned by one hit trigger (channel x = 12 to 15, for each block if a 2D/repeated block is configured with GPDMA_CxBR1.BRC\\[10:0\\] different 0)."]
    B_0x0 = 0,
    #[doc = "1: channel x = 0 to 7, same as 00; channel x (x = 6 to 7), at 2D/repeated block level. The first burst read of a 2D/repeated block transfer is conditioned by one hit trigger."]
    B_0x1 = 1,
    #[doc = "2: at link level: a LLI link transfer is conditioned by one hit trigger. The LLI data transfer (if any) is not conditioned."]
    B_0x2 = 2,
    #[doc = "3: at programmed burst level: If SWREQ = 1, each programmed burst read is conditioned by one hit trigger. If SWREQ = 0, each programmed burst that is requested by the selected peripheral, is conditioned by one hit trigger."]
    B_0x3 = 3,
}
impl From<TRIGM_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGM_A {
    type Ux = u8;
}
impl crate::IsEnum for TRIGM_A {}
#[doc = "Field `TRIGM` reader - trigger mode These bits define the transfer granularity for its conditioning by the trigger."]
pub type TRIGM_R = crate::FieldReader<TRIGM_A>;
impl TRIGM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRIGM_A {
        match self.bits {
            0 => TRIGM_A::B_0x0,
            1 => TRIGM_A::B_0x1,
            2 => TRIGM_A::B_0x2,
            3 => TRIGM_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "at block level: the first burst read of each block transfer is conditioned by one hit trigger (channel x = 12 to 15, for each block if a 2D/repeated block is configured with GPDMA_CxBR1.BRC\\[10:0\\] different 0)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TRIGM_A::B_0x0
    }
    #[doc = "channel x = 0 to 7, same as 00; channel x (x = 6 to 7), at 2D/repeated block level. The first burst read of a 2D/repeated block transfer is conditioned by one hit trigger."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TRIGM_A::B_0x1
    }
    #[doc = "at link level: a LLI link transfer is conditioned by one hit trigger. The LLI data transfer (if any) is not conditioned."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TRIGM_A::B_0x2
    }
    #[doc = "at programmed burst level: If SWREQ = 1, each programmed burst read is conditioned by one hit trigger. If SWREQ = 0, each programmed burst that is requested by the selected peripheral, is conditioned by one hit trigger."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TRIGM_A::B_0x3
    }
}
#[doc = "Field `TRIGM` writer - trigger mode These bits define the transfer granularity for its conditioning by the trigger."]
pub type TRIGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRIGM_A, crate::Safe>;
impl<'a, REG> TRIGM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "at block level: the first burst read of each block transfer is conditioned by one hit trigger (channel x = 12 to 15, for each block if a 2D/repeated block is configured with GPDMA_CxBR1.BRC\\[10:0\\] different 0)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGM_A::B_0x0)
    }
    #[doc = "channel x = 0 to 7, same as 00; channel x (x = 6 to 7), at 2D/repeated block level. The first burst read of a 2D/repeated block transfer is conditioned by one hit trigger."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGM_A::B_0x1)
    }
    #[doc = "at link level: a LLI link transfer is conditioned by one hit trigger. The LLI data transfer (if any) is not conditioned."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGM_A::B_0x2)
    }
    #[doc = "at programmed burst level: If SWREQ = 1, each programmed burst read is conditioned by one hit trigger. If SWREQ = 0, each programmed burst that is requested by the selected peripheral, is conditioned by one hit trigger."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGM_A::B_0x3)
    }
}
#[doc = "Field `TRIGSEL` reader - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per Section 14.3.7), with an active trigger event if TRIGPOL\\[1:0\\] different 00."]
pub type TRIGSEL_R = crate::FieldReader;
#[doc = "Field `TRIGSEL` writer - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per Section 14.3.7), with an active trigger event if TRIGPOL\\[1:0\\] different 00."]
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGPOL_A {
    #[doc = "0: no trigger (masked trigger event)"]
    B_0x0 = 0,
    #[doc = "1: trigger on the rising edge"]
    B_0x1 = 1,
    #[doc = "2: trigger on the falling edge"]
    B_0x2 = 2,
    #[doc = "3: same as 00"]
    B_0x3 = 3,
}
impl From<TRIGPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGPOL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGPOL_A {
    type Ux = u8;
}
impl crate::IsEnum for TRIGPOL_A {}
#[doc = "Field `TRIGPOL` reader - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
pub type TRIGPOL_R = crate::FieldReader<TRIGPOL_A>;
impl TRIGPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRIGPOL_A {
        match self.bits {
            0 => TRIGPOL_A::B_0x0,
            1 => TRIGPOL_A::B_0x1,
            2 => TRIGPOL_A::B_0x2,
            3 => TRIGPOL_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no trigger (masked trigger event)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TRIGPOL_A::B_0x0
    }
    #[doc = "trigger on the rising edge"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TRIGPOL_A::B_0x1
    }
    #[doc = "trigger on the falling edge"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TRIGPOL_A::B_0x2
    }
    #[doc = "same as 00"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TRIGPOL_A::B_0x3
    }
}
#[doc = "Field `TRIGPOL` writer - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
pub type TRIGPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRIGPOL_A, crate::Safe>;
impl<'a, REG> TRIGPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no trigger (masked trigger event)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGPOL_A::B_0x0)
    }
    #[doc = "trigger on the rising edge"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGPOL_A::B_0x1)
    }
    #[doc = "trigger on the falling edge"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGPOL_A::B_0x2)
    }
    #[doc = "same as 00"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGPOL_A::B_0x3)
    }
}
#[doc = "transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI 0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI 0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI 0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI 1 .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCEM_A {
    #[doc = "0: at block level (when GPDMA_CxBR1.BNDT\\[15:0\\] = 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block."]
    B_0x0 = 0,
    #[doc = "1: channel x (0 to 5, same as 00, channel x (x = 6 to 7), at 2D/repeated block level (when GPDMA_CxBR1.BRC\\[10:0\\] = 0 and GPDMA_CxBR1.BNDT\\[15:0\\] = 0). The complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block."]
    B_0x1 = 1,
    #[doc = "2: at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer The LLI data transfer is a block transfer or a 2D/repeated block transfer for channel x (x = 6 to 7), if any data transfer."]
    B_0x2 = 2,
    #[doc = "3: at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI updates the link address GPDMA_CxLLR.LA\\[15:2\\] to zero and clears all the GPDMA_CxLLR update bits (UT1, UT2, UB1, USA, UDA and ULL, plus UT3 and UB2). If the channel transfer is continuous/infinite, no event is generated."]
    B_0x3 = 3,
}
impl From<TCEM_A> for u8 {
    #[inline(always)]
    fn from(variant: TCEM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCEM_A {
    type Ux = u8;
}
impl crate::IsEnum for TCEM_A {}
#[doc = "Field `TCEM` reader - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI 0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI 0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI 0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI 1 ."]
pub type TCEM_R = crate::FieldReader<TCEM_A>;
impl TCEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCEM_A {
        match self.bits {
            0 => TCEM_A::B_0x0,
            1 => TCEM_A::B_0x1,
            2 => TCEM_A::B_0x2,
            3 => TCEM_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "at block level (when GPDMA_CxBR1.BNDT\\[15:0\\] = 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TCEM_A::B_0x0
    }
    #[doc = "channel x (0 to 5, same as 00, channel x (x = 6 to 7), at 2D/repeated block level (when GPDMA_CxBR1.BRC\\[10:0\\] = 0 and GPDMA_CxBR1.BNDT\\[15:0\\] = 0). The complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TCEM_A::B_0x1
    }
    #[doc = "at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer The LLI data transfer is a block transfer or a 2D/repeated block transfer for channel x (x = 6 to 7), if any data transfer."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == TCEM_A::B_0x2
    }
    #[doc = "at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI updates the link address GPDMA_CxLLR.LA\\[15:2\\] to zero and clears all the GPDMA_CxLLR update bits (UT1, UT2, UB1, USA, UDA and ULL, plus UT3 and UB2). If the channel transfer is continuous/infinite, no event is generated."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == TCEM_A::B_0x3
    }
}
#[doc = "Field `TCEM` writer - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI 0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI 0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI 0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI 1 ."]
pub type TCEM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TCEM_A, crate::Safe>;
impl<'a, REG> TCEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "at block level (when GPDMA_CxBR1.BNDT\\[15:0\\] = 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCEM_A::B_0x0)
    }
    #[doc = "channel x (0 to 5, same as 00, channel x (x = 6 to 7), at 2D/repeated block level (when GPDMA_CxBR1.BRC\\[10:0\\] = 0 and GPDMA_CxBR1.BNDT\\[15:0\\] = 0). The complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCEM_A::B_0x1)
    }
    #[doc = "at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer The LLI data transfer is a block transfer or a 2D/repeated block transfer for channel x (x = 6 to 7), if any data transfer."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TCEM_A::B_0x2)
    }
    #[doc = "at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI updates the link address GPDMA_CxLLR.LA\\[15:2\\] to zero and clears all the GPDMA_CxLLR update bits (UT1, UT2, UB1, USA, UDA and ULL, plus UT3 and UB2). If the channel transfer is continuous/infinite, no event is generated."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TCEM_A::B_0x3)
    }
}
impl R {
    #[doc = "Bits 0:7 - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 14.3.4. The user must not assign a same input hardware request (same REQSEL\\[7:0\\] value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
    #[inline(always)]
    pub fn REQSEL(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 9 - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted."]
    #[inline(always)]
    pub fn SWREQ(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported."]
    #[inline(always)]
    pub fn DREQ(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
    #[inline(always)]
    pub fn BREQ(&self) -> BREQ_R {
        BREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 14.3.2 for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\] must be set to 0) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width source data width, GPDMA_CxTR1.PAM\\[1\\] must be set to 0). Note: - GPDMA_CxBR1.BNDT\\[15:0\\] must be programmed as a multiple of the source (peripheral) burst size."]
    #[inline(always)]
    pub fn PFREQ(&self) -> PFREQ_R {
        PFREQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger."]
    #[inline(always)]
    pub fn TRIGM(&self) -> TRIGM_R {
        TRIGM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per Section 14.3.7), with an active trigger event if TRIGPOL\\[1:0\\] different 00."]
    #[inline(always)]
    pub fn TRIGSEL(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
    #[inline(always)]
    pub fn TRIGPOL(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI 0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI 0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI 0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI 1 ."]
    #[inline(always)]
    pub fn TCEM(&self) -> TCEM_R {
        TCEM_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 14.3.4. The user must not assign a same input hardware request (same REQSEL\\[7:0\\] value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
    #[inline(always)]
    pub fn REQSEL(&mut self) -> REQSEL_W<'_, C0TR2_SPEC> {
        REQSEL_W::new(self, 0)
    }
    #[doc = "Bit 9 - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted."]
    #[inline(always)]
    pub fn SWREQ(&mut self) -> SWREQ_W<'_, C0TR2_SPEC> {
        SWREQ_W::new(self, 9)
    }
    #[doc = "Bit 10 - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported."]
    #[inline(always)]
    pub fn DREQ(&mut self) -> DREQ_W<'_, C0TR2_SPEC> {
        DREQ_W::new(self, 10)
    }
    #[doc = "Bit 11 - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
    #[inline(always)]
    pub fn BREQ(&mut self) -> BREQ_W<'_, C0TR2_SPEC> {
        BREQ_W::new(self, 11)
    }
    #[doc = "Bit 12 - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 14.3.2 for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\] must be set to 0) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width source data width, GPDMA_CxTR1.PAM\\[1\\] must be set to 0). Note: - GPDMA_CxBR1.BNDT\\[15:0\\] must be programmed as a multiple of the source (peripheral) burst size."]
    #[inline(always)]
    pub fn PFREQ(&mut self) -> PFREQ_W<'_, C0TR2_SPEC> {
        PFREQ_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger."]
    #[inline(always)]
    pub fn TRIGM(&mut self) -> TRIGM_W<'_, C0TR2_SPEC> {
        TRIGM_W::new(self, 14)
    }
    #[doc = "Bits 16:21 - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per Section 14.3.7), with an active trigger event if TRIGPOL\\[1:0\\] different 00."]
    #[inline(always)]
    pub fn TRIGSEL(&mut self) -> TRIGSEL_W<'_, C0TR2_SPEC> {
        TRIGSEL_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
    #[inline(always)]
    pub fn TRIGPOL(&mut self) -> TRIGPOL_W<'_, C0TR2_SPEC> {
        TRIGPOL_W::new(self, 24)
    }
    #[doc = "Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI 0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI 0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI 0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI 1 ."]
    #[inline(always)]
    pub fn TCEM(&mut self) -> TCEM_W<'_, C0TR2_SPEC> {
        TCEM_W::new(self, 30)
    }
}
#[doc = "GPDMA channel 0 transfer register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c0tr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0tr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C0TR2_SPEC;
impl crate::RegisterSpec for C0TR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c0tr2::R`](R) reader structure"]
impl crate::Readable for C0TR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c0tr2::W`](W) writer structure"]
impl crate::Writable for C0TR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets C0TR2 to value 0"]
impl crate::Resettable for C0TR2_SPEC {}
