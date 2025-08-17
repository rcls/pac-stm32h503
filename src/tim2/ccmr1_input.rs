#[doc = "Register `CCMR1_Input` reader"]
pub type R = crate::R<CCMR1_INPUT_SPEC>;
#[doc = "Register `CCMR1_Input` writer"]
pub type W = crate::W<CCMR1_INPUT_SPEC>;
#[doc = "Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1S_A {
    #[doc = "0: CC1 channel is configured as output"]
    B_0x0 = 0,
    #[doc = "1: CC1 channel is configured as input, tim_ic1 is mapped on tim_ti1"]
    B_0x1 = 1,
    #[doc = "2: CC1 channel is configured as input, tim_ic1 is mapped on tim_ti2"]
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
    #[doc = "CC1 channel is configured as output"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC1S_A::B_0x0
    }
    #[doc = "CC1 channel is configured as input, tim_ic1 is mapped on tim_ti1"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC1S_A::B_0x1
    }
    #[doc = "CC1 channel is configured as input, tim_ic1 is mapped on tim_ti2"]
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
    #[doc = "CC1 channel is configured as output"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S_A::B_0x0)
    }
    #[doc = "CC1 channel is configured as input, tim_ic1 is mapped on tim_ti1"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1S_A::B_0x1)
    }
    #[doc = "CC1 channel is configured as input, tim_ic1 is mapped on tim_ti2"]
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
#[doc = "Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register).\n\nValue on reset: 0"]
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
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register)."]
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
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register)."]
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
#[doc = "Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC1F_A {
    #[doc = "0: No filter, sampling is done at fDTS"]
    B_0x0 = 0,
    #[doc = "1: fSAMPLING=ftim_ker_ck, N=2"]
    B_0x1 = 1,
    #[doc = "2: fSAMPLING=ftim_ker_ck, N=4"]
    B_0x2 = 2,
    #[doc = "3: fSAMPLING=ftim_ker_ck, N=8"]
    B_0x3 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    B_0x4 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    B_0x5 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    B_0x6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    B_0x7 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    B_0x8 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    B_0x9 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    B_0xA = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    B_0xB = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    B_0xC = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    B_0xD = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    B_0xE = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
    B_0xF = 15,
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
#[doc = "Field `IC1F` reader - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
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
            4 => IC1F_A::B_0x4,
            5 => IC1F_A::B_0x5,
            6 => IC1F_A::B_0x6,
            7 => IC1F_A::B_0x7,
            8 => IC1F_A::B_0x8,
            9 => IC1F_A::B_0x9,
            10 => IC1F_A::B_0xA,
            11 => IC1F_A::B_0xB,
            12 => IC1F_A::B_0xC,
            13 => IC1F_A::B_0xD,
            14 => IC1F_A::B_0xE,
            15 => IC1F_A::B_0xF,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IC1F_A::B_0x0
    }
    #[doc = "fSAMPLING=ftim_ker_ck, N=2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IC1F_A::B_0x1
    }
    #[doc = "fSAMPLING=ftim_ker_ck, N=4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == IC1F_A::B_0x2
    }
    #[doc = "fSAMPLING=ftim_ker_ck, N=8"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == IC1F_A::B_0x3
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == IC1F_A::B_0x4
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == IC1F_A::B_0x5
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == IC1F_A::B_0x6
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == IC1F_A::B_0x7
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn is_B_0x8(&self) -> bool {
        *self == IC1F_A::B_0x8
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn is_B_0x9(&self) -> bool {
        *self == IC1F_A::B_0x9
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn is_B_0xA(&self) -> bool {
        *self == IC1F_A::B_0xA
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn is_B_0xB(&self) -> bool {
        *self == IC1F_A::B_0xB
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == IC1F_A::B_0xC
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == IC1F_A::B_0xD
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == IC1F_A::B_0xE
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == IC1F_A::B_0xF
    }
}
#[doc = "Field `IC1F` writer - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type IC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 4, IC1F_A, crate::Safe>;
impl<'a, REG> IC1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x0)
    }
    #[doc = "fSAMPLING=ftim_ker_ck, N=2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x1)
    }
    #[doc = "fSAMPLING=ftim_ker_ck, N=4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x2)
    }
    #[doc = "fSAMPLING=ftim_ker_ck, N=8"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x3)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x4)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x5)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x7)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn B_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn B_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0x9)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn B_0xA(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0xA)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn B_0xB(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0xB)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0xC)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0xD)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0xE)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(IC1F_A::B_0xF)
    }
}
#[doc = "Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC2S_A {
    #[doc = "0: CC2 channel is configured as output."]
    B_0x0 = 0,
    #[doc = "1: CC2 channel is configured as input, tim_ic2 is mapped on tim_ti2."]
    B_0x1 = 1,
    #[doc = "2: CC2 channel is configured as input, tim_ic2 is mapped on tim_ti1."]
    B_0x2 = 2,
    #[doc = "3: CC2 channel is configured as input, tim_ic2 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
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
#[doc = "Field `CC2S` reader - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
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
    #[doc = "CC2 channel is configured as output."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC2S_A::B_0x0
    }
    #[doc = "CC2 channel is configured as input, tim_ic2 is mapped on tim_ti2."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC2S_A::B_0x1
    }
    #[doc = "CC2 channel is configured as input, tim_ic2 is mapped on tim_ti1."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CC2S_A::B_0x2
    }
    #[doc = "CC2 channel is configured as input, tim_ic2 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CC2S_A::B_0x3
    }
}
#[doc = "Field `CC2S` writer - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
pub type CC2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC2S_A, crate::Safe>;
impl<'a, REG> CC2S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC2 channel is configured as output."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S_A::B_0x0)
    }
    #[doc = "CC2 channel is configured as input, tim_ic2 is mapped on tim_ti2."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S_A::B_0x1)
    }
    #[doc = "CC2 channel is configured as input, tim_ic2 is mapped on tim_ti1."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S_A::B_0x2)
    }
    #[doc = "CC2 channel is configured as input, tim_ic2 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC2S_A::B_0x3)
    }
}
#[doc = "Field `IC2PSC` reader - Input capture 2 prescaler"]
pub type IC2PSC_R = crate::FieldReader;
#[doc = "Field `IC2PSC` writer - Input capture 2 prescaler"]
pub type IC2PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC2F` reader - Input capture 2 filter"]
pub type IC2F_R = crate::FieldReader;
#[doc = "Field `IC2F` writer - Input capture 2 filter"]
pub type IC2F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC1S(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register)."]
    #[inline(always)]
    pub fn IC1PSC(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn IC1F(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC2S(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn IC2PSC(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    pub fn IC2F(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC1S(&mut self) -> CC1S_W<'_, CCMR1_INPUT_SPEC> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (tim_ic1). The prescaler is reset as soon as CC1E=0 (TIMx_CCER register)."]
    #[inline(always)]
    pub fn IC1PSC(&mut self) -> IC1PSC_W<'_, CCMR1_INPUT_SPEC> {
        IC1PSC_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample tim_ti1 input and the length of the digital filter applied to tim_ti1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn IC1F(&mut self) -> IC1F_W<'_, CCMR1_INPUT_SPEC> {
        IC1F_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Capture/compare 2 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC2S bits are writable only when the channel is OFF (CC2E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC2S(&mut self) -> CC2S_W<'_, CCMR1_INPUT_SPEC> {
        CC2S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn IC2PSC(&mut self) -> IC2PSC_W<'_, CCMR1_INPUT_SPEC> {
        IC2PSC_W::new(self, 10)
    }
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    pub fn IC2F(&mut self) -> IC2F_W<'_, CCMR1_INPUT_SPEC> {
        IC2F_W::new(self, 12)
    }
}
#[doc = "TIM2 capture/compare mode register 1 \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR1_INPUT_SPEC;
impl crate::RegisterSpec for CCMR1_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_input::R`](R) reader structure"]
impl crate::Readable for CCMR1_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_input::W`](W) writer structure"]
impl crate::Writable for CCMR1_INPUT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCMR1_Input to value 0"]
impl crate::Resettable for CCMR1_INPUT_SPEC {}
