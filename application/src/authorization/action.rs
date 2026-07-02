#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationAction {
    Read,
    Create,
    Update,
    Delete,
    Manage,
}
