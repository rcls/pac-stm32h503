#[doc = "Register `TXBC` reader"]
pub type R = crate::R<TXBC_SPEC>;
#[doc = "Register `TXBC` writer"]
pub type W = crate::W<TXBC_SPEC>;
#[doc = "Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFQM_A {
    #[doc = "0: Tx FIFO operation"]
    B_0x0 = 0,
    #[doc = "1: Tx queue operation."]
    B_0x1 = 1,
}
impl From<TFQM_A> for bool {
    #[inline(always)]
    fn from(variant: TFQM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFQM` reader - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type TFQM_R = crate::BitReader<TFQM_A>;
impl TFQM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFQM_A {
        match self.bits {
            false => TFQM_A::B_0x0,
            true => TFQM_A::B_0x1,
        }
    }
    #[doc = "Tx FIFO operation"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TFQM_A::B_0x0
    }
    #[doc = "Tx queue operation."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TFQM_A::B_0x1
    }
}
#[doc = "Field `TFQM` writer - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
pub type TFQM_W<'a, REG> = crate::BitWriter<'a, REG, TFQM_A>;
impl<'a, REG> TFQM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx FIFO operation"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TFQM_A::B_0x0)
    }
    #[doc = "Tx queue operation."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TFQM_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 24 - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn TFQM(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\] and bit 0 \\[INIT\\] of CCCR register are set to 1."]
    #[inline(always)]
    pub fn TFQM(&mut self) -> TFQM_W<'_, TXBC_SPEC> {
        TFQM_W::new(self, 24)
    }
}
#[doc = "FDCAN Tx buffer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBC_SPEC;
impl crate::RegisterSpec for TXBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbc::R`](R) reader structure"]
impl crate::Readable for TXBC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txbc::W`](W) writer structure"]
impl crate::Writable for TXBC_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TXBC to value 0"]
impl crate::Resettable for TXBC_SPEC {}
