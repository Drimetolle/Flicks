use std::error::Error;
use async_trait::async_trait;
use crate::image::Image;

#[async_trait]
pub trait AvatarChangeStage {
    async fn change_avatar(&self, image: &Image) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait AvatarProvider {
    async fn get(&self) -> Option<Image>;
}

pub struct AvatarPipeline<'a> {
    pub(crate) avatar_provider: &'a dyn AvatarProvider,
    pub(crate) stages: Vec<Box<dyn AvatarChangeStage>>,
}

impl<'a> AvatarPipeline<'a> {
    pub fn new(avatar_provider: &'a impl AvatarProvider) -> Self {
        AvatarPipeline {
            avatar_provider,
            stages: vec![],
        }
    }

    pub fn add(&mut self, stage: impl AvatarChangeStage + 'static) -> &mut Self {
        self.stages.push(Box::new(stage));

        self
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let image = self.avatar_provider.get().await.expect("image should exist");

        for stage in self.stages.iter() {
            stage.change_avatar(&image).await?;
        }

        Ok(())
    }
}