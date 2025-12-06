# CSS Support

TOMT_BevyCSS only supports a subset of CSS at moment, since many properties and features requires more advanced selectors, components and properties which aren't currently implemented.
The goal of this fork however, is to develop additional CSS comforts like additional selectors, and color functions, and possibly support for higher level styling languages such as LESS/SCSS/SASS.

## Component Selector Builtin

TOMT_BevyCSS provides the following component selectors:

|      Selector      |      Component         |
| :----------------: | :--------------------: |
| `background-color` | [`BackgroundColor`][1] |
|       `text`       | [`Text`][2]            |
|      `button`      | [`Button`][3]          |
|       `node`       | [`Node`][4]            |
|      `style`       | [`Style`][5]           |
|     `ui-image`     | [`UiImage`][6]         |
|   `interaction`    | [`Interaction`][7]     |

This list will be expanded to match `bevy_ui` and other `bevy` core components.

## PseudoClass Selector Builtin

TOMT_BevyCSS provides the following pseudo-class selector:

|       Selector        |      Component       |
| :-------------------: | :------------------: |
| `:click` \| `:hover`  | [`Interaction`][7]   |

This list will be exanpded as additional selectors are added into this library.

## Selectors

|    Type       | Details                                                                                                       | Example              |
| :-----------: | :------------------------------------------------------------------------------------------------------------ | :------------------- |
|   _Name_      | Selects by using `bevy` built-int [`Name`][8] component.                                                      | `#inventory { ... }` |
|   _Class_     | Selects by using `Class` component, which is provided by TOMT_BevyCSS.                                        | `.enabled { ... }`   |
| _Component_   | Selects by using any component, but it has to be registered before usage. You can find more details bellow.   | `button { ... }`     |
| _PseudoClass_ | Selects by using an internal list of known rules run over ECS hierarchy.                                      | `:hover { ... }`     |

You may combine any of the above selector types to create a complex selector. For instance, `window.enabled.pop-up` select all `window` componenets, which contain the `enabled` and `pop-up` classes. The same rules of [`CSS Class selectors`][101] applies here. 

_This assumes that `window` is a `bevy_ecs` component and was registered before usage. Also assumes the entities has the `Class` component with at least `enabled pop-up` class name._

Aditionally, TOMT_BevyCSS also supports [`descendant combinator`][102] which selects _all_ entities that are descendant the given selector tree.

```css
#quest-window text {
    color: red;
}
```

The above rule will match _all_ entities which has a [`Text`][2] component and is descendant of any entity which as a [`Name`]() component which the value of `quest-window`.

So it's possible to combine complex composed selectors with descendant combinator.

```css
#main-menu button.enabled .border {
    background-color: #ff03ab;
}
```

This rule will match all components which has a `Class` with the value of `border` and are descendant of any entity which has a `button` component _and_ a `Class` component with the value of `enabled` and also are descendant of any entity which has a `Name` component with value `main-menu`.


## Properties

In the following section, property values will be displayed with the below syntax for brevity.
Hopefully this should serve as a good enough guide for the supported formats for each property value.

|        Notation        |     Description                                                                  |
| :--------------------: | :------------------------------------------------------------------------------- |
|        `00.00%`        | Any percent value, like `93%` or `4.45%`                                         |
|       `00.00px`        | Any dimensional value, like `11px` or `0.99px`                                   |
|        `00.00`         | Any number value, like `0` or `14.2`                                             |
| `<ident>` \| `<ident>` | Only one of the identifiers are allowed, without quotes, like `none` or `hidden` |
|  <`area-short-hand`>   | Allows the [`short hand area constructor`][100] by using either dimensions or percentage, like `10px` or `5% 10px 3% auto`. No global values are supported yet |

Below details the supported properties for a number of Bevy built-in components.

Note that these are properties which are provived by TOMT_BevyCSS but you can also add your own properties at anytime, see [Custom Properties][91]

### [`Style`][5] Properties

