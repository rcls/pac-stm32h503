#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to Section 65.6.3: TIM1 slave mode control register (TIM1_SMCR)), if URS=0 and UDIS=0 in the TIMx_CR1 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIF_A {
    #[doc = "0: No update occurred."]
    B_0x0 = 0,
    #[doc = "1: Update interrupt pending. This bit is set by hardware when the registers are updated:"]
    B_0x1 = 1,
}
impl From<UIF_A> for bool {
    #[inline(always)]
    fn from(variant: UIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to Section 65.6.3: TIM1 slave mode control register (TIM1_SMCR)), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
pub type UIF_R = crate::BitReader<UIF_A>;
impl UIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UIF_A {
        match self.bits {
            false => UIF_A::B_0x0,
            true => UIF_A::B_0x1,
        }
    }
    #[doc = "No update occurred."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == UIF_A::B_0x0
    }
    #[doc = "Update interrupt pending. This bit is set by hardware when the registers are updated:"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == UIF_A::B_0x1
    }
}
#[doc = "Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to Section 65.6.3: TIM1 slave mode control register (TIM1_SMCR)), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG, UIF_A>;
impl<'a, REG> UIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No update occurred."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UIF_A::B_0x0)
    }
    #[doc = "Update interrupt pending. This bit is set by hardware when the registers are updated:"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UIF_A::B_0x1)
    }
}
#[doc = "Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IF_A {
    #[doc = "0: No compare match / No input capture occurred"]
    B_0x0 = 0,
    #[doc = "1: A compare match or an input capture occurred"]
    B_0x1 = 1,
}
impl From<CC1IF_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1IF` reader - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
pub type CC1IF_R = crate::BitReader<CC1IF_A>;
impl CC1IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1IF_A {
        match self.bits {
            false => CC1IF_A::B_0x0,
            true => CC1IF_A::B_0x1,
        }
    }
    #[doc = "No compare match / No input capture occurred"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC1IF_A::B_0x0
    }
    #[doc = "A compare match or an input capture occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC1IF_A::B_0x1
    }
}
#[doc = "Field `CC1IF` writer - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG, CC1IF_A>;
impl<'a, REG> CC1IF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No compare match / No input capture occurred"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IF_A::B_0x0)
    }
    #[doc = "A compare match or an input capture occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IF_A::B_0x1)
    }
}
#[doc = "Field `CC2IF` reader - Capture/compare 2 interrupt flag Refer to CC1IF description"]
pub type CC2IF_R = crate::BitReader;
#[doc = "Field `CC2IF` writer - Capture/compare 2 interrupt flag Refer to CC1IF description"]
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IF` reader - Capture/compare 3 interrupt flag Refer to CC1IF description"]
pub type CC3IF_R = crate::BitReader;
#[doc = "Field `CC3IF` writer - Capture/compare 3 interrupt flag Refer to CC1IF description"]
pub type CC3IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IF` reader - Capture/compare 4 interrupt flag Refer to CC1IF description"]
pub type CC4IF_R = crate::BitReader;
#[doc = "Field `CC4IF` writer - Capture/compare 4 interrupt flag Refer to CC1IF description"]
pub type CC4IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMIF_A {
    #[doc = "0: No COM event occurred."]
    B_0x0 = 0,
    #[doc = "1: COM interrupt pending."]
    B_0x1 = 1,
}
impl From<COMIF_A> for bool {
    #[inline(always)]
    fn from(variant: COMIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMIF` reader - COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
