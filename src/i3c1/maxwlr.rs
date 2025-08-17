#[doc = "Register `MAXWLR` reader"]
pub type R = crate::R<MAXWLR_SPEC>;
#[doc = "Register `MAXWLR` writer"]
pub type W = crate::W<MAXWLR_SPEC>;
#[doc = "Field `MWL` reader - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
pub type MWL_R = crate::FieldReader<u16>;
#[doc = "Field `MWL` writer - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
pub type MWL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
    #[inline(always)]
    pub fn MWL(&self) -> MWL_R {
        MWL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - maximum data write length (when I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0 and updated by hardware on the reception of SETMWL command. Software is notified of a MWL update by the I3C_EVR.MWLUPF and the corresponding interrupt if enabled. This field is used by hardware to return the value on the I3C bus when the target receives a GETMWL CCC."]
    #[inline(always)]
    pub fn MWL(&mut self) -> MWL_W<'_, MAXWLR_SPEC> {
        MWL_W::new(self, 0)
    }
}
#[doc = "I3C maximum write length register\n\nYou can [`read`](crate::Reg::read) this register and get [`maxwlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxwlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAXWLR_SPEC;
impl crate::RegisterSpec for MAXWLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxwlr::R`](R) reader structure"]
impl crate::Readable for MAXWLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maxwlr::W`](W) writer structure"]
impl crate::Writable for MAXWLR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets MAXWLR to value 0"]
impl crate::Resettable for MAXWLR_SPEC {}
