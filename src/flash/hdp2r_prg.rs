#[doc = "Register `HDP2R_PRG` reader"]
pub type R = crate::R<HDP2R_PRG_SPEC>;
#[doc = "Register `HDP2R_PRG` writer"]
pub type W = crate::W<HDP2R_PRG_SPEC>;
#[doc = "Field `HDP2_STRT` reader - Bank 2 HDPL barrier start set in number of 8 Kbytes sectors"]
pub type HDP2_STRT_R = crate::FieldReader;
#[doc = "Field `HDP2_STRT` writer - Bank 2 HDPL barrier start set in number of 8 Kbytes sectors"]
pub type HDP2_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HDP2_END` reader - Bank 2 HDPL barrier end set in number of 8 Kbytes sectors"]
pub type HDP2_END_R = crate::FieldReader;
#[doc = "Field `HDP2_END` writer - Bank 2 HDPL barrier end set in number of 8 Kbytes sectors"]
pub type HDP2_END_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Bank 2 HDPL barrier start set in number of 8 Kbytes sectors"]
    #[inline(always)]
    pub fn HDP2_STRT(&self) -> HDP2_STRT_R {
        HDP2_STRT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - Bank 2 HDPL barrier end set in number of 8 Kbytes sectors"]
    #[inline(always)]
    pub fn HDP2_END(&self) -> HDP2_END_R {
        HDP2_END_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bank 2 HDPL barrier start set in number of 8 Kbytes sectors"]
    #[inline(always)]
    pub fn HDP2_STRT(&mut self) -> HDP2_STRT_W<'_, HDP2R_PRG_SPEC> {
        HDP2_STRT_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - Bank 2 HDPL barrier end set in number of 8 Kbytes sectors"]
    #[inline(always)]
    pub fn HDP2_END(&mut self) -> HDP2_END_W<'_, HDP2R_PRG_SPEC> {
        HDP2_END_W::new(self, 16)
    }
}
#[doc = "FLASH HDP Bank2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`hdp2r_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdp2r_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDP2R_PRG_SPEC;
impl crate::RegisterSpec for HDP2R_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdp2r_prg::R`](R) reader structure"]
impl crate::Readable for HDP2R_PRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hdp2r_prg::W`](W) writer structure"]
impl crate::Writable for HDP2R_PRG_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets HDP2R_PRG to value 0"]
impl crate::Resettable for HDP2R_PRG_SPEC {}
