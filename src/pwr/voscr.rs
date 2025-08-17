#[doc = "Register `VOSCR` reader"]
pub type R = crate::R<VOSCR_SPEC>;
#[doc = "Register `VOSCR` writer"]
pub type W = crate::W<VOSCR_SPEC>;
#[doc = "voltage scaling selection according to performance These bits control the V CORE voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS_A {
    #[doc = "0: scale 3 (default)"]
    B_0x0 = 0,
    #[doc = "1: scale 2"]
    B_0x1 = 1,
    #[doc = "2: scale 1"]
    B_0x2 = 2,
    #[doc = "3: scale 0"]
    B_0x3 = 3,
}
impl From<VOS_A> for u8 {
    #[inline(always)]
    fn from(variant: VOS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VOS_A {
    type Ux = u8;
}
impl crate::IsEnum for VOS_A {}
#[doc = "Field `VOS` reader - voltage scaling selection according to performance These bits control the V CORE voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
pub type VOS_R = crate::FieldReader<VOS_A>;
impl VOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VOS_A {
        match self.bits {
            0 => VOS_A::B_0x0,
            1 => VOS_A::B_0x1,
            2 => VOS_A::B_0x2,
            3 => VOS_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "scale 3 (default)"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == VOS_A::B_0x0
    }
    #[doc = "scale 2"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == VOS_A::B_0x1
    }
    #[doc = "scale 1"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == VOS_A::B_0x2
    }
    #[doc = "scale 0"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == VOS_A::B_0x3
    }
}
#[doc = "Field `VOS` writer - voltage scaling selection according to performance These bits control the V CORE voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, VOS_A, crate::Safe>;
impl<'a, REG> VOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "scale 3 (default)"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VOS_A::B_0x0)
    }
    #[doc = "scale 2"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VOS_A::B_0x1)
    }
    #[doc = "scale 1"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(VOS_A::B_0x2)
    }
    #[doc = "scale 0"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(VOS_A::B_0x3)
    }
}
impl R {
    #[doc = "Bits 4:5 - voltage scaling selection according to performance These bits control the V CORE voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
    #[inline(always)]
    pub fn VOS(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - voltage scaling selection according to performance These bits control the V CORE voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
    #[inline(always)]
    pub fn VOS(&mut self) -> VOS_W<'_, VOSCR_SPEC> {
        VOS_W::new(self, 4)
    }
}
#[doc = "PWR voltage scaling control register\n\nYou can [`read`](crate::Reg::read) this register and get [`voscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`voscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VOSCR_SPEC;
impl crate::RegisterSpec for VOSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`voscr::R`](R) reader structure"]
impl crate::Readable for VOSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`voscr::W`](W) writer structure"]
impl crate::Writable for VOSCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets VOSCR to value 0"]
impl crate::Resettable for VOSCR_SPEC {}
