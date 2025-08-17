#[doc = "Register `ECR` reader"]
pub type R = crate::R<ECR_SPEC>;
#[doc = "Register `ECR` writer"]
pub type W = crate::W<ECR_SPEC>;
#[doc = "Field `TEC` reader - Transmit error counter Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `REC` reader - Receive error counter Actual state of the receive error counter, values between 0 and 127."]
pub type REC_R = crate::FieldReader;
#[doc = "Receive error passive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RP_A {
    #[doc = "0: The receive error counter is below the error passive level of 128."]
    B_0x0 = 0,
    #[doc = "1: The receive error counter has reached the error passive level of 128."]
    B_0x1 = 1,
}
impl From<RP_A> for bool {
    #[inline(always)]
    fn from(variant: RP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RP` reader - Receive error passive"]
pub type RP_R = crate::BitReader<RP_A>;
impl RP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RP_A {
        match self.bits {
            false => RP_A::B_0x0,
            true => RP_A::B_0x1,
        }
    }
    #[doc = "The receive error counter is below the error passive level of 128."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == RP_A::B_0x0
    }
    #[doc = "The receive error counter has reached the error passive level of 128."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == RP_A::B_0x1
    }
}
#[doc = "Field `CEL` reader - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
pub type CEL_R = crate::FieldReader;
#[doc = "Field `CEL` writer - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
pub type CEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit error counter Actual state of the transmit error counter, values between 0 and 255. When CCCR.ASM is set, the CAN protocol controller does not increment TEC and REC when a CAN protocol error is detected, but CEL is still incremented."]
    #[inline(always)]
    pub fn TEC(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive error counter Actual state of the receive error counter, values between 0 and 127."]
    #[inline(always)]
    pub fn REC(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive error passive"]
    #[inline(always)]
    pub fn RP(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn CEL(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - CAN error logging The counter is incremented each time when a CAN protocol error causes the transmit error counter or the receive error counter to be incremented. It is reset by read access to CEL. The counter stops at 0xFF; the next increment of TEC or REC sets interrupt flag IR\\[ELO\\]. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn CEL(&mut self) -> CEL_W<'_, ECR_SPEC> {
        CEL_W::new(self, 16)
    }
}
#[doc = "FDCAN error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for ECR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECR_SPEC {}
