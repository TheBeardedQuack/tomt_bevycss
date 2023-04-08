use crate::selector::Selector;
use smallvec::SmallVec;
use bevy::{
    prelude::{
        Entity,
        Deref, DerefMut,
    },
    utils::HashMap,
};
/// Maps which entities was selected by a [`Selector`]
#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct SelectedEntities(
    HashMap<
        Selector,
        SmallVec<[Entity; 8]>
    >
);

impl SelectedEntities
{
    pub(crate) fn compile(
        &mut self
    ) {
        let mut inverted: HashMap::<Entity, Vec<Selector>> = HashMap::new();

        for (selector, entities) in self.iter_mut()
        {
            for entity in entities.into_iter()
            {
                inverted.entry(*entity)
                    .or_default()
                    .push(selector.clone());
            }
        }

        self.0 = Default::default();
        for (entity, styles) in inverted.iter_mut()
        {
            styles.sort();
            if let Some(style) = styles.last()
            {
                self.entry(style.clone())
                    .or_default()
                    .push(*entity);
            }
        }
    }
}
