mod types;

extern crate wasm_bindgen;
extern crate wee_alloc;

use candid_parser::{candid, check_prog, parse_idl_args, IDLArgs, TypeEnv};
use gloo_utils::format::JsValueSerdeExt;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Candid {
  env: TypeEnv,
  actor: candid::types::Type,
}

#[wasm_bindgen]
impl Candid {
  #[wasm_bindgen(js_name = encodeIdlArgs)]
  pub fn encode_idl_args(
    &self,
    method_name: &str,
    idl_args: &str,
  ) -> Result<Vec<u8>, JsError> {
    let parsed_args = parse_idl_args(idl_args)?;
    let method = self.env.get_method(&self.actor, method_name)?;

    Ok(parsed_args.to_bytes_with_types(&self.env, &method.args)?)
  }

  #[wasm_bindgen(js_name = decodeIdlArgs)]
  pub fn decode_idl_args(
    &self,
    method_name: &str,
    encoded: Vec<u8>,
  ) -> Result<String, JsError> {
    let parsed_args = parse_idl_args(&IDLArgs::from_bytes(&encoded)?.to_string())?;
    let method = self.env.get_method(&self.actor, method_name)?;

    Ok(
      parsed_args
        .annotate_types(true, &TypeEnv::new(), &method.args)?
        .to_string(),
    )
  }

  #[wasm_bindgen(js_name = getFunctions)]
  pub fn get_functions(&self) -> Result<JsValue, JsError> {
    let mut types: BTreeMap<String, types::Function> = BTreeMap::new();

    for (key, value) in self.env.as_service(&self.actor)?.iter() {
      if let Ok(func_type) = self.env.as_func(value) {
        types.insert(key.to_owned(), types::Function::from(func_type));
      }
    }

    Ok(JsValue::from_serde(&types)?)
  }

  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<String, JsError> {
    let mut types: BTreeMap<String, types::TypeInner> = BTreeMap::new();
    types.insert("Service".to_owned(), types::TypeInner::from(&self.actor));

    self.env.0.iter().for_each(|(key, value)| {
      types.insert(key.to_owned(), types::TypeInner::from(value));
    });

    Ok(serde_json::to_string(&types)?)
  }
}

#[wasm_bindgen(js_name = parseCandid)]
pub fn parse_candid_file(source: &str) -> Result<Candid, JsError> {
  let mut env = TypeEnv::new();
  let actor = check_prog(&mut env, &source.parse()?)?.unwrap();

  Ok(Candid { env, actor })
}
