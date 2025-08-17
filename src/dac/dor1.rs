#[doc = "Register `DOR1` reader"]
pub type R = crate::R<DOR1_SPEC>;
#[doc = "Field `DACC1DOR` reader - DAC channel1 data output These bits are read-only, they contain data output for DAC channel1."]
pub type DACC1DOR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC1DORB` reader - DAC channel1 data output These bits are read-only. They contain data output for DAC channel1 B."]
pub type DACC1DORB_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel1 data output These bits are read-only, they contain data output for DAC channel1."]
    #[inline(always)]
    pub fn DACC1DOR(&self) -> DACC1DOR_R {
        DACC1DOR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel1 data output These bits are read-only. They contain data output for DAC channel1 B."]
    #[inline(always)]
    pub fn DACC1DORB(&self) -> DACC1DORB_R {
        DACC1DORB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "DAC channel1 data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOR1_SPEC;
impl crate::RegisterSpec for DOR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor1::R`](R) reader structure"]
impl crate::Readable for DOR1_SPEC {}
#[doc = "`reset()` method sets DOR1 to value 0"]
impl crate::Resettable for DOR1_SPEC {}
