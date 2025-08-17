#[doc = "Register `ALRMBR` reader"]
pub type R = crate::R<ALRMBR_SPEC>;
#[doc = "Register `ALRMBR` writer"]
pub type W = crate::W<ALRMBR_SPEC>;
#[doc = "Field `SU` reader - Second units in BCD format"]
pub type SU_R = crate::FieldReader;
#[doc = "Field `SU` writer - Second units in BCD format"]
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ST` reader - Second tens in BCD format"]
pub type ST_R = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens in BCD format"]
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Alarm B seconds mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK1_A {
    #[doc = "0: Alarm B set if the seconds match"]
    B_0x0 = 0,
    #[doc = "1: Seconds don't care in alarm B comparison"]
    B_0x1 = 1,
}
impl From<MSK1_A> for bool {
    #[inline(always)]
    fn from(variant: MSK1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK1` reader - Alarm B seconds mask"]
pub type MSK1_R = crate::BitReader<MSK1_A>;
impl MSK1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSK1_A {
        match self.bits {
            false => MSK1_A::B_0x0,
            true => MSK1_A::B_0x1,
        }
    }
    #[doc = "Alarm B set if the seconds match"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MSK1_A::B_0x0
    }
    #[doc = "Seconds don't care in alarm B comparison"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MSK1_A::B_0x1
    }
}
#[doc = "Field `MSK1` writer - Alarm B seconds mask"]
pub type MSK1_W<'a, REG> = crate::BitWriter<'a, REG, MSK1_A>;
impl<'a, REG> MSK1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm B set if the seconds match"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSK1_A::B_0x0)
    }
    #[doc = "Seconds don't care in alarm B comparison"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSK1_A::B_0x1)
    }
}
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub type MNU_R = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD format"]
pub type MNU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub type MNT_R = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD format"]
pub type MNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Alarm B minutes mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK2_A {
    #[doc = "0: Alarm B set if the minutes match"]
    B_0x0 = 0,
    #[doc = "1: Minutes don't care in alarm B comparison"]
    B_0x1 = 1,
}
impl From<MSK2_A> for bool {
    #[inline(always)]
    fn from(variant: MSK2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK2` reader - Alarm B minutes mask"]
pub type MSK2_R = crate::BitReader<MSK2_A>;
impl MSK2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSK2_A {
        match self.bits {
            false => MSK2_A::B_0x0,
            true => MSK2_A::B_0x1,
        }
    }
    #[doc = "Alarm B set if the minutes match"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MSK2_A::B_0x0
    }
    #[doc = "Minutes don't care in alarm B comparison"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MSK2_A::B_0x1
    }
}
#[doc = "Field `MSK2` writer - Alarm B minutes mask"]
pub type MSK2_W<'a, REG> = crate::BitWriter<'a, REG, MSK2_A>;
impl<'a, REG> MSK2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm B set if the minutes match"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSK2_A::B_0x0)
    }
    #[doc = "Minutes don't care in alarm B comparison"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSK2_A::B_0x1)
    }
}
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
#[doc = "Alarm B hours mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK3_A {
    #[doc = "0: Alarm B set if the hours match"]
    B_0x0 = 0,
    #[doc = "1: Hours don't care in alarm B comparison"]
    B_0x1 = 1,
}
impl From<MSK3_A> for bool {
    #[inline(always)]
    fn from(variant: MSK3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK3` reader - Alarm B hours mask"]
pub type MSK3_R = crate::BitReader<MSK3_A>;
impl MSK3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSK3_A {
        match self.bits {
            false => MSK3_A::B_0x0,
            true => MSK3_A::B_0x1,
        }
    }
    #[doc = "Alarm B set if the hours match"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MSK3_A::B_0x0
    }
    #[doc = "Hours don't care in alarm B comparison"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MSK3_A::B_0x1
    }
}
#[doc = "Field `MSK3` writer - Alarm B hours mask"]
pub type MSK3_W<'a, REG> = crate::BitWriter<'a, REG, MSK3_A>;
impl<'a, REG> MSK3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm B set if the hours match"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSK3_A::B_0x0)
    }
    #[doc = "Hours don't care in alarm B comparison"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSK3_A::B_0x1)
    }
}
#[doc = "Field `DU` reader - Date units or day in BCD format"]
pub type DU_R = crate::FieldReader;
#[doc = "Field `DU` writer - Date units or day in BCD format"]
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens in BCD format"]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Week day selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDSEL_A {
    #[doc = "0: DU\\[3:0\\] represents the date units"]
    B_0x0 = 0,
    #[doc = "1: DU\\[3:0\\] represents the week day. DT\\[1:0\\] is don't care."]
    B_0x1 = 1,
}
impl From<WDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WDSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDSEL` reader - Week day selection"]
pub type WDSEL_R = crate::BitReader<WDSEL_A>;
impl WDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDSEL_A {
        match self.bits {
            false => WDSEL_A::B_0x0,
            true => WDSEL_A::B_0x1,
        }
    }
    #[doc = "DU\\[3:0\\] represents the date units"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WDSEL_A::B_0x0
    }
    #[doc = "DU\\[3:0\\] represents the week day. DT\\[1:0\\] is don't care."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WDSEL_A::B_0x1
    }
}
#[doc = "Field `WDSEL` writer - Week day selection"]
pub type WDSEL_W<'a, REG> = crate::BitWriter<'a, REG, WDSEL_A>;
impl<'a, REG> WDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DU\\[3:0\\] represents the date units"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDSEL_A::B_0x0)
    }
    #[doc = "DU\\[3:0\\] represents the week day. DT\\[1:0\\] is don't care."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDSEL_A::B_0x1)
    }
}
#[doc = "Alarm B date mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK4_A {
    #[doc = "0: Alarm B set if the date and day match"]
    B_0x0 = 0,
    #[doc = "1: Date and day don't care in alarm B comparison"]
    B_0x1 = 1,
}
impl From<MSK4_A> for bool {
    #[inline(always)]
    fn from(variant: MSK4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK4` reader - Alarm B date mask"]
