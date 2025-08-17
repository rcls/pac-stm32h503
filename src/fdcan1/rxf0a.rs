#[doc = "Register `RXF0A` reader"]
pub type R = crate::R<RXF0A_SPEC>;
#[doc = "Register `RXF0A` writer"]
pub type W = crate::W<RXF0A_SPEC>;
#[doc = "Field `F0AI` reader - Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFO 0 get index RXF0S\\[F0GI\\] to F0AI + 1 and update the FIFO 0 fill level RXF0S\\[F0FL\\]."]
pub type F0AI_R = crate::FieldReader;
#[doc = "Field `F0AI` writer - Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFO 0 get index RXF0S\\[F0GI\\] to F0AI + 1 and update the FIFO 0 fill level RXF0S\\[F0FL\\]."]
pub type F0AI_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFO 0 get index RXF0S\\[F0GI\\] to F0AI + 1 and update the FIFO 0 fill level RXF0S\\[F0FL\\]."]
    #[inline(always)]
    pub fn F0AI(&self) -> F0AI_R {
        F0AI_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFO 0 get index RXF0S\\[F0GI\\] to F0AI + 1 and update the FIFO 0 fill level RXF0S\\[F0FL\\]."]
    #[inline(always)]
    pub fn F0AI(&mut self) -> F0AI_W<'_, RXF0A_SPEC> {
        F0AI_W::new(self, 0)
    }
}
#[doc = "CAN Rx FIFO 0 acknowledge register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXF0A_SPEC;
impl crate::RegisterSpec for RXF0A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0a::R`](R) reader structure"]
impl crate::Readable for RXF0A_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxf0a::W`](W) writer structure"]
impl crate::Writable for RXF0A_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RXF0A to value 0"]
impl crate::Resettable for RXF0A_SPEC {}
