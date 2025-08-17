#[doc = "Register `EXTICR4` reader"]
pub type R = crate::R<EXTICR4_SPEC>;
#[doc = "Register `EXTICR4` writer"]
pub type W = crate::W<EXTICR4_SPEC>;
#[doc = "EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. When EXTI_PRIVCFGR.PRIV12 is disabled, EXTI12 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV12 is enabled, EXTI12 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI12_A {
    #[doc = "0: PA\\[12\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[12\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[12\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI12_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI12_A {}
#[doc = "Field `EXTI12` reader - EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. When EXTI_PRIVCFGR.PRIV12 is disabled, EXTI12 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV12 is enabled, EXTI12 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI12_R = crate::FieldReader<EXTI12_A>;
impl EXTI12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI12_A> {
        match self.bits {
            0 => Some(EXTI12_A::B_0x0),
            1 => Some(EXTI12_A::B_0x1),
            2 => Some(EXTI12_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[12\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI12_A::B_0x0
    }
    #[doc = "PB\\[12\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI12_A::B_0x1
    }
    #[doc = "PC\\[12\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI12_A::B_0x2
    }
}
#[doc = "Field `EXTI12` writer - EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. When EXTI_PRIVCFGR.PRIV12 is disabled, EXTI12 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV12 is enabled, EXTI12 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI12_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI12_A>;
impl<'a, REG> EXTI12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[12\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12_A::B_0x0)
    }
    #[doc = "PB\\[12\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12_A::B_0x1)
    }
    #[doc = "PC\\[12\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI12_A::B_0x2)
    }
}
#[doc = "EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. When EXTI_PRIVCFGR.PRIV13 is disabled, EXTI13 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV13 is enabled, EXTI13 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI13_A {
    #[doc = "0: PA\\[13\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[13\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[13\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI13_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI13_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI13_A {}
#[doc = "Field `EXTI13` reader - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. When EXTI_PRIVCFGR.PRIV13 is disabled, EXTI13 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV13 is enabled, EXTI13 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI13_R = crate::FieldReader<EXTI13_A>;
impl EXTI13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI13_A> {
        match self.bits {
            0 => Some(EXTI13_A::B_0x0),
            1 => Some(EXTI13_A::B_0x1),
            2 => Some(EXTI13_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[13\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI13_A::B_0x0
    }
    #[doc = "PB\\[13\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI13_A::B_0x1
    }
    #[doc = "PC\\[13\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI13_A::B_0x2
    }
}
#[doc = "Field `EXTI13` writer - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. When EXTI_PRIVCFGR.PRIV13 is disabled, EXTI13 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV13 is enabled, EXTI13 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI13_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI13_A>;
impl<'a, REG> EXTI13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[13\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13_A::B_0x0)
    }
    #[doc = "PB\\[13\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13_A::B_0x1)
    }
    #[doc = "PC\\[13\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI13_A::B_0x2)
    }
}
#[doc = "EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. When EXTI_PRIVCFGR.PRIV14 is disabled, EXTI14 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV14 is enabled, EXTI14 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI14_A {
    #[doc = "0: PA\\[14\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[14\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[14\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI14_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI14_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI14_A {}
