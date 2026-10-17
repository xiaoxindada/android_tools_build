// SPDX-FileCopyrightText: Copyright The arm-ffa Contributors.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! FF-A notification data structures and functions.

use crate::interface_args::SuccessArgs;
use thiserror::Error;
use zerocopy::transmute;

/// Rich error types returned by this module. Should be converted to [`crate::FfaError`] when used
/// with the `FFA_ERROR` interface.
#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    #[error("Invalid Flag for Notification Set")]
    InvalidNotificationSetFlag(u32),
    #[error("Invalid notification count")]
    InvalidNotificationCount,
}

/// Flags field of the `FFA_NOTIFICATION_BIND` interface.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct NotificationBindFlags {
    pub per_vcpu_notification: bool,
}

impl NotificationBindFlags {
    const PER_VCPU_NOTIFICATION: u32 = 1;
}

impl From<NotificationBindFlags> for u32 {
    fn from(flags: NotificationBindFlags) -> Self {
        let mut bits: u32 = 0;
        if flags.per_vcpu_notification {
            bits |= NotificationBindFlags::PER_VCPU_NOTIFICATION;
        }
        bits
    }
}

impl From<u32> for NotificationBindFlags {
    fn from(flags: u32) -> Self {
        Self {
            per_vcpu_notification: flags & Self::PER_VCPU_NOTIFICATION != 0,
        }
    }
}

/// Flags field of the `FFA_NOTIFICATION_SET` interface.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct NotificationSetFlags {
    pub delay_schedule_receiver: bool,
    pub vcpu_id: Option<u16>,
}

impl NotificationSetFlags {
    const PER_VCP_NOTIFICATION: u32 = 1 << 0;
    const DELAY_SCHEDULE_RECEIVER: u32 = 1 << 1;
    const VCPU_ID_SHIFT: u32 = 16;

    const MBZ_BITS: u32 = 0xfffc;
}

impl From<NotificationSetFlags> for u32 {
    fn from(flags: NotificationSetFlags) -> Self {
        let mut bits: u32 = 0;

        if flags.delay_schedule_receiver {
            bits |= NotificationSetFlags::DELAY_SCHEDULE_RECEIVER;
        }
        if let Some(vcpu_id) = flags.vcpu_id {
            bits |= NotificationSetFlags::PER_VCP_NOTIFICATION;
            bits |= u32::from(vcpu_id) << NotificationSetFlags::VCPU_ID_SHIFT;
        }

        bits
    }
}

impl TryFrom<u32> for NotificationSetFlags {
    type Error = Error;

    fn try_from(flags: u32) -> Result<Self, Self::Error> {
        if (flags & Self::MBZ_BITS) != 0 {
            return Err(Error::InvalidNotificationSetFlag(flags));
        }

        let tentative_vcpu_id = (flags >> Self::VCPU_ID_SHIFT) as u16;

        let vcpu_id = if (flags & Self::PER_VCP_NOTIFICATION) != 0 {
            Some(tentative_vcpu_id)
        } else {
            if tentative_vcpu_id != 0 {
                return Err(Error::InvalidNotificationSetFlag(flags));
            }
            None
        };

        Ok(Self {
            delay_schedule_receiver: (flags & Self::DELAY_SCHEDULE_RECEIVER) != 0,
            vcpu_id,
        })
    }
}

/// Flags field of the `FFA_NOTIFICATION_GET` interface.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct NotificationGetFlags {
    pub sp_bitmap_id: bool,
    pub vm_bitmap_id: bool,
    pub spm_bitmap_id: bool,
    pub hyp_bitmap_id: bool,
}

impl NotificationGetFlags {
    const SP_BITMAP_ID: u32 = 1;
    const VM_BITMAP_ID: u32 = 1 << 1;
    const SPM_BITMAP_ID: u32 = 1 << 2;
    const HYP_BITMAP_ID: u32 = 1 << 3;
}

impl From<NotificationGetFlags> for u32 {
    fn from(flags: NotificationGetFlags) -> Self {
        let mut bits: u32 = 0;
        if flags.sp_bitmap_id {
            bits |= NotificationGetFlags::SP_BITMAP_ID;
        }
        if flags.vm_bitmap_id {
            bits |= NotificationGetFlags::VM_BITMAP_ID;
        }
        if flags.spm_bitmap_id {
            bits |= NotificationGetFlags::SPM_BITMAP_ID;
        }
        if flags.hyp_bitmap_id {
            bits |= NotificationGetFlags::HYP_BITMAP_ID;
        }
        bits
    }
}

