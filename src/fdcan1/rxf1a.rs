#[doc = "Register `RXF1A` reader"]
pub type R = crate::R<RXF1A_SPEC>;
#[doc = "Register `RXF1A` writer"]
pub type W = crate::W<RXF1A_SPEC>;
#[doc = "Field `F1AI` reader - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
pub type F1AI_R = crate::FieldReader;
#[doc = "Field `F1AI` writer - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
pub type F1AI_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
    #[inline(always)]
    pub fn F1AI(&self) -> F1AI_R {
        F1AI_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\] to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
    #[inline(always)]
    pub fn F1AI(&mut self) -> F1AI_W<'_, RXF1A_SPEC> {
        F1AI_W::new(self, 0)
    }
}
#[doc = "FDCAN Rx FIFO 1 acknowledge register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf1a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf1a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF1A_SPEC;
impl crate::RegisterSpec for RXF1A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf1a::R`](R) reader structure"]
impl crate::Readable for RXF1A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxf1a::W`](W) writer structure"]
impl crate::Writable for RXF1A_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RXF1A to value 0"]
impl crate::Resettable for RXF1A_SPEC {}
