#[doc = "Register `CCMR1` reader"]
pub type R = crate::R<CCMR1_SPEC>;
#[doc = "Register `CCMR1` writer"]
pub type W = crate::W<CCMR1_SPEC>;
#[doc = "Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1SEL_A {
    #[doc = "0: CC1 channel is configured in output PWM mode"]
    B_0x0 = 0,
    #[doc = "1: CC1 channel is configured in input capture mode"]
    B_0x1 = 1,
}
impl From<CC1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CC1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1SEL` reader - Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode."]
pub type CC1SEL_R = crate::BitReader<CC1SEL_A>;
impl CC1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1SEL_A {
        match self.bits {
            false => CC1SEL_A::B_0x0,
            true => CC1SEL_A::B_0x1,
        }
    }
    #[doc = "CC1 channel is configured in output PWM mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC1SEL_A::B_0x0
    }
    #[doc = "CC1 channel is configured in input capture mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC1SEL_A::B_0x1
    }
}
#[doc = "Field `CC1SEL` writer - Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode."]
pub type CC1SEL_W<'a, REG> = crate::BitWriter<'a, REG, CC1SEL_A>;
impl<'a, REG> CC1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC1 channel is configured in output PWM mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1SEL_A::B_0x0)
    }
    #[doc = "CC1 channel is configured in input capture mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1SEL_A::B_0x1)
    }
}
#[doc = "Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM_CCR1) or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1E_A {
    #[doc = "0: Off - OC1 is not active. Writing '0' to the CC1E bit resets the ue_dma_req signal only if all the other LPTIM channels are disabled."]
    B_0x0_CC1_AS_OUTPUT = 0,
    #[doc = "1: On - OC1 signal is output on the corresponding output pin"]
    B_0x1_CC1_AS_OUTPUT = 1,
}
impl From<CC1E_A> for bool {
    #[inline(always)]
    fn from(variant: CC1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1E` reader - Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM_CCR1) or not."]
