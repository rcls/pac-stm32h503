#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2_SPEC>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2_SPEC>;
#[doc = "core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLL_A {
    #[doc = "0: lockup output disconnected from timer break inputs"]
    B_0x0 = 0,
    #[doc = "1: lockup output connected to timer break inputs"]
    B_0x1 = 1,
}
impl From<CLL_A> for bool {
    #[inline(always)]
    fn from(variant: CLL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLL` reader - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs."]
pub type CLL_R = crate::BitReader<CLL_A>;
impl CLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLL_A {
        match self.bits {
            false => CLL_A::B_0x0,
            true => CLL_A::B_0x1,
        }
    }
    #[doc = "lockup output disconnected from timer break inputs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CLL_A::B_0x0
    }
    #[doc = "lockup output connected to timer break inputs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CLL_A::B_0x1
    }
}
#[doc = "Field `CLL` writer - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs."]
pub type CLL_W<'a, REG> = crate::BitWriter<'a, REG, CLL_A>;
impl<'a, REG> CLL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "lockup output disconnected from timer break inputs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CLL_A::B_0x0)
    }
    #[doc = "lockup output connected to timer break inputs"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CLL_A::B_0x1)
    }
}
#[doc = "SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL_A {
    #[doc = "0: SRAM double ECC error flag disconnected from timer break inputs"]
    B_0x0 = 0,
    #[doc = "1: SRAM double ECC error flag connected to timer break inputs"]
    B_0x1 = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1."]
pub type SEL_R = crate::BitReader<SEL_A>;
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::B_0x0,
            true => SEL_A::B_0x1,
        }
    }
    #[doc = "SRAM double ECC error flag disconnected from timer break inputs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SEL_A::B_0x0
    }
    #[doc = "SRAM double ECC error flag connected to timer break inputs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SEL_A::B_0x1
    }
}
#[doc = "Field `SEL` writer - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1."]
pub type SEL_W<'a, REG> = crate::BitWriter<'a, REG, SEL_A>;
impl<'a, REG> SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM double ECC error flag disconnected from timer break inputs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::B_0x0)
    }
    #[doc = "SRAM double ECC error flag connected to timer break inputs"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SEL_A::B_0x1)
    }
}
#[doc = "PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDL_A {
    #[doc = "0: PVD interrupt disconnected from timer break inputs. PVD_EN and PVD_SEL\\[2:0\\] in the PWR registers are read/write."]
    B_0x0 = 0,
    #[doc = "1: PVD interrupt is connected to timer break inputs. PVD_EN and PVD_SEL\\[2:0\\] in the PWR registers are read only"]
    B_0x1 = 1,
}
impl From<PVDL_A> for bool {
    #[inline(always)]
    fn from(variant: PVDL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDL` reader - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs."]
pub type PVDL_R = crate::BitReader<PVDL_A>;
impl PVDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVDL_A {
        match self.bits {
            false => PVDL_A::B_0x0,
            true => PVDL_A::B_0x1,
        }
    }
    #[doc = "PVD interrupt disconnected from timer break inputs. PVD_EN and PVD_SEL\\[2:0\\] in the PWR registers are read/write."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PVDL_A::B_0x0
    }
    #[doc = "PVD interrupt is connected to timer break inputs. PVD_EN and PVD_SEL\\[2:0\\] in the PWR registers are read only"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PVDL_A::B_0x1
    }
}
#[doc = "Field `PVDL` writer - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs."]
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG, PVDL_A>;
impl<'a, REG> PVDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVD interrupt disconnected from timer break inputs. PVD_EN and PVD_SEL\\[2:0\\] in the PWR registers are read/write."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PVDL_A::B_0x0)
    }
    #[doc = "PVD interrupt is connected to timer break inputs. PVD_EN and PVD_SEL\\[2:0\\] in the PWR registers are read only"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PVDL_A::B_0x1)
    }
}
#[doc = "ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCL_A {
    #[doc = "0: double ECC error flag disconnected to timer break inputs"]
    B_0x0 = 0,
    #[doc = "1: double ECC error flag connected to timer break inputs"]
    B_0x1 = 1,
}
impl From<ECCL_A> for bool {
    #[inline(always)]
    fn from(variant: ECCL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCL` reader - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1."]
pub type ECCL_R = crate::BitReader<ECCL_A>;
impl ECCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECCL_A {
        match self.bits {
            false => ECCL_A::B_0x0,
            true => ECCL_A::B_0x1,
        }
    }
    #[doc = "double ECC error flag disconnected to timer break inputs"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ECCL_A::B_0x0
    }
    #[doc = "double ECC error flag connected to timer break inputs"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ECCL_A::B_0x1
    }
}
#[doc = "Field `ECCL` writer - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1."]
pub type ECCL_W<'a, REG> = crate::BitWriter<'a, REG, ECCL_A>;
impl<'a, REG> ECCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "double ECC error flag disconnected to timer break inputs"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ECCL_A::B_0x0)
    }
    #[doc = "double ECC error flag connected to timer break inputs"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ECCL_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs."]
    #[inline(always)]
    pub fn CLL(&self) -> CLL_R {
        CLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1."]
    #[inline(always)]
    pub fn SEL(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs."]
    #[inline(always)]
    pub fn PVDL(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1."]
    #[inline(always)]
    pub fn ECCL(&self) -> ECCL_R {
        ECCL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - core lockup lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the lockup (HardFault) output of Cortex-M33 with TIM1 break inputs."]
    #[inline(always)]
    pub fn CLL(&mut self) -> CLL_W<'_, CFGR2_SPEC> {
        CLL_W::new(self, 0)
    }
    #[doc = "Bit 1 - SRAM ECC error lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the SRAM double ECC error signal with break input of TIM1."]
    #[inline(always)]
    pub fn SEL(&mut self) -> SEL_W<'_, CFGR2_SPEC> {
        SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - PVD lock This bit is set by software and cleared only by a system reset. It can be used to enable and lock the PVD connection with TIM1 break inputs."]
    #[inline(always)]
    pub fn PVDL(&mut self) -> PVDL_W<'_, CFGR2_SPEC> {
        PVDL_W::new(self, 2)
    }
    #[doc = "Bit 3 - ECC lock This bit is set and cleared by software. It can be used to enable and lock the Flash memory double ECC error with break input of TIM1."]
    #[inline(always)]
    pub fn ECCL(&mut self) -> ECCL_W<'_, CFGR2_SPEC> {
        ECCL_W::new(self, 3)
    }
}
#[doc = "SBS Class B register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {}
