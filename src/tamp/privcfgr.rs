#[doc = "Register `PRIVCFGR` reader"]
pub type R = crate::R<PRIVCFGR_SPEC>;
#[doc = "Register `PRIVCFGR` writer"]
pub type W = crate::W<PRIVCFGR_SPEC>;
#[doc = "Monotonic counter 1 privilege protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNT1PRIV_A {
    #[doc = "0: Monotonic counter 1 (TAMP_COUNT1R) can be read and written when the APB access is privileged or non-privileged."]
    B_0x0 = 0,
    #[doc = "1: Monotonic counter 1 (TAMP_COUNT1R) can be read and written only when the APB access is privileged."]
    B_0x1 = 1,
}
impl From<CNT1PRIV_A> for bool {
    #[inline(always)]
    fn from(variant: CNT1PRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNT1PRIV` reader - Monotonic counter 1 privilege protection"]
pub type CNT1PRIV_R = crate::BitReader<CNT1PRIV_A>;
impl CNT1PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNT1PRIV_A {
        match self.bits {
            false => CNT1PRIV_A::B_0x0,
            true => CNT1PRIV_A::B_0x1,
        }
    }
    #[doc = "Monotonic counter 1 (TAMP_COUNT1R) can be read and written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == CNT1PRIV_A::B_0x0
    }
    #[doc = "Monotonic counter 1 (TAMP_COUNT1R) can be read and written only when the APB access is privileged."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == CNT1PRIV_A::B_0x1
    }
}
#[doc = "Field `CNT1PRIV` writer - Monotonic counter 1 privilege protection"]
pub type CNT1PRIV_W<'a, REG> = crate::BitWriter<'a, REG, CNT1PRIV_A>;
impl<'a, REG> CNT1PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Monotonic counter 1 (TAMP_COUNT1R) can be read and written when the APB access is privileged or non-privileged."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CNT1PRIV_A::B_0x0)
    }
    #[doc = "Monotonic counter 1 (TAMP_COUNT1R) can be read and written only when the APB access is privileged."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CNT1PRIV_A::B_0x1)
    }
}
#[doc = "Backup registers zone 1 privilege protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKPRWPRIV_A {
    #[doc = "0: Backup registers zone 1 can be read and written with privileged or unprivileged access."]
    B_0x0 = 0,
    #[doc = "1: Backup registers zone 1 can be read and written only with privileged access"]
    B_0x1 = 1,
}
impl From<BKPRWPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: BKPRWPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKPRWPRIV` reader - Backup registers zone 1 privilege protection"]
pub type BKPRWPRIV_R = crate::BitReader<BKPRWPRIV_A>;
impl BKPRWPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKPRWPRIV_A {
        match self.bits {
            false => BKPRWPRIV_A::B_0x0,
            true => BKPRWPRIV_A::B_0x1,
        }
    }
    #[doc = "Backup registers zone 1 can be read and written with privileged or unprivileged access."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKPRWPRIV_A::B_0x0
    }
    #[doc = "Backup registers zone 1 can be read and written only with privileged access"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKPRWPRIV_A::B_0x1
    }
}
#[doc = "Field `BKPRWPRIV` writer - Backup registers zone 1 privilege protection"]
pub type BKPRWPRIV_W<'a, REG> = crate::BitWriter<'a, REG, BKPRWPRIV_A>;
impl<'a, REG> BKPRWPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Backup registers zone 1 can be read and written with privileged or unprivileged access."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKPRWPRIV_A::B_0x0)
    }
    #[doc = "Backup registers zone 1 can be read and written only with privileged access"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKPRWPRIV_A::B_0x1)
    }
}
#[doc = "Backup registers zone 2 privilege protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKPWPRIV_A {
    #[doc = "0: Backup registers zone 2 can be written with privileged or unprivileged access."]
    B_0x0 = 0,
    #[doc = "1: Backup registers zone 2 can be written only with privileged access."]
    B_0x1 = 1,
}
impl From<BKPWPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: BKPWPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKPWPRIV` reader - Backup registers zone 2 privilege protection"]
pub type BKPWPRIV_R = crate::BitReader<BKPWPRIV_A>;
impl BKPWPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKPWPRIV_A {
        match self.bits {
            false => BKPWPRIV_A::B_0x0,
            true => BKPWPRIV_A::B_0x1,
        }
    }
    #[doc = "Backup registers zone 2 can be written with privileged or unprivileged access."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == BKPWPRIV_A::B_0x0
    }
    #[doc = "Backup registers zone 2 can be written only with privileged access."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == BKPWPRIV_A::B_0x1
    }
}
#[doc = "Field `BKPWPRIV` writer - Backup registers zone 2 privilege protection"]
pub type BKPWPRIV_W<'a, REG> = crate::BitWriter<'a, REG, BKPWPRIV_A>;
impl<'a, REG> BKPWPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Backup registers zone 2 can be written with privileged or unprivileged access."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BKPWPRIV_A::B_0x0)
    }
    #[doc = "Backup registers zone 2 can be written only with privileged access."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BKPWPRIV_A::B_0x1)
    }
}
#[doc = "Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPPRIV_A {
    #[doc = "0: Tamper configuration and interrupt can be written with privileged or unprivileged access."]
    B_0x0 = 0,
    #[doc = "1: Tamper configuration and interrupt can be written only with privileged access."]
    B_0x1 = 1,
}
impl From<TAMPPRIV_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPPRIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPPRIV` reader - Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection."]
pub type TAMPPRIV_R = crate::BitReader<TAMPPRIV_A>;
impl TAMPPRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPPRIV_A {
        match self.bits {
            false => TAMPPRIV_A::B_0x0,
            true => TAMPPRIV_A::B_0x1,
        }
    }
    #[doc = "Tamper configuration and interrupt can be written with privileged or unprivileged access."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMPPRIV_A::B_0x0
    }
    #[doc = "Tamper configuration and interrupt can be written only with privileged access."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMPPRIV_A::B_0x1
    }
}
#[doc = "Field `TAMPPRIV` writer - Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection."]
pub type TAMPPRIV_W<'a, REG> = crate::BitWriter<'a, REG, TAMPPRIV_A>;
impl<'a, REG> TAMPPRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper configuration and interrupt can be written with privileged or unprivileged access."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRIV_A::B_0x0)
    }
    #[doc = "Tamper configuration and interrupt can be written only with privileged access."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRIV_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 15 - Monotonic counter 1 privilege protection"]
    #[inline(always)]
    pub fn CNT1PRIV(&self) -> CNT1PRIV_R {
        CNT1PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 29 - Backup registers zone 1 privilege protection"]
    #[inline(always)]
    pub fn BKPRWPRIV(&self) -> BKPRWPRIV_R {
        BKPRWPRIV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Backup registers zone 2 privilege protection"]
    #[inline(always)]
    pub fn BKPWPRIV(&self) -> BKPWPRIV_R {
        BKPWPRIV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection."]
    #[inline(always)]
    pub fn TAMPPRIV(&self) -> TAMPPRIV_R {
        TAMPPRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Monotonic counter 1 privilege protection"]
    #[inline(always)]
    pub fn CNT1PRIV(&mut self) -> CNT1PRIV_W<'_, PRIVCFGR_SPEC> {
        CNT1PRIV_W::new(self, 15)
    }
    #[doc = "Bit 29 - Backup registers zone 1 privilege protection"]
    #[inline(always)]
    pub fn BKPRWPRIV(&mut self) -> BKPRWPRIV_W<'_, PRIVCFGR_SPEC> {
        BKPRWPRIV_W::new(self, 29)
    }
    #[doc = "Bit 30 - Backup registers zone 2 privilege protection"]
    #[inline(always)]
    pub fn BKPWPRIV(&mut self) -> BKPWPRIV_W<'_, PRIVCFGR_SPEC> {
        BKPWPRIV_W::new(self, 30)
    }
    #[doc = "Bit 31 - Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection."]
    #[inline(always)]
    pub fn TAMPPRIV(&mut self) -> TAMPPRIV_W<'_, PRIVCFGR_SPEC> {
        TAMPPRIV_W::new(self, 31)
    }
}
#[doc = "TAMP privilege configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`privcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIVCFGR_SPEC;
impl crate::RegisterSpec for PRIVCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`privcfgr::R`](R) reader structure"]
impl crate::Readable for PRIVCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`privcfgr::W`](W) writer structure"]
impl crate::Writable for PRIVCFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PRIVCFGR to value 0"]
impl crate::Resettable for PRIVCFGR_SPEC {}
