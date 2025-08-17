#[doc = "Register `VMCR` reader"]
pub type R = crate::R<VMCR_SPEC>;
#[doc = "Register `VMCR` writer"]
pub type W = crate::W<VMCR_SPEC>;
#[doc = "PVD enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDE_A {
    #[doc = "0: PVD disabled"]
    B_0x0 = 0,
    #[doc = "1: PVD enabled"]
    B_0x1 = 1,
}
impl From<PVDE_A> for bool {
    #[inline(always)]
    fn from(variant: PVDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVDE` reader - PVD enable"]
pub type PVDE_R = crate::BitReader<PVDE_A>;
impl PVDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVDE_A {
        match self.bits {
            false => PVDE_A::B_0x0,
            true => PVDE_A::B_0x1,
        }
    }
    #[doc = "PVD disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PVDE_A::B_0x0
    }
    #[doc = "PVD enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PVDE_A::B_0x1
    }
}
#[doc = "Field `PVDE` writer - PVD enable"]
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG, PVDE_A>;
impl<'a, REG> PVDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PVD disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE_A::B_0x0)
    }
    #[doc = "PVD enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PVDE_A::B_0x1)
    }
}
#[doc = "programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLS_A {
    #[doc = "0: 1.95 V"]
    B_0x0 = 0,
    #[doc = "1: 2.1 V"]
    B_0x1 = 1,
    #[doc = "2: 2.25 V"]
    B_0x2 = 2,
    #[doc = "3: 2.4 V"]
    B_0x3 = 3,
    #[doc = "4: 2.55 V"]
    B_0x4 = 4,
    #[doc = "5: 2.7 V"]
    B_0x5 = 5,
    #[doc = "6: 2.85 V"]
    B_0x6 = 6,
    #[doc = "7: PVD_IN pin"]
    B_0x7 = 7,
}
impl From<PLS_A> for u8 {
    #[inline(always)]
    fn from(variant: PLS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLS_A {
    type Ux = u8;
}
impl crate::IsEnum for PLS_A {}
#[doc = "Field `PLS` reader - programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD."]
pub type PLS_R = crate::FieldReader<PLS_A>;
impl PLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLS_A {
        match self.bits {
            0 => PLS_A::B_0x0,
            1 => PLS_A::B_0x1,
            2 => PLS_A::B_0x2,
            3 => PLS_A::B_0x3,
            4 => PLS_A::B_0x4,
            5 => PLS_A::B_0x5,
            6 => PLS_A::B_0x6,
            7 => PLS_A::B_0x7,
            _ => unreachable!(),
        }
    }
    #[doc = "1.95 V"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PLS_A::B_0x0
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PLS_A::B_0x1
    }
    #[doc = "2.25 V"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PLS_A::B_0x2
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PLS_A::B_0x3
    }
    #[doc = "2.55 V"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == PLS_A::B_0x4
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == PLS_A::B_0x5
    }
    #[doc = "2.85 V"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == PLS_A::B_0x6
    }
    #[doc = "PVD_IN pin"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == PLS_A::B_0x7
    }
}
#[doc = "Field `PLS` writer - programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD."]
pub type PLS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PLS_A, crate::Safe>;
impl<'a, REG> PLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.95 V"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLS_A::B_0x0)
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLS_A::B_0x1)
    }
    #[doc = "2.25 V"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PLS_A::B_0x2)
    }
    #[doc = "2.4 V"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PLS_A::B_0x3)
    }
    #[doc = "2.55 V"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PLS_A::B_0x4)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PLS_A::B_0x5)
    }
    #[doc = "2.85 V"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PLS_A::B_0x6)
    }
    #[doc = "PVD_IN pin"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PLS_A::B_0x7)
    }
}
#[doc = "peripheral voltage monitor on V DDA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVDEN_A {
    #[doc = "0: peripheral voltage monitor on V DDA disabled"]
    B_0x0 = 0,
    #[doc = "1: peripheral voltage monitor on V DDA enabled"]
    B_0x1 = 1,
}
impl From<AVDEN_A> for bool {
    #[inline(always)]
    fn from(variant: AVDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVDEN` reader - peripheral voltage monitor on V DDA enable"]
pub type AVDEN_R = crate::BitReader<AVDEN_A>;
impl AVDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AVDEN_A {
        match self.bits {
            false => AVDEN_A::B_0x0,
            true => AVDEN_A::B_0x1,
        }
    }
    #[doc = "peripheral voltage monitor on V DDA disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == AVDEN_A::B_0x0
    }
    #[doc = "peripheral voltage monitor on V DDA enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == AVDEN_A::B_0x1
    }
}
#[doc = "Field `AVDEN` writer - peripheral voltage monitor on V DDA enable"]
pub type AVDEN_W<'a, REG> = crate::BitWriter<'a, REG, AVDEN_A>;
impl<'a, REG> AVDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "peripheral voltage monitor on V DDA disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AVDEN_A::B_0x0)
    }
    #[doc = "peripheral voltage monitor on V DDA enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AVDEN_A::B_0x1)
    }
}
#[doc = "analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALS_A {
    #[doc = "0: 1.7 V"]
    B_0x0 = 0,
    #[doc = "1: 2.1 V"]
    B_0x1 = 1,
    #[doc = "2: 2.5 V"]
    B_0x2 = 2,
    #[doc = "3: 2.8 V"]
    B_0x3 = 3,
}
impl From<ALS_A> for u8 {
    #[inline(always)]
    fn from(variant: ALS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ALS_A {
    type Ux = u8;
}
impl crate::IsEnum for ALS_A {}
#[doc = "Field `ALS` reader - analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD."]
pub type ALS_R = crate::FieldReader<ALS_A>;
impl ALS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALS_A {
        match self.bits {
            0 => ALS_A::B_0x0,
            1 => ALS_A::B_0x1,
            2 => ALS_A::B_0x2,
            3 => ALS_A::B_0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "1.7 V"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ALS_A::B_0x0
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ALS_A::B_0x1
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == ALS_A::B_0x2
    }
    #[doc = "2.8 V"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == ALS_A::B_0x3
    }
}
#[doc = "Field `ALS` writer - analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD."]
pub type ALS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ALS_A, crate::Safe>;
impl<'a, REG> ALS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.7 V"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALS_A::B_0x0)
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALS_A::B_0x1)
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ALS_A::B_0x2)
    }
    #[doc = "2.8 V"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ALS_A::B_0x3)
    }
}
impl R {
    #[doc = "Bit 0 - PVD enable"]
    #[inline(always)]
    pub fn PVDE(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD."]
    #[inline(always)]
    pub fn PLS(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 8 - peripheral voltage monitor on V DDA enable"]
    #[inline(always)]
    pub fn AVDEN(&self) -> AVDEN_R {
        AVDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD."]
    #[inline(always)]
    pub fn ALS(&self) -> ALS_R {
        ALS_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PVD enable"]
    #[inline(always)]
    pub fn PVDE(&mut self) -> PVDE_W<'_, VMCR_SPEC> {
        PVDE_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD."]
    #[inline(always)]
    pub fn PLS(&mut self) -> PLS_W<'_, VMCR_SPEC> {
        PLS_W::new(self, 1)
    }
    #[doc = "Bit 8 - peripheral voltage monitor on V DDA enable"]
    #[inline(always)]
    pub fn AVDEN(&mut self) -> AVDEN_W<'_, VMCR_SPEC> {
        AVDEN_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD."]
    #[inline(always)]
    pub fn ALS(&mut self) -> ALS_W<'_, VMCR_SPEC> {
        ALS_W::new(self, 9)
    }
}
#[doc = "PWR voltage monitor control register\n\nYou can [`read`](crate::Reg::read) this register and get [`vmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMCR_SPEC;
impl crate::RegisterSpec for VMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vmcr::R`](R) reader structure"]
impl crate::Readable for VMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vmcr::W`](W) writer structure"]
impl crate::Writable for VMCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets VMCR to value 0"]
impl crate::Resettable for VMCR_SPEC {}
