#[doc = "Register `CCR5` reader"]
pub type R = crate::R<CCR5_SPEC>;
#[doc = "Register `CCR5` writer"]
pub type W = crate::W<CCR5_SPEC>;
#[doc = "Field `CCR5` reader - Capture/compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc5 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR5\\[15:0\\]. The CCR5\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR5\\[19:4\\]. The CCR5\\[3:0\\] bitfield contains the dithered part."]
pub type CCR5_R = crate::FieldReader<u32>;
#[doc = "Field `CCR5` writer - Capture/compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc5 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR5\\[15:0\\]. The CCR5\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR5\\[19:4\\]. The CCR5\\[3:0\\] bitfield contains the dithered part."]
pub type CCR5_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C1_A {
    #[doc = "0: No effect of oc5ref on oc1refc"]
    B_0x0 = 0,
    #[doc = "1: oc1refc is the logical AND of oc1ref and oc5ref"]
    B_0x1 = 1,
}
impl From<GC5C1_A> for bool {
    #[inline(always)]
    fn from(variant: GC5C1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GC5C1` reader - Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C1_R = crate::BitReader<GC5C1_A>;
impl GC5C1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GC5C1_A {
        match self.bits {
            false => GC5C1_A::B_0x0,
            true => GC5C1_A::B_0x1,
        }
    }
    #[doc = "No effect of oc5ref on oc1refc"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GC5C1_A::B_0x0
    }
    #[doc = "oc1refc is the logical AND of oc1ref and oc5ref"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GC5C1_A::B_0x1
    }
}
#[doc = "Field `GC5C1` writer - Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C1_W<'a, REG> = crate::BitWriter<'a, REG, GC5C1_A>;
impl<'a, REG> GC5C1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect of oc5ref on oc1refc"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C1_A::B_0x0)
    }
    #[doc = "oc1refc is the logical AND of oc1ref and oc5ref"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C1_A::B_0x1)
    }
}
#[doc = "Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C2_A {
    #[doc = "0: No effect of tim_oc5ref on tim_oc2refc"]
    B_0x0 = 0,
    #[doc = "1: tim_oc2refc is the logical AND of tim_oc2ref and tim_oc5ref"]
    B_0x1 = 1,
}
impl From<GC5C2_A> for bool {
    #[inline(always)]
    fn from(variant: GC5C2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GC5C2` reader - Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C2_R = crate::BitReader<GC5C2_A>;
impl GC5C2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GC5C2_A {
        match self.bits {
            false => GC5C2_A::B_0x0,
            true => GC5C2_A::B_0x1,
        }
    }
    #[doc = "No effect of tim_oc5ref on tim_oc2refc"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GC5C2_A::B_0x0
    }
    #[doc = "tim_oc2refc is the logical AND of tim_oc2ref and tim_oc5ref"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GC5C2_A::B_0x1
    }
}
#[doc = "Field `GC5C2` writer - Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C2_W<'a, REG> = crate::BitWriter<'a, REG, GC5C2_A>;
impl<'a, REG> GC5C2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect of tim_oc5ref on tim_oc2refc"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C2_A::B_0x0)
    }
    #[doc = "tim_oc2refc is the logical AND of tim_oc2ref and tim_oc5ref"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C2_A::B_0x1)
    }
}
#[doc = "Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC5C3_A {
    #[doc = "0: No effect of tim_oc5ref on tim_oc3refc"]
    B_0x0 = 0,
    #[doc = "1: tim_oc3refc is the logical AND of tim_oc3ref and tim_oc5ref"]
    B_0x1 = 1,
}
impl From<GC5C3_A> for bool {
    #[inline(always)]
    fn from(variant: GC5C3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GC5C3` reader - Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C3_R = crate::BitReader<GC5C3_A>;
impl GC5C3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GC5C3_A {
        match self.bits {
            false => GC5C3_A::B_0x0,
            true => GC5C3_A::B_0x1,
        }
    }
    #[doc = "No effect of tim_oc5ref on tim_oc3refc"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == GC5C3_A::B_0x0
    }
    #[doc = "tim_oc3refc is the logical AND of tim_oc3ref and tim_oc5ref"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == GC5C3_A::B_0x1
    }
}
#[doc = "Field `GC5C3` writer - Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
pub type GC5C3_W<'a, REG> = crate::BitWriter<'a, REG, GC5C3_A>;
impl<'a, REG> GC5C3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect of tim_oc5ref on tim_oc3refc"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C3_A::B_0x0)
    }
    #[doc = "tim_oc3refc is the logical AND of tim_oc3ref and tim_oc5ref"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GC5C3_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:19 - Capture/compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc5 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR5\\[15:0\\]. The CCR5\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR5\\[19:4\\]. The CCR5\\[3:0\\] bitfield contains the dithered part."]
    #[inline(always)]
    pub fn CCR5(&self) -> CCR5_R {
        CCR5_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 29 - Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn GC5C1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn GC5C2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn GC5C3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Capture/compare 5 value CCR5 is the value to be loaded in the actual capture/compare 5 register (preload value). It is loaded permanently if the preload feature is not selected in the TIMx_CCMR3 register (bit OC5PE). Else the preload value is copied in the active capture/compare 5 register when an update event occurs. The active capture/compare register contains the value to be compared to the counter TIMx_CNT and signaled on tim_oc5 output. Non-dithering mode (DITHEN = 0) The register holds the compare value in CCR5\\[15:0\\]. The CCR5\\[19:16\\] bits are reset. Dithering mode (DITHEN = 1) The register holds the integer part in CCR5\\[19:4\\]. The CCR5\\[3:0\\] bitfield contains the dithered part."]
    #[inline(always)]
    pub fn CCR5(&mut self) -> CCR5_W<'_, CCR5_SPEC> {
        CCR5_W::new(self, 0)
    }
    #[doc = "Bit 29 - Group channel 5 and channel 1 Distortion on channel 1 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn GC5C1(&mut self) -> GC5C1_W<'_, CCR5_SPEC> {
        GC5C1_W::new(self, 29)
    }
    #[doc = "Bit 30 - Group channel 5 and channel 2 Distortion on channel 2 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR1). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn GC5C2(&mut self) -> GC5C2_W<'_, CCR5_SPEC> {
        GC5C2_W::new(self, 30)
    }
    #[doc = "Bit 31 - Group channel 5 and channel 3 Distortion on channel 3 output: This bit can either have immediate effect or be preloaded and taken into account after an update event (if preload feature is selected in TIMxCCMR2). Note: it is also possible to apply this distortion on combined PWM signals."]
    #[inline(always)]
    pub fn GC5C3(&mut self) -> GC5C3_W<'_, CCR5_SPEC> {
        GC5C3_W::new(self, 31)
    }
}
#[doc = "TIM1 capture/compare register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR5_SPEC;
impl crate::RegisterSpec for CCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr5::R`](R) reader structure"]
impl crate::Readable for CCR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr5::W`](W) writer structure"]
impl crate::Writable for CCR5_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCR5 to value 0"]
impl crate::Resettable for CCR5_SPEC {}
