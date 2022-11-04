/// Placeholder for the shnippet name in help text templates
pub const NAME_PLACEHOLDER: &str = "{name}";

/// Placeholder for the shnippet description in help text templates
pub const DESCRIPTION_PLACEHOLDER: &str = "{description}";

// LIST Command
pub const LIST_COMMAND_NAME: &str = "list";
pub const LIST_COMMAND_DESCRIPTION: &str = "List all shnippets";

// NEW Commmand
pub const NEW_COMMAND_NAME: &str = "new";
pub const NEW_COMMAND_DESCRIPTION: &str = "Add new shnippet";

// DELETE Command
pub const DELETE_COMMAND_NAME: &str = "delete";
pub const DESCRIPTION_COMMAND_NAME: &str = "Delete an existing shnippet";
pub const DELETE_DESCRIPTION_TEMPLATE: &str = "Delete {name} ({description})";

// EDIT Command
pub const EDIT_COMMAND_NAME: &str = "edit";
pub const EDIT_COMMAND_DESCRIPTION: &str = "Edit an existing shnippet";
pub const EDIT_DESCRIPTION_TEMPLATE: &&str = &"Edit {name} ({description})";

// EXEC Command
pub const EXEC_COMMAND_NAME: &str = "exec";
pub const EXEC_COMMAND_DESCRIPTION: &str = "Run a shnippet in shell";
