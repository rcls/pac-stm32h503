#[doc = "Register `POL` reader"]
pub type R = crate::R<POL_SPEC>;
#[doc = "Register `POL` writer"]
pub type W = crate::W<POL_SPEC>;
#[doc = "Field `POL` reader - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value."]
pub type POL_R = crate::FieldReader<u32>;
#[doc = "Field `POL` writer - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value."]
pub type POL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value."]
    #[inline(always)]
    pub fn POL(&self) -> POL_R {
        POL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable polynomial This register is used to write the coefficients of the polynomial to be used for CRC calculation. If the polynomial size is less than 32 bits, the least significant bits have to be used to program the correct value."]
    #[inline(always)]
    pub fn POL(&mut self) -> POL_W<'_, POL_SPEC> {
        POL_W::new(self, 0)
    }
}
#[doc = "CRC polynomial\n\nYou can [`read`](crate::Reg::read) this register and get [`pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POL_SPEC;
impl crate::RegisterSpec for POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pol::R`](R) reader structure"]
impl crate::Readable for POL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pol::W`](W) writer structure"]
impl crate::Writable for POL_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets POL to value 0x04c1_1db7"]
impl crate::Resettable for POL_SPEC {
    const RESET_VALUE: u32 = 0x04c1_1db7;
}
