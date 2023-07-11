use crate::{
    prelude::{
        Class,
        StyleSheet,
    },
    system::{
        self,
        ComponentFilterRegistry,
        PrepareParams,
    },
    property::{
        self,
        StyleSheetState,
    },
    stylesheet::{
        StyleSheetAsset,
        StyleSheetLoader,
    },
    RegisterProperty,
    RegisterComponentSelector,
};

use bevy::prelude::{
    Button, Interaction,
    BackgroundColor, Style,
    Node, Text, UiImage,
    PreUpdate, PostUpdate,
};

/// Plugin which add all types, assets, systems and internal resources needed by `tomt_bevycss`.
/// You must add this plugin in order to use `tomt_bevycss`.
#[derive(Default)]
pub struct BevyCssPlugin
{
    hot_reload: bool,
}

impl BevyCssPlugin
{
    pub fn with_hot_reload() -> BevyCssPlugin {
        BevyCssPlugin { hot_reload: true }
    }

    fn register_component_selector(
        app: &mut bevy::prelude::App
    ) {
        app.register_component_selector::<BackgroundColor>("background-color");
        app.register_component_selector::<Text>("text");
        app.register_component_selector::<Button>("button");
        app.register_component_selector::<Node>("node");
        app.register_component_selector::<Style>("style");
        app.register_component_selector::<UiImage>("ui-image");
        app.register_component_selector::<Interaction>("interaction");
    }

    fn register_properties(
        app: &mut bevy::prelude::App
    ) {
        use property::impls::style::*;
    
        app.register_property::<DisplayProperty>();
        app.register_property::<PositionTypeProperty>();
        app.register_property::<DirectionProperty>();
        app.register_property::<FlexDirectionProperty>();
        app.register_property::<FlexWrapProperty>();
        app.register_property::<AlignItemsProperty>();
        app.register_property::<AlignSelfProperty>();
        app.register_property::<AlignContentProperty>();
        app.register_property::<JustifyContentProperty>();
        app.register_property::<OverflowXProperty>();
        app.register_property::<OverflowYProperty>();
    
        app.register_property::<LeftProperty>();
        app.register_property::<RightProperty>();
        app.register_property::<TopProperty>();
        app.register_property::<BottomProperty>();
        app.register_property::<WidthProperty>();
        app.register_property::<HeightProperty>();
        app.register_property::<MinWidthProperty>();
        app.register_property::<MinHeightProperty>();
        app.register_property::<MaxWidthProperty>();
        app.register_property::<MaxHeightProperty>();
        app.register_property::<FlexBasisProperty>();
        app.register_property::<FlexGrowProperty>();
        app.register_property::<FlexShrinkProperty>();
        app.register_property::<AspectRatioProperty>();
    
        app.register_property::<MarginProperty>();
        app.register_property::<PaddingProperty>();
        app.register_property::<BorderProperty>();
    
        {
            use property::text::*;

            app.register_property::<FontColorProperty>();
            app.register_property::<FontProperty>();
            app.register_property::<FontSizeProperty>();
            app.register_property::<TextAlignProperty>();
            app.register_property::<TextContentProperty>();
        }
        
        use property::impls::BackgroundColorProperty;
        app.register_property::<BackgroundColorProperty>();
    }
}

use bevy::{
    prelude::{
        AddAsset,
        Plugin,
        IntoSystemSetConfig,
    },
};

impl Plugin for BevyCssPlugin
{
    fn build(
        &self,
        app: &mut bevy::prelude::App
    ) {
        use system::sets::*;

        // Type registration
        app.register_type::<Class>()
            .register_type::<StyleSheet>();

        // Resources
        let prepared_state = PrepareParams::new(&mut app.world);
        app.add_asset::<StyleSheetAsset>()
            .init_asset_loader::<StyleSheetLoader>()
            .init_resource::<StyleSheetState>()
            .init_resource::<ComponentFilterRegistry>()
            .insert_resource(prepared_state);

        // Systems
        app.configure_set(PreUpdate, BevyCssSet::Prepare.before(BevyCssSet::Apply))
            .configure_set(PostUpdate, BevyCssSet::Cleanup)
            .add_systems(BevyCssSet::Prepare, system::prepare)
            .add_systems(BevyCssSet::Cleanup, system::clear_state);

        if self.hot_reload
        {
            app.configure_set(PostUpdate, BevyCssHotReload)
                .add_systems(BevyCssHotReload, system::hot_reload_style_sheets);
        }

        // CSS registrations
        Self::register_component_selector(app);
        Self::register_properties(app);
    }
}
