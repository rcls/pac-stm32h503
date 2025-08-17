#[doc = "Register `C3CR` reader"]
pub type R = crate::R<C3CR_SPEC>;
#[doc = "Register `C3CR` writer"]
pub type W = crate::W<C3CR_SPEC>;
#[doc = "enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is deasserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, for example if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: write: ignored, read: channel disabled"]
    B_0x0 = 0,
    #[doc = "1: write: enable channel, read: channel enabled"]
    B_0x1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is deasserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, for example if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
pub type EN_R = crate::BitReader<EN_A>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::B_0x0,
            true => EN_A::B_0x1,
        }
    }
    #[doc = "write: ignored, read: channel disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EN_A::B_0x0
    }
    #[doc = "write: enable channel, read: channel enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EN_A::B_0x1
    }
}
#[doc = "Field `EN` writer - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is deasserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, for example if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN_A>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write: ignored, read: channel disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::B_0x0)
    }
    #[doc = "write: enable channel, read: channel enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EN_A::B_0x1)
    }
}
#[doc = "reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (GPDMA_CxSR.SUSPF = 1 and GPDMA_CxSR.IDLEF = GPDMA_CxCR.EN = 1) - channel in disabled state (GPDMA_CxSR.IDLEF = 1 and GPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (GPDMA_CxBR1, GPDMA_CxSAR and GPDMA_CxDAR) before enabling again the channel (see the programming sequence in Figure 44).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    #[doc = "0: no channel reset"]
    B_0x0 = 0,
    #[doc = "1: channel reset"]
    B_0x1 = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (GPDMA_CxSR.SUSPF = 1 and GPDMA_CxSR.IDLEF = GPDMA_CxCR.EN = 1) - channel in disabled state (GPDMA_CxSR.IDLEF = 1 and GPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (GPDMA_CxBR1, GPDMA_CxSAR and GPDMA_CxDAR) before enabling again the channel (see the programming sequence in Figure 44)."]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG, RESET_A>;
impl<'a, REG> RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no channel reset"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::B_0x0)
    }
    #[doc = "channel reset"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::B_0x1)
    }
}
#[doc = "suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel (channel with an ongoing GPDMA transfer over its master ports). The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in Figure 43.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP_A {
    #[doc = "0: write: resume channel, read: channel not suspended"]
    B_0x0 = 0,
    #[doc = "1: write: suspend channel, read: channel suspended."]
    B_0x1 = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSP` reader - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel (channel with an ongoing GPDMA transfer over its master ports). The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in Figure 43."]
pub type SUSP_R = crate::BitReader<SUSP_A>;
impl SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSP_A {
        match self.bits {
            false => SUSP_A::B_0x0,
            true => SUSP_A::B_0x1,
        }
    }
    #[doc = "write: resume channel, read: channel not suspended"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SUSP_A::B_0x0
    }
    #[doc = "write: suspend channel, read: channel suspended."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SUSP_A::B_0x1
    }
}
#[doc = "Field `SUSP` writer - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel (channel with an ongoing GPDMA transfer over its master ports). The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in Figure 43."]
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG, SUSP_A>;
impl<'a, REG> SUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write: resume channel, read: channel not suspended"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SUSP_A::B_0x0)
    }
    #[doc = "write: suspend channel, read: channel suspended."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SUSP_A::B_0x1)
    }
}
#[doc = "transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE_A>;
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::B_0x0,
            true => TCIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TCIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TCIE_A::B_0x1
    }
}
#[doc = "Field `TCIE` writer - transfer complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE_A>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE_A::B_0x0)
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE_A::B_0x1)
    }
}
#[doc = "half transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<HTIE_A> for bool {
    #[inline(always)]
    fn from(variant: HTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIE` reader - half transfer complete interrupt enable"]
