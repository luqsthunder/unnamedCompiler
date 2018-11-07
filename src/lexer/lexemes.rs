pub const FOR: &'static str = "for";
pub const WHILE: &'static str = "while";
pub const IF: &'static str = "if";
pub const ELSE: &'static str = "else";
pub const RETURN: &'static str = "return";

pub const OPRL_LOGIC: &'static str = "or|and";
pub const OPRL_NOT: &'static str = "not";

pub const INT: &'static str = "int";
pub const CHAR: &'static str = "char";
pub const FLOAT: &'static str = "float";
pub const STRING: &'static str = "string";
pub const VEC: &'static str = "::";
pub const VOID: &'static str = "void";

pub const OP_CURLY: &'static str = "\\{";
pub const CL_CURLY: &'static str = "\\}";
pub const OP_PARENT: &'static str = "\\(";
pub const CL_PARENT: &'static str = "\\)";

pub const OPR_PP: &'static str = "\\+\\+|";

pub const OPRP: &'static str = "\\+|-";
pub const OPRM: &'static str = "\\*|/";

pub const OPRLR_REL: &'static str = "<=|>=|==|>|<";

pub const ATTR_TO: &'static str = "=";

pub const ID: &'static str = "([a-zA-Z])([a-zA-Z0-9]+)?";

pub const FLOAT_CONSTANT: &'static str = "(([0-9]*\\.[0-9]+)|([0-9]+))";
pub const INT_CONSTANT: &'static str = "[0-9]+";
pub const CHAR_CONSTANT: &'static str = "\'[a-zA-Z0-9]\'";
pub const STRING_CONSTANT: &'static str = "\"[a-zA-Z0-9_ ]*\"";
