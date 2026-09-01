//! Static preview registrations exported for host catalog merge.

use crate::preview::PreviewRegistration;

#[cfg(feature = "preview")]
use crate::components::tag_catalog_picker::TAGCATALOGPICKER_PREVIEW_REGISTRATION;

orbital_macros::preview_registrations! {
    &TAGCATALOGPICKER_PREVIEW_REGISTRATION,
}
