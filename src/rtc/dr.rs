#[doc = "Register `DR` reader"]
pub type R = crate::R<DR_SPEC>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DR_SPEC>;
#[doc = "Field `DU` reader - Date units in BCD format"]
pub type DU_R = crate::FieldReader;
#[doc = "Field `DU` writer - Date units in BCD format"]
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens in BCD format"]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MU` reader - Month units in BCD format"]
pub type MU_R = crate::FieldReader;
#[doc = "Field `MU` writer - Month units in BCD format"]
pub type MU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MT` reader - Month tens in BCD format"]
pub type MT_R = crate::BitReader;
#[doc = "Field `MT` writer - Month tens in BCD format"]
pub type MT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Week day units ...\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDU_A {
    #[doc = "0: forbidden"]
    B_0x0 = 0,
    #[doc = "1: Monday"]
    B_0x1 = 1,
    #[doc = "7: Sunday"]
    B_0x7 = 7,
}
impl From<WDU_A> for u8 {
    #[inline(always)]
    fn from(variant: WDU_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDU_A {
    type Ux = u8;
}
impl crate::IsEnum for WDU_A {}
#[doc = "Field `WDU` reader - Week day units ..."]
pub type WDU_R = crate::FieldReader<WDU_A>;
impl WDU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WDU_A> {
        match self.bits {
            0 => Some(WDU_A::B_0x0),
            1 => Some(WDU_A::B_0x1),
            7 => Some(WDU_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "forbidden"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == WDU_A::B_0x0
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == WDU_A::B_0x1
    }
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == WDU_A::B_0x7
    }
}
#[doc = "Field `WDU` writer - Week day units ..."]
pub type WDU_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WDU_A>;
impl<'a, REG> WDU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "forbidden"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDU_A::B_0x0)
    }
    #[doc = "Monday"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDU_A::B_0x1)
    }
    #[doc = "Sunday"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(WDU_A::B_0x7)
    }
}
#[doc = "Field `YU` reader - Year units in BCD format"]
pub type YU_R = crate::FieldReader;
#[doc = "Field `YU` writer - Year units in BCD format"]
pub type YU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YT` reader - Year tens in BCD format"]
pub type YT_R = crate::FieldReader;
#[doc = "Field `YT` writer - Year tens in BCD format"]
pub type YT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    pub fn DU(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    pub fn DT(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    pub fn MU(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    pub fn MT(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week day units ..."]
    #[inline(always)]
    pub fn WDU(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Year units in BCD format"]
    #[inline(always)]
    pub fn YU(&self) -> YU_R {
        YU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format"]
    #[inline(always)]
    pub fn YT(&self) -> YT_R {
        YT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    pub fn DU(&mut self) -> DU_W<'_, DR_SPEC> {
        DU_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    pub fn DT(&mut self) -> DT_W<'_, DR_SPEC> {
        DT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    pub fn MU(&mut self) -> MU_W<'_, DR_SPEC> {
        MU_W::new(self, 8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    pub fn MT(&mut self) -> MT_W<'_, DR_SPEC> {
        MT_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Week day units ..."]
    #[inline(always)]
    pub fn WDU(&mut self) -> WDU_W<'_, DR_SPEC> {
        WDU_W::new(self, 13)
    }
    #[doc = "Bits 16:19 - Year units in BCD format"]
    #[inline(always)]
    pub fn YU(&mut self) -> YU_W<'_, DR_SPEC> {
        YU_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Year tens in BCD format"]
    #[inline(always)]
    pub fn YT(&mut self) -> YT_W<'_, DR_SPEC> {
        YT_W::new(self, 20)
    }
}
#[doc = "RTC date register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DR to value 0x2101"]
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: u32 = 0x2101;
}
