#[doc = "Register `DOR2` reader"]
pub type R = crate::R<DOR2_SPEC>;
#[doc = "Field `DACC2DOR` reader - DAC channel2 data output These bits are read-only, they contain data output for DAC channel2."]
pub type DACC2DOR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC2DORB` reader - DAC channel2 data output These bits are read-only. They contain data output for DAC channel2 B."]
pub type DACC2DORB_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel2 data output These bits are read-only, they contain data output for DAC channel2."]
    #[inline(always)]
    pub fn DACC2DOR(&self) -> DACC2DOR_R {
        DACC2DOR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel2 data output These bits are read-only. They contain data output for DAC channel2 B."]
    #[inline(always)]
    pub fn DACC2DORB(&self) -> DACC2DORB_R {
        DACC2DORB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "DAC channel2 data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOR2_SPEC;
impl crate::RegisterSpec for DOR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor2::R`](R) reader structure"]
impl crate::Readable for DOR2_SPEC {}
#[doc = "`reset()` method sets DOR2 to value 0"]
impl crate::Resettable for DOR2_SPEC {}
