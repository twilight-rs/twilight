use super::OptionType;
use serde::{Deserialize, Serialize};

/*
 * # ApplicationCommandInteractionDataOption
 *
 * All options have names, and an option can either be a parameter and
 * input value--in which case `value` will be set--or it can denote a
 * subcommand or group--in which case it will contain a top-level key and
 * another array of `options`.
 *
 * `value` and `options` are mututally exclusive.
 *
 * | Field    | Type                                             |
 * |----------|--------------------------------------------------|
 * | name     | string                                           |
 * | value?   | OptionType                                       |
 * | options? | array of ApplicationCommandInteractionDataOption |
 */
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InteractionDataOption {
    pub name: String,
    pub value: Option<OptionType>,
    pub options: Vec<InteractionDataOption>,
}