pub type HTIE_R = crate::BitReader<HTIE_A>;
impl HTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTIE_A {
        match self.bits {
            false => HTIE_A::B_0x0,
            true => HTIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == HTIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == HTIE_A::B_0x1
    }
}
#[doc = "Field `HTIE` writer - half transfer complete interrupt enable"]
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG, HTIE_A>;
impl<'a, REG> HTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE_A::B_0x0)
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE_A::B_0x1)
    }
}
#[doc = "data transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<DTEIE_A> for bool {
    #[inline(always)]
    fn from(variant: DTEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEIE` reader - data transfer error interrupt enable"]
pub type DTEIE_R = crate::BitReader<DTEIE_A>;
impl DTEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTEIE_A {
        match self.bits {
            false => DTEIE_A::B_0x0,
            true => DTEIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DTEIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DTEIE_A::B_0x1
    }
}
#[doc = "Field `DTEIE` writer - data transfer error interrupt enable"]
pub type DTEIE_W<'a, REG> = crate::BitWriter<'a, REG, DTEIE_A>;
impl<'a, REG> DTEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DTEIE_A::B_0x0)
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DTEIE_A::B_0x1)
    }
}
#[doc = "update link transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULEIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<ULEIE_A> for bool {
    #[inline(always)]
    fn from(variant: ULEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULEIE` reader - update link transfer error interrupt enable"]
pub type ULEIE_R = crate::BitReader<ULEIE_A>;
impl ULEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ULEIE_A {
        match self.bits {
            false => ULEIE_A::B_0x0,
            true => ULEIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ULEIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ULEIE_A::B_0x1
    }
}
#[doc = "Field `ULEIE` writer - update link transfer error interrupt enable"]
pub type ULEIE_W<'a, REG> = crate::BitWriter<'a, REG, ULEIE_A>;
impl<'a, REG> ULEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ULEIE_A::B_0x0)
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ULEIE_A::B_0x1)
    }
}
#[doc = "user setting error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USEIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<USEIE_A> for bool {
    #[inline(always)]
    fn from(variant: USEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEIE` reader - user setting error interrupt enable"]
pub type USEIE_R = crate::BitReader<USEIE_A>;
impl USEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USEIE_A {
        match self.bits {
            false => USEIE_A::B_0x0,
            true => USEIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == USEIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == USEIE_A::B_0x1
    }
}
#[doc = "Field `USEIE` writer - user setting error interrupt enable"]
pub type USEIE_W<'a, REG> = crate::BitWriter<'a, REG, USEIE_A>;
impl<'a, REG> USEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(USEIE_A::B_0x0)
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(USEIE_A::B_0x1)
    }
}
#[doc = "completed suspension interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<SUSPIE_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPIE` reader - completed suspension interrupt enable"]
pub type SUSPIE_R = crate::BitReader<SUSPIE_A>;
impl SUSPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SUSPIE_A {
        match self.bits {
            false => SUSPIE_A::B_0x0,
            true => SUSPIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SUSPIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SUSPIE_A::B_0x1
    }
}
#[doc = "Field `SUSPIE` writer - completed suspension interrupt enable"]
pub type SUSPIE_W<'a, REG> = crate::BitWriter<'a, REG, SUSPIE_A>;
impl<'a, REG> SUSPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPIE_A::B_0x0)
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SUSPIE_A::B_0x1)
    }
}
#[doc = "trigger overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOIE_A {
    #[doc = "0: interrupt disabled"]
    B_0x0 = 0,
    #[doc = "1: interrupt enabled"]
    B_0x1 = 1,
}
impl From<TOIE_A> for bool {
    #[inline(always)]
    fn from(variant: TOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOIE` reader - trigger overrun interrupt enable"]
pub type TOIE_R = crate::BitReader<TOIE_A>;
impl TOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TOIE_A {
        match self.bits {
            false => TOIE_A::B_0x0,
            true => TOIE_A::B_0x1,
        }
    }
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TOIE_A::B_0x0
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TOIE_A::B_0x1
    }
}
#[doc = "Field `TOIE` writer - trigger overrun interrupt enable"]
pub type TOIE_W<'a, REG> = crate::BitWriter<'a, REG, TOIE_A>;
impl<'a, REG> TOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "interrupt disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TOIE_A::B_0x0)
    }
    #[doc = "interrupt enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TOIE_A::B_0x1)
    }
}
#[doc = "Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\] = 0 and GPDMA_CxBR1.BRC\\[10:0\\] = 0. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSM_A {
    #[doc = "0: channel executed for the full linked-list and completed at the end of the last LLI (GPDMA_CxLLR = 0). The 16 low-significant bits of the link address are null (LA\\[15:0\\] = 0) and all the update bits are null (UT1 =UB1 = UT2 = USA = UDA = ULL = 0 and UT3 = UB2 = 0). Then GPDMA_CxBR1.BNDT\\[15:0\\] = 0 and GPDMA_CxBR1.BRC\\[10:0\\] = 0."]
    B_0x0 = 0,
    #[doc = "1: channel executed once for the current LLI"]
    B_0x1 = 1,
}
impl From<LSM_A> for bool {
    #[inline(always)]
    fn from(variant: LSM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSM` reader - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\] = 0 and GPDMA_CxBR1.BRC\\[10:0\\] = 0. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LSM_R = crate::BitReader<LSM_A>;
impl LSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSM_A {
        match self.bits {
            false => LSM_A::B_0x0,
            true => LSM_A::B_0x1,
        }
    }
    #[doc = "channel executed for the full linked-list and completed at the end of the last LLI (GPDMA_CxLLR = 0). The 16 low-significant bits of the link address are null (LA\\[15:0\\] = 0) and all the update bits are null (UT1 =UB1 = UT2 = USA = UDA = ULL = 0 and UT3 = UB2 = 0). Then GPDMA_CxBR1.BNDT\\[15:0\\] = 0 and GPDMA_CxBR1.BRC\\[10:0\\] = 0."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LSM_A::B_0x0
    }
    #[doc = "channel executed once for the current LLI"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LSM_A::B_0x1
    }
}
#[doc = "Field `LSM` writer - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\] = 0 and GPDMA_CxBR1.BRC\\[10:0\\] = 0. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LSM_W<'a, REG> = crate::BitWriter<'a, REG, LSM_A>;
impl<'a, REG> LSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "channel executed for the full linked-list and completed at the end of the last LLI (GPDMA_CxLLR = 0). The 16 low-significant bits of the link address are null (LA\\[15:0\\] = 0) and all the update bits are null (UT1 =UB1 = UT2 = USA = UDA = ULL = 0 and UT3 = UB2 = 0). Then GPDMA_CxBR1.BNDT\\[15:0\\] = 0 and GPDMA_CxBR1.BRC\\[10:0\\] = 0."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSM_A::B_0x0)
    }
    #[doc = "channel executed once for the current LLI"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSM_A::B_0x1)
    }
}
#[doc = "linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LAP_A {
    #[doc = "0: port 0 (AHB) allocated"]
    B_0x0 = 0,
    #[doc = "1: port 1 (AHB) allocated"]
    B_0x1 = 1,
}
impl From<LAP_A> for bool {
    #[inline(always)]
    fn from(variant: LAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LAP` reader - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LAP_R = crate::BitReader<LAP_A>;
impl LAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LAP_A {
        match self.bits {
            false => LAP_A::B_0x0,
            true => LAP_A::B_0x1,
        }
    }
    #[doc = "port 0 (AHB) allocated"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LAP_A::B_0x0
    }
    #[doc = "port 1 (AHB) allocated"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LAP_A::B_0x1
    }
}
#[doc = "Field `LAP` writer - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
pub type LAP_W<'a, REG> = crate::BitWriter<'a, REG, LAP_A>;
impl<'a, REG> LAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "port 0 (AHB) allocated"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LAP_A::B_0x0)
    }
    #[doc = "port 1 (AHB) allocated"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LAP_A::B_0x1)
    }
}
#[doc = "priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIO_A {
    #[doc = "0: low priority, low weight"]
    B_0x0 = 0,
    #[doc = "1: low priority, mid weight"]
    B_0x1 = 1,
    #[doc = "2: low priority, high weight"]
    B_0x2 = 2,
    #[doc = "3: high priority"]
    B_0x3 = 3,
}
impl From<PRIO_A> for u8 {
    #[inline(always)]
    fn from(variant: PRIO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRIO_A {
    type Ux = u8;
}
impl crate::IsEnum for PRIO_A {}
#[doc = "Field `PRIO` reader - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type PRIO_R = crate::FieldReader<PRIO_A>;
impl PRIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIO_A {
        match self.bits {
            0 => PRIO_A::B_0x0,
            1 => PRIO_A::B_0x1,
            2 => PRIO_A::B_0x2,
            3 => PRIO_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "low priority, low weight"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRIO_A::B_0x0
    }
    #[doc = "low priority, mid weight"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRIO_A::B_0x1
    }
    #[doc = "low priority, high weight"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PRIO_A::B_0x2
    }
    #[doc = "high priority"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PRIO_A::B_0x3
    }
}
#[doc = "Field `PRIO` writer - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
pub type PRIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PRIO_A, crate::Safe>;
impl<'a, REG> PRIO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "low priority, low weight"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO_A::B_0x0)
    }
    #[doc = "low priority, mid weight"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO_A::B_0x1)
    }
    #[doc = "low priority, high weight"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO_A::B_0x2)
    }
    #[doc = "high priority"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO_A::B_0x3)
    }
}
impl R {
    #[doc = "Bit 0 - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is deasserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, for example if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
    #[inline(always)]
    pub fn EN(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel (channel with an ongoing GPDMA transfer over its master ports). The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in Figure 43."]
    #[inline(always)]
    pub fn SUSP(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - transfer complete interrupt enable"]
    #[inline(always)]
    pub fn TCIE(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - half transfer complete interrupt enable"]
    #[inline(always)]
    pub fn HTIE(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - data transfer error interrupt enable"]
    #[inline(always)]
    pub fn DTEIE(&self) -> DTEIE_R {
        DTEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - update link transfer error interrupt enable"]
    #[inline(always)]
    pub fn ULEIE(&self) -> ULEIE_R {
        ULEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - user setting error interrupt enable"]
    #[inline(always)]
    pub fn USEIE(&self) -> USEIE_R {
        USEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - completed suspension interrupt enable"]
    #[inline(always)]
    pub fn SUSPIE(&self) -> SUSPIE_R {
        SUSPIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn TOIE(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\] = 0 and GPDMA_CxBR1.BRC\\[10:0\\] = 0. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    pub fn LSM(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    pub fn LAP(&self) -> LAP_R {
        LAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 22:23 - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn PRIO(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is deasserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, for example if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
    #[inline(always)]
    pub fn EN(&mut self) -> EN_W<'_, C3CR_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - reset This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (GPDMA_CxSR.SUSPF = 1 and GPDMA_CxSR.IDLEF = GPDMA_CxCR.EN = 1) - channel in disabled state (GPDMA_CxSR.IDLEF = 1 and GPDMA_CxCR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (GPDMA_CxBR1, GPDMA_CxSAR and GPDMA_CxDAR) before enabling again the channel (see the programming sequence in Figure 44)."]
    #[inline(always)]
    pub fn RESET(&mut self) -> RESET_W<'_, C3CR_SPEC> {
        RESET_W::new(self, 1)
    }
    #[doc = "Bit 2 - suspend Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel (channel with an ongoing GPDMA transfer over its master ports). The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in Figure 43."]
    #[inline(always)]
    pub fn SUSP(&mut self) -> SUSP_W<'_, C3CR_SPEC> {
        SUSP_W::new(self, 2)
    }
    #[doc = "Bit 8 - transfer complete interrupt enable"]
    #[inline(always)]
    pub fn TCIE(&mut self) -> TCIE_W<'_, C3CR_SPEC> {
        TCIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - half transfer complete interrupt enable"]
    #[inline(always)]
    pub fn HTIE(&mut self) -> HTIE_W<'_, C3CR_SPEC> {
        HTIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - data transfer error interrupt enable"]
    #[inline(always)]
    pub fn DTEIE(&mut self) -> DTEIE_W<'_, C3CR_SPEC> {
        DTEIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - update link transfer error interrupt enable"]
    #[inline(always)]
    pub fn ULEIE(&mut self) -> ULEIE_W<'_, C3CR_SPEC> {
        ULEIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - user setting error interrupt enable"]
    #[inline(always)]
    pub fn USEIE(&mut self) -> USEIE_W<'_, C3CR_SPEC> {
        USEIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - completed suspension interrupt enable"]
    #[inline(always)]
    pub fn SUSPIE(&mut self) -> SUSPIE_W<'_, C3CR_SPEC> {
        SUSPIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn TOIE(&mut self) -> TOIE_W<'_, C3CR_SPEC> {
        TOIE_W::new(self, 14)
    }
    #[doc = "Bit 16 - Link step mode First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until GPDMA_CxBR1.BNDT\\[15:0\\] = 0 and GPDMA_CxBR1.BRC\\[10:0\\] = 0. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by GPDMA_CxLLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    pub fn LSM(&mut self) -> LSM_W<'_, C3CR_SPEC> {
        LSM_W::new(self, 16)
    }
    #[doc = "Bit 17 - linked-list allocated port This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
    #[inline(always)]
    pub fn LAP(&mut self) -> LAP_W<'_, C3CR_SPEC> {
        LAP_W::new(self, 17)
    }
    #[doc = "Bits 22:23 - priority level of the channel x GPDMA transfer versus others Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
    #[inline(always)]
    pub fn PRIO(&mut self) -> PRIO_W<'_, C3CR_SPEC> {
        PRIO_W::new(self, 22)
    }
}
#[doc = "GPDMA channel 3 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`c3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3CR_SPEC;
impl crate::RegisterSpec for C3CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c3cr::R`](R) reader structure"]
impl crate::Readable for C3CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c3cr::W`](W) writer structure"]
impl crate::Writable for C3CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets C3CR to value 0"]
impl crate::Resettable for C3CR_SPEC {}
