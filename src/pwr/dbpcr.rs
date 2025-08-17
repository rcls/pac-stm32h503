#[doc = "Register `DBPCR` reader"]
pub type R = crate::R<DBPCR_SPEC>;
#[doc = "Register `DBPCR` writer"]
pub type W = crate::W<DBPCR_SPEC>;
#[doc = "Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP_A {
    #[doc = "0: Write access to Backup domain disabled"]
    B_0x0 = 0,
    #[doc = "1: Write access to Backup domain enabled"]
    B_0x1 = 1,
}
impl From<DBP_A> for bool {
    #[inline(always)]
    fn from(variant: DBP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBP` reader - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub type DBP_R = crate::BitReader<DBP_A>;
impl DBP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBP_A {
        match self.bits {
            false => DBP_A::B_0x0,
            true => DBP_A::B_0x1,
        }
    }
    #[doc = "Write access to Backup domain disabled"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == DBP_A::B_0x0
    }
    #[doc = "Write access to Backup domain enabled"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == DBP_A::B_0x1
    }
}
#[doc = "Field `DBP` writer - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
pub type DBP_W<'a, REG> = crate::BitWriter<'a, REG, DBP_A>;
impl<'a, REG> DBP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write access to Backup domain disabled"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBP_A::B_0x0)
    }
    #[doc = "Write access to Backup domain enabled"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBP_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    pub fn DBP(&self) -> DBP_R {
        DBP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
    #[inline(always)]
    pub fn DBP(&mut self) -> DBP_W<'_, DBPCR_SPEC> {
        DBP_W::new(self, 0)
    }
}
#[doc = "PWR disable backup protection control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBPCR_SPEC;
impl crate::RegisterSpec for DBPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbpcr::R`](R) reader structure"]
impl crate::Readable for DBPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbpcr::W`](W) writer structure"]
impl crate::Writable for DBPCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DBPCR to value 0"]
impl crate::Resettable for DBPCR_SPEC {}
