#[doc = "Register `RAMPVALR` reader"]
pub type R = crate::R<RAMPVALR_SPEC>;
#[doc = "Field `TS1_RAMP_COEFF` reader - Engineering value of the ramp coefficient for the temperature sensor 1. This value is expressed in Hz/ C."]
pub type TS1_RAMP_COEFF_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Engineering value of the ramp coefficient for the temperature sensor 1. This value is expressed in Hz/ C."]
    #[inline(always)]
    pub fn TS1_RAMP_COEFF(&self) -> TS1_RAMP_COEFF_R {
        TS1_RAMP_COEFF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Temperature sensor ramp value register\n\nYou can [`read`](crate::Reg::read) this register and get [`rampvalr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAMPVALR_SPEC;
impl crate::RegisterSpec for RAMPVALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rampvalr::R`](R) reader structure"]
impl crate::Readable for RAMPVALR_SPEC {}
#[doc = "`reset()` method sets RAMPVALR to value 0"]
impl crate::Resettable for RAMPVALR_SPEC {}
