#[doc = "Register `TGTTDR` reader"]
pub type R = crate::R<TGTTDR_SPEC>;
#[doc = "Register `TGTTDR` writer"]
pub type W = crate::W<TGTTDR_SPEC>;
#[doc = "Field `TGTTDCNT` reader - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO."]
pub type TGTTDCNT_R = crate::FieldReader<u16>;
#[doc = "Field `TGTTDCNT` writer - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO."]
pub type TGTTDCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRELOAD_A {
    #[doc = "0: no TX-FIFO preload"]
    B_0x0 = 0,
    #[doc = "1: TX-FIFO preload"]
    B_0x1 = 1,
}
impl From<PRELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: PRELOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRELOAD` reader - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO."]
pub type PRELOAD_R = crate::BitReader<PRELOAD_A>;
impl PRELOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRELOAD_A {
        match self.bits {
            false => PRELOAD_A::B_0x0,
            true => PRELOAD_A::B_0x1,
        }
    }
    #[doc = "no TX-FIFO preload"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PRELOAD_A::B_0x0
    }
    #[doc = "TX-FIFO preload"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PRELOAD_A::B_0x1
    }
}
#[doc = "Field `PRELOAD` writer - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO."]
pub type PRELOAD_W<'a, REG> = crate::BitWriter<'a, REG, PRELOAD_A>;
impl<'a, REG> PRELOAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no TX-FIFO preload"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PRELOAD_A::B_0x0)
    }
    #[doc = "TX-FIFO preload"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PRELOAD_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:15 - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO."]
    #[inline(always)]
    pub fn TGTTDCNT(&self) -> TGTTDCNT_R {
        TGTTDCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO."]
    #[inline(always)]
    pub fn PRELOAD(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - transmit data counter, in bytes (when I3C is configured as target) This field must be written by software in the same access when is asserted PRELOAD, in order to define the number of bytes to preload and to transmit. This field is updated by hardware and reports, when read, the remaining number of bytes to be loaded into the TX-FIFO."]
    #[inline(always)]
    pub fn TGTTDCNT(&mut self) -> TGTTDCNT_W<'_, TGTTDR_SPEC> {
        TGTTDCNT_W::new(self, 0)
    }
    #[doc = "Bit 16 - preload of the TX-FIFO (when I3C is configured as target) This bit must be written and asserted by software in the same access when is written and defined the number of bytes to preload into the TX-FIFO and to transmit. This bit is cleared by hardware when all the data bytes to transmit are loaded into the TX-FIFO."]
    #[inline(always)]
    pub fn PRELOAD(&mut self) -> PRELOAD_W<'_, TGTTDR_SPEC> {
        PRELOAD_W::new(self, 16)
    }
}
#[doc = "I3C target transmit configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tgttdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tgttdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TGTTDR_SPEC;
impl crate::RegisterSpec for TGTTDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tgttdr::R`](R) reader structure"]
impl crate::Readable for TGTTDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tgttdr::W`](W) writer structure"]
impl crate::Writable for TGTTDR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TGTTDR to value 0"]
impl crate::Resettable for TGTTDR_SPEC {}
