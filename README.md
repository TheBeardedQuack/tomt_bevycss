[![TOMT_BevyCSS][30]][34]
[![BevyECSS][40]][41]
[![Bevy][50]][51]

[![MIT/Apache 2.0][1]][35]
[![Realease Doc][2]][32]
[![Rust][36]][37]
[![Crate][3]][31]
[![Bevy tracking][4]][54]

# TOMT BevyCSS

## What is TOMT BevyCSS?

TOMT BevyCSS is a fork project derived from [Bevy ECSS][41], which allows the usage of a subset of [`CSS`][81] to interact with [`bevy_ecs`][53]. It's mainly aimed to apply styling on [`bevy_ui`][52] but it can be used by any component by implementing custom properties.

The primary goals for this project fork are as follows:
- Provide more frequent updates for downstream crates
- Improve upon and expand the existing documentation, tests and examples for downstream users
- Expand on the feature set currently offered by BevyECSS

### Package name?

- `TOMT` = Tome of Many Things
- `BevyCSS` = Bevy + CSS, This library brings CSS features into the Bevy UI

### Who is TOMT (Tome of Many Things)?

Tome of Many Things is a small friendship group with various internet projects, primarily focussing on the [Tome of Many Things YouTube][21] channel.

This library project is brought to you by TheBeardedQuack ([GitHub][33], [YouTube][20]), forked from BevyECSS, and released under the same MIT and Apahe v2.0 licences to allow for continued freedom of use, modification and distribution.

This crate marks the first public release from both Tome of Many Things.

## Changes from BevyECSS

<table>
<tr>
    <th>Version</th>
    <th>Changes</th>
</tr>
<tr>
    <td>0.4.0</td>
    <td><ul>
        <li>Fixes<ul>
            <li>Respects CSS rule ordering</li>
        </ul></li>
        <li>Features<ul>
            <li>Support for pseudo-classes<ul>
                <li>Support for <code>:hover</code> and <code>:click</code> on Interaction component</li>
                <li>No additional pseudo-classes implemented at this time</li>
            </ul></li>
            <li>Placeholder support for pseudo-properties<ul>
                <li>No pseudo-properties implemented at this time</li>
            </ul></li>
            <li>Reapply styles based on ECS changes (add/modify of UI node)</li>
            <li>Additional contexts for <code>auto</code> keyword</li>
        </ul></li>
    </ul></td>
</tr>
</table>

### Future Goals
- Backport fixes into BevyECSS
- Optimise `prepare` system node tree
- Higher level language support (LESS / SCSS / SASS)
- Add CSS color functions

# Getting Started (Usage)

To use TOMT_BevyCSS just add a `StyleSheet` with a loaded `css` file to any entity and all style sheet rules will be applied to the entity and _all_ its [`descendants`][80] (children of children of children and so on).

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

To view the currently supported CSS features, please see [CSS Support][25] (docs/css-support.md).

## Custom Component Selector

TOMT_BevyCSS supports custom component selectors so you're not limited to components provided by bevy.

Should you need a core bevy component available as a selector that is not currently supported, this feature can also be used to roll your own support.
Should you decide to roll your own selector for any bevy built-in componenets, pull requests will be appreciated.

```rust
app.register_component_selector::<MyComponent>("selectorname");
```

See [Custom Components][26] (docs/custom-components.md) for more information.

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

See [Custom Properties][27] (docs/custom-properties.md) for more information.


# Bevy support table
| bevy  | tomt_bevycss |
| :---: | :----------: |
|  0.8  |     0.1      |
|  0.9  |     0.2      |
|  0.10 |     0.3      |
|  0.10 |    0.4.x     |


# Contributing

Got some idea, feedback, question or found a bug? Feel free to open an issue at any time!

# License

TOMT_BevyCSS is dual-licensed under either:

* MIT License (in [repository](10) or from [source][11])
* Apache License, Version 2.0 (in [repository][12] or from [source][13])

This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons][55] to include both.

<!-- Icons -->
[1]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[2]: https://docs.rs/tomt_bevycss/badge.svg
[3]: https://img.shields.io/crates/v/tomt_bevycss.svg
[4]: https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue

<!-- Licenses -->
[10]: LICENSE-MIT
[11]: http://opensource.org/licenses/MIT
[12]: LICENSE-APACHE
[12]: http://www.apache.org/licenses/LICENSE-2.0

<!-- Author Details -->
[20]: https://www.youtube.com/@TheBeardedQuack
[21]: https://www.youtube.com/TomeOfManyThings

<!-- More Readme -->
[25]: https://github.com/TheBeardedQuack/tomt_bevycss/blob/main/docs/css-support.md
[26]: https://github.com/TheBeardedQuack/tomt_bevycss/blob/main/docs/custom-components.md
[27]: https://github.com/TheBeardedQuack/tomt_bevycss/blob/main/docs/custom-properties.md

<!-- TOMT_BevyCSS -->
[30]: assets/branding/tomt_bevycss.png
[31]: https://crates.io/crates/tomt_bevycss
[32]: https://docs.rs/tomt_bevycss
[33]: https://github.com/TheBeardedQuack
[34]: https://github.com/TheBeardedQuack/tomt_bevycss
[35]: https://github.com/TheBeardedQuack/tomt_bevycss#license
[36]: https://github.com/TheBeardedQuack/tomt_bevycss/workflows/CI/badge.svg
[37]: https://github.com/TheBeardedQuack/tomt_bevycss/actions

<!-- BevyECSS crate -->
[40]: assets/branding/bevy_ecss.png
[41]: https://github.com/afonsolage/bevy_ecss

<!-- Bevy Engine -->
[50]: assets/branding/bevy_logo_dark_big.png
[51]: https://bevyengine.org/
[52]: https://crates.io/crates/bevy
[53]: https://crates.io/crates/bevy_ecs
[54]: https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking
[55]: https://github.com/bevyengine/bevy/issues/2373

<!-- Relevent Help Pages -->
[80]: https://stackoverflow.com/questions/1182189/css-child-vs-descendant-selectors
[81]: https://developer.mozilla.org/en-US/docs/Web/CSS
