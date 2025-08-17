#[doc = "Register `IORETR` reader"]
pub type R = crate::R<IORETR_SPEC>;
#[doc = "Register `IORETR` writer"]
pub type W = crate::W<IORETR_SPEC>;
#[doc = "IO retention enable: When entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IORETEN_A {
    #[doc = "0: IO Retention mode is disabled."]
    B_0x0 = 0,
    #[doc = "1: IO Retention mode is enabled for all IOs except the ones supporting the standby functionality (PC13, PC14 and PC15) and JTAG I/Os (PA13, PA14, PA15, PB4)."]
    B_0x1 = 1,
}
impl From<IORETEN_A> for bool {
    #[inline(always)]
    fn from(variant: IORETEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IORETEN` reader - IO retention enable: When entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
pub type IORETEN_R = crate::BitReader<IORETEN_A>;
impl IORETEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IORETEN_A {
        match self.bits {
            false => IORETEN_A::B_0x0,
            true => IORETEN_A::B_0x1,
        }
    }
    #[doc = "IO Retention mode is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == IORETEN_A::B_0x0
    }
    #[doc = "IO Retention mode is enabled for all IOs except the ones supporting the standby functionality (PC13, PC14 and PC15) and JTAG I/Os (PA13, PA14, PA15, PB4)."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == IORETEN_A::B_0x1
    }
}
#[doc = "Field `IORETEN` writer - IO retention enable: When entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
pub type IORETEN_W<'a, REG> = crate::BitWriter<'a, REG, IORETEN_A>;
impl<'a, REG> IORETEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO Retention mode is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IORETEN_A::B_0x0)
    }
    #[doc = "IO Retention mode is enabled for all IOs except the ones supporting the standby functionality (PC13, PC14 and PC15) and JTAG I/Os (PA13, PA14, PA15, PB4)."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IORETEN_A::B_0x1)
    }
}
#[doc = "IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JTAGIORETEN_A {
    #[doc = "0: IO Retention mode is disabled."]
    B_0x0 = 0,
    #[doc = "1: IO Retention mode is enabled for PA13, PA14, PA15 and PB4."]
    B_0x1 = 1,
}
impl From<JTAGIORETEN_A> for bool {
    #[inline(always)]
    fn from(variant: JTAGIORETEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JTAGIORETEN` reader - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode"]
pub type JTAGIORETEN_R = crate::BitReader<JTAGIORETEN_A>;
impl JTAGIORETEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JTAGIORETEN_A {
        match self.bits {
            false => JTAGIORETEN_A::B_0x0,
            true => JTAGIORETEN_A::B_0x1,
        }
    }
    #[doc = "IO Retention mode is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == JTAGIORETEN_A::B_0x0
    }
    #[doc = "IO Retention mode is enabled for PA13, PA14, PA15 and PB4."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == JTAGIORETEN_A::B_0x1
    }
}
#[doc = "Field `JTAGIORETEN` writer - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode"]
pub type JTAGIORETEN_W<'a, REG> = crate::BitWriter<'a, REG, JTAGIORETEN_A>;
impl<'a, REG> JTAGIORETEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IO Retention mode is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(JTAGIORETEN_A::B_0x0)
    }
    #[doc = "IO Retention mode is enabled for PA13, PA14, PA15 and PB4."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(JTAGIORETEN_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - IO retention enable: When entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
    #[inline(always)]
    pub fn IORETEN(&self) -> IORETEN_R {
        IORETEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode"]
    #[inline(always)]
    pub fn JTAGIORETEN(&self) -> JTAGIORETEN_R {
        JTAGIORETEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO retention enable: When entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
    #[inline(always)]
    pub fn IORETEN(&mut self) -> IORETEN_W<'_, IORETR_SPEC> {
        IORETEN_W::new(self, 0)
    }
    #[doc = "Bit 16 - IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode"]
    #[inline(always)]
    pub fn JTAGIORETEN(&mut self) -> JTAGIORETEN_W<'_, IORETR_SPEC> {
        JTAGIORETEN_W::new(self, 16)
    }
}
#[doc = "PWR I/O retention register\n\nYou can [`read`](crate::Reg::read) this register and get [`ioretr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioretr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IORETR_SPEC;
impl crate::RegisterSpec for IORETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioretr::R`](R) reader structure"]
impl crate::Readable for IORETR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioretr::W`](W) writer structure"]
impl crate::Writable for IORETR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IORETR to value 0"]
impl crate::Resettable for IORETR_SPEC {}
