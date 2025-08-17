#[doc = "Register `CRRCR` reader"]
pub type R = crate::R<CRRCR_SPEC>;
#[doc = "Field `HSI48CAL` reader - Internal RC 48 MHz clock calibration Set by hardware by option-byte loading during system reset NRESET. Read-only."]
pub type HSI48CAL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Internal RC 48 MHz clock calibration Set by hardware by option-byte loading during system reset NRESET. Read-only."]
    #[inline(always)]
    pub fn HSI48CAL(&self) -> HSI48CAL_R {
        HSI48CAL_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "RCC clock recovery RC register\n\nYou can [`read`](crate::Reg::read) this register and get [`crrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRRCR_SPEC;
impl crate::RegisterSpec for CRRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crrcr::R`](R) reader structure"]
impl crate::Readable for CRRCR_SPEC {}
#[doc = "`reset()` method sets CRRCR to value 0"]
impl crate::Resettable for CRRCR_SPEC {}
