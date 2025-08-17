#[doc = "Register `OPTCR` reader"]
pub type R = crate::R<OPTCR_SPEC>;
#[doc = "Register `OPTCR` writer"]
pub type W = crate::W<OPTCR_SPEC>;
#[doc = "FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTLOCK_A {
    #[doc = "0: FLASH_OPTCR register unlocked"]
    B_0x0 = 0,
    #[doc = "1: FLASH_OPTCR register locked."]
    B_0x1 = 1,
}
impl From<OPTLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: OPTLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTLOCK` reader - FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change."]
pub type OPTLOCK_R = crate::BitReader<OPTLOCK_A>;
impl OPTLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPTLOCK_A {
        match self.bits {
            false => OPTLOCK_A::B_0x0,
            true => OPTLOCK_A::B_0x1,
        }
    }
    #[doc = "FLASH_OPTCR register unlocked"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == OPTLOCK_A::B_0x0
    }
    #[doc = "FLASH_OPTCR register locked."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == OPTLOCK_A::B_0x1
    }
}
#[doc = "Field `OPTLOCK` writer - FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change."]
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG, OPTLOCK_A>;
impl<'a, REG> OPTLOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FLASH_OPTCR register unlocked"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OPTLOCK_A::B_0x0)
    }
    #[doc = "FLASH_OPTCR register locked."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OPTLOCK_A::B_0x1)
    }
}
#[doc = "Field `OPTSTRT` reader - Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It's set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It's reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG embedded Flash memory register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in non-volatile memory."]
pub type OPTSTRT_R = crate::BitReader;
#[doc = "Field `OPTSTRT` writer - Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It's set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It's reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG embedded Flash memory register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in non-volatile memory."]
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Bank swapping option configuration bit SWAP_BANK controls whether Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register only after reset or POR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP_BANK_A {
    #[doc = "0: Bank1 and Bank2 not swapped"]
    B_0x0 = 0,
    #[doc = "1: Bank1 and Bank2 swapped"]
    B_0x1 = 1,
}
impl From<SWAP_BANK_A> for bool {
    #[inline(always)]
    fn from(variant: SWAP_BANK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP_BANK` reader - Bank swapping option configuration bit SWAP_BANK controls whether Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register only after reset or POR."]
pub type SWAP_BANK_R = crate::BitReader<SWAP_BANK_A>;
impl SWAP_BANK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWAP_BANK_A {
        match self.bits {
            false => SWAP_BANK_A::B_0x0,
            true => SWAP_BANK_A::B_0x1,
        }
    }
    #[doc = "Bank1 and Bank2 not swapped"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == SWAP_BANK_A::B_0x0
    }
    #[doc = "Bank1 and Bank2 swapped"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == SWAP_BANK_A::B_0x1
    }
}
impl R {
    #[doc = "Bit 0 - FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change."]
    #[inline(always)]
    pub fn OPTLOCK(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It's set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It's reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG embedded Flash memory register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in non-volatile memory."]
    #[inline(always)]
    pub fn OPTSTRT(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping option configuration bit SWAP_BANK controls whether Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register only after reset or POR."]
    #[inline(always)]
    pub fn SWAP_BANK(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change."]
    #[inline(always)]
    pub fn OPTLOCK(&mut self) -> OPTLOCK_W<'_, OPTCR_SPEC> {
        OPTLOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It's set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It's reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG embedded Flash memory register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in non-volatile memory."]
    #[inline(always)]
    pub fn OPTSTRT(&mut self) -> OPTSTRT_W<'_, OPTCR_SPEC> {
        OPTSTRT_W::new(self, 1)
    }
}
#[doc = "FLASH option control register\n\nYou can [`read`](crate::Reg::read) this register and get [`optcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTCR_SPEC;
impl crate::RegisterSpec for OPTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optcr::R`](R) reader structure"]
impl crate::Readable for OPTCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`optcr::W`](W) writer structure"]
impl crate::Writable for OPTCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OPTCR to value 0x01"]
impl crate::Resettable for OPTCR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
