#[doc = "Register `TXEFA` reader"]
pub type R = crate::R<TXEFA_SPEC>;
#[doc = "Register `TXEFA` writer"]
pub type W = crate::W<TXEFA_SPEC>;
#[doc = "Field `EFAI` reader - Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\\[EFGI\\] to EFAI + 1 and updates the FIFO 0 fill level TXEFS\\[EFFL\\]."]
pub type EFAI_R = crate::FieldReader;
#[doc = "Field `EFAI` writer - Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\\[EFGI\\] to EFAI + 1 and updates the FIFO 0 fill level TXEFS\\[EFFL\\]."]
pub type EFAI_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\\[EFGI\\] to EFAI + 1 and updates the FIFO 0 fill level TXEFS\\[EFFL\\]."]
    #[inline(always)]
    pub fn EFAI(&self) -> EFAI_R {
        EFAI_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\\[EFGI\\] to EFAI + 1 and updates the FIFO 0 fill level TXEFS\\[EFFL\\]."]
    #[inline(always)]
    pub fn EFAI(&mut self) -> EFAI_W<'_, TXEFA_SPEC> {
        EFAI_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx event FIFO acknowledge register\n\nYou can [`read`](crate::Reg::read) this register and get [`txefa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXEFA_SPEC;
impl crate::RegisterSpec for TXEFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefa::R`](R) reader structure"]
impl crate::Readable for TXEFA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txefa::W`](W) writer structure"]
impl crate::Writable for TXEFA_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TXEFA to value 0"]
impl crate::Resettable for TXEFA_SPEC {}