#[doc = "Field `EXTI14` reader - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. When EXTI_PRIVCFGR.PRIV14 is disabled, EXTI14 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV14 is enabled, EXTI14 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI14_R = crate::FieldReader<EXTI14_A>;
impl EXTI14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI14_A> {
        match self.bits {
            0 => Some(EXTI14_A::B_0x0),
            1 => Some(EXTI14_A::B_0x1),
            2 => Some(EXTI14_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[14\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI14_A::B_0x0
    }
    #[doc = "PB\\[14\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI14_A::B_0x1
    }
    #[doc = "PC\\[14\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI14_A::B_0x2
    }
}
#[doc = "Field `EXTI14` writer - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. When EXTI_PRIVCFGR.PRIV14 is disabled, EXTI14 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV14 is enabled, EXTI14 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI14_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI14_A>;
impl<'a, REG> EXTI14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[14\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14_A::B_0x0)
    }
    #[doc = "PB\\[14\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14_A::B_0x1)
    }
    #[doc = "PC\\[14\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI14_A::B_0x2)
    }
}
#[doc = "EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. When EXTI_PRIVCFGR.PRIV15 is disabled, EXTI15 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV15 is enabled, EXTI15 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI15_A {
    #[doc = "0: PA\\[15\\] pin"]
    B_0x0 = 0,
    #[doc = "1: PB\\[15\\] pin"]
    B_0x1 = 1,
    #[doc = "2: PC\\[15\\] pin"]
    B_0x2 = 2,
}
impl From<EXTI15_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI15_A {
    type Ux = u8;
}
impl crate::IsEnum for EXTI15_A {}
#[doc = "Field `EXTI15` reader - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. When EXTI_PRIVCFGR.PRIV15 is disabled, EXTI15 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV15 is enabled, EXTI15 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI15_R = crate::FieldReader<EXTI15_A>;
impl EXTI15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI15_A> {
        match self.bits {
            0 => Some(EXTI15_A::B_0x0),
            1 => Some(EXTI15_A::B_0x1),
            2 => Some(EXTI15_A::B_0x2),
            _ => None,
        }
    }
    #[doc = "PA\\[15\\] pin"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == EXTI15_A::B_0x0
    }
    #[doc = "PB\\[15\\] pin"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == EXTI15_A::B_0x1
    }
    #[doc = "PC\\[15\\] pin"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == EXTI15_A::B_0x2
    }
}
#[doc = "Field `EXTI15` writer - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. When EXTI_PRIVCFGR.PRIV15 is disabled, EXTI15 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV15 is enabled, EXTI15 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
pub type EXTI15_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI15_A>;
impl<'a, REG> EXTI15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA\\[15\\] pin"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15_A::B_0x0)
    }
    #[doc = "PB\\[15\\] pin"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15_A::B_0x1)
    }
    #[doc = "PC\\[15\\] pin"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI15_A::B_0x2)
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. When EXTI_PRIVCFGR.PRIV12 is disabled, EXTI12 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV12 is enabled, EXTI12 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. When EXTI_PRIVCFGR.PRIV13 is disabled, EXTI13 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV13 is enabled, EXTI13 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. When EXTI_PRIVCFGR.PRIV14 is disabled, EXTI14 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV14 is enabled, EXTI14 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. When EXTI_PRIVCFGR.PRIV15 is disabled, EXTI15 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV15 is enabled, EXTI15 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. When EXTI_PRIVCFGR.PRIV12 is disabled, EXTI12 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV12 is enabled, EXTI12 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI12(&mut self) -> EXTI12_W<'_, EXTICR4_SPEC> {
        EXTI12_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - EXTI13 GPIO port selection These bits are written by software to select the source input for EXTI13 external interrupt. When EXTI_PRIVCFGR.PRIV13 is disabled, EXTI13 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV13 is enabled, EXTI13 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI13(&mut self) -> EXTI13_W<'_, EXTICR4_SPEC> {
        EXTI13_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - EXTI14 GPIO port selection These bits are written by software to select the source input for EXTI14 external interrupt. When EXTI_PRIVCFGR.PRIV14 is disabled, EXTI14 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV14 is enabled, EXTI14 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI14(&mut self) -> EXTI14_W<'_, EXTICR4_SPEC> {
        EXTI14_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - EXTI15 GPIO port selection These bits are written by software to select the source input for EXTI15 external interrupt. When EXTI_PRIVCFGR.PRIV15 is disabled, EXTI15 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV15 is enabled, EXTI15 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
    #[inline(always)]
    pub fn EXTI15(&mut self) -> EXTI15_W<'_, EXTICR4_SPEC> {
        EXTI15_W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr4::R`](R) reader structure"]
impl crate::Readable for EXTICR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exticr4::W`](W) writer structure"]
impl crate::Writable for EXTICR4_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets EXTICR4 to value 0"]
impl crate::Resettable for EXTICR4_SPEC {}
