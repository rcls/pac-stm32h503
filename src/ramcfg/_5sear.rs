#[doc = "Register `5SEAR` reader"]
pub type R = crate::R<_5SEAR_SPEC>;
#[doc = "Field `ESEA` reader - ECC single error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC single error."]
pub type ESEA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC single error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC single error."]
    #[inline(always)]
    pub fn ESEA(&self) -> ESEA_R {
        ESEA_R::new(self.bits)
    }
}
#[doc = "RAMCFG memory 5 ECC single error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`_5sear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _5SEAR_SPEC;
impl crate::RegisterSpec for _5SEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_5sear::R`](R) reader structure"]
impl crate::Readable for _5SEAR_SPEC {}
#[doc = "`reset()` method sets 5SEAR to value 0"]
impl crate::Resettable for _5SEAR_SPEC {}
