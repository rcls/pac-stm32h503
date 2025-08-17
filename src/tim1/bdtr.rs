#[doc = "Register `BDTR` reader"]
pub type R = crate::R<BDTR_SPEC>;
#[doc = "Register `BDTR` writer"]
pub type W = crate::W<BDTR_SPEC>;
#[doc = "Field `DTG` reader - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx = DT=DTG\\[7:0\\]x t dtg with t dtg =t DTS . DTG\\[7:5\\]=10x = DT=(64+DTG\\[5:0\\])xt dtg with T dtg =2xt DTS . DTG\\[7:5\\]=110 = DT=(32+DTG\\[4:0\\])xt dtg with T dtg =8xt DTS . DTG\\[7:5\\]=111 = DT=(32+DTG\\[4:0\\])xt dtg with T dtg =16xt DTS . Example if T DTS =125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTG_R = crate::FieldReader;
#[doc = "Field `DTG` writer - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx = DT=DTG\\[7:0\\]x t dtg with t dtg =t DTS . DTG\\[7:5\\]=10x = DT=(64+DTG\\[5:0\\])xt dtg with T dtg =2xt DTS . DTG\\[7:5\\]=110 = DT=(32+DTG\\[4:0\\])xt dtg with T dtg =8xt DTS . DTG\\[7:5\\]=111 = DT=(32+DTG\\[4:0\\])xt dtg with T dtg =16xt DTS . Example if T DTS =125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type DTG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_A {
    #[doc = "0: LOCK OFF - No bit is write protected."]
    B_0x0 = 0,
    #[doc = "1: LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKBID/BK2BID/BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written."]
    B_0x1 = 1,
    #[doc = "2: LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
    B_0x2 = 2,
    #[doc = "3: LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
    B_0x3 = 3,
}
impl From<LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCK_A {
    type Ux = u8;
}
impl crate::IsEnum for LOCK_A {}
#[doc = "Field `LOCK` reader - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
pub type LOCK_R = crate::FieldReader<LOCK_A>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK_A {
        match self.bits {
            0 => LOCK_A::B_0x0,
            1 => LOCK_A::B_0x1,
            2 => LOCK_A::B_0x2,
            3 => LOCK_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "LOCK OFF - No bit is write protected."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == LOCK_A::B_0x0
    }
    #[doc = "LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKBID/BK2BID/BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == LOCK_A::B_0x1
    }
    #[doc = "LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == LOCK_A::B_0x2
    }
    #[doc = "LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == LOCK_A::B_0x3
    }
}
#[doc = "Field `LOCK` writer - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 2, LOCK_A, crate::Safe>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LOCK OFF - No bit is write protected."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_A::B_0x0)
    }
    #[doc = "LOCK Level 1 = DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKBID/BK2BID/BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_A::B_0x1)
    }
    #[doc = "LOCK Level 2 = LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_A::B_0x2)
    }
    #[doc = "LOCK Level 3 = LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK_A::B_0x3)
    }
}
#[doc = "Off-state selection for idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSI_A {
    #[doc = "0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic and which imposes a Hi-Z state)."]
    B_0x0 = 0,
    #[doc = "1: When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output."]
    B_0x1 = 1,
}
impl From<OSSI_A> for bool {
    #[inline(always)]
    fn from(variant: OSSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSSI` reader - Off-state selection for idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OSSI_R = crate::BitReader<OSSI_A>;
impl OSSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSSI_A {
        match self.bits {
            false => OSSI_A::B_0x0,
            true => OSSI_A::B_0x1,
        }
    }
    #[doc = "When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic and which imposes a Hi-Z state)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSSI_A::B_0x0
    }
    #[doc = "When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSSI_A::B_0x1
    }
}
#[doc = "Field `OSSI` writer - Off-state selection for idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG, OSSI_A>;
impl<'a, REG> OSSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic and which imposes a Hi-Z state)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI_A::B_0x0)
    }
    #[doc = "When inactive, OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime. The timer maintains its control over the output."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI_A::B_0x1)
    }
}
#[doc = "Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSR_A {
    #[doc = "0: When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic, which forces a Hi-Z state)."]
    B_0x0 = 0,
    #[doc = "1: When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer)."]
    B_0x1 = 1,
}
impl From<OSSR_A> for bool {
    #[inline(always)]
    fn from(variant: OSSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSSR` reader - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OSSR_R = crate::BitReader<OSSR_A>;
impl OSSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSSR_A {
        match self.bits {
            false => OSSR_A::B_0x0,
            true => OSSR_A::B_0x1,
        }
    }
    #[doc = "When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic, which forces a Hi-Z state)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OSSR_A::B_0x0
    }
    #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OSSR_A::B_0x1
    }
}
#[doc = "Field `OSSR` writer - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG, OSSR_A>;
impl<'a, REG> OSSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When inactive, OC/OCN outputs are disabled (the timer releases the output control which is taken over by the GPIO logic, which forces a Hi-Z state)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR_A::B_0x0)
    }
    #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1 (the output is still controlled by the timer)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR_A::B_0x1)
    }
}
#[doc = "Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per Figure 635: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKE_A {
    #[doc = "0: Break function disabled"]
    B_0x0 = 0,
    #[doc = "1: Break function enabled"]
    B_0x1 = 1,
}
impl From<BKE_A> for bool {
    #[inline(always)]
    fn from(variant: BKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKE` reader - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per Figure 635: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKE_R = crate::BitReader<BKE_A>;
impl BKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKE_A {
        match self.bits {
            false => BKE_A::B_0x0,
            true => BKE_A::B_0x1,
        }
    }
    #[doc = "Break function disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKE_A::B_0x0
    }
    #[doc = "Break function enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKE_A::B_0x1
    }
}
#[doc = "Field `BKE` writer - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per Figure 635: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG, BKE_A>;
impl<'a, REG> BKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break function disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKE_A::B_0x0)
    }
    #[doc = "Break function enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKE_A::B_0x1)
    }
}
#[doc = "Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP_A {
    #[doc = "0: Break input tim_brk is active low"]
    B_0x0 = 0,
    #[doc = "1: Break input tim_brk is active high"]
    B_0x1 = 1,
}
impl From<BKP_A> for bool {
    #[inline(always)]
    fn from(variant: BKP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKP` reader - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKP_R = crate::BitReader<BKP_A>;
impl BKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKP_A {
        match self.bits {
            false => BKP_A::B_0x0,
            true => BKP_A::B_0x1,
        }
    }
    #[doc = "Break input tim_brk is active low"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKP_A::B_0x0
    }
    #[doc = "Break input tim_brk is active high"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKP_A::B_0x1
    }
}
#[doc = "Field `BKP` writer - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG, BKP_A>;
impl<'a, REG> BKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input tim_brk is active low"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKP_A::B_0x0)
    }
    #[doc = "Break input tim_brk is active high"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKP_A::B_0x1)
    }
}
#[doc = "Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOE_A {
    #[doc = "0: MOE can be set only by software"]
    B_0x0 = 0,
    #[doc = "1: MOE can be set by software or automatically at the next update event (if none of the break inputs tim_brk and tim_brk2 is active)"]
    B_0x1 = 1,
}
impl From<AOE_A> for bool {
    #[inline(always)]
    fn from(variant: AOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOE` reader - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type AOE_R = crate::BitReader<AOE_A>;
impl AOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AOE_A {
        match self.bits {
            false => AOE_A::B_0x0,
            true => AOE_A::B_0x1,
        }
    }
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AOE_A::B_0x0
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if none of the break inputs tim_brk and tim_brk2 is active)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AOE_A::B_0x1
    }
}
#[doc = "Field `AOE` writer - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type AOE_W<'a, REG> = crate::BitWriter<'a, REG, AOE_A>;
impl<'a, REG> AOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AOE_A::B_0x0)
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if none of the break inputs tim_brk and tim_brk2 is active)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AOE_A::B_0x1)
    }
}
#[doc = "Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (tim_brk or tim_brk2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER)).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOE_A {
    #[doc = "0: In response to a break 2 event. OC and OCN outputs are disabled"]
    B_0x0 = 0,
    #[doc = "1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)."]
    B_0x1 = 1,
}
impl From<MOE_A> for bool {
    #[inline(always)]
    fn from(variant: MOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOE` reader - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (tim_brk or tim_brk2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER))."]
pub type MOE_R = crate::BitReader<MOE_A>;
impl MOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MOE_A {
        match self.bits {
            false => MOE_A::B_0x0,
            true => MOE_A::B_0x1,
        }
    }
    #[doc = "In response to a break 2 event. OC and OCN outputs are disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MOE_A::B_0x0
    }
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MOE_A::B_0x1
    }
}
#[doc = "Field `MOE` writer - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (tim_brk or tim_brk2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER))."]
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG, MOE_A>;
impl<'a, REG> MOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In response to a break 2 event. OC and OCN outputs are disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MOE_A::B_0x0)
    }
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MOE_A::B_0x1)
    }
}
#[doc = "Break filter This bit-field defines the frequency used to sample tim_brk input and the length of the digital filter applied to tim_brk. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BKF_A {
    #[doc = "0: No filter, tim_brk acts asynchronously"]
    B_0x0 = 0,
    #[doc = "1: f SAMPLING =f tim_ker_ck , N=2"]
    B_0x1 = 1,
    #[doc = "2: f SAMPLING =f tim_ker_ck , N=4"]
    B_0x2 = 2,
    #[doc = "3: f SAMPLING =f tim_ker_ck , N=8"]
    B_0x3 = 3,
    #[doc = "4: f SAMPLING =f DTS /2, N=6"]
    B_0x4 = 4,
    #[doc = "5: f SAMPLING =f DTS /2, N=8"]
    B_0x5 = 5,
    #[doc = "6: f SAMPLING =f DTS /4, N=6"]
    B_0x6 = 6,
    #[doc = "7: f SAMPLING =f DTS /4, N=8"]
    B_0x7 = 7,
    #[doc = "8: f SAMPLING =f DTS /8, N=6"]
    B_0x8 = 8,
    #[doc = "9: f SAMPLING =f DTS /8, N=8"]
    B_0x9 = 9,
    #[doc = "10: f SAMPLING =f DTS /16, N=5"]
    B_0xA = 10,
    #[doc = "11: f SAMPLING =f DTS /16, N=6"]
    B_0xB = 11,
    #[doc = "12: f SAMPLING =f DTS /16, N=8"]
    B_0xC = 12,
    #[doc = "13: f SAMPLING =f DTS /32, N=5"]
    B_0xD = 13,
    #[doc = "14: f SAMPLING =f DTS /32, N=6"]
    B_0xE = 14,
    #[doc = "15: f SAMPLING =f DTS /32, N=8"]
    B_0xF = 15,
}
impl From<BKF_A> for u8 {
    #[inline(always)]
    fn from(variant: BKF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BKF_A {
    type Ux = u8;
}
impl crate::IsEnum for BKF_A {}
#[doc = "Field `BKF` reader - Break filter This bit-field defines the frequency used to sample tim_brk input and the length of the digital filter applied to tim_brk. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKF_R = crate::FieldReader<BKF_A>;
impl BKF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKF_A {
        match self.bits {
            0 => BKF_A::B_0x0,
            1 => BKF_A::B_0x1,
            2 => BKF_A::B_0x2,
            3 => BKF_A::B_0x3,
            4 => BKF_A::B_0x4,
            5 => BKF_A::B_0x5,
            6 => BKF_A::B_0x6,
            7 => BKF_A::B_0x7,
            8 => BKF_A::B_0x8,
            9 => BKF_A::B_0x9,
            10 => BKF_A::B_0xA,
            11 => BKF_A::B_0xB,
            12 => BKF_A::B_0xC,
            13 => BKF_A::B_0xD,
            14 => BKF_A::B_0xE,
            15 => BKF_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, tim_brk acts asynchronously"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKF_A::B_0x0
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKF_A::B_0x1
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == BKF_A::B_0x2
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=8"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == BKF_A::B_0x3
    }
    #[doc = "f SAMPLING =f DTS /2, N=6"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == BKF_A::B_0x4
    }
    #[doc = "f SAMPLING =f DTS /2, N=8"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == BKF_A::B_0x5
    }
    #[doc = "f SAMPLING =f DTS /4, N=6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == BKF_A::B_0x6
    }
    #[doc = "f SAMPLING =f DTS /4, N=8"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == BKF_A::B_0x7
    }
    #[doc = "f SAMPLING =f DTS /8, N=6"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == BKF_A::B_0x8
    }
    #[doc = "f SAMPLING =f DTS /8, N=8"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == BKF_A::B_0x9
    }
    #[doc = "f SAMPLING =f DTS /16, N=5"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == BKF_A::B_0xA
    }
    #[doc = "f SAMPLING =f DTS /16, N=6"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == BKF_A::B_0xB
    }
    #[doc = "f SAMPLING =f DTS /16, N=8"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == BKF_A::B_0xC
    }
    #[doc = "f SAMPLING =f DTS /32, N=5"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == BKF_A::B_0xD
    }
    #[doc = "f SAMPLING =f DTS /32, N=6"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == BKF_A::B_0xE
    }
    #[doc = "f SAMPLING =f DTS /32, N=8"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == BKF_A::B_0xF
    }
}
#[doc = "Field `BKF` writer - Break filter This bit-field defines the frequency used to sample tim_brk input and the length of the digital filter applied to tim_brk. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BKF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BKF_A, crate::Safe>;
impl<'a, REG> BKF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, tim_brk acts asynchronously"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0x0)
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0x1)
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0x2)
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=8"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0x3)
    }
    #[doc = "f SAMPLING =f DTS /2, N=6"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0x4)
    }
    #[doc = "f SAMPLING =f DTS /2, N=8"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0x5)
    }
    #[doc = "f SAMPLING =f DTS /4, N=6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0x6)
    }
    #[doc = "f SAMPLING =f DTS /4, N=8"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0x7)
    }
    #[doc = "f SAMPLING =f DTS /8, N=6"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0x8)
    }
    #[doc = "f SAMPLING =f DTS /8, N=8"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0x9)
    }
    #[doc = "f SAMPLING =f DTS /16, N=5"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0xA)
    }
    #[doc = "f SAMPLING =f DTS /16, N=6"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0xB)
    }
    #[doc = "f SAMPLING =f DTS /16, N=8"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0xC)
    }
    #[doc = "f SAMPLING =f DTS /32, N=5"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0xD)
    }
    #[doc = "f SAMPLING =f DTS /32, N=6"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0xE)
    }
    #[doc = "f SAMPLING =f DTS /32, N=8"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(BKF_A::B_0xF)
    }
}
#[doc = "Break 2 filter This bit-field defines the frequency used to sample tim_brk2 input and the length of the digital filter applied to tim_brk2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BK2F_A {
    #[doc = "0: No filter, tim_brk2 acts asynchronously"]
    B_0x0 = 0,
    #[doc = "1: f SAMPLING =f tim_ker_ck , N=2"]
    B_0x1 = 1,
    #[doc = "2: f SAMPLING =f tim_ker_ck , N=4"]
    B_0x2 = 2,
    #[doc = "3: f SAMPLING =f tim_ker_ck , N=8"]
    B_0x3 = 3,
    #[doc = "4: f SAMPLING =f DTS /2, N=6"]
    B_0x4 = 4,
    #[doc = "5: f SAMPLING =f DTS /2, N=8"]
    B_0x5 = 5,
    #[doc = "6: f SAMPLING =f DTS /4, N=6"]
    B_0x6 = 6,
    #[doc = "7: f SAMPLING =f DTS /4, N=8"]
    B_0x7 = 7,
    #[doc = "8: f SAMPLING =f DTS /8, N=6"]
    B_0x8 = 8,
    #[doc = "9: f SAMPLING =f DTS /8, N=8"]
    B_0x9 = 9,
    #[doc = "10: f SAMPLING =f DTS /16, N=5"]
    B_0xA = 10,
    #[doc = "11: f SAMPLING =f DTS /16, N=6"]
    B_0xB = 11,
    #[doc = "12: f SAMPLING =f DTS /16, N=8"]
    B_0xC = 12,
    #[doc = "13: f SAMPLING =f DTS /32, N=5"]
    B_0xD = 13,
    #[doc = "14: f SAMPLING =f DTS /32, N=6"]
    B_0xE = 14,
    #[doc = "15: f SAMPLING =f DTS /32, N=8"]
    B_0xF = 15,
}
impl From<BK2F_A> for u8 {
    #[inline(always)]
    fn from(variant: BK2F_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BK2F_A {
    type Ux = u8;
}
impl crate::IsEnum for BK2F_A {}
#[doc = "Field `BK2F` reader - Break 2 filter This bit-field defines the frequency used to sample tim_brk2 input and the length of the digital filter applied to tim_brk2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2F_R = crate::FieldReader<BK2F_A>;
impl BK2F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2F_A {
        match self.bits {
            0 => BK2F_A::B_0x0,
            1 => BK2F_A::B_0x1,
            2 => BK2F_A::B_0x2,
            3 => BK2F_A::B_0x3,
            4 => BK2F_A::B_0x4,
            5 => BK2F_A::B_0x5,
            6 => BK2F_A::B_0x6,
            7 => BK2F_A::B_0x7,
            8 => BK2F_A::B_0x8,
            9 => BK2F_A::B_0x9,
            10 => BK2F_A::B_0xA,
            11 => BK2F_A::B_0xB,
            12 => BK2F_A::B_0xC,
            13 => BK2F_A::B_0xD,
            14 => BK2F_A::B_0xE,
            15 => BK2F_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, tim_brk2 acts asynchronously"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2F_A::B_0x0
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2F_A::B_0x1
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == BK2F_A::B_0x2
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=8"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == BK2F_A::B_0x3
    }
    #[doc = "f SAMPLING =f DTS /2, N=6"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == BK2F_A::B_0x4
    }
    #[doc = "f SAMPLING =f DTS /2, N=8"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == BK2F_A::B_0x5
    }
    #[doc = "f SAMPLING =f DTS /4, N=6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == BK2F_A::B_0x6
    }
    #[doc = "f SAMPLING =f DTS /4, N=8"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == BK2F_A::B_0x7
    }
    #[doc = "f SAMPLING =f DTS /8, N=6"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == BK2F_A::B_0x8
    }
    #[doc = "f SAMPLING =f DTS /8, N=8"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == BK2F_A::B_0x9
    }
    #[doc = "f SAMPLING =f DTS /16, N=5"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == BK2F_A::B_0xA
    }
    #[doc = "f SAMPLING =f DTS /16, N=6"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == BK2F_A::B_0xB
    }
    #[doc = "f SAMPLING =f DTS /16, N=8"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == BK2F_A::B_0xC
    }
    #[doc = "f SAMPLING =f DTS /32, N=5"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == BK2F_A::B_0xD
    }
    #[doc = "f SAMPLING =f DTS /32, N=6"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == BK2F_A::B_0xE
    }
    #[doc = "f SAMPLING =f DTS /32, N=8"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == BK2F_A::B_0xF
    }
}
#[doc = "Field `BK2F` writer - Break 2 filter This bit-field defines the frequency used to sample tim_brk2 input and the length of the digital filter applied to tim_brk2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type BK2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BK2F_A, crate::Safe>;
impl<'a, REG> BK2F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, tim_brk2 acts asynchronously"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0x0)
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0x1)
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0x2)
    }
    #[doc = "f SAMPLING =f tim_ker_ck , N=8"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0x3)
    }
    #[doc = "f SAMPLING =f DTS /2, N=6"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0x4)
    }
    #[doc = "f SAMPLING =f DTS /2, N=8"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0x5)
    }
    #[doc = "f SAMPLING =f DTS /4, N=6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0x6)
    }
    #[doc = "f SAMPLING =f DTS /4, N=8"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0x7)
    }
    #[doc = "f SAMPLING =f DTS /8, N=6"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0x8)
    }
    #[doc = "f SAMPLING =f DTS /8, N=8"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0x9)
    }
    #[doc = "f SAMPLING =f DTS /16, N=5"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0xA)
    }
    #[doc = "f SAMPLING =f DTS /16, N=6"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0xB)
    }
    #[doc = "f SAMPLING =f DTS /16, N=8"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0xC)
    }
    #[doc = "f SAMPLING =f DTS /32, N=5"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0xD)
    }
    #[doc = "f SAMPLING =f DTS /32, N=6"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0xE)
    }
    #[doc = "f SAMPLING =f DTS /32, N=8"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(BK2F_A::B_0xF)
    }
}
#[doc = "Break 2 enable This bit enables the complete break 2 protection (including all sources connected to bk_acth and BKIN sources, as per Figure 635: Break and Break2 circuitry overview). Note: The BRKIN2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2E_A {
    #[doc = "0: Break2 function disabled"]
    B_0x0 = 0,
    #[doc = "1: Break2 function enabled"]
    B_0x1 = 1,
}
impl From<BK2E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2E` reader - Break 2 enable This bit enables the complete break 2 protection (including all sources connected to bk_acth and BKIN sources, as per Figure 635: Break and Break2 circuitry overview). Note: The BRKIN2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BK2E_R = crate::BitReader<BK2E_A>;
impl BK2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2E_A {
        match self.bits {
            false => BK2E_A::B_0x0,
            true => BK2E_A::B_0x1,
        }
    }
    #[doc = "Break2 function disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2E_A::B_0x0
    }
    #[doc = "Break2 function enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2E_A::B_0x1
    }
}
#[doc = "Field `BK2E` writer - Break 2 enable This bit enables the complete break 2 protection (including all sources connected to bk_acth and BKIN sources, as per Figure 635: Break and Break2 circuitry overview). Note: The BRKIN2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BK2E_W<'a, REG> = crate::BitWriter<'a, REG, BK2E_A>;
impl<'a, REG> BK2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break2 function disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2E_A::B_0x0)
    }
    #[doc = "Break2 function enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2E_A::B_0x1)
    }
}
#[doc = "Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2P_A {
    #[doc = "0: Break input tim_brk2 is active low"]
    B_0x0 = 0,
    #[doc = "1: Break input tim_brk2 is active high"]
    B_0x1 = 1,
}
impl From<BK2P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BK2P` reader - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BK2P_R = crate::BitReader<BK2P_A>;
impl BK2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BK2P_A {
        match self.bits {
            false => BK2P_A::B_0x0,
            true => BK2P_A::B_0x1,
        }
    }
    #[doc = "Break input tim_brk2 is active low"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BK2P_A::B_0x0
    }
    #[doc = "Break input tim_brk2 is active high"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BK2P_A::B_0x1
    }
}
#[doc = "Field `BK2P` writer - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BK2P_W<'a, REG> = crate::BitWriter<'a, REG, BK2P_A>;
impl<'a, REG> BK2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input tim_brk2 is active low"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BK2P_A::B_0x0)
    }
    #[doc = "Break input tim_brk2 is active high"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BK2P_A::B_0x1)
    }
}
#[doc = "Break disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKDSRM_A {
    #[doc = "0: Break input tim_brk is armed"]
    B_0x0 = 0,
    #[doc = "1: Break input tim_brk is disarmed"]
    B_0x1 = 1,
}
impl From<BKDSRM_A> for bool {
    #[inline(always)]
    fn from(variant: BKDSRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKDSRM` reader - Break disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKDSRM_R = crate::BitReader<BKDSRM_A>;
impl BKDSRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKDSRM_A {
        match self.bits {
            false => BKDSRM_A::B_0x0,
            true => BKDSRM_A::B_0x1,
        }
    }
    #[doc = "Break input tim_brk is armed"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKDSRM_A::B_0x0
    }
    #[doc = "Break input tim_brk is disarmed"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKDSRM_A::B_0x1
    }
}
#[doc = "Field `BKDSRM` writer - Break disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKDSRM_W<'a, REG> = crate::BitWriter<'a, REG, BKDSRM_A>;
impl<'a, REG> BKDSRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input tim_brk is armed"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKDSRM_A::B_0x0)
    }
    #[doc = "Break input tim_brk is disarmed"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKDSRM_A::B_0x1)
    }
}
#[doc = "Field `BK2DSRM` reader - Break2 disarm Refer to BKDSRM description"]
pub type BK2DSRM_R = crate::BitReader;
#[doc = "Field `BK2DSRM` writer - Break2 disarm Refer to BKDSRM description"]
pub type BK2DSRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Break bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKBID_A {
    #[doc = "0: Break input tim_brk in input mode"]
    B_0x0 = 0,
    #[doc = "1: Break input tim_brk in bidirectional mode"]
    B_0x1 = 1,
}
impl From<BKBID_A> for bool {
    #[inline(always)]
    fn from(variant: BKBID_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKBID` reader - Break bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKBID_R = crate::BitReader<BKBID_A>;
impl BKBID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKBID_A {
        match self.bits {
            false => BKBID_A::B_0x0,
            true => BKBID_A::B_0x1,
        }
    }
    #[doc = "Break input tim_brk in input mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKBID_A::B_0x0
    }
    #[doc = "Break input tim_brk in bidirectional mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKBID_A::B_0x1
    }
}
#[doc = "Field `BKBID` writer - Break bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKBID_W<'a, REG> = crate::BitWriter<'a, REG, BKBID_A>;
impl<'a, REG> BKBID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input tim_brk in input mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKBID_A::B_0x0)
    }
    #[doc = "Break input tim_brk in bidirectional mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKBID_A::B_0x1)
    }
}
#[doc = "Field `BK2BID` reader - Break2 bidirectional Refer to BKBID description"]
pub type BK2BID_R = crate::BitReader;
#[doc = "Field `BK2BID` writer - Break2 bidirectional Refer to BKBID description"]
pub type BK2BID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx = DT=DTG\\[7:0\\]x t dtg with t dtg =t DTS . DTG\\[7:5\\]=10x = DT=(64+DTG\\[5:0\\])xt dtg with T dtg =2xt DTS . DTG\\[7:5\\]=110 = DT=(32+DTG\\[4:0\\])xt dtg with T dtg =8xt DTS . DTG\\[7:5\\]=111 = DT=(32+DTG\\[4:0\\])xt dtg with T dtg =16xt DTS . Example if T DTS =125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn DTG(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
    #[inline(always)]
    pub fn LOCK(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn OSSI(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn OSSR(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per Figure 635: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn BKE(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn BKP(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn AOE(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (tim_brk or tim_brk2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER))."]
    #[inline(always)]
    pub fn MOE(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Break filter This bit-field defines the frequency used to sample tim_brk input and the length of the digital filter applied to tim_brk. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKF(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Break 2 filter This bit-field defines the frequency used to sample tim_brk2 input and the length of the digital filter applied to tim_brk2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2F(&self) -> BK2F_R {
        BK2F_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Break 2 enable This bit enables the complete break 2 protection (including all sources connected to bk_acth and BKIN sources, as per Figure 635: Break and Break2 circuitry overview). Note: The BRKIN2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn BK2E(&self) -> BK2E_R {
        BK2E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn BK2P(&self) -> BK2P_R {
        BK2P_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Break disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn BKDSRM(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Break2 disarm Refer to BKDSRM description"]
    #[inline(always)]
    pub fn BK2DSRM(&self) -> BK2DSRM_R {
        BK2DSRM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Break bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn BKBID(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Break2 bidirectional Refer to BKBID description"]
    #[inline(always)]
    pub fn BK2BID(&self) -> BK2BID_R {
        BK2BID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx = DT=DTG\\[7:0\\]x t dtg with t dtg =t DTS . DTG\\[7:5\\]=10x = DT=(64+DTG\\[5:0\\])xt dtg with T dtg =2xt DTS . DTG\\[7:5\\]=110 = DT=(32+DTG\\[4:0\\])xt dtg with T dtg =8xt DTS . DTG\\[7:5\\]=111 = DT=(32+DTG\\[4:0\\])xt dtg with T dtg =16xt DTS . Example if T DTS =125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 us to 31750 ns by 250 ns steps, 32 us to 63us by 1 us steps, 64 us to 126 us by 2 us steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn DTG(&mut self) -> DTG_W<'_, BDTR_SPEC> {
        DTG_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIMx_BDTR register has been written, their content is frozen until the next reset."]
    #[inline(always)]
    pub fn LOCK(&mut self) -> LOCK_W<'_, BDTR_SPEC> {
        LOCK_W::new(self, 8)
    }
    #[doc = "Bit 10 - Off-state selection for idle mode This bit is used when MOE=0 due to a break event or by a software write, on channels configured as outputs. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn OSSI(&mut self) -> OSSI_W<'_, BDTR_SPEC> {
        OSSI_W::new(self, 10)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode This bit is used when MOE=1 on channels having a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER)). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn OSSR(&mut self) -> OSSR_W<'_, BDTR_SPEC> {
        OSSR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Break enable This bit enables the complete break protection (including all sources connected to bk_acth and BKIN sources, as per Figure 635: Break and Break2 circuitry overview). Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn BKE(&mut self) -> BKE_W<'_, BDTR_SPEC> {
        BKE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn BKP(&mut self) -> BKP_W<'_, BDTR_SPEC> {
        BKP_W::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn AOE(&mut self) -> AOE_W<'_, BDTR_SPEC> {
        AOE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as one of the break inputs is active (tim_brk or tim_brk2). It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. In response to a break event or if MOE is written to 0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit. See OC/OCN enable description for more details (Section 65.6.11: TIM1 capture/compare enable register (TIM1_CCER))."]
    #[inline(always)]
    pub fn MOE(&mut self) -> MOE_W<'_, BDTR_SPEC> {
        MOE_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Break filter This bit-field defines the frequency used to sample tim_brk input and the length of the digital filter applied to tim_brk. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BKF(&mut self) -> BKF_W<'_, BDTR_SPEC> {
        BKF_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Break 2 filter This bit-field defines the frequency used to sample tim_brk2 input and the length of the digital filter applied to tim_brk2. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn BK2F(&mut self) -> BK2F_W<'_, BDTR_SPEC> {
        BK2F_W::new(self, 20)
    }
    #[doc = "Bit 24 - Break 2 enable This bit enables the complete break 2 protection (including all sources connected to bk_acth and BKIN sources, as per Figure 635: Break and Break2 circuitry overview). Note: The BRKIN2 must only be used with OSSR = OSSI = 1. Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn BK2E(&mut self) -> BK2E_W<'_, BDTR_SPEC> {
        BK2E_W::new(self, 24)
    }
    #[doc = "Bit 25 - Break 2 polarity Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn BK2P(&mut self) -> BK2P_W<'_, BDTR_SPEC> {
        BK2P_W::new(self, 25)
    }
    #[doc = "Bit 26 - Break disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled it until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn BKDSRM(&mut self) -> BKDSRM_W<'_, BDTR_SPEC> {
        BKDSRM_W::new(self, 26)
    }
    #[doc = "Bit 27 - Break2 disarm Refer to BKDSRM description"]
    #[inline(always)]
    pub fn BK2DSRM(&mut self) -> BK2DSRM_W<'_, BDTR_SPEC> {
        BK2DSRM_W::new(self, 27)
    }
    #[doc = "Bit 28 - Break bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn BKBID(&mut self) -> BKBID_W<'_, BDTR_SPEC> {
        BKBID_W::new(self, 28)
    }
    #[doc = "Bit 29 - Break2 bidirectional Refer to BKBID description"]
    #[inline(always)]
    pub fn BK2BID(&mut self) -> BK2BID_W<'_, BDTR_SPEC> {
        BK2BID_W::new(self, 29)
    }
}
#[doc = "TIM1 break and dead-time register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDTR_SPEC;
impl crate::RegisterSpec for BDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdtr::R`](R) reader structure"]
impl crate::Readable for BDTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bdtr::W`](W) writer structure"]
impl crate::Writable for BDTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets BDTR to value 0"]
impl crate::Resettable for BDTR_SPEC {}