pub type MSK4_R = crate::BitReader<MSK4_A>;
impl MSK4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSK4_A {
        match self.bits {
            false => MSK4_A::B_0x0,
            true => MSK4_A::B_0x1,
        }
    }
    #[doc = "Alarm B set if the date and day match"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MSK4_A::B_0x0
    }
    #[doc = "Date and day don't care in alarm B comparison"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MSK4_A::B_0x1
    }
}
#[doc = "Field `MSK4` writer - Alarm B date mask"]
pub type MSK4_W<'a, REG> = crate::BitWriter<'a, REG, MSK4_A>;
impl<'a, REG> MSK4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm B set if the date and day match"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSK4_A::B_0x0)
    }
    #[doc = "Date and day don't care in alarm B comparison"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSK4_A::B_0x1)
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
    #[doc = "Bit 7 - Alarm B seconds mask"]
    #[inline(always)]
    pub fn MSK1(&self) -> MSK1_R {
        MSK1_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 15 - Alarm B minutes mask"]
    #[inline(always)]
    pub fn MSK2(&self) -> MSK2_R {
        MSK2_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 23 - Alarm B hours mask"]
    #[inline(always)]
    pub fn MSK3(&self) -> MSK3_R {
        MSK3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    pub fn DU(&self) -> DU_R {
        DU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    pub fn DT(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn WDSEL(&self) -> WDSEL_R {
        WDSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm B date mask"]
    #[inline(always)]
    pub fn MSK4(&self) -> MSK4_R {
        MSK4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn SU(&mut self) -> SU_W<'_, ALRMBR_SPEC> {
        SU_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn ST(&mut self) -> ST_W<'_, ALRMBR_SPEC> {
        ST_W::new(self, 4)
    }
    #[doc = "Bit 7 - Alarm B seconds mask"]
    #[inline(always)]
    pub fn MSK1(&mut self) -> MSK1_W<'_, ALRMBR_SPEC> {
        MSK1_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn MNU(&mut self) -> MNU_W<'_, ALRMBR_SPEC> {
        MNU_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn MNT(&mut self) -> MNT_W<'_, ALRMBR_SPEC> {
        MNT_W::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm B minutes mask"]
    #[inline(always)]
    pub fn MSK2(&mut self) -> MSK2_W<'_, ALRMBR_SPEC> {
        MSK2_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn HU(&mut self) -> HU_W<'_, ALRMBR_SPEC> {
        HU_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn HT(&mut self) -> HT_W<'_, ALRMBR_SPEC> {
        HT_W::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn PM(&mut self) -> PM_W<'_, ALRMBR_SPEC> {
        PM_W::new(self, 22)
    }
    #[doc = "Bit 23 - Alarm B hours mask"]
    #[inline(always)]
    pub fn MSK3(&mut self) -> MSK3_W<'_, ALRMBR_SPEC> {
        MSK3_W::new(self, 23)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    pub fn DU(&mut self) -> DU_W<'_, ALRMBR_SPEC> {
        DU_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    pub fn DT(&mut self) -> DT_W<'_, ALRMBR_SPEC> {
        DT_W::new(self, 28)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn WDSEL(&mut self) -> WDSEL_W<'_, ALRMBR_SPEC> {
        WDSEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - Alarm B date mask"]
    #[inline(always)]
    pub fn MSK4(&mut self) -> MSK4_W<'_, ALRMBR_SPEC> {
        MSK4_W::new(self, 31)
    }
}
#[doc = "RTC alarm B register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRMBR_SPEC;
impl crate::RegisterSpec for ALRMBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrmbr::R`](R) reader structure"]
impl crate::Readable for ALRMBR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alrmbr::W`](W) writer structure"]
impl crate::Writable for ALRMBR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ALRMBR to value 0"]
impl crate::Resettable for ALRMBR_SPEC {}
