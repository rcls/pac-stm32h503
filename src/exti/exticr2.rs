#[doc = "Register `EXTICR2` reader"]
pub type R = crate::R<EXTICR2_SPEC>;
#[doc = "Register `EXTICR2` writer"]
pub type W = crate::W<EXTICR2_SPEC>;
#[doc = "EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_PRIVCFGR.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI4_A {
    #[doc = "0: PA\\[4\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[4\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[4\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI4_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI4_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI4_A {}
#[doc = "Field `EXTI4` reader - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_PRIVCFGR.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI4_R = crate::FieldReader<EXTI4_A>;
impl EXTI4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI4_A> {
        match self.bits {
            0 => Some(EXTI4_A::B_0x0),
            1 => Some(EXTI4_A::B_0x1),
            2 => Some(EXTI4_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[4\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI4_A::B_0x0
    }
    #[doc = "PB\\[4\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI4_A::B_0x1
    }
    #[doc = "PC\\[4\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI4_A::B_0x2
    }
}
#[doc = "Field `EXTI4` writer - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_PRIVCFGR.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI4_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI4_A>;
impl<'a, REG> EXTI4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[4\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4_A::B_0x0)
    }
    #[doc = "PB\\[4\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4_A::B_0x1)
    }
    #[doc = "PC\\[4\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI4_A::B_0x2)
    }
}
#[doc = "EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_PRIVCFGR.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI5_A {
    #[doc = "0: PA\\[5\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[5\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[5\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI5_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI5_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI5_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI5_A {}
#[doc = "Field `EXTI5` reader - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_PRIVCFGR.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI5_R = crate::FieldReader<EXTI5_A>;
impl EXTI5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI5_A> {
        match self.bits {
            0 => Some(EXTI5_A::B_0x0),
            1 => Some(EXTI5_A::B_0x1),
            2 => Some(EXTI5_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[5\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI5_A::B_0x0
    }
    #[doc = "PB\\[5\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI5_A::B_0x1
    }
    #[doc = "PC\\[5\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI5_A::B_0x2
    }
}
#[doc = "Field `EXTI5` writer - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_PRIVCFGR.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI5_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI5_A>;
impl<'a, REG> EXTI5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[5\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5_A::B_0x0)
    }
    #[doc = "PB\\[5\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5_A::B_0x1)
    }
    #[doc = "PC\\[5\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI5_A::B_0x2)
    }
}
#[doc = "EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_PRIVCFGR.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI6_A {
    #[doc = "0: PA\\[6\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[6\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[6\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI6_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI6_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI6_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI6_A {}
#[doc = "Field `EXTI6` reader - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_PRIVCFGR.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI6_R = crate::FieldReader<EXTI6_A>;
impl EXTI6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI6_A> {
        match self.bits {
            0 => Some(EXTI6_A::B_0x0),
            1 => Some(EXTI6_A::B_0x1),
            2 => Some(EXTI6_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[6\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI6_A::B_0x0
    }
    #[doc = "PB\\[6\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI6_A::B_0x1
    }
    #[doc = "PC\\[6\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI6_A::B_0x2
    }
}
#[doc = "Field `EXTI6` writer - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_PRIVCFGR.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI6_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI6_A>;
impl<'a, REG> EXTI6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[6\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6_A::B_0x0)
    }
    #[doc = "PB\\[6\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6_A::B_0x1)
    }
    #[doc = "PC\\[6\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI6_A::B_0x2)
    }
}
#[doc = "EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_PRIVCFGR.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI7_A {
    #[doc = "0: PA\\[7\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[7\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[7\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI7_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI7_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI7_A {}
#[doc = "Field `EXTI7` reader - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_PRIVCFGR.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI7_R = crate::FieldReader<EXTI7_A>;
impl EXTI7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI7_A> {
        match self.bits {
            0 => Some(EXTI7_A::B_0x0),
            1 => Some(EXTI7_A::B_0x1),
            2 => Some(EXTI7_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[7\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI7_A::B_0x0
    }
    #[doc = "PB\\[7\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI7_A::B_0x1
    }
    #[doc = "PC\\[7\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI7_A::B_0x2
    }
}
#[doc = "Field `EXTI7` writer - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_PRIVCFGR.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI7_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI7_A>;
impl<'a, REG> EXTI7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[7\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7_A::B_0x0)
    }
    #[doc = "PB\\[7\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7_A::B_0x1)
    }
    #[doc = "PC\\[7\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI7_A::B_0x2)
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_PRIVCFGR.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_PRIVCFGR.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_PRIVCFGR.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_PRIVCFGR.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI4 GPIO port selection These bits are written by software to select the source input for EXTI4 external interrupt. When EXTI_PRIVCFGR.PRIV4 is disabled, EXTI4 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV4 is enabled, EXTI4 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI4(&mut self) -> EXTI4_W<'_, EXTICR2_SPEC> {
        EXTI4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTI5 GPIO port selection These bits are written by software to select the source input for EXTI5 external interrupt. When EXTI_PRIVCFGR.PRIV5 is disabled, EXTI5 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV5 is enabled, EXTI5 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI5(&mut self) -> EXTI5_W<'_, EXTICR2_SPEC> {
        EXTI5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTI6 GPIO port selection These bits are written by software to select the source input for EXTI6 external interrupt. When EXTI_PRIVCFGR.PRIV6 is disabled, EXTI6 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV6 is enabled, EXTI6 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI6(&mut self) -> EXTI6_W<'_, EXTICR2_SPEC> {
        EXTI6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTI7 GPIO port selection These bits are written by software to select the source input for EXTI7 external interrupt. When EXTI_PRIVCFGR.PRIV7 is disabled, EXTI7 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV7 is enabled, EXTI7 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI7(&mut self) -> EXTI7_W<'_, EXTICR2_SPEC> {
        EXTI7_W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR2_SPEC;
impl crate::RegisterSpec for EXTICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr2::R`](R) reader structure"]
impl crate::Readable for EXTICR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exticr2::W`](W) writer structure"]
impl crate::Writable for EXTICR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2_SPEC {}
