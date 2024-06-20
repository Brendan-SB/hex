use crate::{
    components::{Camera, Sprite, Trans},
    renderer_manager::Draw,
    renderer_manager::Renderer,
    ComponentManager, Context, EntityManager,
};
use std::sync::{Arc, RwLock};

pub struct SpriteRenderer;

impl Renderer for SpriteRenderer {
    fn draw(
        &mut self,
        draw: &mut Draw,
        context: Arc<RwLock<Context>>,
        em: Arc<RwLock<EntityManager>>,
        cm: Arc<RwLock<ComponentManager>>,
    ) -> anyhow::Result<()> {
        let context = context.read().unwrap();
        let em = em.read().unwrap();
        let cm = cm.read().unwrap();

        if let Some((c, ct)) = em.entities().keys().cloned().find_map(|e| {
            Some((
                cm.get::<Camera>(e)
                    .and_then(|c| c.read().unwrap().active.then_some(c))?,
                cm.get::<Trans>(e)
                    .and_then(|t| t.read().unwrap().active.then_some(t))?,
            ))
        }) {
            let sprites = {
                let mut sprites: Vec<_> = em
                    .entities()
                    .keys()
                    .cloned()
                    .filter_map(|e| {
                        Some((
                            cm.get::<Sprite>(e)
                                .and_then(|s| s.read().unwrap().active.then_some(s))?,
                            cm.get::<Trans>(e)
                                .and_then(|t| t.read().unwrap().active.then_some(t))?,
                        ))
                    })
                    .collect();

                sprites.sort_by(|(s1, _), (s2, _)| {
                    s1.read().unwrap().layer.cmp(&s2.read().unwrap().layer)
                });

                sprites
            };

            for (s, t) in sprites {
                let d = {
                    let s = s.write().unwrap();

                    s.drawable.clone()
                };

                d.draw(
                    s.clone(),
                    t.clone(),
                    (c.clone(), ct.clone()),
                    &context,
                    draw,
                )?;
            }
        }

        Ok(())
    }
}