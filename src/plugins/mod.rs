//! Plugins
//!
//! Any entity located directly in this module is [`Plugin`](bevy::app::Plugin).

use std::sync::Arc;

use crate::{
    assets::{
        bundle::{BundleAssetLoader, ConcurrentFluentBundle},
        resource::ResourceAssetLoader,
    },
    BundleAsset, ResourceAsset,
};
use bevy::{app::PluginGroupBuilder, prelude::*};

/// Adds support for Fluent file loading to applications
pub struct FluentPlugin {
    pub customize_bundle: Arc<dyn Fn(&mut ConcurrentFluentBundle) + Send + Sync + 'static>,
}

impl FluentPlugin {
    pub fn new(
        customize_bundle: impl Fn(&mut ConcurrentFluentBundle) + Send + Sync + 'static,
    ) -> Self {
        Self {
            customize_bundle: Arc::new(customize_bundle),
        }
    }
}

#[derive(Default)]
pub struct DefaultFluentPlugins;

impl PluginGroup for DefaultFluentPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().set(FluentPlugin::new(|_| {}))
    }
}

impl Plugin for FluentPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(ResourceAssetLoader)
            .init_asset::<ResourceAsset>()
            .register_asset_loader(BundleAssetLoader {
                customize_bundle: self.customize_bundle.clone(),
            })
            .init_asset::<BundleAsset>();
    }
}
