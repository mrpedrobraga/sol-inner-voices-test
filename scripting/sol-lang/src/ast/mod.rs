use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SolASTNode {
    Grouping(Vec<SolASTNode>),
    Command(CommandKey, Vec<CommandFragment>),
    Dialog(Vec<DialogFragment>),
    Nat(SolNat),
    Int(SolInt),
    String(SolString),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandKey(String);

impl CommandKey {
    pub fn debug(what: &str) -> Self {
        Self(what.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandFragment {
    Word(String),
    Expression(SolASTNode),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DialogFragment {
    Text(SolString),
    Expr(SolASTNode),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolNat(u32);

impl Into<u32> for SolNat {
    fn into(self) -> u32 {
        self.0
    }
}

impl Into<SolNat> for u32 {
    fn into(self) -> SolNat {
        SolNat(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolInt(i32);

impl Into<i32> for SolInt {
    fn into(self) -> i32 {
        self.0
    }
}

impl Into<SolInt> for i32 {
    fn into(self) -> SolInt {
        SolInt(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolString(String);

impl Into<String> for SolString {
    fn into(self) -> String {
        self.0
    }
}

impl Into<SolString> for String {
    fn into(self) -> SolString {
        SolString(self)
    }
}
