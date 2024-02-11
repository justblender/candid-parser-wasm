use candid_parser::candid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Label {
  Id(u32),
  Named(String),
  Unnamed(u32),
}

impl From<&candid::types::Label> for Label {
  fn from(value: &candid::types::Label) -> Self {
    match value {
      candid::types::Label::Id(arg1) => Label::Id(*arg1),
      candid::types::Label::Named(arg1) => Label::Named(arg1.to_owned()),
      candid::types::Label::Unnamed(arg1) => Label::Unnamed(*arg1),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
  pub id: Label,
  pub ty: TypeInner,
}

impl From<&candid::types::Field> for Field {
  fn from(value: &candid::types::Field) -> Self {
    Field {
      id: value.id.as_ref().into(),
      ty: value.ty.0.as_ref().into(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "kind")]
pub enum FuncMode {
  Oneway,
  Query,
  CompositeQuery,
}

impl From<&candid::types::FuncMode> for FuncMode {
  fn from(value: &candid::types::FuncMode) -> Self {
    match value {
      candid::types::FuncMode::Oneway => FuncMode::Oneway,
      candid::types::FuncMode::Query => FuncMode::Query,
      candid::types::FuncMode::CompositeQuery => FuncMode::CompositeQuery,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Function {
  pub modes: Vec<FuncMode>,
  pub args: Vec<TypeInner>,
  pub rets: Vec<TypeInner>,
}

impl From<&candid::types::Function> for Function {
  fn from(value: &candid::types::Function) -> Self {
    Function {
      modes: value.modes.iter().map(|x| x.into()).collect(),
      args: value.args.iter().map(|x| x.0.as_ref().into()).collect(),
      rets: value.rets.iter().map(|x| x.0.as_ref().into()).collect(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TypeId {
  pub name: String,
}

impl From<&candid::types::TypeId> for TypeId {
  fn from(value: &candid::types::TypeId) -> Self {
    TypeId {
      name: value.name.to_owned(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TypeInner {
  Null,
  Bool,
  Nat,
  Int,
  Nat8,
  Nat16,
  Nat32,
  Nat64,
  Int8,
  Int16,
  Int32,
  Int64,
  Float32,
  Float64,
  Text,
  Reserved,
  Empty,
  Knot(TypeId), // For recursive types from Rust
  Var(String),  // For variables from Candid file
  Unknown,
  Opt(Box<TypeInner>),
  Vec(Box<TypeInner>),
  Record(Vec<Field>),
  Variant(Vec<Field>),
  Func(Function),
  Service(Vec<(String, Box<TypeInner>)>),
  Class(Vec<Box<TypeInner>>, Box<TypeInner>),
  Principal,
  Future,
}

impl From<&candid::types::Type> for TypeInner {
  fn from(value: &candid::types::Type) -> Self {
    value.0.as_ref().into()
  }
}

impl From<&candid::types::TypeInner> for TypeInner {
  fn from(value: &candid::types::TypeInner) -> Self {
    match value {
      candid::types::TypeInner::Null => TypeInner::Null,
      candid::types::TypeInner::Bool => TypeInner::Bool,
      candid::types::TypeInner::Nat => TypeInner::Nat,
      candid::types::TypeInner::Int => TypeInner::Int,
      candid::types::TypeInner::Nat8 => TypeInner::Nat8,
      candid::types::TypeInner::Nat16 => TypeInner::Nat16,
      candid::types::TypeInner::Nat32 => TypeInner::Nat32,
      candid::types::TypeInner::Nat64 => TypeInner::Nat64,
      candid::types::TypeInner::Int8 => TypeInner::Int8,
      candid::types::TypeInner::Int16 => TypeInner::Int16,
      candid::types::TypeInner::Int32 => TypeInner::Int32,
      candid::types::TypeInner::Int64 => TypeInner::Int64,
      candid::types::TypeInner::Float32 => TypeInner::Float32,
      candid::types::TypeInner::Float64 => TypeInner::Float64,
      candid::types::TypeInner::Text => TypeInner::Text,
      candid::types::TypeInner::Reserved => TypeInner::Reserved,
      candid::types::TypeInner::Empty => TypeInner::Empty,
      candid::types::TypeInner::Knot(arg1) => TypeInner::Knot(arg1.into()),
      candid::types::TypeInner::Var(arg1) => TypeInner::Var(arg1.into()),
      candid::types::TypeInner::Unknown => TypeInner::Unknown,
      candid::types::TypeInner::Opt(arg1) => TypeInner::Opt(Box::new(arg1.into())),
      candid::types::TypeInner::Vec(arg1) => TypeInner::Vec(Box::new(arg1.into())),
      candid::types::TypeInner::Record(arg1) => {
        TypeInner::Record(arg1.iter().map(|v| v.into()).collect())
      },
      candid::types::TypeInner::Variant(arg1) => {
        TypeInner::Variant(arg1.iter().map(|v| v.into()).collect())
      },
      candid::types::TypeInner::Func(arg1) => TypeInner::Func(arg1.into()),
      candid::types::TypeInner::Service(arg1) => TypeInner::Service(
        arg1
          .iter()
          .map(|(v1, v2)| (v1.to_owned(), Box::new(v2.into())))
          .collect(),
      ),
      candid::types::TypeInner::Class(arg1, arg2) => TypeInner::Class(
        arg1.iter().map(|x| Box::new(x.into())).collect(),
        Box::new(arg2.into()),
      ),
      candid::types::TypeInner::Principal => TypeInner::Principal,
      candid::types::TypeInner::Future => TypeInner::Future,
    }
  }
}
