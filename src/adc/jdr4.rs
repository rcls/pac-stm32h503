#[doc = "Register `JDR4` reader"]
pub type R = crate::R<JDR4_SPEC>;
#[doc = "Field `JDATA` reader - Injected data These bits are read-only. They contain the conversion result from injected channel y. The data are left -or right-aligned as described in ."]
pub type JDATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Injected data These bits are read-only. They contain the conversion result from injected channel y. The data are left -or right-aligned as described in ."]
    #[inline(always)]
    pub fn JDATA(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC injected channel 4 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JDR4_SPEC;
impl crate::RegisterSpec for JDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr4::R`](R) reader structure"]
impl crate::Readable for JDR4_SPEC {}
#[doc = "`reset()` method sets JDR4 to value 0"]
impl crate::Resettable for JDR4_SPEC {}