pub type CC1E_R = crate::BitReader<CC1E_A>;
impl CC1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1E_A {
        match self.bits {
            false => CC1E_A::B_0x0_CC1_AS_OUTPUT,
            true => CC1E_A::B_0x1_CC1_AS_OUTPUT,
        }
    }
    #[doc = "Off - OC1 is not active. Writing '0' to the CC1E bit resets the ue_dma_req signal only if all the other LPTIM channels are disabled."]
    #[inline(always)]
    pub fn is_B_0x0_CC1_AS_OUTPUT(&self) -> bool {
        *self == CC1E_A::B_0x0_CC1_AS_OUTPUT
    }
    #[doc = "On - OC1 signal is output on the corresponding output pin"]
    #[inline(always)]
    pub fn is_B_0x1_CC1_AS_OUTPUT(&self) -> bool {
        *self == CC1E_A::B_0x1_CC1_AS_OUTPUT
    }
}
#[doc = "Field `CC1E` writer - Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM_CCR1) or not."]
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG, CC1E_A>;
impl<'a, REG> CC1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off - OC1 is not active. Writing '0' to the CC1E bit resets the ue_dma_req signal only if all the other LPTIM channels are disabled."]
    #[inline(always)]
    pub fn B_0x0_CC1_AS_OUTPUT(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E_A::B_0x0_CC1_AS_OUTPUT)
    }
    #[doc = "On - OC1 signal is output on the corresponding output pin"]
    #[inline(always)]
    pub fn B_0x1_CC1_AS_OUTPUT(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E_A::B_0x1_CC1_AS_OUTPUT)
    }
}
#[doc = "Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1P_A {
    #[doc = "0: OC1 active high, the LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CCRx registers"]
    B_0x0_CC1_AS_OUTPUT = 0,
    #[doc = "1: OC1 active low, the LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CCRx registers"]
    B_0x1_CC1_AS_OUTPUT = 1,
    #[doc = "2: reserved, do not use this configuration."]
    B_0x2_CC1_AS_INPUT = 2,
    #[doc = "3: both edges, circuit is sensitive to both IC1 rising and falling edges."]
    B_0x3_CC1_AS_INPUT = 3,
}
impl From<CC1P_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1P_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC1P_A {
    type Ux = u8;
}
impl crate::IsEnum for CC1P_A {}
#[doc = "Field `CC1P` reader - Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
pub type CC1P_R = crate::FieldReader<CC1P_A>;
impl CC1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1P_A {
        match self.bits {
            0 => CC1P_A::B_0x0_CC1_AS_OUTPUT,
            1 => CC1P_A::B_0x1_CC1_AS_OUTPUT,
            2 => CC1P_A::B_0x2_CC1_AS_INPUT,
            3 => CC1P_A::B_0x3_CC1_AS_INPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "OC1 active high, the LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CCRx registers"]
    #[inline(always)]
    pub fn is_B_0x0_CC1_AS_OUTPUT(&self) -> bool {
        *self == CC1P_A::B_0x0_CC1_AS_OUTPUT
    }
    #[doc = "OC1 active low, the LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CCRx registers"]
    #[inline(always)]
    pub fn is_B_0x1_CC1_AS_OUTPUT(&self) -> bool {
        *self == CC1P_A::B_0x1_CC1_AS_OUTPUT
    }
    #[doc = "reserved, do not use this configuration."]
    #[inline(always)]
    pub fn is_B_0x2_CC1_AS_INPUT(&self) -> bool {
        *self == CC1P_A::B_0x2_CC1_AS_INPUT
    }
    #[doc = "both edges, circuit is sensitive to both IC1 rising and falling edges."]
    #[inline(always)]
    pub fn is_B_0x3_CC1_AS_INPUT(&self) -> bool {
        *self == CC1P_A::B_0x3_CC1_AS_INPUT
    }
}
#[doc = "Field `CC1P` writer - Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
pub type CC1P_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC1P_A, crate::Safe>;
impl<'a, REG> CC1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OC1 active high, the LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CCRx registers"]
    #[inline(always)]
    pub fn B_0x0_CC1_AS_OUTPUT(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P_A::B_0x0_CC1_AS_OUTPUT)
    }
    #[doc = "OC1 active low, the LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CCRx registers"]
    #[inline(always)]
    pub fn B_0x1_CC1_AS_OUTPUT(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P_A::B_0x1_CC1_AS_OUTPUT)
    }
    #[doc = "reserved, do not use this configuration."]
    #[inline(always)]
    pub fn B_0x2_CC1_AS_INPUT(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P_A::B_0x2_CC1_AS_INPUT)
    }
    #[doc = "both edges, circuit is sensitive to both IC1 rising and falling edges."]
    #[inline(always)]
    pub fn B_0x3_CC1_AS_INPUT(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P_A::B_0x3_CC1_AS_INPUT)
    }
}
#[doc = "Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC1PSC_A {
    #[doc = "0: no prescaler, capture is done each time an edge is detected on the capture input"]
    B_0x0 = 0,
    #[doc = "1: capture is done once every 2 events"]
    B_0x1 = 1,
    #[doc = "2: capture is done once every 4 events"]
    B_0x2 = 2,
    #[doc = "3: capture is done once every 8 events"]
    B_0x3 = 3,
}
impl From<IC1PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: IC1PSC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IC1PSC_A {
    type Ux = u8;
}
impl crate::IsEnum for IC1PSC_A {}
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
pub type IC1PSC_R = crate::FieldReader<IC1PSC_A>;
impl IC1PSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IC1PSC_A {
        match self.bits {
            0 => IC1PSC_A::B_0x0,
            1 => IC1PSC_A::B_0x1,
            2 => IC1PSC_A::B_0x2,
            3 => IC1PSC_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IC1PSC_A::B_0x0
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IC1PSC_A::B_0x1
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == IC1PSC_A::B_0x2
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == IC1PSC_A::B_0x3
    }
}
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
pub type IC1PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IC1PSC_A, crate::Safe>;
impl<'a, REG> IC1PSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IC1PSC_A::B_0x0)
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IC1PSC_A::B_0x1)
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IC1PSC_A::B_0x2)
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(IC1PSC_A::B_0x3)
    }
}
#[doc = "Input capture 1 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC1F_A {
    #[doc = "0: any external input capture signal level change is considered as a valid transition"]
    B_0x0 = 0,
    #[doc = "1: external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    B_0x1 = 1,
    #[doc = "2: external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    B_0x2 = 2,
    #[doc = "3: external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    B_0x3 = 3,
}
impl From<IC1F_A> for u8 {
    #[inline(always)]
    fn from(variant: IC1F_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IC1F_A {
    type Ux = u8;
}
impl crate::IsEnum for IC1F_A {}
#[doc = "Field `IC1F` reader - Input capture 1 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type IC1F_R = crate::FieldReader<IC1F_A>;
impl IC1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IC1F_A {
        match self.bits {
            0 => IC1F_A::B_0x0,
            1 => IC1F_A::B_0x1,
            2 => IC1F_A::B_0x2,
            3 => IC1F_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "any external input capture signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IC1F_A::B_0x0
    }
    #[doc = "external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IC1F_A::B_0x1
    }
    #[doc = "external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == IC1F_A::B_0x2
    }
    #[doc = "external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == IC1F_A::B_0x3
    }
}
#[doc = "Field `IC1F` writer - Input capture 1 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type IC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IC1F_A, crate::Safe>;
impl<'a, REG> IC1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "any external input capture signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x0)
    }
    #[doc = "external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x1)
    }
    #[doc = "external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x2)
    }
    #[doc = "external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x3)
    }
}
#[doc = "Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC2SEL_A {
    #[doc = "0: CC2 channel is configured in output PWM mode"]
    B_0x0 = 0,
    #[doc = "1: CC2 channel is configured in input capture mode"]
    B_0x1 = 1,
}
impl From<CC2SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CC2SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2SEL` reader - Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
pub type CC2SEL_R = crate::BitReader<CC2SEL_A>;
impl CC2SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC2SEL_A {
        match self.bits {
            false => CC2SEL_A::B_0x0,
            true => CC2SEL_A::B_0x1,
        }
    }
    #[doc = "CC2 channel is configured in output PWM mode"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC2SEL_A::B_0x0
    }
    #[doc = "CC2 channel is configured in input capture mode"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC2SEL_A::B_0x1
    }
}
#[doc = "Field `CC2SEL` writer - Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
pub type CC2SEL_W<'a, REG> = crate::BitWriter<'a, REG, CC2SEL_A>;
impl<'a, REG> CC2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CC2 channel is configured in output PWM mode"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC2SEL_A::B_0x0)
    }
    #[doc = "CC2 channel is configured in input capture mode"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC2SEL_A::B_0x1)
    }
}
#[doc = "Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM_CCR2) or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC2E_A {
    #[doc = "0: Off - OC2 is not active. Writing '0' to the CC2E bit resets the ue_dma_req signal only if all the other LPTIM channels are disabled."]
    B_0x0_CC2_AS_OUTPUT = 0,
    #[doc = "1: On - OC2 signal is output on the corresponding output pin"]
    B_0x1_CC2_AS_OUTPUT = 1,
}
impl From<CC2E_A> for bool {
    #[inline(always)]
    fn from(variant: CC2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2E` reader - Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM_CCR2) or not."]
