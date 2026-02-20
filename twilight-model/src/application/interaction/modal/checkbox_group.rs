#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModalInteractionCheckboxGroup {
    /// User defined identifier for the component.
    ///
    /// See [Discord Docs/Custom ID].
    ///
    /// Unique identifier for the component.
    pub id: i32,
    /// [Discord Docs/Custom ID]: https://discord.com/developers/docs/components/reference#anatomy-of-a-component-custom-id
    pub custom_id: String,
    /// Value submitted by the user.
    pub values: Vec<String>,
}