impl From<u32> for NotificationGetFlags {
    // This is a "from" instead of a "try_from" because Reserved Bits are SBZ, *not* MBZ.
    fn from(flags: u32) -> Self {
        Self {
            sp_bitmap_id: (flags & Self::SP_BITMAP_ID) != 0,
            vm_bitmap_id: (flags & Self::VM_BITMAP_ID) != 0,
            spm_bitmap_id: (flags & Self::SPM_BITMAP_ID) != 0,
            hyp_bitmap_id: (flags & Self::HYP_BITMAP_ID) != 0,
        }
    }
}

/// `FFA_NOTIFICATION_GET` specific success argument structure.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct SuccessArgsNotificationGet {
    pub sp_notifications: Option<u64>,
    pub vm_notifications: Option<u64>,
    pub spm_notifications: Option<u32>,
    pub hypervisor_notifications: Option<u32>,
}

impl From<SuccessArgsNotificationGet> for SuccessArgs {
    fn from(value: SuccessArgsNotificationGet) -> Self {
        let mut args = [0; 6];

        if let Some(bitmap) = value.sp_notifications {
            args[0] = bitmap as u32;
            args[1] = (bitmap >> 32) as u32;
        }

        if let Some(bitmap) = value.vm_notifications {
            args[2] = bitmap as u32;
            args[3] = (bitmap >> 32) as u32;
        }

        if let Some(bitmap) = value.spm_notifications {
            args[4] = bitmap;
        }

        if let Some(bitmap) = value.hypervisor_notifications {
            args[5] = bitmap;
        }

        Self::Args32(args)
    }
}

impl TryFrom<(NotificationGetFlags, SuccessArgs)> for SuccessArgsNotificationGet {
    type Error = crate::Error;

    fn try_from(value: (NotificationGetFlags, SuccessArgs)) -> Result<Self, Self::Error> {
        let (flags, value) = value;
        let args = value.try_get_args32()?;

        let sp_notifications = if flags.sp_bitmap_id {
            Some(u64::from(args[0]) | (u64::from(args[1]) << 32))
        } else {
            None
        };

        let vm_notifications = if flags.vm_bitmap_id {
            Some(u64::from(args[2]) | (u64::from(args[3]) << 32))
        } else {
            None
        };

        let spm_notifications = if flags.spm_bitmap_id {
            Some(args[4])
        } else {
            None
        };

        let hypervisor_notifications = if flags.hyp_bitmap_id {
            Some(args[5])
        } else {
            None
        };

        Ok(Self {
            sp_notifications,
            vm_notifications,
            spm_notifications,
            hypervisor_notifications,
        })
    }
}

/// `FFA_NOTIFICATION_INFO_GET` specific success argument structure. The `MAX_COUNT` parameter
/// depends on the 32-bit or 64-bit packing.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct SuccessArgsNotificationInfoGet<const MAX_COUNT: usize> {
    pub more_pending_notifications: bool,
    pub(crate) list_count: usize,
    pub(crate) id_counts: [u8; MAX_COUNT],
    pub(crate) ids: [u16; MAX_COUNT],
}

impl<const MAX_COUNT: usize> Default for SuccessArgsNotificationInfoGet<MAX_COUNT> {
    fn default() -> Self {
        Self {
            more_pending_notifications: false,
            list_count: 0,
            id_counts: [0; MAX_COUNT],
            ids: [0; MAX_COUNT],
        }
    }
}

impl<const MAX_COUNT: usize> SuccessArgsNotificationInfoGet<MAX_COUNT> {
    const MORE_PENDING_NOTIFICATIONS_FLAG: u64 = 1 << 0;
    const LIST_COUNT_SHIFT: usize = 7;
    const LIST_COUNT_MASK: u64 = 0x1f;
    const ID_COUNT_SHIFT: usize = 12;
    const ID_COUNT_MASK: u64 = 0x03;
    const ID_COUNT_BITS: usize = 2;

