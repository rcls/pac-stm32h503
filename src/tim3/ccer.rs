#[doc = "Register `CCER` reader"]
pub type R = crate::R<CCER_SPEC>;
#[doc = "Register `CCER` writer"]
pub type W = crate::W<CCER_SPEC>;
#[doc = "Capture/Compare 1 output enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1E_A {
    #[doc = "0: Capture mode disabled / OC1 is not active"]
    B_0x0 = 0,
    #[doc = "1: Capture mode enabled / OC1 signal is output on the corresponding output pin"]
    B_0x1 = 1,
}
impl From<CC1E_A> for bool {
    #[inline(always)]
    fn from(variant: CC1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1E` reader - Capture/Compare 1 output enable."]
pub type CC1E_R = crate::BitReader<CC1E_A>;
impl CC1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1E_A {
        match self.bits {
            false => CC1E_A::B_0x0,
            true => CC1E_A::B_0x1,
        }
    }
    #[doc = "Capture mode disabled / OC1 is not active"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC1E_A::B_0x0
    }
    #[doc = "Capture mode enabled / OC1 signal is output on the corresponding output pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC1E_A::B_0x1
    }
}
#[doc = "Field `CC1E` writer - Capture/Compare 1 output enable."]
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG, CC1E_A>;
impl<'a, REG> CC1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture mode disabled / OC1 is not active"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E_A::B_0x0)
    }
    #[doc = "Capture mode enabled / OC1 signal is output on the corresponding output pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E_A::B_0x1)
    }
}
#[doc = "Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1P_A {
    #[doc = "0: OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)"]
    B_0x0 = 0,
    #[doc = "1: OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)"]
    B_0x1 = 1,
}
impl From<CC1P_A> for bool {
    #[inline(always)]
    fn from(variant: CC1P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1P` reader - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used."]
pub type CC1P_R = crate::BitReader<CC1P_A>;
impl CC1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1P_A {
        match self.bits {
            false => CC1P_A::B_0x0,
            true => CC1P_A::B_0x1,
        }
    }
    #[doc = "OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC1P_A::B_0x0
    }
    #[doc = "OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC1P_A::B_0x1
    }
}
#[doc = "Field `CC1P` writer - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used."]
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG, CC1P_A>;
impl<'a, REG> CC1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P_A::B_0x0)
    }
    #[doc = "OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P_A::B_0x1)
    }
}
#[doc = "Field `CC1NP` reader - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define tim_ti1fp1/tim_ti2fp1 polarity. refer to CC1P description."]
pub type CC1NP_R = crate::BitReader;
#[doc = "Field `CC1NP` writer - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define tim_ti1fp1/tim_ti2fp1 polarity. refer to CC1P description."]
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2E` reader - Capture/Compare 2 output enable. Refer to CC1E description"]
pub type CC2E_R = crate::BitReader;
#[doc = "Field `CC2E` writer - Capture/Compare 2 output enable. Refer to CC1E description"]
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - Capture/Compare 2 output Polarity. refer to CC1P description"]
pub type CC2P_R = crate::BitReader;
#[doc = "Field `CC2P` writer - Capture/Compare 2 output Polarity. refer to CC1P description"]
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - Capture/Compare 2 output Polarity. Refer to CC1NP description"]
pub type CC2NP_R = crate::BitReader;
#[doc = "Field `CC2NP` writer - Capture/Compare 2 output Polarity. Refer to CC1NP description"]
pub type CC2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3E` reader - Capture/Compare 3 output enable. Refer to CC1E description"]
pub type CC3E_R = crate::BitReader;
#[doc = "Field `CC3E` writer - Capture/Compare 3 output enable. Refer to CC1E description"]
pub type CC3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3P` reader - Capture/Compare 3 output Polarity. Refer to CC1P description"]
pub type CC3P_R = crate::BitReader;
#[doc = "Field `CC3P` writer - Capture/Compare 3 output Polarity. Refer to CC1P description"]
pub type CC3P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NP` reader - Capture/Compare 3 output Polarity. Refer to CC1NP description"]
pub type CC3NP_R = crate::BitReader;
#[doc = "Field `CC3NP` writer - Capture/Compare 3 output Polarity. Refer to CC1NP description"]
pub type CC3NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4E` reader - Capture/Compare 4 output enable. refer to CC1E description"]
pub type CC4E_R = crate::BitReader;
#[doc = "Field `CC4E` writer - Capture/Compare 4 output enable. refer to CC1E description"]
pub type CC4E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4P` reader - Capture/Compare 4 output Polarity. Refer to CC1P description"]
pub type CC4P_R = crate::BitReader;
#[doc = "Field `CC4P` writer - Capture/Compare 4 output Polarity. Refer to CC1P description"]
pub type CC4P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4NP` reader - Capture/Compare 4 output Polarity. Refer to CC1NP description"]
pub type CC4NP_R = crate::BitReader;
#[doc = "Field `CC4NP` writer - Capture/Compare 4 output Polarity. Refer to CC1NP description"]
pub type CC4NP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/Compare 1 output enable."]
    #[inline(always)]
    pub fn CC1E(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used."]
    #[inline(always)]
    pub fn CC1P(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define tim_ti1fp1/tim_ti2fp1 polarity. refer to CC1P description."]
    #[inline(always)]
    pub fn CC1NP(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable. Refer to CC1E description"]
    #[inline(always)]
    pub fn CC2E(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity. refer to CC1P description"]
    #[inline(always)]
    pub fn CC2P(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity. Refer to CC1NP description"]
    #[inline(always)]
    pub fn CC2NP(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable. Refer to CC1E description"]
    #[inline(always)]
    pub fn CC3E(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity. Refer to CC1P description"]
    #[inline(always)]
    pub fn CC3P(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity. Refer to CC1NP description"]
    #[inline(always)]
    pub fn CC3NP(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable. refer to CC1E description"]
    #[inline(always)]
    pub fn CC4E(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture/Compare 4 output Polarity. Refer to CC1P description"]
    #[inline(always)]
    pub fn CC4P(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Capture/Compare 4 output Polarity. Refer to CC1NP description"]
    #[inline(always)]
    pub fn CC4NP(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/Compare 1 output enable."]
    #[inline(always)]
    pub fn CC1E(&mut self) -> CC1E_W<'_, CCER_SPEC> {
        CC1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity. When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges. The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used."]
    #[inline(always)]
    pub fn CC1P(&mut self) -> CC1P_W<'_, CCER_SPEC> {
        CC1P_W::new(self, 1)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity. CC1 channel configured as output: CC1NP must be kept cleared in this case. CC1 channel configured as input: This bit is used in conjunction with CC1P to define tim_ti1fp1/tim_ti2fp1 polarity. refer to CC1P description."]
    #[inline(always)]
    pub fn CC1NP(&mut self) -> CC1NP_W<'_, CCER_SPEC> {
        CC1NP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable. Refer to CC1E description"]
    #[inline(always)]
    pub fn CC2E(&mut self) -> CC2E_W<'_, CCER_SPEC> {
        CC2E_W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity. refer to CC1P description"]
    #[inline(always)]
    pub fn CC2P(&mut self) -> CC2P_W<'_, CCER_SPEC> {
        CC2P_W::new(self, 5)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity. Refer to CC1NP description"]
    #[inline(always)]
    pub fn CC2NP(&mut self) -> CC2NP_W<'_, CCER_SPEC> {
        CC2NP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable. Refer to CC1E description"]
    #[inline(always)]
    pub fn CC3E(&mut self) -> CC3E_W<'_, CCER_SPEC> {
        CC3E_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity. Refer to CC1P description"]
    #[inline(always)]
    pub fn CC3P(&mut self) -> CC3P_W<'_, CCER_SPEC> {
        CC3P_W::new(self, 9)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity. Refer to CC1NP description"]
    #[inline(always)]
    pub fn CC3NP(&mut self) -> CC3NP_W<'_, CCER_SPEC> {
        CC3NP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable. refer to CC1E description"]
    #[inline(always)]
    pub fn CC4E(&mut self) -> CC4E_W<'_, CCER_SPEC> {
        CC4E_W::new(self, 12)
    }
    #[doc = "Bit 13 - Capture/Compare 4 output Polarity. Refer to CC1P description"]
    #[inline(always)]
    pub fn CC4P(&mut self) -> CC4P_W<'_, CCER_SPEC> {
        CC4P_W::new(self, 13)
    }
    #[doc = "Bit 15 - Capture/Compare 4 output Polarity. Refer to CC1NP description"]
    #[inline(always)]
    pub fn CC4NP(&mut self) -> CC4NP_W<'_, CCER_SPEC> {
        CC4NP_W::new(self, 15)
    }
}
#[doc = "TIM3 capture/compare enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCER_SPEC;
impl crate::RegisterSpec for CCER_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ccer::R`](R) reader structure"]
impl crate::Readable for CCER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccer::W`](W) writer structure"]
impl crate::Writable for CCER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCER to value 0"]
impl crate::Resettable for CCER_SPEC {}
