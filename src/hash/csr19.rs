#[doc = "Register `CSR19` reader"]
pub type R = crate::R<CSR19_SPEC>;
#[doc = "Register `CSR19` writer"]
pub type W = crate::W<CSR19_SPEC>;
#[doc = "Field `CS19` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS19_R = crate::FieldReader<u32>;
#[doc = "Field `CS19` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS19_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS19(&self) -> CS19_R {
        CS19_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS19(&mut self) -> CS19_W<'_, CSR19_SPEC> {
        CS19_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 19\n\nYou can [`read`](crate::Reg::read) this register and get [`csr19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR19_SPEC;
impl crate::RegisterSpec for CSR19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr19::R`](R) reader structure"]
impl crate::Readable for CSR19_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr19::W`](W) writer structure"]
impl crate::Writable for CSR19_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR19 to value 0"]
impl crate::Resettable for CSR19_SPEC {}
