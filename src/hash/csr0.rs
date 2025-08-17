#[doc = "Register `CSR0` reader"]
pub type R = crate::R<CSR0_SPEC>;
#[doc = "Register `CSR0` writer"]
pub type W = crate::W<CSR0_SPEC>;
#[doc = "Field `CS0` reader - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS0_R = crate::FieldReader<u32>;
#[doc = "Field `CS0` writer - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
pub type CS0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS0(&self) -> CS0_R {
        CS0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Context swap x Refer to Section 24.7.7: HASH context swap registers introduction."]
    #[inline(always)]
    pub fn CS0(&mut self) -> CS0_W<'_, CSR0_SPEC> {
        CS0_W::new(self, 0)
    }
}
#[doc = "HASH context swap register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR0_SPEC;
impl crate::RegisterSpec for CSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr0::R`](R) reader structure"]
impl crate::Readable for CSR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr0::W`](W) writer structure"]
impl crate::Writable for CSR0_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CSR0 to value 0"]
impl crate::Resettable for CSR0_SPEC {}
