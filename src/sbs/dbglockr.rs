#[doc = "Register `DBGLOCKR` reader"]
pub type R = crate::R<DBGLOCKR_SPEC>;
#[doc = "Register `DBGLOCKR` writer"]
pub type W = crate::W<DBGLOCKR_SPEC>;
#[doc = "debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored\n\nValue on reset: 180"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBGCFG_LOCK_A {
    #[doc = "180: Writes to SBS_DBGCR allowed (default)"]
    B_0xB4 = 180,
}
impl From<DBGCFG_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: DBGCFG_LOCK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBGCFG_LOCK_A {
    type Ux = u8;
}
impl crate::IsEnum for DBGCFG_LOCK_A {}
#[doc = "Field `DBGCFG_LOCK` reader - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
pub type DBGCFG_LOCK_R = crate::FieldReader<DBGCFG_LOCK_A>;
impl DBGCFG_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBGCFG_LOCK_A> {
        match self.bits {
            180 => Some(DBGCFG_LOCK_A::B_0xB4),
            _ => None,
        }
    }
    #[doc = "Writes to SBS_DBGCR allowed (default)"]
    #[inline(always)]
    pub fn is_B_0xB4(&self) -> bool {
        *self == DBGCFG_LOCK_A::B_0xB4
    }
}
#[doc = "Field `DBGCFG_LOCK` writer - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
pub type DBGCFG_LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8, DBGCFG_LOCK_A>;
impl<'a, REG> DBGCFG_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writes to SBS_DBGCR allowed (default)"]
    #[inline(always)]
    pub fn B_0xB4(self) -> &'a mut crate::W<REG> {
        self.variant(DBGCFG_LOCK_A::B_0xB4)
    }
}
impl R {
    #[doc = "Bits 0:7 - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
    #[inline(always)]
    pub fn DBGCFG_LOCK(&self) -> DBGCFG_LOCK_R {
        DBGCFG_LOCK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - debug configuration lock Reading this bitfield returns 0x6A if the bitfield value is different from 0xB4. 0xC3 is the recommended value to lock the debug configuration using this bitfield. Other: Writes to SBS_DBGCR ignored"]
    #[inline(always)]
    pub fn DBGCFG_LOCK(&mut self) -> DBGCFG_LOCK_W<'_, DBGLOCKR_SPEC> {
        DBGCFG_LOCK_W::new(self, 0)
    }
}
#[doc = "SBS debug lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbglockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbglockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGLOCKR_SPEC;
impl crate::RegisterSpec for DBGLOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbglockr::R`](R) reader structure"]
impl crate::Readable for DBGLOCKR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbglockr::W`](W) writer structure"]
impl crate::Writable for DBGLOCKR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets DBGLOCKR to value 0xb4"]
impl crate::Resettable for DBGLOCKR_SPEC {
    const RESET_VALUE: u32 = 0xb4;
}
