#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Field `TSIZE` reader - number of data at current transfer When these bits are changed by software, the SPI has to be disabled. Endless transaction is initialized when CSTART is set while zero value is stored at TSIZE. TSIZE cannot be set to 0xFFFF respective 0x3FFF value when CRC is enabled. Note: TSIZE\\[15:10\\] bits are reserved at limited feature set instances and must be kept at reset value."]
pub type TSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `TSIZE` writer - number of data at current transfer When these bits are changed by software, the SPI has to be disabled. Endless transaction is initialized when CSTART is set while zero value is stored at TSIZE. TSIZE cannot be set to 0xFFFF respective 0x3FFF value when CRC is enabled. Note: TSIZE\\[15:10\\] bits are reserved at limited feature set instances and must be kept at reset value."]
pub type TSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - number of data at current transfer When these bits are changed by software, the SPI has to be disabled. Endless transaction is initialized when CSTART is set while zero value is stored at TSIZE. TSIZE cannot be set to 0xFFFF respective 0x3FFF value when CRC is enabled. Note: TSIZE\\[15:10\\] bits are reserved at limited feature set instances and must be kept at reset value."]
    #[inline(always)]
    pub fn TSIZE(&self) -> TSIZE_R {
        TSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - number of data at current transfer When these bits are changed by software, the SPI has to be disabled. Endless transaction is initialized when CSTART is set while zero value is stored at TSIZE. TSIZE cannot be set to 0xFFFF respective 0x3FFF value when CRC is enabled. Note: TSIZE\\[15:10\\] bits are reserved at limited feature set instances and must be kept at reset value."]
    #[inline(always)]
    pub fn TSIZE(&mut self) -> TSIZE_W<'_, CR2_SPEC> {
        TSIZE_W::new(self, 0)
    }
}
#[doc = "SPI/I2S control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {}
