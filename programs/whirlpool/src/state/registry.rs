use bytemuck::{Pod, Zeroable};
use solana_program::{account_info::AccountInfo, pubkey, pubkey::Pubkey};

const MAX_ITEMS: usize = 64;

/// The registry account stores a mapping of registered segmenter accounts
#[derive(Pod, Zeroable, Copy, Clone)]
#[repr(C)]
pub struct Registry {
    pub registered_segmenters: [Pubkey; MAX_ITEMS],
}

impl Registry {
    pub const PROGRAM_ID: Pubkey = pubkey!("SRegZsVZDDqwc7W5iMUSsmKNnXzgfczKzFpimRp5iWw");
    pub const DISCRIMINATOR: [u8; 8] = [47, 174, 110, 246, 184, 182, 252, 218];

    pub fn is_segmenter_registered(&self, key: &Pubkey) -> bool {
        self.registered_segmenters.binary_search(key).is_ok()
    }

    pub fn deserialize(bytes: &[u8]) -> &Self {
        bytemuck::from_bytes(&bytes[8..])
    }
}

/// Checks whether the invocation was signed by a registered segmenter.
///
/// This function performs a series of checks:
/// 1. It checks that the registry account specified is actually a registry account by checking its owner and discriminator.
/// 2. It checks that the `registered_segmenter` is a signer.
/// 3. It checks that the `registered_segmenter` is included in the registry account.
///
/// # Arguments
///
/// * `registry` - The registry account which holds the list of registered segmenters.
/// * `registered_segmenter` - The segmenter that signed the invocation.
///
/// # Returns
///
/// * `true` if the `registered_segmenter` is a registered segmenter and signed the invocation; otherwise, returns `false`.
pub fn is_invoked_by_segmenter<'info>(
    registry: &AccountInfo<'info>,
    registered_segmenter: &AccountInfo<'info>,
) -> bool {
    if *registry.owner != Registry::PROGRAM_ID {
        return false;
    }
    if !registered_segmenter.is_signer {
        return false;
    }

    let registry_account_data = registry.data.borrow();
    if registry_account_data[..8] != Registry::DISCRIMINATOR {
        return false;
    }

    let registry_state = Registry::deserialize(&registry_account_data);
    registry_state.is_segmenter_registered(registered_segmenter.key)
}
