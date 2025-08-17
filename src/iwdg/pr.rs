#[doc = "Register `PR` reader"]
pub type R = crate::R<PR_SPEC>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PR_SPEC>;
#[doc = "Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PR_A {
    #[doc = "0: divider / 4"]
    B_0x0 = 0,
    #[doc = "1: divider / 8"]
    B_0x1 = 1,
    #[doc = "2: divider / 16"]
    B_0x2 = 2,
    #[doc = "3: divider / 32"]
    B_0x3 = 3,
    #[doc = "4: divider / 64"]
    B_0x4 = 4,
    #[doc = "5: divider / 128"]
    B_0x5 = 5,
    #[doc = "6: divider / 256"]
    B_0x6 = 6,
    #[doc = "7: divider / 512"]
    B_0x7 = 7,
}
impl From<PR_A> for u8 {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PR_A {
    type Ux = u8;
}
impl crate::IsEnum for PR_A {}
#[doc = "Field `PR` reader - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
pub type PR_R = crate::FieldReader<PR_A>;
impl PR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PR_A> {
        match self.bits {
            0 => Some(PR_A::B_0x0),
            1 => Some(PR_A::B_0x1),
            2 => Some(PR_A::B_0x2),
            3 => Some(PR_A::B_0x3),
            4 => Some(PR_A::B_0x4),
            5 => Some(PR_A::B_0x5),
            6 => Some(PR_A::B_0x6),
            7 => Some(PR_A::B_0x7),
            _ => None,
        }
    }
    #[doc = "divider / 4"]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == PR_A::B_0x0
    }
    #[doc = "divider / 8"]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == PR_A::B_0x1
    }
    #[doc = "divider / 16"]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == PR_A::B_0x2
    }
    #[doc = "divider / 32"]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == PR_A::B_0x3
    }
    #[doc = "divider / 64"]
    #[inline(always)]
    pub fn is_B_0x4(&self) -> bool {
        *self == PR_A::B_0x4
    }
    #[doc = "divider / 128"]
    #[inline(always)]
    pub fn is_B_0x5(&self) -> bool {
        *self == PR_A::B_0x5
    }
    #[doc = "divider / 256"]
    #[inline(always)]
    pub fn is_B_0x6(&self) -> bool {
        *self == PR_A::B_0x6
    }
    #[doc = "divider / 512"]
    #[inline(always)]
    pub fn is_B_0x7(&self) -> bool {
        *self == PR_A::B_0x7
    }
}
#[doc = "Field `PR` writer - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PR_A>;
impl<'a, REG> PR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "divider / 4"]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0x0)
    }
    #[doc = "divider / 8"]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0x1)
    }
    #[doc = "divider / 16"]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0x2)
    }
    #[doc = "divider / 32"]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0x3)
    }
    #[doc = "divider / 64"]
    #[inline(always)]
    pub fn B_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0x4)
    }
    #[doc = "divider / 128"]
    #[inline(always)]
    pub fn B_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0x5)
    }
    #[doc = "divider / 256"]
    #[inline(always)]
    pub fn B_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0x6)
    }
    #[doc = "divider / 512"]
    #[inline(always)]
    pub fn B_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::B_0x7)
    }
}
impl R {
    #[doc = "Bits 0:3 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn PR(&self) -> PR_R {
        PR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn PR(&mut self) -> PR_W<'_, PR_SPEC> {
        PR_W::new(self, 0)
    }
}
#[doc = "IWDG prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PR_SPEC {}
