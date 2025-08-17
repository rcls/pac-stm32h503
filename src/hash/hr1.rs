#[doc = "Register `HR1` reader"]
pub type R = crate::R<HR1_SPEC>;
#[doc = "Field `H1` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn H1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
#[doc = "HASH digest register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR1_SPEC;
impl crate::RegisterSpec for HR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr1::R`](R) reader structure"]
impl crate::Readable for HR1_SPEC {}
#[doc = "`reset()` method sets HR1 to value 0"]
impl crate::Resettable for HR1_SPEC {}
