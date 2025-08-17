#[doc = "Register `TR` reader"]
pub type R = crate::R<TR_SPEC>;
#[doc = "Register `TR` writer"]
pub type W = crate::W<TR_SPEC>;
#[doc = "Field `SU` reader - Second units in BCD format"]
pub type SU_R = crate::FieldReader;
#[doc = "Field `SU` writer - Second units in BCD format"]
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ST` reader - Second tens in BCD format"]
pub type ST_R = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens in BCD format"]
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub type MNU_R = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD format"]
pub type MNU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub type MNT_R = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD format"]
pub type MNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HU` reader - Hour units in BCD format"]
pub type HU_R = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units in BCD format"]
pub type HU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HT` reader - Hour tens in BCD format"]
pub type HT_R = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens in BCD format"]
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "AM/PM notation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM_A {
    #[doc = "0: AM or 24-hour format"]
    B_0x0 = 0,
    #[doc = "1: PM"]
    B_0x1 = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PM_R = crate::BitReader<PM_A>;
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::B_0x0,
            true => PM_A::B_0x1,
        }
    }
    #[doc = "AM or 24-hour format"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PM_A::B_0x0
    }
    #[doc = "PM"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PM_A::B_0x1
    }
}
#[doc = "Field `PM` writer - AM/PM notation"]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG, PM_A>;
impl<'a, REG> PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AM or 24-hour format"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::B_0x0)
    }
    #[doc = "PM"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::B_0x1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn SU(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn ST(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn MNU(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn MNT(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn HU(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn HT(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn PM(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn SU(&mut self) -> SU_W<'_, TR_SPEC> {
        SU_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn ST(&mut self) -> ST_W<'_, TR_SPEC> {
        ST_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn MNU(&mut self) -> MNU_W<'_, TR_SPEC> {
        MNU_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn MNT(&mut self) -> MNT_W<'_, TR_SPEC> {
        MNT_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn HU(&mut self) -> HU_W<'_, TR_SPEC> {
        HU_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn HT(&mut self) -> HT_W<'_, TR_SPEC> {
        HT_W::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn PM(&mut self) -> PM_W<'_, TR_SPEC> {
        PM_W::new(self, 22)
    }
}
#[doc = "RTC time register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TR_SPEC;
impl crate::RegisterSpec for TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr::R`](R) reader structure"]
impl crate::Readable for TR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tr::W`](W) writer structure"]
impl crate::Writable for TR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TR_SPEC {}
