# Custom Property Setters

TOMT_bevycss supports the ability to register your own properties so they can be set by the provided CSS rule.

## Custom Property - Example
Let's implement a custom `alpha` property with will set the alpha channel of any [`BackgroundColor`](https://docs.rs/bevy/latest/bevy/prelude/struct.BackgroundColor.html).

```rust
use bevy::{
   ecs::query::QueryItem,
   prelude::*
};
use tomt_bevycss::prelude::*;

#[derive(Default)]
pub(crate) struct AlphaProperty;

impl Property for AlphaProperty {
    // This is the cached value to be used when applying the property value.
    // It is evaluated only on the first time and futures runs are cached for performance reasons.
    type Cache = f32;
    
    // Which components the property needs when applying the cached value.
    // It is the same as using bevy_ecs Query<C, F>.
    type Components = (
        Option<&'static mut BackgroundColor>,
        Option<&'static mut UiImage>,
    );
    
    // If this property can be set only when there is another property, it's possible to filter here.
    // It's not recommended to use only With<> and Without<>.
    type Filters = ();

    fn name() -> &'static str {
        // The name of property. prefer kebab-case for consistency.
        "alpha"
    }

    fn parse<'a>(values: &PropertyValues) -> Result<Self::Cache, BevyCssError> {
        // PropertyValues::f32 tries to parse property value into a numeric value
        if let Some(value) = values.f32() {
            Ok(value)
        } else {
            Err(BevyCssError::InvalidPropertyValue(Self::name().to_string()))
        }
    }

    // This function will be called for every entity matched on every rule selector.
    fn apply<'w>(
        cache: &Self::Cache,
        (bg, img): QueryItem<Self::Components>,
        _asset_server: &AssetServer,
        _commands: &mut Commands,
    ) {
        if let Some(mut img) = img {
            img.color.set_alpha(*cache);
        } else if let Some(mut bg) = bg {
            bg.0.set_alpha(*cache);
        }
    }
}
```

Now just register the property on your `App` instance:
```rust ignore
app.register_property::<AlphaProperty>();
```

Done!

Whenever an `alpha` property is found on any `css` file, the `AlphaProperty` will be applied.
You can find this full example [`here`](https://github.com/TheBeardedQuack/tomt_bevycss/blob/main/examples/alpha.rs).
