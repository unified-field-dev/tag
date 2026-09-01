//! Top-level route pages for the Tags app: list, create, and detail/edit.

mod create;
mod detail;
mod list;

pub use create::TagCreatePage;
pub use detail::TagDetailPage;
pub use list::TagListPage;
