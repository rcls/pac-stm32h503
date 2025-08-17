#[doc = "Register `TIMINGR0` reader"]
pub type R = crate::R<TIMINGR0_SPEC>;
#[doc = "Register `TIMINGR0` writer"]
pub type W = crate::W<TIMINGR0_SPEC>;
#[doc = "Field `SCLL_PP` reader - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
pub type SCLL_PP_R = crate::FieldReader;
#[doc = "Field `SCLL_PP` writer - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
pub type SCLL_PP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLH_I3C` reader - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
pub type SCLH_I3C_R = crate::FieldReader;
#[doc = "Field `SCLH_I3C` writer - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
pub type SCLH_I3C_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLL_OD` reader - SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
pub type SCLL_OD_R = crate::FieldReader;
#[doc = "Field `SCLL_OD` writer - SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
pub type SCLL_OD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLH_I2C` reader - SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
pub type SCLH_I2C_R = crate::FieldReader;
#[doc = "Field `SCLH_I2C` writer - SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
pub type SCLH_I2C_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
    #[inline(always)]
    pub fn SCLL_PP(&self) -> SCLL_PP_R {
        SCLL_PP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
    #[inline(always)]
    pub fn SCLH_I3C(&self) -> SCLH_I3C_R {
        SCLH_I3C_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
    #[inline(always)]
    pub fn SCLL_OD(&self) -> SCLL_OD_R {
        SCLL_OD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
    #[inline(always)]
    pub fn SCLH_I2C(&self) -> SCLH_I2C_R {
        SCLH_I2C_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low duration in I3C push-pull phases, in number of kernel clocks cycles: tSCLL_PP = (SCLL_PP + 1) x tI3CCLK SCLL_PP is used to generate tLOW (I3C) timing."]
    #[inline(always)]
    pub fn SCLL_PP(&mut self) -> SCLL_PP_W<'_, TIMINGR0_SPEC> {
        SCLL_PP_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles: tSCLH_I3C = (SCLH_I3C + 1) x tI3CCLK SCLH_I3C is used to generate both tHIGH (I3C) and tHIGH_MIXED timings."]
    #[inline(always)]
    pub fn SCLH_I3C(&mut self) -> SCLH_I3C_W<'_, TIMINGR0_SPEC> {
        SCLH_I3C_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - SCL low duration in open-drain phases, used for legacy I2C commands and for I3C open-drain phases (address header phase following a START, not a Repeated START), in number of kernel clocks cycles: tSCLL_OD = (SCLL_OD + 1) x tI3CCLK SCLL_OD is used to generate both tLOW (I2C) and tLOW_OD timings (max. of the two)."]
    #[inline(always)]
    pub fn SCLL_OD(&mut self) -> SCLL_OD_W<'_, TIMINGR0_SPEC> {
        SCLL_OD_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - SCL high duration, used for legacy I2C commands, in number of kernel clocks cycles: tSCLH_I2C = (SCLH_I2C + 1) x tI3CCLK SCLH_I2C is used to generate tHIGH (I2C) timing."]
    #[inline(always)]
    pub fn SCLH_I2C(&mut self) -> SCLH_I2C_W<'_, TIMINGR0_SPEC> {
        SCLH_I2C_W::new(self, 24)
    }
}
#[doc = "I3C timing register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`timingr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMINGR0_SPEC;
impl crate::RegisterSpec for TIMINGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timingr0::R`](R) reader structure"]
impl crate::Readable for TIMINGR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timingr0::W`](W) writer structure"]
impl crate::Writable for TIMINGR0_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TIMINGR0 to value 0"]
impl crate::Resettable for TIMINGR0_SPEC {}
