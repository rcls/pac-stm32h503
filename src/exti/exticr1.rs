#[doc = "Register `EXTICR1` reader"]
pub type R = crate::R<EXTICR1_SPEC>;
#[doc = "Register `EXTICR1` writer"]
pub type W = crate::W<EXTICR1_SPEC>;
#[doc = "EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_PRIVCFGR.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI0_A {
    #[doc = "0: PA\\[0\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[0\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[0\\] pin"]
    B_0x2 = 2,
    #[doc = "7: PH\\[0\\] pin"]
    B_0x7 = 7,
}
impl From<EXTI0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI0_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI0_A {}
#[doc = "Field `EXTI0` reader - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_PRIVCFGR.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI0_R = crate::FieldReader<EXTI0_A>;
impl EXTI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI0_A> {
        match self.bits {
            0 => Some(EXTI0_A::B_0x0),
            1 => Some(EXTI0_A::B_0x1),
            2 => Some(EXTI0_A::B_0x2),
            7 => Some(EXTI0_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "PA\\[0\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI0_A::B_0x0
    }
    #[doc = "PB\\[0\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI0_A::B_0x1
    }
    #[doc = "PC\\[0\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI0_A::B_0x2
    }
    #[doc = "PH\\[0\\] pin"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == EXTI0_A::B_0x7
    }
}
#[doc = "Field `EXTI0` writer - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_PRIVCFGR.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI0_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI0_A>;
impl<'a, REG> EXTI0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[0\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_A::B_0x0)
    }
    #[doc = "PB\\[0\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_A::B_0x1)
    }
    #[doc = "PC\\[0\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_A::B_0x2)
    }
    #[doc = "PH\\[0\\] pin"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_A::B_0x7)
    }
}
#[doc = "EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_PRIVCFGR.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI1_A {
    #[doc = "0: PA\\[1\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[1\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[1\\] pin"]
    B_0x2 = 2,
    #[doc = "7: PH\\[1\\] pin"]
    B_0x7 = 7,
}
impl From<EXTI1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI1_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI1_A {}
#[doc = "Field `EXTI1` reader - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_PRIVCFGR.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI1_R = crate::FieldReader<EXTI1_A>;
impl EXTI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI1_A> {
        match self.bits {
            0 => Some(EXTI1_A::B_0x0),
            1 => Some(EXTI1_A::B_0x1),
            2 => Some(EXTI1_A::B_0x2),
            7 => Some(EXTI1_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "PA\\[1\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI1_A::B_0x0
    }
    #[doc = "PB\\[1\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI1_A::B_0x1
    }
    #[doc = "PC\\[1\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI1_A::B_0x2
    }
    #[doc = "PH\\[1\\] pin"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == EXTI1_A::B_0x7
    }
}
#[doc = "Field `EXTI1` writer - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_PRIVCFGR.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI1_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI1_A>;
impl<'a, REG> EXTI1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[1\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1_A::B_0x0)
    }
    #[doc = "PB\\[1\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1_A::B_0x1)
    }
    #[doc = "PC\\[1\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1_A::B_0x2)
    }
    #[doc = "PH\\[1\\] pin"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI1_A::B_0x7)
    }
}
#[doc = "EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_PRIVCFGR.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI2_A {
    #[doc = "0: PA\\[2\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[2\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[2\\] pin"]
    B_0x2 = 2,
    #[doc = "3: PD\\[2\\] pin"]
    B_0x3 = 3,
}
impl From<EXTI2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI2_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI2_A {}
#[doc = "Field `EXTI2` reader - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_PRIVCFGR.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI2_R = crate::FieldReader<EXTI2_A>;
impl EXTI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI2_A> {
        match self.bits {
            0 => Some(EXTI2_A::B_0x0),
            1 => Some(EXTI2_A::B_0x1),
            2 => Some(EXTI2_A::B_0x2),
            3 => Some(EXTI2_A::B_0x3),
            _ => None,
        }
    }
    #[doc = "PA\\[2\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI2_A::B_0x0
    }
    #[doc = "PB\\[2\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI2_A::B_0x1
    }
    #[doc = "PC\\[2\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI2_A::B_0x2
    }
    #[doc = "PD\\[2\\] pin"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == EXTI2_A::B_0x3
    }
}
#[doc = "Field `EXTI2` writer - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_PRIVCFGR.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI2_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI2_A>;
impl<'a, REG> EXTI2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[2\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2_A::B_0x0)
    }
    #[doc = "PB\\[2\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2_A::B_0x1)
    }
    #[doc = "PC\\[2\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2_A::B_0x2)
    }
    #[doc = "PD\\[2\\] pin"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI2_A::B_0x3)
    }
}
#[doc = "EXTI3 GPIO port selectio These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_PRIVCFGR.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI3_A {
    #[doc = "0: PA\\[3\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[3\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[3\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI3_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI3_A {}
#[doc = "Field `EXTI3` reader - EXTI3 GPIO port selectio These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_PRIVCFGR.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI3_R = crate::FieldReader<EXTI3_A>;
impl EXTI3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI3_A> {
        match self.bits {
            0 => Some(EXTI3_A::B_0x0),
            1 => Some(EXTI3_A::B_0x1),
            2 => Some(EXTI3_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[3\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI3_A::B_0x0
    }
    #[doc = "PB\\[3\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI3_A::B_0x1
    }
    #[doc = "PC\\[3\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI3_A::B_0x2
    }
}
#[doc = "Field `EXTI3` writer - EXTI3 GPIO port selectio These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_PRIVCFGR.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI3_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI3_A>;
impl<'a, REG> EXTI3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[3\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3_A::B_0x0)
    }
    #[doc = "PB\\[3\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3_A::B_0x1)
    }
    #[doc = "PC\\[3\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI3_A::B_0x2)
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_PRIVCFGR.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_PRIVCFGR.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_PRIVCFGR.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI3 GPIO port selectio These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_PRIVCFGR.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI0 GPIO port selection These bits are written by software to select the source input for EXTI0 external interrupt. When EXTI_PRIVCFGR.PRIV0 is disabled, EXTI0 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV0 is enabled, EXTI0 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI0(&mut self) -> EXTI0_W<'_, EXTICR1_SPEC> {
        EXTI0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTI1 GPIO port selection These bits are written by software to select the source input for EXTI1 external interrupt. When EXTI_PRIVCFGR.PRIV1 is disabled, EXTI1 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV1 is enabled, EXTI1 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI1(&mut self) -> EXTI1_W<'_, EXTICR1_SPEC> {
        EXTI1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTI2 GPIO port selection These bits are written by software to select the source input for EXTI2 external interrupt. When EXTI_PRIVCFGR.PRIV2 is disabled, EXTI2 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV2 is enabled, EXTI2 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI2(&mut self) -> EXTI2_W<'_, EXTICR1_SPEC> {
        EXTI2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTI3 GPIO port selectio These bits are written by software to select the source input for EXTI3 external interrupt. When EXTI_PRIVCFGR.PRIV3 is disabled, EXTI3 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV3 is enabled, EXTI3 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI3(&mut self) -> EXTI3_W<'_, EXTICR1_SPEC> {
        EXTI3_W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR1_SPEC;
impl crate::RegisterSpec for EXTICR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr1::R`](R) reader structure"]
impl crate::Readable for EXTICR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exticr1::W`](W) writer structure"]
impl crate::Writable for EXTICR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EXTICR1 to value 0"]
impl crate::Resettable for EXTICR1_SPEC {}