pub type CC2E_R = crate::BitReader<CC2E_A>;
impl CC2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC2E_A {
        match self.bits {
            false => CC2E_A::B_0x0_CC2_AS_OUTPUT,
            true => CC2E_A::B_0x1_CC2_AS_OUTPUT,
        }
    }
    #[doc = "Off - OC2 is not active. Writing '0' to the CC2E bit resets the ue_dma_req signal only if all the other LPTIM channels are disabled."]
    #[inline(always)]
    pub fn is_B_0x0_CC2_AS_OUTPUT(&self) -> bool {
        *self == CC2E_A::B_0x0_CC2_AS_OUTPUT
    }
    #[doc = "On - OC2 signal is output on the corresponding output pin"]
    #[inline(always)]
    pub fn is_B_0x1_CC2_AS_OUTPUT(&self) -> bool {
        *self == CC2E_A::B_0x1_CC2_AS_OUTPUT
    }
}
#[doc = "Field `CC2E` writer - Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM_CCR2) or not."]
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG, CC2E_A>;
impl<'a, REG> CC2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off - OC2 is not active. Writing '0' to the CC2E bit resets the ue_dma_req signal only if all the other LPTIM channels are disabled."]
    #[inline(always)]
    pub fn B_0x0_CC2_AS_OUTPUT(self) -> &'a mut crate::W<REG> {
        self.variant(CC2E_A::B_0x0_CC2_AS_OUTPUT)
    }
    #[doc = "On - OC2 signal is output on the corresponding output pin"]
    #[inline(always)]
    pub fn B_0x1_CC2_AS_OUTPUT(self) -> &'a mut crate::W<REG> {
        self.variant(CC2E_A::B_0x1_CC2_AS_OUTPUT)
    }
}
#[doc = "Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC2P_A {
    #[doc = "0: OC2 active high"]
    B_0x0_CC2_AS_OUTPUT = 0,
    #[doc = "1: OC2 active low"]
    B_0x1_CC2_AS_OUTPUT = 1,
    #[doc = "2: reserved, do not use this configuration."]
    B_0x2_CC2_AS_INPUT = 2,
    #[doc = "3: both edges, circuit is sensitive to both IC2 rising and falling edges."]
    B_0x3_CC2_AS_INPUT = 3,
}
impl From<CC2P_A> for u8 {
    #[inline(always)]
    fn from(variant: CC2P_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC2P_A {
    type Ux = u8;
}
impl crate::IsEnum for CC2P_A {}
#[doc = "Field `CC2P` reader - Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations."]
pub type CC2P_R = crate::FieldReader<CC2P_A>;
impl CC2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC2P_A {
        match self.bits {
            0 => CC2P_A::B_0x0_CC2_AS_OUTPUT,
            1 => CC2P_A::B_0x1_CC2_AS_OUTPUT,
            2 => CC2P_A::B_0x2_CC2_AS_INPUT,
            3 => CC2P_A::B_0x3_CC2_AS_INPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "OC2 active high"]
    #[inline(always)]
    pub fn is_B_0x0_CC2_AS_OUTPUT(&self) -> bool {
        *self == CC2P_A::B_0x0_CC2_AS_OUTPUT
    }
    #[doc = "OC2 active low"]
    #[inline(always)]
    pub fn is_B_0x1_CC2_AS_OUTPUT(&self) -> bool {
        *self == CC2P_A::B_0x1_CC2_AS_OUTPUT
    }
    #[doc = "reserved, do not use this configuration."]
    #[inline(always)]
    pub fn is_B_0x2_CC2_AS_INPUT(&self) -> bool {
        *self == CC2P_A::B_0x2_CC2_AS_INPUT
    }
    #[doc = "both edges, circuit is sensitive to both IC2 rising and falling edges."]
    #[inline(always)]
    pub fn is_B_0x3_CC2_AS_INPUT(&self) -> bool {
        *self == CC2P_A::B_0x3_CC2_AS_INPUT
    }
}
#[doc = "Field `CC2P` writer - Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations."]
pub type CC2P_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC2P_A, crate::Safe>;
impl<'a, REG> CC2P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OC2 active high"]
    #[inline(always)]
    pub fn B_0x0_CC2_AS_OUTPUT(self) -> &'a mut crate::W<REG> {
        self.variant(CC2P_A::B_0x0_CC2_AS_OUTPUT)
    }
    #[doc = "OC2 active low"]
    #[inline(always)]
    pub fn B_0x1_CC2_AS_OUTPUT(self) -> &'a mut crate::W<REG> {
        self.variant(CC2P_A::B_0x1_CC2_AS_OUTPUT)
    }
    #[doc = "reserved, do not use this configuration."]
    #[inline(always)]
    pub fn B_0x2_CC2_AS_INPUT(self) -> &'a mut crate::W<REG> {
        self.variant(CC2P_A::B_0x2_CC2_AS_INPUT)
    }
    #[doc = "both edges, circuit is sensitive to both IC2 rising and falling edges."]
    #[inline(always)]
    pub fn B_0x3_CC2_AS_INPUT(self) -> &'a mut crate::W<REG> {
        self.variant(CC2P_A::B_0x3_CC2_AS_INPUT)
    }
}
#[doc = "Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC2PSC_A {
    #[doc = "0: no prescaler, capture is done each time an edge is detected on the capture input"]
    B_0x0 = 0,
    #[doc = "1: capture is done once every 2 events"]
    B_0x1 = 1,
    #[doc = "2: capture is done once every 4 events"]
    B_0x2 = 2,
    #[doc = "3: capture is done once every 8 events"]
    B_0x3 = 3,
}
impl From<IC2PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: IC2PSC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IC2PSC_A {
    type Ux = u8;
}
impl crate::IsEnum for IC2PSC_A {}
#[doc = "Field `IC2PSC` reader - Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2)."]
pub type IC2PSC_R = crate::FieldReader<IC2PSC_A>;
impl IC2PSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IC2PSC_A {
        match self.bits {
            0 => IC2PSC_A::B_0x0,
            1 => IC2PSC_A::B_0x1,
            2 => IC2PSC_A::B_0x2,
            3 => IC2PSC_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IC2PSC_A::B_0x0
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IC2PSC_A::B_0x1
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == IC2PSC_A::B_0x2
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == IC2PSC_A::B_0x3
    }
}
#[doc = "Field `IC2PSC` writer - Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2)."]
pub type IC2PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IC2PSC_A, crate::Safe>;
impl<'a, REG> IC2PSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IC2PSC_A::B_0x0)
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IC2PSC_A::B_0x1)
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IC2PSC_A::B_0x2)
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(IC2PSC_A::B_0x3)
    }
}
#[doc = "Input capture 2 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC2F_A {
    #[doc = "0: any external input capture signal level change is considered as a valid transition"]
    B_0x0 = 0,
    #[doc = "1: external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    B_0x1 = 1,
    #[doc = "2: external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    B_0x2 = 2,
    #[doc = "3: external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    B_0x3 = 3,
}
impl From<IC2F_A> for u8 {
    #[inline(always)]
    fn from(variant: IC2F_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IC2F_A {
    type Ux = u8;
}
impl crate::IsEnum for IC2F_A {}
#[doc = "Field `IC2F` reader - Input capture 2 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type IC2F_R = crate::FieldReader<IC2F_A>;
impl IC2F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IC2F_A {
        match self.bits {
            0 => IC2F_A::B_0x0,
            1 => IC2F_A::B_0x1,
            2 => IC2F_A::B_0x2,
            3 => IC2F_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "any external input capture signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IC2F_A::B_0x0
    }
    #[doc = "external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IC2F_A::B_0x1
    }
    #[doc = "external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == IC2F_A::B_0x2
    }
    #[doc = "external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == IC2F_A::B_0x3
    }
}
#[doc = "Field `IC2F` writer - Input capture 2 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
pub type IC2F_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IC2F_A, crate::Safe>;
impl<'a, REG> IC2F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "any external input capture signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IC2F_A::B_0x0)
    }
    #[doc = "external input capture signal level change must be stable for at least 2 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IC2F_A::B_0x1)
    }
    #[doc = "external input capture signal level change must be stable for at least 4 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IC2F_A::B_0x2)
    }
    #[doc = "external input capture signal level change must be stable for at least 8 clock periods before it is considered as valid transition."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(IC2F_A::B_0x3)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode."]
    #[inline(always)]
    pub fn CC1SEL(&self) -> CC1SEL_R {
        CC1SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM_CCR1) or not."]
    #[inline(always)]
    pub fn CC1E(&self) -> CC1E_R {
        CC1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
    #[inline(always)]
    pub fn CC1P(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
    #[inline(always)]
    pub fn IC1PSC(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Input capture 1 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    pub fn IC1F(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
    #[inline(always)]
    pub fn CC2SEL(&self) -> CC2SEL_R {
        CC2SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM_CCR2) or not."]
    #[inline(always)]
    pub fn CC2E(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations."]
    #[inline(always)]
    pub fn CC2P(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2)."]
    #[inline(always)]
    pub fn IC2PSC(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Input capture 2 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    pub fn IC2F(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare 1 selection This bitfield defines the direction of the channel input (capture) or output mode."]
    #[inline(always)]
    pub fn CC1SEL(&mut self) -> CC1SEL_W<'_, CCMR1_SPEC> {
        CC1SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 1 (LPTIM_CCR1) or not."]
    #[inline(always)]
    pub fn CC1E(&mut self) -> CC1E_W<'_, CCMR1_SPEC> {
        CC1E_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Capture/compare 1 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC1 polarity for capture operations."]
    #[inline(always)]
    pub fn CC1P(&mut self) -> CC1P_W<'_, CCMR1_SPEC> {
        CC1P_W::new(self, 2)
    }
    #[doc = "Bits 8:9 - Input capture 1 prescaler This bitfield defines the ratio of the prescaler acting on the CC1 input (IC1)."]
    #[inline(always)]
    pub fn IC1PSC(&mut self) -> IC1PSC_W<'_, CCMR1_SPEC> {
        IC1PSC_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - Input capture 1 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    pub fn IC1F(&mut self) -> IC1F_W<'_, CCMR1_SPEC> {
        IC1F_W::new(self, 12)
    }
    #[doc = "Bit 16 - Capture/compare 2 selection This bitfield defines the direction of the channel, input (capture) or output mode."]
    #[inline(always)]
    pub fn CC2SEL(&mut self) -> CC2SEL_W<'_, CCMR1_SPEC> {
        CC2SEL_W::new(self, 16)
    }
    #[doc = "Bit 17 - Capture/compare 2 output enable. This bit determines if a capture of the counter value can actually be done into the input capture/compare register 2 (LPTIM_CCR2) or not."]
    #[inline(always)]
    pub fn CC2E(&mut self) -> CC2E_W<'_, CCMR1_SPEC> {
        CC2E_W::new(self, 17)
    }
    #[doc = "Bits 18:19 - Capture/compare 2 output polarity. Only bit2 is used to set polarity when output mode is enabled, bit3 is don't care. This field is used to select the IC2 polarity for capture operations."]
    #[inline(always)]
    pub fn CC2P(&mut self) -> CC2P_W<'_, CCMR1_SPEC> {
        CC2P_W::new(self, 18)
    }
    #[doc = "Bits 24:25 - Input capture 2 prescaler This bitfield defines the ratio of the prescaler acting on the CC2 input (IC2)."]
    #[inline(always)]
    pub fn IC2PSC(&mut self) -> IC2PSC_W<'_, CCMR1_SPEC> {
        IC2PSC_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Input capture 2 filter This bitfield defines the number of consecutive equal samples that should be detected when a level change occurs on an external input capture signal before it is considered as a valid level transition. An internal clock source must be present to use this feature."]
    #[inline(always)]
    pub fn IC2F(&mut self) -> IC2F_W<'_, CCMR1_SPEC> {
        IC2F_W::new(self, 28)
    }
}
#[doc = "LPTIM capture/compare mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR1_SPEC;
impl crate::RegisterSpec for CCMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1::R`](R) reader structure"]
impl crate::Readable for CCMR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr1::W`](W) writer structure"]
impl crate::Writable for CCMR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCMR1 to value 0"]
impl crate::Resettable for CCMR1_SPEC {}
