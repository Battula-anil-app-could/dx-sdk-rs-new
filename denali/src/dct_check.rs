use super::*;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum CheckDct {
    Star,
    Equal(BTreeMap<BytesKey, CheckValue<BigUintValue>>),
}

impl InterpretableFrom<CheckDctRaw> for CheckDct {
    fn interpret_from(from: CheckDctRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckDctRaw::Unspecified => CheckDct::Equal(BTreeMap::new()),
            CheckDctRaw::Star => CheckDct::Star,
            CheckDctRaw::Equal(m) => CheckDct::Equal(
                m.into_iter()
                    .map(|(k, v)| {
                        (
                            BytesKey::interpret_from(k, context),
                            CheckValue::<BigUintValue>::interpret_from(v, context),
                        )
                    })
                    .collect(),
            ),
        }
    }
}

impl CheckDct {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckDct::Star)
    }
}
