#[doc = "Register `IPDR` reader"]
pub type R = crate::R<IPDR_SPEC>;
#[doc = "Field `ID` reader - Peripheral identifier These bits returns the ADC identifier. ID\\[31:0\\] = 0x0011 0006: c7amba_aditf5_90_v1"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral identifier These bits returns the ADC identifier. ID\\[31:0\\] = 0x0011 0006: c7amba_aditf5_90_v1"]
    #[inline(always)]
    pub fn ID(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "ADC identification register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPDR_SPEC;
impl crate::RegisterSpec for IPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipdr::R`](R) reader structure"]
impl crate::Readable for IPDR_SPEC {}
#[doc = "`reset()` method sets IPDR to value 0x0011_0006"]
impl crate::Resettable for IPDR_SPEC {
    const RESET_VALUE: u32 = 0x0011_0006;
}
