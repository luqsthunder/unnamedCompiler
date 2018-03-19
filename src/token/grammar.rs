//pub const INITIALS: &'static [str] = &["{", "[", "(a-z|A-Z)"];

pub const FOR: &'static str = "for";
pub const WHILE: &'static str = "while";
pub const IF: &'static str = "if";
pub const ELSE: &'static str = "else";

pub const INT: &'static str = "int";
pub const CHAR: &'static str = "char";
pub const FLOAT: &'static str = "float";
pub const STRING: &'static str = "string";
pub const VEC: &'static str = "::";

pub const OP_BRACKET: &'static str = "\\[";
pub const CL_BRACKET: &'static str = "\\]";
pub const OP_CURLY: &'static str = "\\{";
pub const CL_CURLY: &'static str = "\\}";
pub const OP_PARENT: &'static str = "\\(";
pub const CL_PARENT: &'static str = "\\)";
pub const COMMA: &'static str = ",";

pub const OPRP: &'static str = "\\+|-";
pub const OPRM: &'static str = "\\*|/";

pub const OPRLR_LGT: &'static str = "<|>";
pub const OPRLR_LGT_EQ: &'static str = "<=|>=";
pub const OPRLR_EQ: &'static str = "!=|==";

pub const DREAD: &'static str = "#";

pub const ID: &'static str = "([a-z]|[A-Z]|[0-9]*)?";

pub const FLOAT_CONSTANT: &'static str = "([0-9]*\\.[0-9]+)";
pub const INT_CONSTANT: &'static str = "([1-9]*[0-9]+)";
pub const CHAR_CONSTANT: &'static str = "";
pub const STRING_CONSTANT: &'static str = "";
