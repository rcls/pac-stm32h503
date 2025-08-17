#[doc = "Register `CSICFGR` reader"]
pub type R = crate::R<CSICFGR_SPEC>;
#[doc = "Register `CSICFGR` writer"]
pub type W = crate::W<CSICFGR_SPEC>;
#[doc = "Field `CSICAL` reader - CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value."]
pub type CSICAL_R = crate::FieldReader;
#[doc = "Field `CSICAL` writer - CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value."]
pub type CSICAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSITRIM` reader - CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20."]
pub type CSITRIM_R = crate::FieldReader;
#[doc = "Field `CSITRIM` writer - CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20."]
pub type CSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value."]
    #[inline(always)]
    pub fn CSICAL(&self) -> CSICAL_R {
        CSICAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20."]
    #[inline(always)]
    pub fn CSITRIM(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value."]
    #[inline(always)]
    pub fn CSICAL(&mut self) -> CSICAL_W<'_, CSICFGR_SPEC> {
        CSICAL_W::new(self, 0)
    }
    #[doc = "Bits 16:21 - CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20."]
    #[inline(always)]
    pub fn CSITRIM(&mut self) -> CSITRIM_W<'_, CSICFGR_SPEC> {
        CSITRIM_W::new(self, 16)
    }
}
#[doc = "RCC CSI calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`csicfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csicfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSICFGR_SPEC;
impl crate::RegisterSpec for CSICFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csicfgr::R`](R) reader structure"]
impl crate::Readable for CSICFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csicfgr::W`](W) writer structure"]
impl crate::Writable for CSICFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSICFGR to value 0x0020_0000"]
impl crate::Resettable for CSICFGR_SPEC {
    const RESET_VALUE: u32 = 0x0020_0000;
}
