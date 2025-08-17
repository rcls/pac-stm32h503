#[doc = "Register `CCMR2_Input` reader"]
pub type R = crate::R<CCMR2_INPUT_SPEC>;
#[doc = "Register `CCMR2_Input` writer"]
pub type W = crate::W<CCMR2_INPUT_SPEC>;
#[doc = "Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC3S_A {
    #[doc = "0: CC3 channel is configured as output"]
    B_0x0 = 0,
    #[doc = "1: CC3 channel is configured as input, tim_ic3 is mapped on tim_ti3"]
    B_0x1 = 1,
    #[doc = "2: CC3 channel is configured as input, tim_ic3 is mapped on tim_ti4"]
    B_0x2 = 2,
    #[doc = "3: CC3 channel is configured as input, tim_ic3 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B_0x3 = 3,
}
impl From<CC3S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC3S_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC3S_A {
    type Ux = u8;
}
impl crate::IsEnum for CC3S_A {}
#[doc = "Field `CC3S` reader - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER)."]
pub type CC3S_R = crate::FieldReader<CC3S_A>;
impl CC3S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC3S_A {
        match self.bits {
            0 => CC3S_A::B_0x0,
            1 => CC3S_A::B_0x1,
            2 => CC3S_A::B_0x2,
            3 => CC3S_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC3 channel is configured as output"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC3S_A::B_0x0
    }
    #[doc = "CC3 channel is configured as input, tim_ic3 is mapped on tim_ti3"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC3S_A::B_0x1
    }
    #[doc = "CC3 channel is configured as input, tim_ic3 is mapped on tim_ti4"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CC3S_A::B_0x2
    }
    #[doc = "CC3 channel is configured as input, tim_ic3 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CC3S_A::B_0x3
    }
}
#[doc = "Field `CC3S` writer - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER)."]
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC3S_A, crate::Safe>;
impl<'a, REG> CC3S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC3 channel is configured as output"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S_A::B_0x0)
    }
    #[doc = "CC3 channel is configured as input, tim_ic3 is mapped on tim_ti3"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S_A::B_0x1)
    }
    #[doc = "CC3 channel is configured as input, tim_ic3 is mapped on tim_ti4"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S_A::B_0x2)
    }
    #[doc = "CC3 channel is configured as input, tim_ic3 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S_A::B_0x3)
    }
}
#[doc = "Field `IC3PSC` reader - Input capture 3 prescaler"]
pub type IC3PSC_R = crate::FieldReader;
#[doc = "Field `IC3PSC` writer - Input capture 3 prescaler"]
pub type IC3PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC3F` reader - Input capture 3 filter"]
pub type IC3F_R = crate::FieldReader;
#[doc = "Field `IC3F` writer - Input capture 3 filter"]
pub type IC3F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC4S_A {
    #[doc = "0: CC4 channel is configured as output"]
    B_0x0 = 0,
    #[doc = "1: CC4 channel is configured as input, tim_ic4 is mapped on tim_ti4"]
    B_0x1 = 1,
    #[doc = "2: CC4 channel is configured as input, tim_ic4 is mapped on tim_ti3"]
    B_0x2 = 2,
    #[doc = "3: CC4 channel is configured as input, tim_ic4 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    B_0x3 = 3,
}
impl From<CC4S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC4S_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC4S_A {
    type Ux = u8;
}
impl crate::IsEnum for CC4S_A {}
#[doc = "Field `CC4S` reader - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER)."]
pub type CC4S_R = crate::FieldReader<CC4S_A>;
impl CC4S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC4S_A {
        match self.bits {
            0 => CC4S_A::B_0x0,
            1 => CC4S_A::B_0x1,
            2 => CC4S_A::B_0x2,
            3 => CC4S_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "CC4 channel is configured as output"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC4S_A::B_0x0
    }
    #[doc = "CC4 channel is configured as input, tim_ic4 is mapped on tim_ti4"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC4S_A::B_0x1
    }
    #[doc = "CC4 channel is configured as input, tim_ic4 is mapped on tim_ti3"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == CC4S_A::B_0x2
    }
    #[doc = "CC4 channel is configured as input, tim_ic4 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == CC4S_A::B_0x3
    }
}
#[doc = "Field `CC4S` writer - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER)."]
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC4S_A, crate::Safe>;
impl<'a, REG> CC4S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC4 channel is configured as output"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S_A::B_0x0)
    }
    #[doc = "CC4 channel is configured as input, tim_ic4 is mapped on tim_ti4"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S_A::B_0x1)
    }
    #[doc = "CC4 channel is configured as input, tim_ic4 is mapped on tim_ti3"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S_A::B_0x2)
    }
    #[doc = "CC4 channel is configured as input, tim_ic4 is mapped on tim_trc. This mode is working only if an internal trigger input is selected through TS bit (TIMx_SMCR register)"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S_A::B_0x3)
    }
}
#[doc = "Field `IC4PSC` reader - Input capture 4 prescaler"]
pub type IC4PSC_R = crate::FieldReader;
#[doc = "Field `IC4PSC` writer - Input capture 4 prescaler"]
pub type IC4PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC4F` reader - Input capture 4 filter"]
pub type IC4F_R = crate::FieldReader;
#[doc = "Field `IC4F` writer - Input capture 4 filter"]
pub type IC4F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC3S(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn IC3PSC(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn IC3F(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC4S(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn IC4PSC(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn IC4F(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC3S(&mut self) -> CC3S_W<'_, CCMR2_INPUT_SPEC> {
        CC3S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn IC3PSC(&mut self) -> IC3PSC_W<'_, CCMR2_INPUT_SPEC> {
        IC3PSC_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn IC3F(&mut self) -> IC3F_W<'_, CCMR2_INPUT_SPEC> {
        IC3F_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn CC4S(&mut self) -> CC4S_W<'_, CCMR2_INPUT_SPEC> {
        CC4S_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn IC4PSC(&mut self) -> IC4PSC_W<'_, CCMR2_INPUT_SPEC> {
        IC4PSC_W::new(self, 10)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn IC4F(&mut self) -> IC4F_W<'_, CCMR2_INPUT_SPEC> {
        IC4F_W::new(self, 12)
    }
}
#[doc = "TIM2 capture/compare mode register 2 \\[alternate\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR2_INPUT_SPEC;
impl crate::RegisterSpec for CCMR2_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_input::R`](R) reader structure"]
impl crate::Readable for CCMR2_INPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_input::W`](W) writer structure"]
impl crate::Writable for CCMR2_INPUT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCMR2_Input to value 0"]
impl crate::Resettable for CCMR2_INPUT_SPEC {}
