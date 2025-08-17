#[doc = "Register `CCER` reader"]
pub type R = crate::R<CCER_SPEC>;
#[doc = "Register `CCER` writer"]
pub type W = crate::W<CCER_SPEC>;
#[doc = "Capture/compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 619 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1E_A {
    #[doc = "0: Capture mode disabled / OC1 is not active (see below)"]
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
#[doc = "Field `CC1E` reader - Capture/compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 619 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
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
    #[doc = "Capture mode disabled / OC1 is not active (see below)"]
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
#[doc = "Field `CC1E` writer - Capture/compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 619 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG, CC1E_A>;
impl<'a, REG> CC1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture mode disabled / OC1 is not active (see below)"]
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
#[doc = "Capture/compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: the configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.\n\nValue on reset: 0"]
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
#[doc = "Field `CC1P` reader - Capture/compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: the configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
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
#[doc = "Field `CC1P` writer - Capture/compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: the configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
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
#[doc = "Capture/compare 1 complementary output enable Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NE_A {
    #[doc = "0: Off - tim_oc1n is not active. tim_oc1n level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    B_0x0 = 0,
    #[doc = "1: On - tim_oc1n signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    B_0x1 = 1,
}
impl From<CC1NE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1NE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1NE` reader - Capture/compare 1 complementary output enable Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1NE_R = crate::BitReader<CC1NE_A>;
impl CC1NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1NE_A {
        match self.bits {
            false => CC1NE_A::B_0x0,
            true => CC1NE_A::B_0x1,
        }
    }
    #[doc = "Off - tim_oc1n is not active. tim_oc1n level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC1NE_A::B_0x0
    }
    #[doc = "On - tim_oc1n signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC1NE_A::B_0x1
    }
}
#[doc = "Field `CC1NE` writer - Capture/compare 1 complementary output enable Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1NE_W<'a, REG> = crate::BitWriter<'a, REG, CC1NE_A>;
impl<'a, REG> CC1NE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off - tim_oc1n is not active. tim_oc1n level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NE_A::B_0x0)
    }
    #[doc = "On - tim_oc1n signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NE_A::B_0x1)
    }
}
#[doc = "Capture/compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of tim_ti1fp1 and tim_ti2fp1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NP_A {
    #[doc = "0: tim_oc1n active high."]
    B_0x0 = 0,
    #[doc = "1: tim_oc1n active low."]
    B_0x1 = 1,
}
impl From<CC1NP_A> for bool {
    #[inline(always)]
    fn from(variant: CC1NP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1NP` reader - Capture/compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of tim_ti1fp1 and tim_ti2fp1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1NP_R = crate::BitReader<CC1NP_A>;
