#[doc = "Register `2DEAR` reader"]
pub type R = crate::R<_2DEAR_SPEC>;
#[doc = "Field `EDEA` reader - ECC double error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC double error."]
pub type EDEA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC double error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC double error."]
    #[inline(always)]
    pub fn EDEA(&self) -> EDEA_R {
        EDEA_R::new(self.bits)
    }
}
#[doc = "RAMCFG memory 2 ECC double error address register\n\nYou can [`read`](crate::Reg::read) this register and get [`_2dear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _2DEAR_SPEC;
impl crate::RegisterSpec for _2DEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_2dear::R`](R) reader structure"]
impl crate::Readable for _2DEAR_SPEC {}
#[doc = "`reset()` method sets 2DEAR to value 0"]
impl crate::Resettable for _2DEAR_SPEC {}
