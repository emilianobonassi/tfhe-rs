use crate::boolean::client_key::ClientKey;

use serde::{Deserialize, Serialize};

use super::{parameters::BooleanParameterSet, FheBoolParameters, types::r#static::StaticBoolParameters};

 #[cfg_attr(doc, cfg(feature = "boolean"))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenericBoolClientKey<P>
where
    P: BooleanParameterSet,
{
    pub(in crate::typed_api::booleans) key: ClientKey,
    _marker: std::marker::PhantomData<P>,
}

impl From<FheBoolParameters> for GenericBoolClientKey<StaticBoolParameters>
{
    fn from(parameters: FheBoolParameters) -> Self {
        Self {
            key: ClientKey::new(&parameters.into()),
            _marker: Default::default(),
        }
    }
}
