#[doc = "Register `HR0` reader"]
pub type R = crate::R<HR0_SPEC>;
#[doc = "Field `H0` reader - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
pub type H0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to Section 24.7.4: HASH digest registers introduction."]
    #[inline(always)]
    pub fn H0(&self) -> H0_R {
        H0_R::new(self.bits)
    }
}
#[doc = "HASH digest register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR0_SPEC;
impl crate::RegisterSpec for HR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr0::R`](R) reader structure"]
impl crate::Readable for HR0_SPEC {}
#[doc = "`reset()` method sets HR0 to value 0"]
impl crate::Resettable for HR0_SPEC {}