    pub fn add_list(&mut self, endpoint: u16, vcpu_ids: &[u16]) -> Result<(), Error> {
        if self.list_count >= MAX_COUNT || vcpu_ids.len() > Self::ID_COUNT_MASK as usize {
            return Err(Error::InvalidNotificationCount);
        }

        // Each list contains at least one ID: the partition ID, followed by vCPU IDs. The number
        // of vCPU IDs is recorded in `id_counts`.
        let mut current_id_index = self.list_count + self.id_counts.iter().sum::<u8>() as usize;
        if current_id_index + 1 + vcpu_ids.len() > MAX_COUNT {
            // The new list does not fit into the available space for IDs.
            return Err(Error::InvalidNotificationCount);
        }

        self.id_counts[self.list_count] = vcpu_ids.len() as u8;
        self.list_count += 1;

        // The first ID is the endpoint ID.
        self.ids[current_id_index] = endpoint;
        current_id_index += 1;

        // Insert the vCPU IDs.
        self.ids[current_id_index..current_id_index + vcpu_ids.len()].copy_from_slice(vcpu_ids);

        Ok(())
    }

    pub fn iter(&self) -> NotificationInfoGetIterator<'_> {
        NotificationInfoGetIterator {
            list_index: 0,
            id_index: 0,
            id_count: &self.id_counts[0..self.list_count],
            ids: &self.ids,
        }
    }

    /// Pack flags field and IDs.
    fn pack(self) -> (u64, [u16; MAX_COUNT]) {
        let mut flags = if self.more_pending_notifications {
            Self::MORE_PENDING_NOTIFICATIONS_FLAG
        } else {
            0
        };

        flags |= (self.list_count as u64) << Self::LIST_COUNT_SHIFT;
        for (count, shift) in self.id_counts.iter().take(self.list_count).zip(
            (Self::ID_COUNT_SHIFT..Self::ID_COUNT_SHIFT + Self::ID_COUNT_BITS * MAX_COUNT)
                .step_by(Self::ID_COUNT_BITS),
        ) {
            flags |= u64::from(*count) << shift;
        }

        (flags, self.ids)
    }

    /// Unpack flags field and IDs.
    fn unpack(flags: u64, ids: [u16; MAX_COUNT]) -> Result<Self, Error> {
        let count_of_lists = ((flags >> Self::LIST_COUNT_SHIFT) & Self::LIST_COUNT_MASK) as usize;

        if count_of_lists > MAX_COUNT {
            return Err(Error::InvalidNotificationCount);
        }

        let mut count_of_ids = [0; MAX_COUNT];
        let mut count_of_ids_bits = flags >> Self::ID_COUNT_SHIFT;

        for id in count_of_ids.iter_mut().take(count_of_lists) {
            *id = (count_of_ids_bits & Self::ID_COUNT_MASK) as u8;
            count_of_ids_bits >>= Self::ID_COUNT_BITS;
        }

        let id_field_count = count_of_lists + count_of_ids.iter().sum::<u8>() as usize;
        if id_field_count > MAX_COUNT {
            return Err(Error::InvalidNotificationCount);
        }

        Ok(Self {
            more_pending_notifications: (flags & Self::MORE_PENDING_NOTIFICATIONS_FLAG) != 0,
            list_count: count_of_lists,
            id_counts: count_of_ids,
            ids,
        })
    }
}

/// `FFA_NOTIFICATION_INFO_GET_32` specific success argument structure.
pub type SuccessArgsNotificationInfoGet32 = SuccessArgsNotificationInfoGet<10>;

impl From<SuccessArgsNotificationInfoGet32> for SuccessArgs {
    fn from(value: SuccessArgsNotificationInfoGet32) -> Self {
        let (flags, ids) = value.pack();
        let id_regs: [u32; 5] = transmute!(ids);

        let mut args = [0; 6];
        args[0] = flags as u32;
        args[1..6].copy_from_slice(&id_regs);

        SuccessArgs::Args32(args)
    }
}

impl TryFrom<SuccessArgs> for SuccessArgsNotificationInfoGet32 {
    type Error = crate::Error;

    fn try_from(value: SuccessArgs) -> Result<Self, Self::Error> {
        let args = value.try_get_args32()?;
        let flags = args[0].into();
        let id_regs: [u32; 5] = args[1..6].try_into().unwrap();
        Self::unpack(flags, transmute!(id_regs)).map_err(|e| e.into())
    }
}