pub type COMIF_R = crate::BitReader<COMIF_A>;
impl COMIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMIF_A {
        match self.bits {
            false => COMIF_A::B_0x0,
            true => COMIF_A::B_0x1,
        }
    }
    #[doc = "No COM event occurred."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == COMIF_A::B_0x0
    }
    #[doc = "COM interrupt pending."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == COMIF_A::B_0x1
    }
}
#[doc = "Field `COMIF` writer - COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
pub type COMIF_W<'a, REG> = crate::BitWriter<'a, REG, COMIF_A>;
impl<'a, REG> COMIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No COM event occurred."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMIF_A::B_0x0)
    }
    #[doc = "COM interrupt pending."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMIF_A::B_0x1)
    }
}
#[doc = "Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIF_A {
    #[doc = "0: No trigger event occurred."]
    B_0x0 = 0,
    #[doc = "1: Trigger interrupt pending."]
    B_0x1 = 1,
}
impl From<TIF_A> for bool {
    #[inline(always)]
    fn from(variant: TIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF` reader - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
pub type TIF_R = crate::BitReader<TIF_A>;
impl TIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIF_A {
        match self.bits {
            false => TIF_A::B_0x0,
            true => TIF_A::B_0x1,
        }
    }
    #[doc = "No trigger event occurred."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TIF_A::B_0x0
    }
    #[doc = "Trigger interrupt pending."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TIF_A::B_0x1
    }
}
#[doc = "Field `TIF` writer - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG, TIF_A>;
impl<'a, REG> TIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No trigger event occurred."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIF_A::B_0x0)
    }
    #[doc = "Trigger interrupt pending."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIF_A::B_0x1)
    }
}
#[doc = "Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIF_A {
    #[doc = "0: No break event occurred."]
    B_0x0 = 0,
    #[doc = "1: An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    B_0x1 = 1,
}
impl From<BIF_A> for bool {
    #[inline(always)]
    fn from(variant: BIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIF` reader - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
pub type BIF_R = crate::BitReader<BIF_A>;
impl BIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIF_A {
        match self.bits {
            false => BIF_A::B_0x0,
            true => BIF_A::B_0x1,
        }
    }
    #[doc = "No break event occurred."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BIF_A::B_0x0
    }
    #[doc = "An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BIF_A::B_0x1
    }
}
#[doc = "Field `BIF` writer - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
pub type BIF_W<'a, REG> = crate::BitWriter<'a, REG, BIF_A>;
impl<'a, REG> BIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No break event occurred."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BIF_A::B_0x0)
    }
    #[doc = "An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BIF_A::B_0x1)
    }
}
#[doc = "Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B2IF_A {
    #[doc = "0: No break event occurred."]
    B_0x0 = 0,
    #[doc = "1: An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    B_0x1 = 1,
}
impl From<B2IF_A> for bool {
    #[inline(always)]
    fn from(variant: B2IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B2IF` reader - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
pub type B2IF_R = crate::BitReader<B2IF_A>;
impl B2IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B2IF_A {
        match self.bits {
            false => B2IF_A::B_0x0,
            true => B2IF_A::B_0x1,
        }
    }
    #[doc = "No break event occurred."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == B2IF_A::B_0x0
    }
    #[doc = "An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == B2IF_A::B_0x1
    }
}
#[doc = "Field `B2IF` writer - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
pub type B2IF_W<'a, REG> = crate::BitWriter<'a, REG, B2IF_A>;
impl<'a, REG> B2IF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No break event occurred."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(B2IF_A::B_0x0)
    }
    #[doc = "An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(B2IF_A::B_0x1)
    }
}
#[doc = "Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OF_A {
    #[doc = "0: No overcapture has been detected."]
    B_0x0 = 0,
    #[doc = "1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set"]
    B_0x1 = 1,
}
impl From<CC1OF_A> for bool {
    #[inline(always)]
    fn from(variant: CC1OF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1OF` reader - Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'."]
pub type CC1OF_R = crate::BitReader<CC1OF_A>;
impl CC1OF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1OF_A {
        match self.bits {
            false => CC1OF_A::B_0x0,
            true => CC1OF_A::B_0x1,
        }
    }
    #[doc = "No overcapture has been detected."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC1OF_A::B_0x0
    }
    #[doc = "The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC1OF_A::B_0x1
    }
}
#[doc = "Field `CC1OF` writer - Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'."]
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG, CC1OF_A>;
impl<'a, REG> CC1OF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overcapture has been detected."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1OF_A::B_0x0)
    }
    #[doc = "The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1OF_A::B_0x1)
    }
}
#[doc = "Field `CC2OF` reader - Capture/compare 2 overcapture flag Refer to CC1OF description"]
pub type CC2OF_R = crate::BitReader;
#[doc = "Field `CC2OF` writer - Capture/compare 2 overcapture flag Refer to CC1OF description"]
pub type CC2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3OF` reader - Capture/compare 3 overcapture flag Refer to CC1OF description"]
pub type CC3OF_R = crate::BitReader;
#[doc = "Field `CC3OF` writer - Capture/compare 3 overcapture flag Refer to CC1OF description"]
pub type CC3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4OF` reader - Capture/compare 4 overcapture flag Refer to CC1OF description"]
pub type CC4OF_R = crate::BitReader;
#[doc = "Field `CC4OF` writer - Capture/compare 4 overcapture flag Refer to CC1OF description"]
pub type CC4OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBIF_A {
    #[doc = "0: No break event occurred."]
    B_0x0 = 0,
    #[doc = "1: An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    B_0x1 = 1,
}
impl From<SBIF_A> for bool {
    #[inline(always)]
    fn from(variant: SBIF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBIF` reader - System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
pub type SBIF_R = crate::BitReader<SBIF_A>;
impl SBIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBIF_A {
        match self.bits {
            false => SBIF_A::B_0x0,
            true => SBIF_A::B_0x1,
        }
    }
    #[doc = "No break event occurred."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SBIF_A::B_0x0
    }
    #[doc = "An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SBIF_A::B_0x1
    }
}
#[doc = "Field `SBIF` writer - System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
pub type SBIF_W<'a, REG> = crate::BitWriter<'a, REG, SBIF_A>;
impl<'a, REG> SBIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No break event occurred."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SBIF_A::B_0x0)
    }
    #[doc = "An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SBIF_A::B_0x1)
    }
}
#[doc = "Field `CC5IF` reader - Compare 5 interrupt flag Refer to CC1IF description Note: Channel 5 can only be configured as output."]
pub type CC5IF_R = crate::BitReader;
#[doc = "Field `CC5IF` writer - Compare 5 interrupt flag Refer to CC1IF description Note: Channel 5 can only be configured as output."]
pub type CC5IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6IF` reader - Compare 6 interrupt flag Refer to CC1IF description Note: Channel 6 can only be configured as output."]
pub type CC6IF_R = crate::BitReader;
#[doc = "Field `CC6IF` writer - Compare 6 interrupt flag Refer to CC1IF description Note: Channel 6 can only be configured as output."]
pub type CC6IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to '0'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDXF_A {
    #[doc = "0: No index event occurred."]
    B_0x0 = 0,
    #[doc = "1: An index event has occurred"]
    B_0x1 = 1,
}
impl From<IDXF_A> for bool {
    #[inline(always)]
    fn from(variant: IDXF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDXF` reader - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to '0'."]
pub type IDXF_R = crate::BitReader<IDXF_A>;
impl IDXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDXF_A {
        match self.bits {
            false => IDXF_A::B_0x0,
            true => IDXF_A::B_0x1,
        }
    }
    #[doc = "No index event occurred."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IDXF_A::B_0x0
    }
    #[doc = "An index event has occurred"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IDXF_A::B_0x1
    }
}
#[doc = "Field `IDXF` writer - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to '0'."]
pub type IDXF_W<'a, REG> = crate::BitWriter<'a, REG, IDXF_A>;
impl<'a, REG> IDXF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No index event occurred."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IDXF_A::B_0x0)
    }
    #[doc = "An index event has occurred"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IDXF_A::B_0x1)
    }
}
#[doc = "Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to '0'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRF_A {
    #[doc = "0: No direction change"]
    B_0x0 = 0,
    #[doc = "1: Direction change"]
    B_0x1 = 1,
}
impl From<DIRF_A> for bool {
    #[inline(always)]
    fn from(variant: DIRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRF` reader - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to '0'."]
pub type DIRF_R = crate::BitReader<DIRF_A>;
impl DIRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIRF_A {
        match self.bits {
            false => DIRF_A::B_0x0,
            true => DIRF_A::B_0x1,
        }
    }
    #[doc = "No direction change"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DIRF_A::B_0x0
    }
    #[doc = "Direction change"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DIRF_A::B_0x1
    }
}
#[doc = "Field `DIRF` writer - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to '0'."]
pub type DIRF_W<'a, REG> = crate::BitWriter<'a, REG, DIRF_A>;
impl<'a, REG> DIRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No direction change"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DIRF_A::B_0x0)
    }
    #[doc = "Direction change"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DIRF_A::B_0x1)
    }
}
#[doc = "Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to '0'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IERRF_A {
    #[doc = "0: No index error has been detected."]
    B_0x0 = 0,
    #[doc = "1: An index error has been detected"]
    B_0x1 = 1,
}
impl From<IERRF_A> for bool {
    #[inline(always)]
    fn from(variant: IERRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IERRF` reader - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to '0'."]
pub type IERRF_R = crate::BitReader<IERRF_A>;
impl IERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IERRF_A {
        match self.bits {
            false => IERRF_A::B_0x0,
            true => IERRF_A::B_0x1,
        }
    }
    #[doc = "No index error has been detected."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IERRF_A::B_0x0
    }
    #[doc = "An index error has been detected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IERRF_A::B_0x1
    }
}
#[doc = "Field `IERRF` writer - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to '0'."]
pub type IERRF_W<'a, REG> = crate::BitWriter<'a, REG, IERRF_A>;
impl<'a, REG> IERRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No index error has been detected."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IERRF_A::B_0x0)
    }
    #[doc = "An index error has been detected"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IERRF_A::B_0x1)
    }
}
#[doc = "Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to '0'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERRF_A {
    #[doc = "0: No encoder transition error has been detected."]
    B_0x0 = 0,
    #[doc = "1: An encoder transition error has been detected"]
    B_0x1 = 1,
}
impl From<TERRF_A> for bool {
    #[inline(always)]
    fn from(variant: TERRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERRF` reader - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to '0'."]
pub type TERRF_R = crate::BitReader<TERRF_A>;
impl TERRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TERRF_A {
        match self.bits {
            false => TERRF_A::B_0x0,
            true => TERRF_A::B_0x1,
        }
    }
    #[doc = "No encoder transition error has been detected."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TERRF_A::B_0x0
    }
    #[doc = "An encoder transition error has been detected"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TERRF_A::B_0x1
    }
}
#[doc = "Field `TERRF` writer - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to '0'."]
pub type TERRF_W<'a, REG> = crate::BitWriter<'a, REG, TERRF_A>;
impl<'a, REG> TERRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No encoder transition error has been detected."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TERRF_A::B_0x0)
    }
    #[doc = "An encoder transition error has been detected"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TERRF_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to Section 65.6.3: TIM1 slave mode control register (TIM1_SMCR)), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    pub fn UIF(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC1IF(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare 2 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn CC2IF(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare 3 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn CC3IF(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/compare 4 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn CC4IF(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
    #[inline(always)]
    pub fn COMIF(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
    #[inline(always)]
    pub fn TIF(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
    #[inline(always)]
    pub fn BIF(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
    #[inline(always)]
    pub fn B2IF(&self) -> B2IF_R {
        B2IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'."]
    #[inline(always)]
    pub fn CC1OF(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn CC2OF(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/compare 3 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn CC3OF(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/compare 4 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn CC4OF(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
    #[inline(always)]
    pub fn SBIF(&self) -> SBIF_R {
        SBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag Refer to CC1IF description Note: Channel 5 can only be configured as output."]
    #[inline(always)]
    pub fn CC5IF(&self) -> CC5IF_R {
        CC5IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Compare 6 interrupt flag Refer to CC1IF description Note: Channel 6 can only be configured as output."]
    #[inline(always)]
    pub fn CC6IF(&self) -> CC6IF_R {
        CC6IF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to '0'."]
    #[inline(always)]
    pub fn IDXF(&self) -> IDXF_R {
        IDXF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to '0'."]
    #[inline(always)]
    pub fn DIRF(&self) -> DIRF_R {
        DIRF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to '0'."]
    #[inline(always)]
    pub fn IERRF(&self) -> IERRF_R {
        IERRF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to '0'."]
    #[inline(always)]
    pub fn TERRF(&self) -> TERRF_R {
        TERRF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to Section 65.6.3: TIM1 slave mode control register (TIM1_SMCR)), if URS=0 and UDIS=0 in the TIMx_CR1 register."]
    #[inline(always)]
    pub fn UIF(&mut self) -> UIF_W<'_, SR_SPEC> {
        UIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when the content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in downcounting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC1IF(&mut self) -> CC1IF_W<'_, SR_SPEC> {
        CC1IF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/compare 2 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn CC2IF(&mut self) -> CC2IF_W<'_, SR_SPEC> {
        CC2IF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare 3 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn CC3IF(&mut self) -> CC3IF_W<'_, SR_SPEC> {
        CC3IF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare 4 interrupt flag Refer to CC1IF description"]
    #[inline(always)]
    pub fn CC4IF(&mut self) -> CC4IF_W<'_, SR_SPEC> {
        CC4IF_W::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt flag This flag is set by hardware on COM event (when capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software."]
    #[inline(always)]
    pub fn COMIF(&mut self) -> COMIF_W<'_, SR_SPEC> {
        COMIF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on tim_trgi input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software."]
    #[inline(always)]
    pub fn TIF(&mut self) -> TIF_W<'_, SR_SPEC> {
        TIF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active."]
    #[inline(always)]
    pub fn BIF(&mut self) -> BIF_W<'_, SR_SPEC> {
        BIF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active."]
    #[inline(always)]
    pub fn B2IF(&mut self) -> B2IF_W<'_, SR_SPEC> {
        B2IF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0'."]
    #[inline(always)]
    pub fn CC1OF(&mut self) -> CC1OF_W<'_, SR_SPEC> {
        CC1OF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn CC2OF(&mut self) -> CC2OF_W<'_, SR_SPEC> {
        CC2OF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/compare 3 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn CC3OF(&mut self) -> CC3OF_W<'_, SR_SPEC> {
        CC3OF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/compare 4 overcapture flag Refer to CC1OF description"]
    #[inline(always)]
    pub fn CC4OF(&mut self) -> CC4OF_W<'_, SR_SPEC> {
        CC4OF_W::new(self, 12)
    }
    #[doc = "Bit 13 - System break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation."]
    #[inline(always)]
    pub fn SBIF(&mut self) -> SBIF_W<'_, SR_SPEC> {
        SBIF_W::new(self, 13)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag Refer to CC1IF description Note: Channel 5 can only be configured as output."]
    #[inline(always)]
    pub fn CC5IF(&mut self) -> CC5IF_W<'_, SR_SPEC> {
        CC5IF_W::new(self, 16)
    }
    #[doc = "Bit 17 - Compare 6 interrupt flag Refer to CC1IF description Note: Channel 6 can only be configured as output."]
    #[inline(always)]
    pub fn CC6IF(&mut self) -> CC6IF_W<'_, SR_SPEC> {
        CC6IF_W::new(self, 17)
    }
    #[doc = "Bit 20 - Index interrupt flag This flag is set by hardware when an index event is detected. It is cleared by software by writing it to '0'."]
    #[inline(always)]
    pub fn IDXF(&mut self) -> IDXF_W<'_, SR_SPEC> {
        IDXF_W::new(self, 20)
    }
    #[doc = "Bit 21 - Direction change interrupt flag This flag is set by hardware when the direction changes in encoder mode (DIR bit value in TIMx_CR is changing). It is cleared by software by writing it to '0'."]
    #[inline(always)]
    pub fn DIRF(&mut self) -> DIRF_W<'_, SR_SPEC> {
        DIRF_W::new(self, 21)
    }
    #[doc = "Bit 22 - Index error interrupt flag This flag is set by hardware when an index error is detected. It is cleared by software by writing it to '0'."]
    #[inline(always)]
    pub fn IERRF(&mut self) -> IERRF_W<'_, SR_SPEC> {
        IERRF_W::new(self, 22)
    }
    #[doc = "Bit 23 - Transition error interrupt flag This flag is set by hardware when a transition error is detected in encoder mode. It is cleared by software by writing it to '0'."]
    #[inline(always)]
    pub fn TERRF(&mut self) -> TERRF_W<'_, SR_SPEC> {
        TERRF_W::new(self, 23)
    }
}
#[doc = "TIM1 status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {}
