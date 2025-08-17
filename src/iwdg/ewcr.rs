#[doc = "Register `EWCR` reader"]
pub type R = crate::R<EWCR_SPEC>;
#[doc = "Register `EWCR` writer"]
pub type W = crate::W<EWCR_SPEC>;
#[doc = "Field `EWIT` reader - Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\] - 1. EWIT\\[11:0\\] must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset."]
pub type EWIT_R = crate::FieldReader<u16>;
#[doc = "Field `EWIT` writer - Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\] - 1. EWIT\\[11:0\\] must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset."]
pub type EWIT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `EWIC` writer - Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wakeup interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0."]
pub type EWIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIE_A {
    #[doc = "0: The early interrupt interface is disabled."]
    B_0x0 = 0,
    #[doc = "1: The early interrupt interface is enabled."]
    B_0x1 = 1,
}
impl From<EWIE_A> for bool {
    #[inline(always)]
    fn from(variant: EWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIE` reader - Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit."]
pub type EWIE_R = crate::BitReader<EWIE_A>;
impl EWIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EWIE_A {
        match self.bits {
            false => EWIE_A::B_0x0,
            true => EWIE_A::B_0x1,
        }
    }
    #[doc = "The early interrupt interface is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EWIE_A::B_0x0
    }
    #[doc = "The early interrupt interface is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EWIE_A::B_0x1
    }
}
#[doc = "Field `EWIE` writer - Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit."]
pub type EWIE_W<'a, REG> = crate::BitWriter<'a, REG, EWIE_A>;
impl<'a, REG> EWIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The early interrupt interface is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EWIE_A::B_0x0)
    }
    #[doc = "The early interrupt interface is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EWIE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:11 - Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\] - 1. EWIT\\[11:0\\] must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset."]
    #[inline(always)]
    pub fn EWIT(&self) -> EWIT_R {
        EWIT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit."]
    #[inline(always)]
    pub fn EWIE(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\\[11:0\\] - 1. EWIT\\[11:0\\] must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset."]
    #[inline(always)]
    pub fn EWIT(&mut self) -> EWIT_W<'_, EWCR_SPEC> {
        EWIT_W::new(self, 0)
    }
    #[doc = "Bit 14 - Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wakeup interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0."]
    #[inline(always)]
    pub fn EWIC(&mut self) -> EWIC_W<'_, EWCR_SPEC> {
        EWIC_W::new(self, 14)
    }
    #[doc = "Bit 15 - Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit."]
    #[inline(always)]
    pub fn EWIE(&mut self) -> EWIE_W<'_, EWCR_SPEC> {
        EWIE_W::new(self, 15)
    }
}
#[doc = "IWDG early wakeup interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`ewcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EWCR_SPEC;
impl crate::RegisterSpec for EWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ewcr::R`](R) reader structure"]
impl crate::Readable for EWCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ewcr::W`](W) writer structure"]
impl crate::Writable for EWCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EWCR to value 0"]
impl crate::Resettable for EWCR_SPEC {}