|     Property      |                                            Values                                             | Description                                                                                                                  |
|:-----------------:| :-------------------------------------------------------------------------------------------: | :----------------------------------------------------------------------------------------------------------------------------|
|     `display`     |                                       `flex` \| `none`                                        | Applies the  `display`         property on [`display`][20]         field of all sections on matched [`Style`][5] components. |
|  `position-type`  |                                   `absolute` \| `relative`                                    | Applies the  `position-type`   property on [`position_type`][21]   field of all sections on matched [`Style`][5] components. |
|    `direction`    |                        `inherit` \| `left-to-right` \| `right-to-left`                        | Applies the  `direction`       property on [`direction`][22]       field of all sections on matched [`Style`][5] components. |
| `flex-direction`  |                    `row` \| `column` \| `row-reverse` \| `column-reverse`                     | Applies the  `flex-direction`  property on [`flex_direction`][23]  field of all sections on matched [`Style`][5] components. |
|    `flex-wrap`    |                             `no-wrap` \| `wrap` \| `wrap-reverse`                             | Applies the  `flex-wrap`       property on [`flex_wrap`][24]       field of all sections on matched [`Style`][5] components. |
|   `align-items`   |               `flex-start` \| `flex-end` \| `center` \| `baseline` \| `stretch`               | Applies the  `align-items`     property on [`align_items`][25]     field of all sections on matched [`Style`][5] components. |
|   `align-self`    |          `auto` \| `flex-start` \| `flex-end` \| `center` \| `baseline` \| `stretch`          | Applies the  `align-self`      property on [`align_self`][26]      field of all sections on matched [`Style`][5] components. |
|  `align-content`  |   `flex-start` \| `flex-end` \| `center` \| `stretch` \| `space-between` \| `space-around`    | Applies the  `align-content`   property on [`align_content`][27]   field of all sections on matched [`Style`][5] components. |
| `justify-content` | `flex-start` \| `flex-end` \| `center` \| `space-between` \| `space-around` \| `space-evenly` | Applies the  `justify-content` property on [`justify_content`][28] field of all sections on matched [`Style`][5] components. |
|    `overflow`     |                                     `visible` \| `hidden`                                     | Applies the  `overflow`        property on [`overflow`][29]        field of all sections on matched [`Style`][5] components. |
|      `left`       |                                     `00.00%` \| `00.00px`                                     | Applies the property on [`position.left`][30]   field of all matched components.                                             |
|      `right`      |                                     `00.00%` \| `00.00px`                                     | Applies the property on [`position.right`][30]  field of all matched components.                                             |
|       `top`       |                                     `00.00%` \| `00.00px`                                     | Applies the property on [`position.top`][30]    field of all matched components.                                             |
|     `bottom`      |                                     `00.00%` \| `00.00px`                                     | Applies the property on [`position.bottom`][30] field of all matched components.                                             |
|      `width`      |                                     `00.00%` \| `00.00px`                                     | Applies the property on [`size.width`][31]      field of all matched components.                                             |
|     `height`      |                                     `00.00%` \| `00.00px`                                     | Applies the property on [`size.height`][31]     field of all matched components.                                             |
|    `min-width`    |                                     `00.00%` \| `00.00px`                                     | Applies the property on [`min_size.width`][32]  field of all matched components.                                             |
|   `min-height`    |                                     `00.00%` \| `00.00px`                                     | Applies the property on [`min_size.height`][32] field of all matched components.                                             |
|    `max-width`    |                                     `00.00%` \| `00.00px`                                     | Applies the property on [`max_size.width`][33]  field of all matched components.                                             |
|   `flex-basis`    |                                     `00.00%` \| `00.00px`                                     | Applies the property on [`max_size.height`][33] field of all matched components.                                             |
|   `max-height`    |                                     `00.00%` \| `00.00px`                                     | Applies the property on [`max_size.height`][33] field of all matched components.                                             |
|    `flex-grow`    |                                       `0` \| `1` \| `2`                                       | Applies the property on [`flex_grow`][34]       field of all matched components.                                             |
|   `flex-shrink`   |                                       `0` \| `1` \| `2`                                       | Applies the property on [`flex_shrink`][35]     field of all matched components.                                             |
|  `aspect-ratio`   |                                       `00.00` \| `none`                                       | Applies the property on [`aspect_ratio`][36]    field of all matched components.                                             |
|     `margin`      |                                      <`area-short-hand`>                                      | Applies the property on [`margin`][37]          field of all matched components.                                             |
|     `padding`     |                                      <`area-short-hand`>                                      | Applies the property on [`padding`][38]         field of all matched components.                                             |
|     `border`      |                                      <`area-short-hand`>                                      | Applies the property on [`border`][39]          field of all matched components.                                             |

