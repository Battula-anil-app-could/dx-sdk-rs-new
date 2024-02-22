use super::{TypeAbi, TypeDescriptionContainerImpl, TypeName};

#[derive(Clone, Debug)]
pub struct DctAttributeAbi {
    pub ticker: &'static str,
    pub ty: TypeName,
    pub type_descriptions: TypeDescriptionContainerImpl,
}

impl DctAttributeAbi {
    pub fn new<T: TypeAbi>(arg_name: &'static str) -> DctAttributeAbi {
        let mut type_descriptions = TypeDescriptionContainerImpl::default();
        T::provide_type_descriptions(&mut type_descriptions);
        DctAttributeAbi {
            ticker: arg_name,
            ty: T::type_name(),
            type_descriptions,
        }
    }
}
