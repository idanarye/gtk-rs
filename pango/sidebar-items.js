initSidebarItems({"constant":[["NONE_ENGINE_LANG",""],["NONE_ENGINE_SHAPE",""],["NONE_FONT",""],["NONE_FONTSET",""],["NONE_FONT_FACE",""],["NONE_FONT_FAMILY",""],["NONE_FONT_MAP",""],["NONE_RENDERER",""],["SCALE",""],["SCALE_LARGE","The scale factor for one magnification step (1.2)."],["SCALE_MEDIUM","The scale factor for normal size (1.0)."],["SCALE_SMALL","The scale factor for one shrinking step (1 / 1.2)."],["SCALE_XX_LARGE","The scale factor for three magnification steps (1.2 * 1.2 * 1.2)."],["SCALE_XX_SMALL","The scale factor for three shrinking steps (1 / (1.2 * 1.2 * 1.2))."],["SCALE_X_LARGE","The scale factor for two magnification steps (1.2 * 1.2)."],["SCALE_X_SMALL","The scale factor for two shrinking steps (1 / (1.2 * 1.2))."]],"enum":[["Alignment","A `Alignment` describes how to align the lines of a `Layout` within the available space. If the `Layout` is set to justify using `Layout::set_justify`, this only has effect for partial lines."],["AttrType","The `AttrType` distinguishes between different types of attributes. Along with the predefined values, it is possible to allocate additional values for custom attributes using `AttrType::register`. The predefined values are given below. The type of structure used to store the attribute is listed in parentheses after the description."],["BidiType","The `BidiType` type represents the bidirectional character type of a Unicode character as specified by the Unicode bidirectional algorithm`</ulink>`."],["CoverageLevel","Used to indicate how well a font can represent a particular Unicode character point for a particular script."],["Direction","The `Direction` type represents a direction in the Unicode bidirectional algorithm; not every value in this enumeration makes sense for every usage of `Direction`; for example, the return value of `pango_unichar_direction` and `pango_find_base_dir` cannot be `Direction::WeakLtr` or `Direction::WeakRtl`, since every character is either neutral or has a strong direction; on the other hand `Direction::Neutral` doesn’t make sense to pass to `pango_itemize_with_base_dir`."],["EllipsizeMode","The `EllipsizeMode` type describes what sort of (if any) ellipsization should be applied to a line of text. In the ellipsization process characters are removed from the text in order to make it fit to a given width and replaced with an ellipsis."],["Gravity",""],["GravityHint","The `GravityHint` defines how horizontal scripts should behave in a vertical context. That is, English excerpt in a vertical paragraph for example."],["Overline","The `Overline` enumeration is used to specify whether text should be overlined, and if so, the type of line."],["RenderPart","`RenderPart` defines different items to render for such purposes as setting colors."],["Script","The `Script` enumeration identifies different writing systems. The values correspond to the names as defined in the Unicode standard. See Unicode Standard Annex `24`: Script names`</ulink>`."],["Stretch","An enumeration specifying the width of the font relative to other designs within a family."],["Style","An enumeration specifying the various slant styles possible for a font."],["TabAlign","A `TabAlign` specifies where a tab stop appears relative to the text."],["Underline","The `Underline` enumeration is used to specify whether text should be underlined, and if so, the type of underlining."],["Variant","An enumeration specifying capitalization variant of the font."],["Weight","An enumeration specifying the weight (boldness) of a font. This is a numerical value ranging from 100 to 1000, but there are some predefined values:"],["WrapMode",""]],"fn":[["extents_to_pixels",""],["find_base_dir",""],["find_paragraph_boundary",""],["is_zero_width",""],["itemize",""],["itemize_with_base_dir",""],["parse_enum",""],["parse_markup",""],["parse_stretch",""],["parse_style",""],["parse_variant",""],["parse_weight",""],["quantize_line_geometry",""],["reorder_items",""],["shape",""],["shape_full",""],["shape_with_flags",""],["split_file_list",""],["trim_string",""],["unichar_direction",""],["units_from_double",""],["units_to_double",""],["version",""],["version_check",""],["version_string",""]],"mod":[["analysis",""],["attr_class",""],["attr_iterator",""],["attr_list",""],["attribute",""],["functions",""],["glyph",""],["item",""],["language",""],["layout",""],["prelude","Traits and essential types inteded for blanket imports."],["rectangle",""]],"static":[["ENGINE_TYPE_LANG",""],["ENGINE_TYPE_SHAPE",""],["RENDER_TYPE_NONE",""]],"struct":[["AttrIterator",""],["AttrList",""],["Attribute",""],["Color",""],["Context",""],["Coverage",""],["EngineLang",""],["EngineShape",""],["Font",""],["FontDescription",""],["FontFace",""],["FontFamily",""],["FontMap",""],["FontMask",""],["FontMetrics",""],["Fontset",""],["FontsetSimple",""],["GlyphItem",""],["GlyphItemIter",""],["GlyphString",""],["Item",""],["Layout",""],["LayoutIter",""],["LayoutLine",""],["Matrix",""],["Renderer",""],["ShapeFlags",""],["ShowFlags",""],["TabArray",""]],"trait":[["FontExt","Trait containing all `Font` methods."],["FontFaceExt","Trait containing all `FontFace` methods."],["FontFamilyExt","Trait containing all `FontFamily` methods."],["FontMapExt","Trait containing all `FontMap` methods."],["FontsetExt","Trait containing all `Fontset` methods."],["RendererExt","Trait containing all `Renderer` methods."]],"type":[["Glyph",""],["GlyphUnit",""],["LayoutRun",""]]});