### [`Text`][2] Properties

|     Property     |        Values                                |                    Description                                                                  |
| :--------------: | :------------------------------------------: | :---------------------------------------------------------------------------------------------- |
|     `color`      | [`named-colors`][103] \| [`hex_colors`][104] | Applies the property on [`style.color`][50]     for all [`sections`][51] of matched components. |
|      `font`      | `"path/to/font.ttf"`                         | Applies the property on [`style.font`][50]      for all [`sections`][51] of matched components. |
|   `font-size`    | `00.00`                                      | Applies the property on [`style.font_size`][50] for all [`sections`][51] of matched components. |
|  `text-content`  | `"Some text value"`                          | Applies the property on [`value`][52]           for all [`sections`][51] of matched components. |
|   `text-align`   | `left` \| `center` \| `right`                | Applies the property on [`alignment`][53] of all matched components.                            |

### Component Properties

|      Property      |                                Values                                | Description                                                               |
|:------------------:|:--------------------------------------------------------------------:|:--------------------------------------------------------------------------|
| `background-color` |             [`named-colors`][103] \| [`hex_colors`][104]             | Applies the property on [`BackgroundColor`][1] of all matched components. |
|  `border-radius`   |                         <`area-short-hand`>                          | Applies the property on [`BorderRadius`][9] of all matched components.    |
|   `border-color`   |             [`named-colors`][103] \| [`hex_colors`][104]             | Applies the property on [`BorderColor`][10] of all matched components.    |

[1]: https://docs.rs/bevy/latest/bevy/prelude/struct.BackgroundColor.html
[2]: https://docs.rs/bevy/latest/bevy/text/struct.Text.html
[3]: https://docs.rs/bevy/latest/bevy/prelude/struct.Button.html
[4]: https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html
[5]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html
[6]: https://docs.rs/bevy/latest/bevy/prelude/struct.UiImage.html
[7]: https://docs.rs/bevy/latest/bevy/prelude/enum.Interaction.html
[8]: https://docs.rs/bevy/latest/bevy/core/struct.Name.html
[9]: https://docs.rs/bevy/latest/bevy/prelude/struct.BorderRadius.html
[10]: https://docs.rs/bevy/latest/bevy/prelude/struct.BorderColor.html

[20]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.display
[21]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.position_type
[22]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.direction
[23]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_direction
[24]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_wrap
[25]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.align_items
[26]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.align_self
[27]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.align_content
[28]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.justify_content
[29]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.overflow
[30]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.position
[31]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.size
[32]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.min_size
[33]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.max_size
[34]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_grow
[35]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_shrink
[36]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.aspect_ratio
[37]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.margin
[38]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.padding
[39]: https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.border

[50]: https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html#structfield.style
[51]: https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html
[52]: https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html#structfield.value
[53]: https://docs.rs/bevy/latest/bevy/text/struct.Text.html#structfield.alignment

[91]: https://github.com/TheBeardedQuack/tomt_bevycss/blob/main/docs/custom-properties.md

[100]: https://developer.mozilla.org/en-US/docs/Web/CSS/margin#syntax
[101]: https://developer.mozilla.org/en-US/docs/Web/CSS/Class_selectors
[102]: https://developer.mozilla.org/en-US/docs/Web/CSS/Descendant_combinator
[103]: https://developer.mozilla.org/en-US/docs/Web/CSS/named-color
[104]: https://developer.mozilla.org/en-US/docs/Web/CSS/hex-color
