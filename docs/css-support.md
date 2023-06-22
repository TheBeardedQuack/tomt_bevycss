# CSS Support

TOMT_BevyCSS only supports a subset of CSS at moment, since many properties and features requires more advanced selectors, components and properties which aren't currently implemented.
The goal of this fork however, is to develop additional CSS comforts like additional selectors, and color functions, and possibly support for higher level styling languages such as LESS/SCSS/SASS.

## Component Selector Builtin

TOMT_BevyCSS provides the following component selectors:

|      Selector      |                                         Component                                         |
| :----------------: | :---------------------------------------------------------------------------------------: |
| `background-color` | [`BackgroundColor`](https://docs.rs/bevy/latest/bevy/prelude/struct.BackgroundColor.html) |
|       `text`       |             [`Text`](https://docs.rs/bevy/latest/bevy/text/struct.Text.html)              |
|      `button`      |          [`Button`](https://docs.rs/bevy/latest/bevy/prelude/struct.Button.html)          |
|       `node`       |            [`Node`](https://docs.rs/bevy/latest/bevy/prelude/struct.Node.html)            |
|      `style`       |           [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html)           |
|     `ui-image`     |         [`UiImage`](https://docs.rs/bevy/latest/bevy/prelude/struct.UiImage.html)         |
|   `interaction`    |      [`Interaction`](https://docs.rs/bevy/latest/bevy/prelude/enum.Interaction.html)      |

This list will be expanded to match `bevy_ui` and other `bevy` core components.

## PseudoClass Selector Builtin

TOMT_BevyCSS provides the following pseudo-class selector:

|       Selector        |                                       Component                                   |
| :-------------------: | :-------------------------------------------------------------------------------: |
| `:click` \| `:hover`  | [`Interaction`](https://docs.rs/bevy/latest/bevy/prelude/enum.Interaction.html)   |

This list will be exanpded as additional selectors are added into this library.

## Selectors

|    Type       | Details                                                                                                       | Example              |
| :-----------: | :------------------------------------------------------------------------------------------------------------ | :------------------- |
|   _Name_      | Selects by using `bevy` built-int [`Name`](https://docs.rs/bevy/latest/bevy/core/struct.Name.html) component. | `#inventory { ... }` |
|   _Class_     | Selects by using `Class` component, which is provided by TOMT_BevyCSS.                                        | `.enabled { ... }`   |
| _Component_   | Selects by using any component, but it has to be registered before usage. You can find more details bellow.   | `button { ... }`     |
| _PseudoClass_ | Selects by using an internal list of known rules run over ECS hierarchy.                                      | `:hover { ... }`     |

You may combine any of the above selector types to create a complex selector. For instance, `window.enabled.pop-up` select all `window` componenets, which contain the `enabled` and `pop-up` classes. The same rules of [`CSS Class selectors`](https://developer.mozilla.org/en-US/docs/Web/CSS/Class_selectors) applies here. 

_This assumes that `window` is a `bevy_ecs` component and was registered before usage. Also assumes the entities has the `Class` component with at least `enabled pop-up` class name._

Aditionally, TOMT_BevyCSS also supports [`descendant combinator`](https://developer.mozilla.org/en-US/docs/Web/CSS/Descendant_combinator) which selects _all_ entities that are descendant the given selector tree.

```css
#quest-window text {
    color: red;
}
```

The above rule will match _all_ entities which has a [`Text`](https://docs.rs/bevy/latest/bevy/text/struct.Text.html) component and is descendant of any entity which as a [`Name`](https://docs.rs/bevy/latest/bevy/core/struct.Name.html) component which the value of `quest-window`.

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
|  <`area-short-hand`>   | Allows the [`short hand area constructor`](https://developer.mozilla.org/en-US/docs/Web/CSS/margin#syntax) by using either dimensions or percentage, like `10px` or `5% 10px 3% auto`. No global values are supported yet |

Below details the supported properties for a number of Bevy built-in components.

Note that these are properties which are provived by TOMT_BevyCSS but you can also add your own properties at anytime, see [Custom Properties](./custom-properties.md)

### [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) Properties

|     Property      |                                            Values                                             | Description                                                                                                                                                                                                                                                               |
| :---------------: | :-------------------------------------------------------------------------------------------: | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
|     `display`     |                                       `flex` \| `none`                                        | Applies the  `display`         property on [`display`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.display) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.                 |
|  `position-type`  |                                   `absolute` \| `relative`                                    | Applies the  `position-type`   property on [`position_type`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.position_type) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.     |
|    `direction`    |                        `inherit` \| `left-to-right` \| `right-to-left`                        | Applies the  `direction`       property on [`direction`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.direction) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.             |
| `flex-direction`  |                    `row` \| `column` \| `row-reverse` \| `column-reverse`                     | Applies the  `flex-direction`  property on [`flex_direction`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_direction) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.   |
|    `flex-wrap`    |                             `no-wrap` \| `wrap` \| `wrap-reverse`                             | Applies the  `flex-wrap`       property on [`flex_wrap`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_wrap) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.             |
|   `align-items`   |               `flex-start` \| `flex-end` \| `center` \| `baseline` \| `stretch`               | Applies the  `align-items`     property on [`align_items`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.align_items) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.         |
|   `align-self`    |          `auto` \| `flex-start` \| `flex-end` \| `center` \| `baseline` \| `stretch`          | Applies the  `align-self`      property on [`align_self`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.align_self) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.           |
|  `align-content`  |   `flex-start` \| `flex-end` \| `center` \| `stretch` \| `space-between` \| `space-around`    | Applies the  `align-content`   property on [`align_content`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.align_content) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.     |
| `justify-content` | `flex-start` \| `flex-end` \| `center` \| `space-between` \| `space-around` \| `space-evenly` | Applies the  `justify-content` property on [`justify_content`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.justify_content) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components. |
|    `overflow`     |                                     `visible` \| `hidden`                                     | Applies the  `overflow`       property on [`overflow`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.overflow) field of all sections on matched [`Style`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html) components.                |
|      `left`       |                                     `00.00%` \| `00.00px`                                     | Applies the             property on [`position.left`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.position) field of all matched components.                                                                                                   |
|      `right`      |                                     `00.00%` \| `00.00px`                                     | Applies the             property on [`position.right`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.position) field of all matched components.                                                                                                  |
|       `top`       |                                     `00.00%` \| `00.00px`                                     | Applies the             property on [`position.top`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.position) field of all matched components.                                                                                                    |
|     `bottom`      |                                     `00.00%` \| `00.00px`                                     | Applies the             property on [`position.bottom`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.position) field of all matched components.                                                                                                 |
|      `width`      |                                     `00.00%` \| `00.00px`                                     | Applies the             property on [`size.width`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.size) field of all matched components.                                                                                                          |
|     `height`      |                                     `00.00%` \| `00.00px`                                     | Applies the             property on [`size.height`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.size) field of all matched components.                                                                                                         |
|    `min-width`    |                                     `00.00%` \| `00.00px`                                     | Applies the             property on [`min_size.width`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.min_size) field of all matched components.                                                                                                  |
|   `min-height`    |                                     `00.00%` \| `00.00px`                                     | Applies the             property on [`min_size.height`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.min_size) field of all matched components.                                                                                                 |
|    `max-width`    |                                     `00.00%` \| `00.00px`                                     | Applies the             property on [`max_size.width`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.max_size) field of all matched components.                                                                                                  |
|   `max-height`    |                                     `00.00%` \| `00.00px`                                     | Applies the             property on [`max_size.height`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.max_size) field of all matched components.                                                                                                 |
|   `flex-basis`    |                                     `00.00%` \| `00.00px`                                     | Applies the             property on [`max_size.height`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.max_size) field of all matched components.                                                                                                 |
|    `flex-grow`    |                                       `0` \| `1` \| `2`                                       | Applies the             property on [`flex_grow`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_grow)   field of all matched components.                                                                                                    |
|   `flex-shrink`   |                                       `0` \| `1` \| `2`                                       | Applies the             property on [`flex_shrink`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.flex_shrink) field of all matched components.                                                                                                  |
|  `aspect-ratio`   |                                       `00.00` \| `none`                                       | Applies the             property on [`aspect_ratio`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.aspect_ratio) field of all matched components.                                                                                                |
|     `margin`      |                                      <`area-short-hand`>                                      | Applies the             property on [`margin`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.margin) field of all matched components.                                                                                                            |
|     `padding`     |                                      <`area-short-hand`>                                      | Applies the             property on [`padding`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.padding) field of all matched components.                                                                                                          |
|     `border`      |                                      <`area-short-hand`>                                      | Applies the             property on [`border`](https://docs.rs/bevy/latest/bevy/prelude/struct.Style.html#structfield.border) field of all matched components.                                                                                                            |

### [`Text`](https://docs.rs/bevy/latest/bevy/prelude/struct.Text.html) Properties

|     Property     |                                                                            Values                                                                            | Description                                                                                                                                                                                                                             |
| :--------------: | :----------------------------------------------------------------------------------------------------------------------------------------------------------: | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|     `color`      | [`named-colors`](https://developer.mozilla.org/en-US/docs/Web/CSS/named-color) \| [`hex_colors`](https://developer.mozilla.org/en-US/docs/Web/CSS/hex-color) | Applies the property on [`style.color`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html#structfield.style) for all [`sections`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html) of matched components.     |
|                  |
|      `font`      |                                                                     `"path/to/font.ttf"`                                                                     | Applies the property on [`style.font`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html#structfield.style) for all [`sections`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html) of matched components.      |
|                  |
|   `font-size`    |                                                                           `00.00`                                                                            | Applies the property on [`style.font_size`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html#structfield.style) for all [`sections`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html) of matched components. |
|                  |
|  `text-content`  |                                                                     `"Some text value"`                                                                      | Applies the property on [`value`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html#structfield.value) for all [`sections`](https://docs.rs/bevy/latest/bevy/text/struct.TextSection.html) of matched components.           |
|                  |
|   `text-align`   |                                                                `left` \| `center` \| `right`                                                                 | Applies the property on [`alignment`](https://docs.rs/bevy/latest/bevy/text/struct.Text.html#structfield.alignment) of all matched components.                                                                                          |
|                  |

### Component Properties

|      Property      |                                                                            Values                                                                            | Description                                                                                                                                  |
| :----------------: | :----------------------------------------------------------------------------------------------------------------------------------------------------------: | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `background-color` | [`named-colors`](https://developer.mozilla.org/en-US/docs/Web/CSS/named-color) \| [`hex_colors`](https://developer.mozilla.org/en-US/docs/Web/CSS/hex-color) | Applies the property on [`BackgroundColor`](https://docs.rs/bevy/latest/bevy/prelude/struct.BackgroundColor.html) of all matched components. |
