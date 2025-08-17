#[doc = "Register `CCMR1_Output` reader"]
pub type R = crate::R<CCMR1_OUTPUT_SPEC>;
#[doc = "Register `CCMR1_Output` writer"]
pub type W = crate::W<CCMR1_OUTPUT_SPEC>;
#[doc = "Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1S_A {
    #[doc = "0: CC1 channel is configured as output."]
    B_0x0 = 0,
    #[doc = "1: CC1 channel is configured as input, tim_ic1 is mapped on tim_ti1."]
    B_0x1 = 1,
    #[doc = "2: CC1 channel is configured as input, tim_ic1 is mapped on tim_ti2."]
    B_0x2 = 2,
    #[doc = "3: CC1 channel is configured as input, tim_ic1 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B_0x3 = 3,
}
impl From<CC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1S_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC1S_A {
    type Ux = u8;
}
impl crate::IsEnum for CC1S_A {}
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type CC1S_R = crate::FieldReader<CC1S_A>;
impl CC1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1S_A {
        match self.bits {
            0 => CC1S_A::B_0x0,
            1 => CC1S_A::B_0x1,
            2 => CC1S_A::B_0x2,
            3 => CC1S_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC1 channel is configured as output."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC1S_A::B_0x0
    }
    #[doc = "CC1 channel is configured as input, tim_ic1 is mapped on tim_ti1."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC1S_A::B_0x1
    }
    #[doc = "CC1 channel is configured as input, tim_ic1 is mapped on tim_ti2."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CC1S_A::B_0x2
    }
    #[doc = "CC1 channel is configured as input, tim_ic1 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CC1S_A::B_0x3
    }
}
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC1S_A, crate::Safe>;
impl<'a, REG> CC1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC1 channel is configured as output."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S_A::B_0x0)
    }
    #[doc = "CC1 channel is configured as input, tim_ic1 is mapped on tim_ti1."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S_A::B_0x1)
    }
    #[doc = "CC1 channel is configured as input, tim_ic1 is mapped on tim_ti2."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S_A::B_0x2)
    }
    #[doc = "CC1 channel is configured as input, tim_ic1 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S_A::B_0x3)
    }
}
#[doc = "Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1FE_A {
    #[doc = "0: CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles."]
    B_0x0 = 0,
    #[doc = "1: An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
    B_0x1 = 1,
}
impl From<OC1FE_A> for bool {
    #[inline(always)]
    fn from(variant: OC1FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OC1FE` reader - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
pub type OC1FE_R = crate::BitReader<OC1FE_A>;
impl OC1FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC1FE_A {
        match self.bits {
            false => OC1FE_A::B_0x0,
            true => OC1FE_A::B_0x1,
        }
    }
    #[doc = "CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OC1FE_A::B_0x0
    }
    #[doc = "An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OC1FE_A::B_0x1
    }
}
#[doc = "Field `OC1FE` writer - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG, OC1FE_A>;
impl<'a, REG> OC1FE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC1 behaves normally depending on counter and CCR1 values even when the trigger is ON. The minimum delay to activate CC1 output when an edge occurs on the trigger input is 5 clock cycles."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OC1FE_A::B_0x0)
    }
    #[doc = "An active edge on the trigger input acts like a compare match on CC1 output. Then, OC is set to the compare level independently from the result of the comparison. Delay to sample the trigger input and to activate CC1 output is reduced to 3 clock cycles. OCFE acts only if the channel is configured in PWM1 or PWM2 mode."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OC1FE_A::B_0x1)
    }
}
#[doc = "Output compare 1 preload enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1PE_A {
    #[doc = "0: Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately."]
    B_0x0 = 0,
    #[doc = "1: Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event."]
    B_0x1 = 1,
}
impl From<OC1PE_A> for bool {
    #[inline(always)]
    fn from(variant: OC1PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OC1PE` reader - Output compare 1 preload enable"]
pub type OC1PE_R = crate::BitReader<OC1PE_A>;
impl OC1PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC1PE_A {
        match self.bits {
            false => OC1PE_A::B_0x0,
            true => OC1PE_A::B_0x1,
        }
    }
    #[doc = "Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OC1PE_A::B_0x0
    }
    #[doc = "Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OC1PE_A::B_0x1
    }
}
#[doc = "Field `OC1PE` writer - Output compare 1 preload enable"]
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG, OC1PE_A>;
impl<'a, REG> OC1PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Preload register on TIMx_CCR1 disabled. TIMx_CCR1 can be written at anytime, the new value is taken in account immediately."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OC1PE_A::B_0x0)
    }
    #[doc = "Preload register on TIMx_CCR1 enabled. Read/Write operations access the preload register. TIMx_CCR1 preload value is loaded in the active register at each update event."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OC1PE_A::B_0x1)
    }
}
#[doc = "Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OC1M1_A {
    #[doc = "0: Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    B_0x0 = 0,
    #[doc = "1: Set channel 1 to active level on match. tim_oc1ref signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    B_0x1 = 1,
    #[doc = "2: Set channel 1 to inactive level on match. tim_oc1ref signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    B_0x2 = 2,
    #[doc = "3: Toggle - tim_oc1ref toggles when TIMx_CNT=TIMx_CCR1."]
    B_0x3 = 3,
    #[doc = "4: Force inactive level - tim_oc1ref is forced low."]
    B_0x4 = 4,
    #[doc = "5: Force active level - tim_oc1ref is forced high."]
    B_0x5 = 5,
    #[doc = "6: PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNTTIMx_CCR1 else inactive. In downcounting, channel 1 is inactive (tim_oc1ref=0) as long as TIMx_CNTTIMx_CCR1 else active (tim_oc1ref=1)."]
    B_0x6 = 6,
    #[doc = "7: PWM mode 2 - In upcounting, channel 1 is inactive as long as TIMx_CNTTIMx_CCR1 else active. In downcounting, channel 1 is active as long as TIMx_CNTTIMx_CCR1 else inactive."]
    B_0x7 = 7,
}
impl From<OC1M1_A> for u8 {
    #[inline(always)]
    fn from(variant: OC1M1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OC1M1_A {
    type Ux = u8;
}
impl crate::IsEnum for OC1M1_A {}
#[doc = "Field `OC1M1` reader - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode."]
pub type OC1M1_R = crate::FieldReader<OC1M1_A>;
impl OC1M1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC1M1_A {
        match self.bits {
            0 => OC1M1_A::B_0x0,
            1 => OC1M1_A::B_0x1,
            2 => OC1M1_A::B_0x2,
            3 => OC1M1_A::B_0x3,
            4 => OC1M1_A::B_0x4,
            5 => OC1M1_A::B_0x5,
            6 => OC1M1_A::B_0x6,
            7 => OC1M1_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OC1M1_A::B_0x0
    }
    #[doc = "Set channel 1 to active level on match. tim_oc1ref signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OC1M1_A::B_0x1
    }
    #[doc = "Set channel 1 to inactive level on match. tim_oc1ref signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == OC1M1_A::B_0x2
    }
    #[doc = "Toggle - tim_oc1ref toggles when TIMx_CNT=TIMx_CCR1."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == OC1M1_A::B_0x3
    }
    #[doc = "Force inactive level - tim_oc1ref is forced low."]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == OC1M1_A::B_0x4
    }
    #[doc = "Force active level - tim_oc1ref is forced high."]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == OC1M1_A::B_0x5
    }
    #[doc = "PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNTTIMx_CCR1 else inactive. In downcounting, channel 1 is inactive (tim_oc1ref=0) as long as TIMx_CNTTIMx_CCR1 else active (tim_oc1ref=1)."]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == OC1M1_A::B_0x6
    }
    #[doc = "PWM mode 2 - In upcounting, channel 1 is inactive as long as TIMx_CNTTIMx_CCR1 else active. In downcounting, channel 1 is active as long as TIMx_CNTTIMx_CCR1 else inactive."]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == OC1M1_A::B_0x7
    }
}
#[doc = "Field `OC1M1` writer - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode."]
pub type OC1M1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, OC1M1_A, crate::Safe>;
impl<'a, REG> OC1M1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M1_A::B_0x0)
    }
    #[doc = "Set channel 1 to active level on match. tim_oc1ref signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M1_A::B_0x1)
    }
    #[doc = "Set channel 1 to inactive level on match. tim_oc1ref signal is forced low when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M1_A::B_0x2)
    }
    #[doc = "Toggle - tim_oc1ref toggles when TIMx_CNT=TIMx_CCR1."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M1_A::B_0x3)
    }
    #[doc = "Force inactive level - tim_oc1ref is forced low."]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M1_A::B_0x4)
    }
    #[doc = "Force active level - tim_oc1ref is forced high."]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M1_A::B_0x5)
    }
    #[doc = "PWM mode 1 - In upcounting, channel 1 is active as long as TIMx_CNTTIMx_CCR1 else inactive. In downcounting, channel 1 is inactive (tim_oc1ref=0) as long as TIMx_CNTTIMx_CCR1 else active (tim_oc1ref=1)."]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M1_A::B_0x6)
    }
    #[doc = "PWM mode 2 - In upcounting, channel 1 is inactive as long as TIMx_CNTTIMx_CCR1 else active. In downcounting, channel 1 is active as long as TIMx_CNTTIMx_CCR1 else inactive."]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M1_A::B_0x7)
    }
}
#[doc = "Output compare 1 clear enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1CE_A {
    #[doc = "0: tim_oc1ref is not affected by the tim_ocref_clr_int input"]
    B_0x0 = 0,
    #[doc = "1: tim_oc1ref is cleared as soon as a High level is detected on tim_ocref_clr_int input"]
    B_0x1 = 1,
}
impl From<OC1CE_A> for bool {
    #[inline(always)]
    fn from(variant: OC1CE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OC1CE` reader - Output compare 1 clear enable"]
pub type OC1CE_R = crate::BitReader<OC1CE_A>;
impl OC1CE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC1CE_A {
        match self.bits {
            false => OC1CE_A::B_0x0,
            true => OC1CE_A::B_0x1,
        }
    }
    #[doc = "tim_oc1ref is not affected by the tim_ocref_clr_int input"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OC1CE_A::B_0x0
    }
    #[doc = "tim_oc1ref is cleared as soon as a High level is detected on tim_ocref_clr_int input"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OC1CE_A::B_0x1
    }
}
#[doc = "Field `OC1CE` writer - Output compare 1 clear enable"]
pub type OC1CE_W<'a, REG> = crate::BitWriter<'a, REG, OC1CE_A>;
impl<'a, REG> OC1CE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_oc1ref is not affected by the tim_ocref_clr_int input"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OC1CE_A::B_0x0)
    }
    #[doc = "tim_oc1ref is cleared as soon as a High level is detected on tim_ocref_clr_int input"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OC1CE_A::B_0x1)
    }
}
#[doc = "Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC2S_A {
    #[doc = "0: CC2 channel is configured as output"]
    B_0x0 = 0,
    #[doc = "1: CC2 channel is configured as input, tim_ic2 is mapped on tim_ti2"]
    B_0x1 = 1,
    #[doc = "2: CC2 channel is configured as input, tim_ic2 is mapped on tim_ti1"]
    B_0x2 = 2,
    #[doc = "3: CC2 channel is configured as input, tim_ic2 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)"]
    B_0x3 = 3,
}
impl From<CC2S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC2S_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC2S_A {
    type Ux = u8;
}
impl crate::IsEnum for CC2S_A {}
#[doc = "Field `CC2S` reader - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type CC2S_R = crate::FieldReader<CC2S_A>;
impl CC2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC2S_A {
        match self.bits {
            0 => CC2S_A::B_0x0,
            1 => CC2S_A::B_0x1,
            2 => CC2S_A::B_0x2,
            3 => CC2S_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC2 channel is configured as output"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC2S_A::B_0x0
    }
    #[doc = "CC2 channel is configured as input, tim_ic2 is mapped on tim_ti2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC2S_A::B_0x1
    }
    #[doc = "CC2 channel is configured as input, tim_ic2 is mapped on tim_ti1"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CC2S_A::B_0x2
    }
    #[doc = "CC2 channel is configured as input, tim_ic2 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CC2S_A::B_0x3
    }
}
#[doc = "Field `CC2S` writer - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC2S_A, crate::Safe>;
impl<'a, REG> CC2S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC2 channel is configured as output"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S_A::B_0x0)
    }
    #[doc = "CC2 channel is configured as input, tim_ic2 is mapped on tim_ti2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S_A::B_0x1)
    }
    #[doc = "CC2 channel is configured as input, tim_ic2 is mapped on tim_ti1"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S_A::B_0x2)
    }
    #[doc = "CC2 channel is configured as input, tim_ic2 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through the TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S_A::B_0x3)
    }
}
#[doc = "Field `OC2FE` reader - Output compare 2 fast enable"]
pub type OC2FE_R = crate::BitReader;
#[doc = "Field `OC2FE` writer - Output compare 2 fast enable"]
pub type OC2FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2PE` reader - Output compare 2 preload enable"]
pub type OC2PE_R = crate::BitReader;
#[doc = "Field `OC2PE` writer - Output compare 2 preload enable"]
pub type OC2PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M1` reader - Output compare 2 mode refer to OC1M description on bits 6:4"]
pub type OC2M1_R = crate::FieldReader;
#[doc = "Field `OC2M1` writer - Output compare 2 mode refer to OC1M description on bits 6:4"]
pub type OC2M1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC2CE` reader - Output compare 2 clear enable"]
pub type OC2CE_R = crate::BitReader;
#[doc = "Field `OC2CE` writer - Output compare 2 clear enable"]
pub type OC2CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OC1M2_A {
    #[doc = "0: Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    B_0x0 = 0,
    #[doc = "1: Set channel 1 to active level on match. tim_oc1ref signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    B_0x1 = 1,
}
impl From<OC1M2_A> for bool {
    #[inline(always)]
    fn from(variant: OC1M2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OC1M2` reader - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode."]
pub type OC1M2_R = crate::BitReader<OC1M2_A>;
impl OC1M2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OC1M2_A {
        match self.bits {
            false => OC1M2_A::B_0x0,
            true => OC1M2_A::B_0x1,
        }
    }
    #[doc = "Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OC1M2_A::B_0x0
    }
    #[doc = "Set channel 1 to active level on match. tim_oc1ref signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OC1M2_A::B_0x1
    }
}
#[doc = "Field `OC1M2` writer - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode."]
pub type OC1M2_W<'a, REG> = crate::BitWriter<'a, REG, OC1M2_A>;
impl<'a, REG> OC1M2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frozen - The comparison between the output compare register TIMx_CCR1 and the counter TIMx_CNT has no effect on the outputs.(this mode is used to generate a timing base)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M2_A::B_0x0)
    }
    #[doc = "Set channel 1 to active level on match. tim_oc1ref signal is forced high when the counter TIMx_CNT matches the capture/compare register 1 (TIMx_CCR1)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OC1M2_A::B_0x1)
    }
}
#[doc = "Field `OC2M2` reader - Output compare 2 mode refer to OC1M description on bits 6:4"]
pub type OC2M2_R = crate::BitReader;
#[doc = "Field `OC2M2` writer - Output compare 2 mode refer to OC1M description on bits 6:4"]
pub type OC2M2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC1S(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    pub fn OC1FE(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    pub fn OC1PE(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode."]
    #[inline(always)]
    pub fn OC1M1(&self) -> OC1M1_R {
        OC1M1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 1 clear enable"]
    #[inline(always)]
    pub fn OC1CE(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC2S(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output compare 2 fast enable"]
    #[inline(always)]
    pub fn OC2FE(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    pub fn OC2PE(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 2 mode refer to OC1M description on bits 6:4"]
    #[inline(always)]
    pub fn OC2M1(&self) -> OC2M1_R {
        OC2M1_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 2 clear enable"]
    #[inline(always)]
    pub fn OC2CE(&self) -> OC2CE_R {
        OC2CE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode."]
    #[inline(always)]
    pub fn OC1M2(&self) -> OC1M2_R {
        OC1M2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Output compare 2 mode refer to OC1M description on bits 6:4"]
    #[inline(always)]
    pub fn OC2M2(&self) -> OC2M2_R {
        OC2M2_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC1S(&mut self) -> CC1S_W<'_, CCMR1_OUTPUT_SPEC> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bit 2 - Output compare 1 fast enable This bit decreases the latency between a trigger event and a transition on the timer output. It must be used in one-pulse mode (OPM bit set in TIMx_CR1 register), to have the output pulse starting as soon as possible after the starting trigger."]
    #[inline(always)]
    pub fn OC1FE(&mut self) -> OC1FE_W<'_, CCMR1_OUTPUT_SPEC> {
        OC1FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    pub fn OC1PE(&mut self) -> OC1PE_W<'_, CCMR1_OUTPUT_SPEC> {
        OC1PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode."]
    #[inline(always)]
    pub fn OC1M1(&mut self) -> OC1M1_W<'_, CCMR1_OUTPUT_SPEC> {
        OC1M1_W::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 1 clear enable"]
    #[inline(always)]
    pub fn OC1CE(&mut self) -> OC1CE_W<'_, CCMR1_OUTPUT_SPEC> {
        OC1CE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC2S(&mut self) -> CC2S_W<'_, CCMR1_OUTPUT_SPEC> {
        CC2S_W::new(self, 8)
    }
    #[doc = "Bit 10 - Output compare 2 fast enable"]
    #[inline(always)]
    pub fn OC2FE(&mut self) -> OC2FE_W<'_, CCMR1_OUTPUT_SPEC> {
        OC2FE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    pub fn OC2PE(&mut self) -> OC2PE_W<'_, CCMR1_OUTPUT_SPEC> {
        OC2PE_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 2 mode refer to OC1M description on bits 6:4"]
    #[inline(always)]
    pub fn OC2M1(&mut self) -> OC2M1_W<'_, CCMR1_OUTPUT_SPEC> {
        OC2M1_W::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 2 clear enable"]
    #[inline(always)]
    pub fn OC2CE(&mut self) -> OC2CE_W<'_, CCMR1_OUTPUT_SPEC> {
        OC2CE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Output compare 1 mode These bits define the behavior of the output reference signal tim_oc1ref from which tim_oc1 is derived. tim_oc1ref is active high whereas tim_oc1 active level depends on CC1P bit. Note: In PWM mode, the tim_ocref_clr level changes only when the result of the comparison changes or when the output compare mode switches from 'frozen' mode to 'PWM' mode."]
    #[inline(always)]
    pub fn OC1M2(&mut self) -> OC1M2_W<'_, CCMR1_OUTPUT_SPEC> {
        OC1M2_W::new(self, 16)
    }
    #[doc = "Bit 24 - Output compare 2 mode refer to OC1M description on bits 6:4"]
    #[inline(always)]
    pub fn OC2M2(&mut self) -> OC2M2_W<'_, CCMR1_OUTPUT_SPEC> {
        OC2M2_W::new(self, 24)
    }
}
#[doc = "TIM2 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR1_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR1_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_output::R`](R) reader structure"]
impl crate::Readable for CCMR1_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure"]
impl crate::Writable for CCMR1_OUTPUT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCMR1_Output to value 0"]
impl crate::Resettable for CCMR1_OUTPUT_SPEC {}