impl CC1NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1NP_A {
        match self.bits {
            false => CC1NP_A::B_0x0,
            true => CC1NP_A::B_0x1,
        }
    }
    #[doc = "tim_oc1n active high."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CC1NP_A::B_0x0
    }
    #[doc = "tim_oc1n active low."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CC1NP_A::B_0x1
    }
}
#[doc = "Field `CC1NP` writer - Capture/compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of tim_ti1fp1 and tim_ti2fp1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG, CC1NP_A>;
impl<'a, REG> CC1NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "tim_oc1n active high."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NP_A::B_0x0)
    }
    #[doc = "tim_oc1n active low."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NP_A::B_0x1)
    }
}
#[doc = "Field `CC2E` reader - Capture/compare 2 output enable Refer to CC1E description"]
pub type CC2E_R = crate::BitReader;
#[doc = "Field `CC2E` writer - Capture/compare 2 output enable Refer to CC1E description"]
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - Capture/compare 2 output polarity Refer to CC1P description"]
pub type CC2P_R = crate::BitReader;
#[doc = "Field `CC2P` writer - Capture/compare 2 output polarity Refer to CC1P description"]
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NE` reader - Capture/compare 2 complementary output enable Refer to CC1NE description"]
pub type CC2NE_R = crate::BitReader;
#[doc = "Field `CC2NE` writer - Capture/compare 2 complementary output enable Refer to CC1NE description"]
pub type CC2NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - Capture/compare 2 complementary output polarity Refer to CC1NP description"]
pub type CC2NP_R = crate::BitReader;
#[doc = "Field `CC2NP` writer - Capture/compare 2 complementary output polarity Refer to CC1NP description"]
pub type CC2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3E` reader - Capture/compare 3 output enable Refer to CC1E description"]
pub type CC3E_R = crate::BitReader;
#[doc = "Field `CC3E` writer - Capture/compare 3 output enable Refer to CC1E description"]
pub type CC3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3P` reader - Capture/compare 3 output polarity Refer to CC1P description"]
pub type CC3P_R = crate::BitReader;
#[doc = "Field `CC3P` writer - Capture/compare 3 output polarity Refer to CC1P description"]
pub type CC3P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NE` reader - Capture/compare 3 complementary output enable Refer to CC1NE description"]
pub type CC3NE_R = crate::BitReader;
#[doc = "Field `CC3NE` writer - Capture/compare 3 complementary output enable Refer to CC1NE description"]
pub type CC3NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NP` reader - Capture/compare 3 complementary output polarity Refer to CC1NP description"]
pub type CC3NP_R = crate::BitReader;
#[doc = "Field `CC3NP` writer - Capture/compare 3 complementary output polarity Refer to CC1NP description"]
pub type CC3NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4E` reader - Capture/compare 4 output enable Refer to CC1E description"]
pub type CC4E_R = crate::BitReader;
#[doc = "Field `CC4E` writer - Capture/compare 4 output enable Refer to CC1E description"]
pub type CC4E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4P` reader - Capture/compare 4 output polarity Refer to CC1P description"]
pub type CC4P_R = crate::BitReader;
#[doc = "Field `CC4P` writer - Capture/compare 4 output polarity Refer to CC1P description"]
pub type CC4P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4NE` reader - Capture/compare 4 complementary output enable Refer to CC1NE description"]
pub type CC4NE_R = crate::BitReader;
#[doc = "Field `CC4NE` writer - Capture/compare 4 complementary output enable Refer to CC1NE description"]
pub type CC4NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4NP` reader - Capture/compare 4 complementary output polarity Refer to CC1NP description"]
pub type CC4NP_R = crate::BitReader;
#[doc = "Field `CC4NP` writer - Capture/compare 4 complementary output polarity Refer to CC1NP description"]
pub type CC4NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5E` reader - Capture/compare 5 output enable Refer to CC1E description"]
pub type CC5E_R = crate::BitReader;
#[doc = "Field `CC5E` writer - Capture/compare 5 output enable Refer to CC1E description"]
pub type CC5E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5P` reader - Capture/compare 5 output polarity Refer to CC1P description"]
pub type CC5P_R = crate::BitReader;
#[doc = "Field `CC5P` writer - Capture/compare 5 output polarity Refer to CC1P description"]
pub type CC5P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6E` reader - Capture/compare 6 output enable Refer to CC1E description"]
pub type CC6E_R = crate::BitReader;
#[doc = "Field `CC6E` writer - Capture/compare 6 output enable Refer to CC1E description"]
pub type CC6E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6P` reader - Capture/compare 6 output polarity Refer to CC1P description"]
pub type CC6P_R = crate::BitReader;
#[doc = "Field `CC6P` writer - Capture/compare 6 output polarity Refer to CC1P description"]
pub type CC6P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 619 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn CC1E(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: the configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn CC1P(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare 1 complementary output enable Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn CC1NE(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of tim_ti1fp1 and tim_ti2fp1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn CC1NP(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/compare 2 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn CC2E(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/compare 2 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn CC2P(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture/compare 2 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    pub fn CC2NE(&self) -> CC2NE_R {
        CC2NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/compare 2 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    pub fn CC2NP(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture/compare 3 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn CC3E(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/compare 3 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn CC3P(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/compare 3 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    pub fn CC3NE(&self) -> CC3NE_R {
        CC3NE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/compare 3 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    pub fn CC3NP(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/compare 4 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn CC4E(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture/compare 4 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn CC4P(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Capture/compare 4 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    pub fn CC4NE(&self) -> CC4NE_R {
        CC4NE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Capture/compare 4 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    pub fn CC4NP(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture/compare 5 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn CC5E(&self) -> CC5E_R {
        CC5E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture/compare 5 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn CC5P(&self) -> CC5P_R {
        CC5P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Capture/compare 6 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn CC6E(&self) -> CC6E_R {
        CC6E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Capture/compare 6 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn CC6P(&self) -> CC6P_R {
        CC6P_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to Table 619 for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn CC1E(&mut self) -> CC1E_W<'_, CCER_SPEC> {
        CC1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: the configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn CC1P(&mut self) -> CC1P_W<'_, CCER_SPEC> {
        CC1P_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/compare 1 complementary output enable Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn CC1NE(&mut self) -> CC1NE_W<'_, CCER_SPEC> {
        CC1NE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of tim_ti1fp1 and tim_ti2fp1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn CC1NP(&mut self) -> CC1NP_W<'_, CCER_SPEC> {
        CC1NP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare 2 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn CC2E(&mut self) -> CC2E_W<'_, CCER_SPEC> {
        CC2E_W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/compare 2 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn CC2P(&mut self) -> CC2P_W<'_, CCER_SPEC> {
        CC2P_W::new(self, 5)
    }
    #[doc = "Bit 6 - Capture/compare 2 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    pub fn CC2NE(&mut self) -> CC2NE_W<'_, CCER_SPEC> {
        CC2NE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Capture/compare 2 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    pub fn CC2NP(&mut self) -> CC2NP_W<'_, CCER_SPEC> {
        CC2NP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Capture/compare 3 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn CC3E(&mut self) -> CC3E_W<'_, CCER_SPEC> {
        CC3E_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/compare 3 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn CC3P(&mut self) -> CC3P_W<'_, CCER_SPEC> {
        CC3P_W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/compare 3 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    pub fn CC3NE(&mut self) -> CC3NE_W<'_, CCER_SPEC> {
        CC3NE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/compare 3 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    pub fn CC3NP(&mut self) -> CC3NP_W<'_, CCER_SPEC> {
        CC3NP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/compare 4 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn CC4E(&mut self) -> CC4E_W<'_, CCER_SPEC> {
        CC4E_W::new(self, 12)
    }
    #[doc = "Bit 13 - Capture/compare 4 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn CC4P(&mut self) -> CC4P_W<'_, CCER_SPEC> {
        CC4P_W::new(self, 13)
    }
    #[doc = "Bit 14 - Capture/compare 4 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    pub fn CC4NE(&mut self) -> CC4NE_W<'_, CCER_SPEC> {
        CC4NE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Capture/compare 4 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    pub fn CC4NP(&mut self) -> CC4NP_W<'_, CCER_SPEC> {
        CC4NP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Capture/compare 5 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn CC5E(&mut self) -> CC5E_W<'_, CCER_SPEC> {
        CC5E_W::new(self, 16)
    }
    #[doc = "Bit 17 - Capture/compare 5 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn CC5P(&mut self) -> CC5P_W<'_, CCER_SPEC> {
        CC5P_W::new(self, 17)
    }
    #[doc = "Bit 20 - Capture/compare 6 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn CC6E(&mut self) -> CC6E_W<'_, CCER_SPEC> {
        CC6E_W::new(self, 20)
    }
    #[doc = "Bit 21 - Capture/compare 6 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn CC6P(&mut self) -> CC6P_W<'_, CCER_SPEC> {
        CC6P_W::new(self, 21)
    }
}
#[doc = "TIM1 capture/compare enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCER_SPEC;
impl crate::RegisterSpec for CCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccer::R`](R) reader structure"]
impl crate::Readable for CCER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccer::W`](W) writer structure"]
impl crate::Writable for CCER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCER to value 0"]
impl crate::Resettable for CCER_SPEC {}
