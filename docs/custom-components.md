# Custom Component Selectors

TOMT_bevycss supports the ability to register your own components so they're visible to the CSS selector system.

## Custom Component - Example
Simply call `register_component_selector::<MyCustomComponent>("my-custom-component")` on your `App` instance.

```rust
use bevy::prelude::*;
use tomt_bevycss::prelude::*;

#[derive(Component)]
struct MyFancyComponentSelector;

#[derive(Component)]
struct FancyColor;

fn some_main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugin(BevyCssPlugin::default());

    // Register your own custom selector:
    // fancy-pants {
    //      background-color: pink;
    // }
    app.register_component_selector::<MyFancyComponentSelector>("fancy-pants");

    // Or you override the default implementation for an existing component selector.
    app.register_component_selector::<FancyColor>("background-color");
}
```

## Clean Code - Example

A simple way to keep the styling code separated from core gameplay code, would be to place the initialization within its own plugin.

```rust
use bevy::prelude::*;
use tomt_bevycss::prelude::*;

struct StylePlugin;

impl Plugin for StylePlugin
{
    fn build(
        app: &mut App
    ) {
        app.register_component_selector::<Health>();
        app.register_component_selector::<Mana>();

        // Now would also be a good time to load any stylesheets we'll need immediately
        // Assuming asset plugin has been initialized before us
    }
}

fn main()
{
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins)
        .add_plugin(StylePlugin)
        .add_plugin(MainGamePlugin);
    
    app.run();
}
```
