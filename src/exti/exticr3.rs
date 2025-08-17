#[doc = "Register `EXTICR3` reader"]
pub type R = crate::R<EXTICR3_SPEC>;
#[doc = "Register `EXTICR3` writer"]
pub type W = crate::W<EXTICR3_SPEC>;
#[doc = "EXTI8 GPIO port selection These bits are written by software to select the source input for EXTIm external interrupt. When EXTI_PRIVCFGR.PRIV8 is disabled, EXTI8 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV8 is enabled, EXTI8 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI8_A {
    #[doc = "0: PA\\[8\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[8\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[8\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI8_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI8_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI8_A {}
#[doc = "Field `EXTI8` reader - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTIm external interrupt. When EXTI_PRIVCFGR.PRIV8 is disabled, EXTI8 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV8 is enabled, EXTI8 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI8_R = crate::FieldReader<EXTI8_A>;
impl EXTI8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI8_A> {
        match self.bits {
            0 => Some(EXTI8_A::B_0x0),
            1 => Some(EXTI8_A::B_0x1),
            2 => Some(EXTI8_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[8\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI8_A::B_0x0
    }
    #[doc = "PB\\[8\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI8_A::B_0x1
    }
    #[doc = "PC\\[8\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI8_A::B_0x2
    }
}
#[doc = "Field `EXTI8` writer - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTIm external interrupt. When EXTI_PRIVCFGR.PRIV8 is disabled, EXTI8 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV8 is enabled, EXTI8 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI8_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI8_A>;
impl<'a, REG> EXTI8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[8\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8_A::B_0x0)
    }
    #[doc = "PB\\[8\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8_A::B_0x1)
    }
    #[doc = "PC\\[8\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI8_A::B_0x2)
    }
}
#[doc = "EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. When EXTI_PRIVCFGR.PRIV9 is disabled, EXTI9 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV9 is enabled, EXTI9 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI9_A {
    #[doc = "0: PA\\[9\\] pin"]
    B_0x0 = 0,
    #[doc = "2: PC\\[9\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI9_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI9_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI9_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI9_A {}
#[doc = "Field `EXTI9` reader - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. When EXTI_PRIVCFGR.PRIV9 is disabled, EXTI9 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV9 is enabled, EXTI9 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI9_R = crate::FieldReader<EXTI9_A>;
impl EXTI9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI9_A> {
        match self.bits {
            0 => Some(EXTI9_A::B_0x0),
            2 => Some(EXTI9_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[9\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI9_A::B_0x0
    }
    #[doc = "PC\\[9\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI9_A::B_0x2
    }
}
#[doc = "Field `EXTI9` writer - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. When EXTI_PRIVCFGR.PRIV9 is disabled, EXTI9 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV9 is enabled, EXTI9 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI9_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI9_A>;
impl<'a, REG> EXTI9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[9\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI9_A::B_0x0)
    }
    #[doc = "PC\\[9\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI9_A::B_0x2)
    }
}
#[doc = "EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. When EXTI_PRIVCFGR.PRIV10 is disabled, EXTI10 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV10 is enabled, EXTI10 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI10_A {
    #[doc = "0: PA\\[10\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[10\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[10\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI10_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI10_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI10_A {}
#[doc = "Field `EXTI10` reader - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. When EXTI_PRIVCFGR.PRIV10 is disabled, EXTI10 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV10 is enabled, EXTI10 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI10_R = crate::FieldReader<EXTI10_A>;
impl EXTI10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI10_A> {
        match self.bits {
            0 => Some(EXTI10_A::B_0x0),
            1 => Some(EXTI10_A::B_0x1),
            2 => Some(EXTI10_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[10\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI10_A::B_0x0
    }
    #[doc = "PB\\[10\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI10_A::B_0x1
    }
    #[doc = "PC\\[10\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI10_A::B_0x2
    }
}
#[doc = "Field `EXTI10` writer - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. When EXTI_PRIVCFGR.PRIV10 is disabled, EXTI10 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV10 is enabled, EXTI10 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI10_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI10_A>;
impl<'a, REG> EXTI10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[10\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10_A::B_0x0)
    }
    #[doc = "PB\\[10\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10_A::B_0x1)
    }
    #[doc = "PC\\[10\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI10_A::B_0x2)
    }
}
#[doc = "EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. When EXTI_PRIVCFGR.PRIV11 is disabled, EXTI11 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV11 is enabled, EXTI11 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI11_A {
    #[doc = "0: PA\\[11\\] pin"]
    B_0x0 = 0,
    #[doc = "2: PC\\[11\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI11_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI11_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI11_A {}
#[doc = "Field `EXTI11` reader - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. When EXTI_PRIVCFGR.PRIV11 is disabled, EXTI11 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV11 is enabled, EXTI11 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI11_R = crate::FieldReader<EXTI11_A>;
impl EXTI11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI11_A> {
        match self.bits {
            0 => Some(EXTI11_A::B_0x0),
            2 => Some(EXTI11_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[11\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI11_A::B_0x0
    }
    #[doc = "PC\\[11\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI11_A::B_0x2
    }
}
#[doc = "Field `EXTI11` writer - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. When EXTI_PRIVCFGR.PRIV11 is disabled, EXTI11 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV11 is enabled, EXTI11 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI11_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI11_A>;
impl<'a, REG> EXTI11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[11\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI11_A::B_0x0)
    }
    #[doc = "PC\\[11\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI11_A::B_0x2)
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTIm external interrupt. When EXTI_PRIVCFGR.PRIV8 is disabled, EXTI8 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV8 is enabled, EXTI8 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. When EXTI_PRIVCFGR.PRIV9 is disabled, EXTI9 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV9 is enabled, EXTI9 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. When EXTI_PRIVCFGR.PRIV10 is disabled, EXTI10 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV10 is enabled, EXTI10 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. When EXTI_PRIVCFGR.PRIV11 is disabled, EXTI11 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV11 is enabled, EXTI11 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI8 GPIO port selection These bits are written by software to select the source input for EXTIm external interrupt. When EXTI_PRIVCFGR.PRIV8 is disabled, EXTI8 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV8 is enabled, EXTI8 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI8(&mut self) -> EXTI8_W<'_, EXTICR3_SPEC> {
        EXTI8_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTI9 GPIO port selection These bits are written by software to select the source input for EXTI9 external interrupt. When EXTI_PRIVCFGR.PRIV9 is disabled, EXTI9 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV9 is enabled, EXTI9 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI9(&mut self) -> EXTI9_W<'_, EXTICR3_SPEC> {
        EXTI9_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTI10 GPIO port selection These bits are written by software to select the source input for EXTI10 external interrupt. When EXTI_PRIVCFGR.PRIV10 is disabled, EXTI10 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV10 is enabled, EXTI10 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI10(&mut self) -> EXTI10_W<'_, EXTICR3_SPEC> {
        EXTI10_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTI11 GPIO port selection These bits are written by software to select the source input for EXTI11 external interrupt. When EXTI_PRIVCFGR.PRIV11 is disabled, EXTI11 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV11 is enabled, EXTI11 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI11(&mut self) -> EXTI11_W<'_, EXTICR3_SPEC> {
        EXTI11_W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR3_SPEC;
impl crate::RegisterSpec for EXTICR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr3::R`](R) reader structure"]
impl crate::Readable for EXTICR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exticr3::W`](W) writer structure"]
impl crate::Writable for EXTICR3_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EXTICR3 to value 0"]
impl crate::Resettable for EXTICR3_SPEC {}
