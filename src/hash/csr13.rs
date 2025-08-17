#[doc = "Register `CSR13` reader"]
pub type R = crate::R<CSR13_SPEC>;
#[doc = "Register `CSR13` writer"]
pub type W = crate::W<CSR13_SPEC>;
#[doc = "Field `CS13` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS13_R = crate::FieldReader<u32>;
#[doc = "Field `CS13` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS13_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS13(&self) -> CS13_R {
        CS13_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS13(&mut self) -> CS13_W<'_, CSR13_SPEC> {
        CS13_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`csr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR13_SPEC;
impl crate::RegisterSpec for CSR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr13::R`](R) reader structure"]
impl crate::Readable for CSR13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr13::W`](W) writer structure"]
impl crate::Writable for CSR13_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR13 to value 0"]
impl crate::Resettable for CSR13_SPEC {}
