#[doc = "Register `WRPSGN1R_PRG` reader"]
pub type R = crate::R<WRPSGN1R_PRG_SPEC>;
#[doc = "Register `WRPSGN1R_PRG` writer"]
pub type W = crate::W<WRPSGN1R_PRG_SPEC>;
#[doc = "Field `WRPSG1` reader - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)"]
pub type WRPSG1_R = crate::FieldReader;
#[doc = "Field `WRPSG1` writer - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)"]
pub type WRPSG1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)"]
    #[inline(always)]
    pub fn WRPSG1(&self) -> WRPSG1_R {
        WRPSG1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank1 sector protection option status byte Setting WRPSG1 bits to 0 write protects the corresponding sectors in bank 1 (0: write protected; 1: not write protected)"]
    #[inline(always)]
    pub fn WRPSG1(&mut self) -> WRPSG1_W<'_, WRPSGN1R_PRG_SPEC> {
        WRPSG1_W::new(self, 0)
    }
}
#[doc = "FLASH write sector protection for Bank1\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpsgn1r_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpsgn1r_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPSGN1R_PRG_SPEC;
impl crate::RegisterSpec for WRPSGN1R_PRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpsgn1r_prg::R`](R) reader structure"]
impl crate::Readable for WRPSGN1R_PRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wrpsgn1r_prg::W`](W) writer structure"]
impl crate::Writable for WRPSGN1R_PRG_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets WRPSGN1R_PRG to value 0"]
impl crate::Resettable for WRPSGN1R_PRG_SPEC {}
