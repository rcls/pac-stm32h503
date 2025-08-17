#[doc = "Register `DBGCR` reader"]
pub type R = crate::R<DBGCR_SPEC>;
#[doc = "Register `DBGCR` writer"]
pub type W = crate::W<DBGCR_SPEC>;
#[doc = "Field `AP_UNLOCK` reader - access port unlock Write 0xB4 to this bitfield to open the device access port."]
pub type AP_UNLOCK_R = crate::FieldReader;
#[doc = "Field `AP_UNLOCK` writer - access port unlock Write 0xB4 to this bitfield to open the device access port."]
pub type AP_UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_UNLOCK` reader - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register."]
pub type DBG_UNLOCK_R = crate::FieldReader;
#[doc = "Field `DBG_UNLOCK` writer - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register."]
pub type DBG_UNLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBG_AUTH_HDPL_A {
    #[doc = "81: HDPL1"]
    B_0x51 = 81,
    #[doc = "138: HDPL2"]
    B_0x8A = 138,
    #[doc = "111: HDPL3"]
    B_0x6F = 111,
}
impl From<DBG_AUTH_HDPL_A> for u8 {
    #[inline(always)]
    fn from(variant: DBG_AUTH_HDPL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBG_AUTH_HDPL_A {
    type Ux = u8;
}
impl crate::IsEnum for DBG_AUTH_HDPL_A {}
#[doc = "Field `DBG_AUTH_HDPL` reader - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens."]
pub type DBG_AUTH_HDPL_R = crate::FieldReader<DBG_AUTH_HDPL_A>;
impl DBG_AUTH_HDPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBG_AUTH_HDPL_A> {
        match self.bits {
            81 => Some(DBG_AUTH_HDPL_A::B_0x51),
            138 => Some(DBG_AUTH_HDPL_A::B_0x8A),
            111 => Some(DBG_AUTH_HDPL_A::B_0x6F),
            _ => None,
        }
    }
    #[doc = "HDPL1"]
    #[inline(always)]
    pub fn is_B_0x51(&self) -> bool {
        *self == DBG_AUTH_HDPL_A::B_0x51
    }
    #[doc = "HDPL2"]
    #[inline(always)]
    pub fn is_B_0x8A(&self) -> bool {
        *self == DBG_AUTH_HDPL_A::B_0x8A
    }
    #[doc = "HDPL3"]
    #[inline(always)]
    pub fn is_B_0x6F(&self) -> bool {
        *self == DBG_AUTH_HDPL_A::B_0x6F
    }
}
#[doc = "Field `DBG_AUTH_HDPL` writer - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens."]
pub type DBG_AUTH_HDPL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, DBG_AUTH_HDPL_A>;
impl<'a, REG> DBG_AUTH_HDPL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HDPL1"]
    #[inline(always)]
    pub fn B_0x51(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_AUTH_HDPL_A::B_0x51)
    }
    #[doc = "HDPL2"]
    #[inline(always)]
    pub fn B_0x8A(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_AUTH_HDPL_A::B_0x8A)
    }
    #[doc = "HDPL3"]
    #[inline(always)]
    pub fn B_0x6F(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_AUTH_HDPL_A::B_0x6F)
    }
}
impl R {
    #[doc = "Bits 0:7 - access port unlock Write 0xB4 to this bitfield to open the device access port."]
    #[inline(always)]
    pub fn AP_UNLOCK(&self) -> AP_UNLOCK_R {
        AP_UNLOCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register."]
    #[inline(always)]
    pub fn DBG_UNLOCK(&self) -> DBG_UNLOCK_R {
        DBG_UNLOCK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens."]
    #[inline(always)]
    pub fn DBG_AUTH_HDPL(&self) -> DBG_AUTH_HDPL_R {
        DBG_AUTH_HDPL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - access port unlock Write 0xB4 to this bitfield to open the device access port."]
    #[inline(always)]
    pub fn AP_UNLOCK(&mut self) -> AP_UNLOCK_W<'_, DBGCR_SPEC> {
        AP_UNLOCK_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register."]
    #[inline(always)]
    pub fn DBG_UNLOCK(&mut self) -> DBG_UNLOCK_W<'_, DBGCR_SPEC> {
        DBG_UNLOCK_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens."]
    #[inline(always)]
    pub fn DBG_AUTH_HDPL(&mut self) -> DBG_AUTH_HDPL_W<'_, DBGCR_SPEC> {
        DBG_AUTH_HDPL_W::new(self, 16)
    }
}
#[doc = "SBS debug control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGCR_SPEC;
impl crate::RegisterSpec for DBGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgcr::R`](R) reader structure"]
impl crate::Readable for DBGCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbgcr::W`](W) writer structure"]
impl crate::Writable for DBGCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DBGCR to value 0"]
impl crate::Resettable for DBGCR_SPEC {}
