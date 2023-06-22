[![TOMT_BevyCSS][1]][11]
[![BevyECSS][2]][12]
[![Bevy][3]][13]

[![MIT/Apache 2.0][4]][14]
[![Realease Doc][5]][15]
[![Rust][6]][16]
[![Crate][7]][17]
[![Bevy tracking][8]][18]

# TOMT BevyCSS

## What is TOMT BevyCSS?

TOMT BevyCSS is a fork project derived from [Bevy ECSS][12] is a crate which allows the usage of a subset of [`CSS`][20] to interact with [`bevy_ecs`][21]. It's mainly aimed to apply styling on [`bevy_ui`][22] but it can be used by any component by implementing custom properties.

### Package name?

- `TOMT` = Tome of Many Things
- `BevyCSS` = Bevy + CSS, This library brings CSS features into the Bevy UI

### Who is TOMT (Tome of Many Things)?

Tome of Many Things is a small friendship group with various internet projects, primarily focussing on the [Tome of Many Things YouTube][26] channel.

This library project is brought to you by TheBeardedQuack ([GitHub][23], [YouTube][24]). Forked from BevyECSS, and released under the same MIT and Apahe v2.0 licences to allow for continued freedom of use, modification and distribution.

# Usage

To use TOMT_BevyCSS just add a `StyleSheet` with a loaded `css` file to any entity and all style sheet rules will be applied to the entity and _all_ its [`descendants`][25] (children of children of children and so on).

```rust
use bevy::prelude::*;
use tomt_bevycss::prelude::*;

fn setup_awesome_ui(root: Entity, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .entity(root)
        .insert(StyleSheet::new(asset_server.load("sheets/awesome.css")));
}
```

That's it, now your UI will indeed look _awesome_!


# CSS Support

This crate aims to support the most common CSS features first, and add more over time as suggestions are provided.
This for now means that only a subset of CSS features are provided.

To view the currently supported CSS features, please see [CSS Support](docs/css-support.md) (docs/css-support.md).

## Custom Component Selector

TOMT_BevyCSS supports custom component selectors so you're not limited to components provided by bevy.

Should you need a core bevy component available as a selector that is not currently supported, this feature can also be used to roll your own support.
Should you decide to roll your own selector for any bevy built-in componenets, pull requests will be appreciated.

```rust
app.register_component_selector::<MyComponent>("selectorname");
```

See [Custom Components](docs/custom-components.md) (docs/custom-components.md) for more information.

## Custom Property

TOMT_BevyCSS supports custom property setters.

This with the above custom component select should enable you to use the CSS system for more than just styling if you so desired.

```rust
#[derive(Default)]
pub(crate) struct MyProperty;

impl Property for MyProperty
{
    /* For implementation details please see:
       https://github.com/TheBeardedQuack/tomt_bevycss/tree/main/docs/custom-properties.md
    */
}

fn main()
{
    let mut app = App::new();
    app.register_property::<MyProperty>();
}
```

See [Custom Properties](docs/custom-properties.md) (docs/custom-properties.md) for more information.


# Bevy support table
| bevy  | tomt_bevycss |
| :---: | :-------: |
|  0.8  |    0.1    |
|  0.9  |    0.2    |
|  0.10 |    0.3    |
|  0.10 |   0.4.x   |


# Contributing

Got some idea, feedback, question or found a bug? Feel free to open an issue at any time!

# License

TOMT_BevyCSS is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.



[1]: assets/branding/tomt_bevycss.png
[2]: assets/branding/bevy_ecss.png
[3]: assets/branding/bevy_logo_dark_big.png
[4]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[5]: https://docs.rs/tomt_bevycss/badge.svg
[6]: https://github.com/TheBeardedQuack/tomt_bevycss/workflows/CI/badge.svg
[7]: https://img.shields.io/crates/v/tomt_bevycss.svg
[8]: https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue

[11]: https://github.com/TheBeardedQuack/tomt_bevycss
[12]: https://github.com/afonsolage/bevy_ecss
[13]: https://bevyengine.org/
[14]: https://github.com/TheBeardedQuack/tomt_bevycss#license
[15]: https://docs.rs/tomt_bevycss
[16]: https://github.com/TheBeardedQuack/tomt_bevycss/actions
[17]: https://crates.io/crates/tomt_bevycss
[18]: https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking

[20]: https://developer.mozilla.org/en-US/docs/Web/CSS
[21]: https://crates.io/crates/bevy_ecs
[22]: https://crates.io/crates/bevy
[23]: https://github.com/TheBeardedQuack
[24]: https://www.youtube.com/@TheBeardedQuack
[25]: https://stackoverflow.com/questions/1182189/css-child-vs-descendant-selectors
[26]: https://youtube.com/tomeofmanythings