/// `FFA_NOTIFICATION_INFO_GET_64` specific success argument structure.
pub type SuccessArgsNotificationInfoGet64 = SuccessArgsNotificationInfoGet<20>;

impl From<SuccessArgsNotificationInfoGet64> for SuccessArgs {
    fn from(value: SuccessArgsNotificationInfoGet64) -> Self {
        let (flags, ids) = value.pack();
        let id_regs: [u64; 5] = transmute!(ids);

        let mut args = [0; 16];
        args[0] = flags;
        args[1..6].copy_from_slice(&id_regs);

        SuccessArgs::Args64(args)
    }
}

impl TryFrom<SuccessArgs> for SuccessArgsNotificationInfoGet64 {
    type Error = crate::Error;

    fn try_from(value: SuccessArgs) -> Result<Self, Self::Error> {
        let args = value.try_get_args64()?;
        let flags = args[0];
        let id_regs: [u64; 5] = args[1..6].try_into().unwrap();
        Self::unpack(flags, transmute!(id_regs)).map_err(|e| e.into())
    }
}

/// Iterator implementation for parsing the (partition ID, vCPU ID list) pairs of the `FFA_SUCCESS`
/// of an `FFA_NOTIFICATION_INFO_GET` call.
pub struct NotificationInfoGetIterator<'a> {
    list_index: usize,
    id_index: usize,
    id_count: &'a [u8],
    ids: &'a [u16],
}

impl<'a> Iterator for NotificationInfoGetIterator<'a> {
    type Item = (u16, &'a [u16]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.list_index < self.id_count.len() {
            let partition_id = self.ids[self.id_index];
            let id_range =
                (self.id_index + 1)..=(self.id_index + self.id_count[self.list_index] as usize);

            self.id_index += 1 + self.id_count[self.list_index] as usize;
            self.list_index += 1;

            Some((partition_id, &self.ids[id_range]))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_args_notification_info_get32() {
        let mut notifications = SuccessArgsNotificationInfoGet32::default();

        // 16.7.1.1 Example usage
        notifications.add_list(0x0000, &[0, 2, 3]).unwrap();
        notifications.add_list(0x0000, &[4, 6]).unwrap();
        notifications.add_list(0x0002, &[]).unwrap();
        notifications.add_list(0x0003, &[1]).unwrap();

        let args: SuccessArgs = notifications.into();
        assert_eq!(
            SuccessArgs::Args32([
                0x0004_b200,
                0x0000_0000,
                0x0003_0002,
                0x0004_0000,
                0x0002_0006,
                0x0001_0003
            ]),
            args
        );

        let notifications = SuccessArgsNotificationInfoGet32::try_from(args).unwrap();
        let mut iter = notifications.iter();
        assert_eq!(Some((0x0000, &[0, 2, 3][..])), iter.next());
        assert_eq!(Some((0x0000, &[4, 6][..])), iter.next());
        assert_eq!(Some((0x0002, &[][..])), iter.next());
        assert_eq!(Some((0x0003, &[1][..])), iter.next());
    }

    #[test]
    fn success_args_notification_info_get64() {
        let mut notifications = SuccessArgsNotificationInfoGet64::default();

        // 16.7.1.1 Example usage
        notifications.add_list(0x0000, &[0, 2, 3]).unwrap();
        notifications.add_list(0x0000, &[4, 6]).unwrap();
        notifications.add_list(0x0002, &[]).unwrap();
        notifications.add_list(0x0003, &[1]).unwrap();

        let args: SuccessArgs = notifications.into();
        assert_eq!(
            SuccessArgs::Args64([
                0x0004_b200,
                0x0003_0002_0000_0000,
                0x0002_0006_0004_0000,
                0x0000_0000_0001_0003,
                0x0000_0000_0000_0000,
                0x0000_0000_0000_0000,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0
            ]),
            args
        );

        let notifications = SuccessArgsNotificationInfoGet64::try_from(args).unwrap();
        let mut iter = notifications.iter();
        assert_eq!(Some((0x0000, &[0, 2, 3][..])), iter.next());
        assert_eq!(Some((0x0000, &[4, 6][..])), iter.next());
        assert_eq!(Some((0x0002, &[][..])), iter.next());
        assert_eq!(Some((0x0003, &[1][..])), iter.next());
    }
}
