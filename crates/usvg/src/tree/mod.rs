// Copyright 2019 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod filter;
mod geom;
mod text;

use std::fmt::Display;
use std::sync::Arc;

pub use strict_num::{self, ApproxEqUlps, NonZeroPositiveF32, NormalizedF32, PositiveF32};

pub use tiny_skia_path;

pub use self::geom::*;
pub use self::text::*;

use crate::OptionLog;

/// An alias to `NormalizedF32`.
pub type Opacity = NormalizedF32;

/// A non-empty string used as an element ID.
///
/// Must not be clone-able to preserve ID uniqueness.
#[derive(Debug)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// Creates a new non-empty string, returning `None` when trimmed input is empty.
    pub fn new(string: String) -> Option<Self> {
        if string.trim().is_empty() {
            return None;
        }

        Some(NonEmptyString(string))
    }

    /// Returns the underlying string.
    pub fn get(&self) -> &str {
        &self.0
    }

    pub(crate) fn take(self) -> String {
        self.0
    }
}

/// A non-zero `f32`.
///
/// Just like `f32` but immutable and guarantee to never be zero.
#[derive(Clone, Copy, Debug)]
pub struct NonZeroF32(f32);

impl NonZeroF32 {
    /// Creates a new `NonZeroF32` value.
    #[inline]
    pub fn new(n: f32) -> Option<Self> {
        if n.approx_eq_ulps(&0.0, 4) {
            None
        } else {
            Some(NonZeroF32(n))
        }
    }

    /// Returns an underlying value.
    #[inline]
    pub fn get(&self) -> f32 {
        self.0
    }
}

/// Units of a paint server, pattern or clip/mask region.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Units {
    /// Absolute units in the user coordinate system.
    UserSpaceOnUse,
    /// Units relative to the object's bounding box.
    ObjectBoundingBox,
}

// `Units` cannot have a default value, because it changes depending on an element.

impl std::fmt::Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Units::UserSpaceOnUse => write!(f, "userSpaceOnUse"),
            Units::ObjectBoundingBox => write!(f, "objectBoundingBox"),
        }
    }
}

/// A visibility property.
///
/// `visibility` attribute in the SVG.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Visibility {
    Visible,
    Hidden,
    Collapse,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Visible
    }
}

/// A shape rendering method.
///
/// `shape-rendering` attribute in the SVG.
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum ShapeRendering {
    OptimizeSpeed,
    CrispEdges,
    GeometricPrecision,
}

impl ShapeRendering {
    /// Checks if anti-aliasing should be enabled.
    pub fn use_shape_antialiasing(self) -> bool {
        match self {
            ShapeRendering::OptimizeSpeed => false,
            ShapeRendering::CrispEdges => false,
            ShapeRendering::GeometricPrecision => true,
        }
    }
}

impl Default for ShapeRendering {
    fn default() -> Self {
        Self::GeometricPrecision
    }
}

impl std::str::FromStr for ShapeRendering {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "optimizeSpeed" => Ok(ShapeRendering::OptimizeSpeed),
            "crispEdges" => Ok(ShapeRendering::CrispEdges),
            "geometricPrecision" => Ok(ShapeRendering::GeometricPrecision),
            _ => Err("invalid"),
        }
    }
}

/// A text rendering method.
///
/// `text-rendering` attribute in the SVG.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextRendering {
    OptimizeSpeed,
    OptimizeLegibility,
    GeometricPrecision,
}

impl Default for TextRendering {
    fn default() -> Self {
        Self::OptimizeLegibility
    }
}

impl std::str::FromStr for TextRendering {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "optimizeSpeed" => Ok(TextRendering::OptimizeSpeed),
            "optimizeLegibility" => Ok(TextRendering::OptimizeLegibility),
            "geometricPrecision" => Ok(TextRendering::GeometricPrecision),
            _ => Err("invalid"),
        }
    }
}

/// An image rendering method.
///
/// `image-rendering` attribute in the SVG.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ImageRendering {
    OptimizeQuality,
    OptimizeSpeed,
    // The following can only appear as presentation attributes.
    Smooth,
    HighQuality,
    CrispEdges,
    Pixelated,
}

impl Default for ImageRendering {
    fn default() -> Self {
        Self::OptimizeQuality
    }
}

impl std::str::FromStr for ImageRendering {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "optimizeQuality" => Ok(ImageRendering::OptimizeQuality),
            "optimizeSpeed" => Ok(ImageRendering::OptimizeSpeed),
            "smooth" => Ok(ImageRendering::Smooth),
            "high-quality" => Ok(ImageRendering::HighQuality),
            "crisp-edges" => Ok(ImageRendering::CrispEdges),
            "pixelated" => Ok(ImageRendering::Pixelated),
            _ => Err("invalid"),
        }
    }
}

/// A blending mode property.
///
/// `mix-blend-mode` attribute in the SVG.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::Normal
    }
}

impl Display for BlendMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blend_mode = match self {
            BlendMode::Normal => "normal",
            BlendMode::Multiply => "multiply",
            BlendMode::Screen => "screen",
            BlendMode::Overlay => "overlay",
            BlendMode::Darken => "darken",
            BlendMode::Lighten => "lighten",
            BlendMode::ColorDodge => "color-dodge",
            BlendMode::ColorBurn => "color-burn",
            BlendMode::HardLight => "hard-light",
            BlendMode::SoftLight => "soft-light",
            BlendMode::Difference => "difference",
            BlendMode::Exclusion => "exclusion",
            BlendMode::Hue => "hue",
            BlendMode::Saturation => "saturation",
            BlendMode::Color => "color",
            BlendMode::Luminosity => "luminosity",
        };
        write!(f, "{blend_mode}")
    }
}

/// A spread method.
///
/// `spreadMethod` attribute in the SVG.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SpreadMethod {
    Pad,
    Reflect,
    Repeat,
}

impl Default for SpreadMethod {
    fn default() -> Self {
        Self::Pad
    }
}

/// A generic gradient.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct BaseGradient {
    pub id: NonEmptyString,
    pub units: Units,
    pub transform: Transform,
    pub spread_method: SpreadMethod,
    pub stops: Vec<Stop>,
}

impl BaseGradient {
    /// Creates a new base gradient.
    pub fn new(
        id: NonEmptyString,
        units: Units,
        transform: Transform,
        spread_method: SpreadMethod,
        stops: Vec<Stop>,
    ) -> Self {
        BaseGradient {
            id,
            units,
            transform,
            spread_method,
            stops,
        }
    }

    /// Element's ID.
    ///
    /// Taken from the SVG itself.
    /// Used only during SVG writing. `resvg` doesn't rely on this property.
    pub fn id(&self) -> &str {
        self.id.get()
    }

    /// Gradient transform.
    ///
    /// `gradientTransform` in SVG.
    pub fn transform(&self) -> Transform {
        self.transform
    }

    /// Gradient spreading method.
    ///
    /// `spreadMethod` in SVG.
    pub fn spread_method(&self) -> SpreadMethod {
        self.spread_method
    }

    /// A list of `stop` elements.
    pub fn stops(&self) -> &[Stop] {
        &self.stops
    }
}

/// A linear gradient.
///
/// `linearGradient` element in SVG.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct LinearGradient {
    pub base: BaseGradient,
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl LinearGradient {
    /// Creates a new linear gradient.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        base: BaseGradient,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    ) -> Self {
        LinearGradient {
            base,
            x1,
            y1,
            x2,
            y2,
        }
    }

    /// `x1` coordinate.
    pub fn x1(&self) -> f32 {
        self.x1
    }

    /// `y1` coordinate.
    pub fn y1(&self) -> f32 {
        self.y1
    }

    /// `x2` coordinate.
    pub fn x2(&self) -> f32 {
        self.x2
    }

    /// `y2` coordinate.
    pub fn y2(&self) -> f32 {
        self.y2
    }
}

impl std::ops::Deref for LinearGradient {
    type Target = BaseGradient;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// A radial gradient.
///
/// `radialGradient` element in SVG.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct RadialGradient {
    pub base: BaseGradient,
    pub cx: f32,
    pub cy: f32,
    pub r: PositiveF32,
    pub fx: f32,
    pub fy: f32,
    pub fr: PositiveF32,
}

impl RadialGradient {
    /// Creates a new radial gradient.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        base: BaseGradient,
        cx: f32,
        cy: f32,
        r: PositiveF32,
        fx: f32,
        fy: f32,
        fr: PositiveF32,
    ) -> Self {
        RadialGradient {
            base,
            cx,
            cy,
            r,
            fx,
            fy,
            fr,
        }
    }

    /// `cx` coordinate.
    pub fn cx(&self) -> f32 {
        self.cx
    }

    /// `cy` coordinate.
    pub fn cy(&self) -> f32 {
        self.cy
    }

    /// Gradient radius.
    pub fn r(&self) -> PositiveF32 {
        self.r
    }

    /// `fx` coordinate.
    pub fn fx(&self) -> f32 {
        self.fx
    }

    /// `fy` coordinate.
    pub fn fy(&self) -> f32 {
        self.fy
    }

    /// Focal radius.
    pub fn fr(&self) -> PositiveF32 {
        self.fr
    }
}

impl std::ops::Deref for RadialGradient {
    type Target = BaseGradient;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// An alias to `NormalizedF32`.
pub type StopOffset = NormalizedF32;

/// Gradient's stop element.
///
/// `stop` element in SVG.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct Stop {
    pub offset: StopOffset,
    pub color: Color,
    pub opacity: Opacity,
}

impl Stop {
    /// Creates a new gradient stop.
    pub fn new(offset: StopOffset, color: Color, opacity: Opacity) -> Self {
        Stop {
            offset,
            color,
            opacity,
        }
    }

    /// Gradient stop offset.
    ///
    /// `offset` in SVG.
    pub fn offset(&self) -> StopOffset {
        self.offset
    }

    /// Gradient stop color.
    ///
    /// `stop-color` in SVG.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Gradient stop opacity.
    ///
    /// `stop-opacity` in SVG.
    pub fn opacity(&self) -> Opacity {
        self.opacity
    }
}

/// A pattern element.
///
/// `pattern` element in SVG.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct Pattern {
    pub id: NonEmptyString,
    pub units: Units,
    pub content_units: Units,
    pub transform: Transform,
    pub rect: NonZeroRect,
    pub view_box: Option<ViewBox>,
    pub root: Group,
}

impl Pattern {
    /// Creates a new pattern.
    pub fn new(
        id: NonEmptyString,
        units: Units,
        content_units: Units,
        transform: Transform,
        rect: NonZeroRect,
        view_box: Option<ViewBox>,
        root: Group,
    ) -> Self {
        Pattern {
            id,
            units,
            content_units,
            transform,
            rect,
            view_box,
            root,
        }
    }

    /// Element's ID.
    ///
    /// Taken from the SVG itself.
    /// Used only during SVG writing. `resvg` doesn't rely on this property.
    pub fn id(&self) -> &str {
        self.id.get()
    }

    /// Pattern transform.
    ///
    /// `patternTransform` in SVG.
    pub fn transform(&self) -> Transform {
        self.transform
    }

    /// Pattern rectangle.
    ///
    /// `x`, `y`, `width` and `height` in SVG.
    pub fn rect(&self) -> NonZeroRect {
        self.rect
    }

    /// Pattern children.
    pub fn root(&self) -> &Group {
        &self.root
    }
}

/// An alias to `NonZeroPositiveF32`.
pub type StrokeWidth = NonZeroPositiveF32;

/// A `stroke-miterlimit` value.
///
/// Just like `f32` but immutable and guarantee to be >=1.0.
#[derive(Clone, Copy, Debug)]
pub struct StrokeMiterlimit(f32);

impl StrokeMiterlimit {
    /// Creates a new `StrokeMiterlimit` value.
    #[inline]
    pub fn new(n: f32) -> Self {
        debug_assert!(n.is_finite());
        debug_assert!(n >= 1.0);

        let n = if !(n >= 1.0) { 1.0 } else { n };

        StrokeMiterlimit(n)
    }

    /// Returns an underlying value.
    #[inline]
    pub fn get(&self) -> f32 {
        self.0
    }
}

impl Default for StrokeMiterlimit {
    #[inline]
    fn default() -> Self {
        StrokeMiterlimit::new(4.0)
    }
}

impl From<f32> for StrokeMiterlimit {
    #[inline]
    fn from(n: f32) -> Self {
        Self::new(n)
    }
}

impl PartialEq for StrokeMiterlimit {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.approx_eq_ulps(&other.0, 4)
    }
}

/// A line cap.
///
/// `stroke-linecap` attribute in the SVG.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

impl Default for LineCap {
    fn default() -> Self {
        Self::Butt
    }
}

/// A line join.
///
/// `stroke-linejoin` attribute in the SVG.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LineJoin {
    Miter,
    MiterClip,
    Round,
    Bevel,
}

impl Default for LineJoin {
    fn default() -> Self {
        Self::Miter
    }
}

/// A stroke style.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct Stroke {
    pub paint: Paint,
    pub dasharray: Option<Vec<f32>>,
    pub dashoffset: f32,
    pub miterlimit: StrokeMiterlimit,
    pub opacity: Opacity,
    pub width: StrokeWidth,
    pub linecap: LineCap,
    pub linejoin: LineJoin,
    // Whether the current stroke needs to be resolved relative
    // to a context element.
    pub context_element: Option<ContextElement>,
}

impl Stroke {
    /// Creates a new stroke with default options.
    pub fn new(paint: Paint) -> Self {
        Stroke {
            paint,
            dasharray: None,
            dashoffset: 0.0,
            miterlimit: StrokeMiterlimit::default(),
            opacity: Opacity::ONE,
            width: StrokeWidth::new(1.0).unwrap(),
            linecap: LineCap::default(),
            linejoin: LineJoin::default(),
            context_element: None,
        }
    }

    /// Stroke paint.
    pub fn paint(&self) -> &Paint {
        &self.paint
    }

    /// Stroke dash array.
    pub fn dasharray(&self) -> Option<&[f32]> {
        self.dasharray.as_deref()
    }

    /// Stroke dash offset.
    pub fn dashoffset(&self) -> f32 {
        self.dashoffset
    }

    /// Stroke miter limit.
    pub fn miterlimit(&self) -> StrokeMiterlimit {
        self.miterlimit
    }

    /// Stroke opacity.
    pub fn opacity(&self) -> Opacity {
        self.opacity
    }

    /// Stroke width.
    pub fn width(&self) -> StrokeWidth {
        self.width
    }

    /// Stroke linecap.
    pub fn linecap(&self) -> LineCap {
        self.linecap
    }

    /// Stroke linejoin.
    pub fn linejoin(&self) -> LineJoin {
        self.linejoin
    }

    /// Converts into a `tiny_skia_path::Stroke` type.
    pub fn to_tiny_skia(&self) -> tiny_skia_path::Stroke {
        let mut stroke = tiny_skia_path::Stroke {
            width: self.width.get(),
            miter_limit: self.miterlimit.get(),
            line_cap: match self.linecap {
                LineCap::Butt => tiny_skia_path::LineCap::Butt,
                LineCap::Round => tiny_skia_path::LineCap::Round,
                LineCap::Square => tiny_skia_path::LineCap::Square,
            },
            line_join: match self.linejoin {
                LineJoin::Miter => tiny_skia_path::LineJoin::Miter,
                LineJoin::MiterClip => tiny_skia_path::LineJoin::MiterClip,
                LineJoin::Round => tiny_skia_path::LineJoin::Round,
                LineJoin::Bevel => tiny_skia_path::LineJoin::Bevel,
            },
            // According to the spec, dash should not be accounted during
            // bbox calculation.
            dash: None,
        };

        if let Some(ref list) = self.dasharray {
            stroke.dash = tiny_skia_path::StrokeDash::new(list.clone(), self.dashoffset);
        }

        stroke
    }
}

/// A fill rule.
///
/// `fill-rule` attribute in the SVG.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FillRule {
    NonZero,
    EvenOdd,
}

impl Default for FillRule {
    fn default() -> Self {
        Self::NonZero
    }
}

/// The context element used to resolve a paint server's units.
#[derive(Clone, Copy, Debug)]
pub enum ContextElement {
    /// The current context element is a use node. Since we can get
    /// the bounding box of a use node only once we have converted
    /// all elements, we need to fix the transform and units of
    /// the stroke/fill after converting the whole tree.
    UseNode,
    /// The current context element is a path node (i.e. only applicable
    /// if we draw the marker of a path). Since we already know the bounding
    /// box of the path when rendering the markers, we can convert them directly,
    /// so we do it while parsing.
    PathNode(Transform, Option<NonZeroRect>),
}

/// A fill style.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct Fill {
    pub paint: Paint,
    pub opacity: Opacity,
    pub rule: FillRule,
    // Whether the current fill needs to be resolved relative
    // to a context element.
    pub context_element: Option<ContextElement>,
}

impl Fill {
    /// Creates a new fill with default options.
    pub fn new(paint: Paint) -> Self {
        Fill {
            paint,
            opacity: Opacity::ONE,
            rule: FillRule::default(),
            context_element: None,
        }
    }

    /// Fill paint.
    pub fn paint(&self) -> &Paint {
        &self.paint
    }

    /// Fill opacity.
    pub fn opacity(&self) -> Opacity {
        self.opacity
    }

    /// Fill rule.
    pub fn rule(&self) -> FillRule {
        self.rule
    }
}

impl Default for Fill {
    fn default() -> Self {
        Fill {
            paint: Paint::Color(Color::black()),
            opacity: Opacity::ONE,
            rule: FillRule::default(),
            context_element: None,
        }
    }
}

/// A 8-bit RGB color.
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    /// Constructs a new `Color` from RGB values.
    #[inline]
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    /// Constructs a new `Color` set to black.
    #[inline]
    pub fn black() -> Color {
        Color::new_rgb(0, 0, 0)
    }

    /// Constructs a new `Color` set to white.
    #[inline]
    pub fn white() -> Color {
        Color::new_rgb(255, 255, 255)
    }
}

/// A paint style.
///
/// `paint` value type in the SVG.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Paint {
    Color(Color),
    LinearGradient(Arc<LinearGradient>),
    RadialGradient(Arc<RadialGradient>),
    Pattern(Arc<Pattern>),
}

impl PartialEq for Paint {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Color(lc), Self::Color(rc)) => lc == rc,
            (Self::LinearGradient(lg1), Self::LinearGradient(lg2)) => Arc::ptr_eq(lg1, lg2),
            (Self::RadialGradient(rg1), Self::RadialGradient(rg2)) => Arc::ptr_eq(rg1, rg2),
            (Self::Pattern(p1), Self::Pattern(p2)) => Arc::ptr_eq(p1, p2),
            _ => false,
        }
    }
}

/// A clip-path element.
///
/// `clipPath` element in SVG.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct ClipPath {
    pub id: NonEmptyString,
    pub transform: Transform,
    pub clip_path: Option<Arc<ClipPath>>,
    pub root: Group,
}

impl ClipPath {
    /// Creates an empty clip path with the given ID.
    pub fn empty(id: NonEmptyString) -> Self {
        ClipPath {
            id,
            transform: Transform::default(),
            clip_path: None,
            root: Group::empty(),
        }
    }

    /// Creates a new clip path.
    pub fn new(
        id: NonEmptyString,
        transform: Transform,
        clip_path: Option<Arc<ClipPath>>,
        root: Group,
    ) -> Self {
        ClipPath {
            id,
            transform,
            clip_path,
            root,
        }
    }

    /// Element's ID.
    ///
    /// Taken from the SVG itself.
    /// Used only during SVG writing. `resvg` doesn't rely on this property.
    pub fn id(&self) -> &str {
        self.id.get()
    }

    /// Clip path transform.
    ///
    /// `transform` in SVG.
    pub fn transform(&self) -> Transform {
        self.transform
    }

    /// Additional clip path.
    ///
    /// `clip-path` in SVG.
    pub fn clip_path(&self) -> Option<&ClipPath> {
        self.clip_path.as_deref()
    }

    /// Clip path children.
    pub fn root(&self) -> &Group {
        &self.root
    }
}

/// A mask type.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MaskType {
    /// Indicates that the luminance values of the mask should be used.
    Luminance,
    /// Indicates that the alpha values of the mask should be used.
    Alpha,
}

impl Default for MaskType {
    fn default() -> Self {
        Self::Luminance
    }
}

/// A mask element.
///
/// `mask` element in SVG.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct Mask {
    pub id: NonEmptyString,
    pub rect: NonZeroRect,
    pub kind: MaskType,
    pub mask: Option<Arc<Mask>>,
    pub root: Group,
}

impl Mask {
    /// Creates a new mask.
    pub fn new(
        id: NonEmptyString,
        rect: NonZeroRect,
        kind: MaskType,
        mask: Option<Arc<Mask>>,
        root: Group,
    ) -> Self {
        Mask {
            id,
            rect,
            kind,
            mask,
            root,
        }
    }

    /// Element's ID.
    ///
    /// Taken from the SVG itself.
    /// Used only during SVG writing. `resvg` doesn't rely on this property.
    pub fn id(&self) -> &str {
        self.id.get()
    }

    /// Mask rectangle.
    ///
    /// `x`, `y`, `width` and `height` in SVG.
    pub fn rect(&self) -> NonZeroRect {
        self.rect
    }

    /// Mask type.
    ///
    /// `mask-type` in SVG.
    pub fn kind(&self) -> MaskType {
        self.kind
    }

    /// Additional mask.
    ///
    /// `mask` in SVG.
    pub fn mask(&self) -> Option<&Mask> {
        self.mask.as_deref()
    }

    /// Mask children.
    ///
    /// A mask can have no children, in which case the whole element should be masked out.
    pub fn root(&self) -> &Group {
        &self.root
    }
}

/// Node's kind.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Node {
    Group(Box<Group>),
    Path(Box<Path>),
    Image(Box<Image>),
    Text(Box<Text>),
}

impl Node {
    /// Returns node's ID.
    pub fn id(&self) -> &str {
        match self {
            Node::Group(e) => e.id.as_str(),
            Node::Path(e) => e.id.as_str(),
            Node::Image(e) => e.id.as_str(),
            Node::Text(e) => e.id.as_str(),
        }
    }

    /// Returns node's absolute transform.
    ///
    /// This method is cheap since absolute transforms are already resolved.
    pub fn abs_transform(&self) -> Transform {
        match self {
            Node::Group(group) => group.abs_transform(),
            Node::Path(path) => path.abs_transform(),
            Node::Image(image) => image.abs_transform(),
            Node::Text(text) => text.abs_transform(),
        }
    }

    /// Returns node's bounding box in object coordinates, if any.
    pub fn bounding_box(&self) -> Rect {
        match self {
            Node::Group(group) => group.bounding_box(),
            Node::Path(path) => path.bounding_box(),
            Node::Image(image) => image.bounding_box(),
            Node::Text(text) => text.bounding_box(),
        }
    }

    /// Returns node's bounding box in canvas coordinates, if any.
    pub fn abs_bounding_box(&self) -> Rect {
        match self {
            Node::Group(group) => group.abs_bounding_box(),
            Node::Path(path) => path.abs_bounding_box(),
            Node::Image(image) => image.abs_bounding_box(),
            Node::Text(text) => text.abs_bounding_box(),
        }
    }

    /// Returns node's bounding box, including stroke, in object coordinates, if any.
    pub fn stroke_bounding_box(&self) -> Rect {
        match self {
            Node::Group(group) => group.stroke_bounding_box(),
            Node::Path(path) => path.stroke_bounding_box(),
            // Image cannot be stroked.
            Node::Image(image) => image.bounding_box(),
            Node::Text(text) => text.stroke_bounding_box(),
        }
    }

    /// Returns node's bounding box, including stroke, in canvas coordinates, if any.
    pub fn abs_stroke_bounding_box(&self) -> Rect {
        match self {
            Node::Group(group) => group.abs_stroke_bounding_box(),
            Node::Path(path) => path.abs_stroke_bounding_box(),
            // Image cannot be stroked.
            Node::Image(image) => image.abs_bounding_box(),
            Node::Text(text) => text.abs_stroke_bounding_box(),
        }
    }

    /// Element's "layer" bounding box in canvas units, if any.
    ///
    /// For most nodes this is just `abs_bounding_box`,
    /// but for groups this is `abs_layer_bounding_box`.
    ///
    /// See [`Group::layer_bounding_box`] for details.
    pub fn abs_layer_bounding_box(&self) -> Option<NonZeroRect> {
        match self {
            Node::Group(group) => Some(group.abs_layer_bounding_box()),
            // Hor/ver path without stroke can return None. This is expected.
            Node::Path(path) => path.abs_bounding_box().to_non_zero_rect(),
            Node::Image(image) => image.abs_bounding_box().to_non_zero_rect(),
            Node::Text(text) => text.abs_bounding_box().to_non_zero_rect(),
        }
    }

    /// Calls a closure for each subroot this `Node` has.
    ///
    /// The [`Tree::root`](Tree::root) field contain only render-able SVG elements.
    /// But some elements, specifically clip paths, masks, patterns and feImage
    /// can store their own SVG subtrees.
    /// And while one can access them manually, it's pretty verbose.
    /// This methods allows looping over _all_ SVG elements present in the `Tree`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// fn all_nodes(parent: &usvg::Group) {
    ///     for node in parent.children() {
    ///         // do stuff...
    ///
    ///         if let usvg::Node::Group(g) = node {
    ///             all_nodes(g);
    ///         }
    ///
    ///         // handle subroots as well
    ///         node.subroots(|subroot| all_nodes(subroot));
    ///     }
    /// }
    /// ```
    pub fn subroots<F: FnMut(&Group)>(&self, mut f: F) {
        match self {
            Node::Group(group) => group.subroots(&mut f),
            Node::Path(path) => path.subroots(&mut f),
            Node::Image(image) => image.subroots(&mut f),
            Node::Text(text) => text.subroots(&mut f),
        }
    }
}

/// A group container.
///
/// The preprocessor will remove all groups that don't impact rendering.
/// Those that left is just an indicator that a new canvas should be created.
///
/// `g` element in SVG.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct Group {
    pub id: String,
    pub transform: Transform,
    pub abs_transform: Transform,
    pub opacity: Opacity,
    pub blend_mode: BlendMode,
    pub isolate: bool,
    pub clip_path: Option<Arc<ClipPath>>,
    /// Whether the group is a context element (i.e. a use node)
    pub is_context_element: bool,
    pub mask: Option<Arc<Mask>>,
    pub filters: Vec<Arc<filter::Filter>>,
    pub(crate) bounding_box: Rect,
    pub(crate) abs_bounding_box: Rect,
    pub(crate) stroke_bounding_box: Rect,
    pub(crate) abs_stroke_bounding_box: Rect,
    pub(crate) layer_bounding_box: NonZeroRect,
    pub(crate) abs_layer_bounding_box: NonZeroRect,
    pub children: Vec<Node>,
}

impl Group {
    /// Creates an empty group.
    pub fn empty() -> Self {
        let dummy = Rect::from_xywh(0.0, 0.0, 0.0, 0.0).unwrap();
        Group {
            id: String::new(),
            transform: Transform::default(),
            abs_transform: Transform::default(),
            opacity: Opacity::ONE,
            blend_mode: BlendMode::Normal,
            isolate: false,
            clip_path: None,
            mask: None,
            filters: Vec::new(),
            is_context_element: false,
            bounding_box: dummy,
            abs_bounding_box: dummy,
            stroke_bounding_box: dummy,
            abs_stroke_bounding_box: dummy,
            layer_bounding_box: NonZeroRect::from_xywh(0.0, 0.0, 1.0, 1.0).unwrap(),
            abs_layer_bounding_box: NonZeroRect::from_xywh(0.0, 0.0, 1.0, 1.0).unwrap(),
            children: Vec::new(),
        }
    }

    /// Element's ID.
    ///
    /// Taken from the SVG itself.
    /// Isn't automatically generated.
    /// Can be empty.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Element's transform.
    ///
    /// This is a relative transform. The one that is set via the `transform` attribute in SVG.
    pub fn transform(&self) -> Transform {
        self.transform
    }

    /// Element's absolute transform.
    ///
    /// Contains all ancestors transforms including group's transform.
    ///
    /// Note that subroots, like clipPaths, masks and patterns, have their own root transform,
    /// which isn't affected by the node that references this subroot.
    pub fn abs_transform(&self) -> Transform {
        self.abs_transform
    }

    /// Group opacity.
    ///
    /// After the group is rendered we should combine
    /// it with a parent group using the specified opacity.
    pub fn opacity(&self) -> Opacity {
        self.opacity
    }

    /// Group blend mode.
    ///
    /// `mix-blend-mode` in SVG.
    pub fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }

    /// Group isolation.
    ///
    /// `isolation` in SVG.
    pub fn isolate(&self) -> bool {
        self.isolate
    }

    /// Element's clip path.
    pub fn clip_path(&self) -> Option<&ClipPath> {
        self.clip_path.as_deref()
    }

    /// Element's mask.
    pub fn mask(&self) -> Option<&Mask> {
        self.mask.as_deref()
    }

    /// Element's filters.
    pub fn filters(&self) -> &[Arc<filter::Filter>] {
        &self.filters
    }

    /// Element's object bounding box.
    ///
    /// `objectBoundingBox` in SVG terms. Meaning it doesn't affected by parent transforms.
    ///
    /// Can be set to `None` in case of an empty group.
    pub fn bounding_box(&self) -> Rect {
        self.bounding_box
    }

    /// Element's bounding box in canvas coordinates.
    ///
    /// `userSpaceOnUse` in SVG terms.
    pub fn abs_bounding_box(&self) -> Rect {
        self.abs_bounding_box
    }

    /// Element's object bounding box including stroke.
    ///
    /// Similar to `bounding_box`, but includes stroke.
    pub fn stroke_bounding_box(&self) -> Rect {
        self.stroke_bounding_box
    }

    /// Element's bounding box including stroke in user coordinates.
    ///
    /// Similar to `abs_bounding_box`, but includes stroke.
    pub fn abs_stroke_bounding_box(&self) -> Rect {
        self.abs_stroke_bounding_box
    }

    /// Element's "layer" bounding box in object units.
    ///
    /// Conceptually, this is `stroke_bounding_box` expanded and/or clipped
    /// by `filters_bounding_box`, but also including all the children.
    /// This is the bounding box `resvg` will later use to allocate layers/pixmaps
    /// during isolated groups rendering.
    ///
    /// Only groups have it, because only groups can have filters.
    /// For other nodes layer bounding box is the same as stroke bounding box.
    ///
    /// Unlike other bounding boxes, cannot have zero size.
    ///
    /// Returns 0x0x1x1 for empty groups.
    pub fn layer_bounding_box(&self) -> NonZeroRect {
        self.layer_bounding_box
    }

    /// Element's "layer" bounding box in canvas units.
    pub fn abs_layer_bounding_box(&self) -> NonZeroRect {
        self.abs_layer_bounding_box
    }

    /// Group's children.
    pub fn children(&self) -> &[Node] {
        &self.children
    }

    /// Appends a child node to this group.
    pub fn push_child(&mut self, child: Node) {
        self.children.push(child);
    }

    /// Checks if this group should be isolated during rendering.
    pub fn should_isolate(&self) -> bool {
        self.isolate
            || self.opacity != Opacity::ONE
            || self.clip_path.is_some()
            || self.mask.is_some()
            || !self.filters.is_empty()
            || self.blend_mode != BlendMode::Normal // TODO: probably not needed?
    }

    /// Returns `true` if the group has any children.
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Calculates a node's filter bounding box.
    ///
    /// Filters with `objectBoundingBox` and missing or zero `bounding_box` would be ignored.
    ///
    /// Note that a filter region can act like a clipping rectangle,
    /// therefore this function can produce a bounding box smaller than `bounding_box`.
    ///
    /// Returns `None` when then group has no filters.
    ///
    /// This function is very fast, that's why we do not store this bbox as a `Group` field.
    pub fn filters_bounding_box(&self) -> Option<NonZeroRect> {
        let mut full_region = BBox::default();
        for filter in &self.filters {
            full_region = full_region.expand(filter.rect);
        }

        full_region.to_non_zero_rect()
    }

    fn subroots(&self, f: &mut dyn FnMut(&Group)) {
        if let Some(ref clip) = self.clip_path {
            f(&clip.root);

            if let Some(ref sub_clip) = clip.clip_path {
                f(&sub_clip.root);
            }
        }

        if let Some(ref mask) = self.mask {
            f(&mask.root);

            if let Some(ref sub_mask) = mask.mask {
                f(&sub_mask.root);
            }
        }

        for filter in &self.filters {
            for primitive in &filter.primitives {
                if let filter::Kind::Image(ref image) = primitive.kind {
                    f(image.root());
                }
            }
        }
    }
}

/// Representation of the [`paint-order`] property.
///
/// `usvg` will handle `markers` automatically,
/// therefore we provide only `fill` and `stroke` variants.
///
/// [`paint-order`]: https://www.w3.org/TR/SVG2/painting.html#PaintOrder
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub enum PaintOrder {
    FillAndStroke,
    StrokeAndFill,
}

impl Default for PaintOrder {
    fn default() -> Self {
        Self::FillAndStroke
    }
}

/// A path element.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct Path {
    pub id: String,
    pub visible: bool,
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
    pub paint_order: PaintOrder,
    pub rendering_mode: ShapeRendering,
    pub data: Arc<tiny_skia_path::Path>,
    pub abs_transform: Transform,
    pub(crate) bounding_box: Rect,
    pub(crate) abs_bounding_box: Rect,
    pub(crate) stroke_bounding_box: Rect,
    pub(crate) abs_stroke_bounding_box: Rect,
}

impl Path {
    /// Creates a new path from the given geometry with no style or transform.
    pub fn new_simple(data: Arc<tiny_skia_path::Path>) -> Option<Self> {
        Self::new(
            String::new(),
            true,
            None,
            None,
            PaintOrder::default(),
            ShapeRendering::default(),
            data,
            Transform::default(),
        )
    }

    /// Creates a new path node.
    ///
    /// The bounding boxes are computed immediately from `data` and `abs_transform`.
    pub fn new(
        id: String,
        visible: bool,
        fill: Option<Fill>,
        stroke: Option<Stroke>,
        paint_order: PaintOrder,
        rendering_mode: ShapeRendering,
        data: Arc<tiny_skia_path::Path>,
        abs_transform: Transform,
    ) -> Option<Self> {
        let bounding_box = data.compute_tight_bounds()?;
        let stroke_bounding_box =
            Path::calculate_stroke_bbox(stroke.as_ref(), &data).unwrap_or(bounding_box);

        let abs_bounding_box: Rect;
        let abs_stroke_bounding_box: Rect;
        if abs_transform.has_skew() {
            // TODO: avoid re-alloc
            let path2 = data.as_ref().clone();
            let path2 = path2.transform(abs_transform)?;
            abs_bounding_box = path2.compute_tight_bounds()?;
            abs_stroke_bounding_box =
                Path::calculate_stroke_bbox(stroke.as_ref(), &path2).unwrap_or(abs_bounding_box);
        } else {
            // A transform without a skew can be performed just on a bbox.
            abs_bounding_box = bounding_box.transform(abs_transform)?;
            abs_stroke_bounding_box = stroke_bounding_box.transform(abs_transform)?;
        }

        Some(Path {
            id,
            visible,
            fill,
            stroke,
            paint_order,
            rendering_mode,
            data,
            abs_transform,
            bounding_box,
            abs_bounding_box,
            stroke_bounding_box,
            abs_stroke_bounding_box,
        })
    }

    /// Element's ID.
    ///
    /// Taken from the SVG itself.
    /// Isn't automatically generated.
    /// Can be empty.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Element visibility.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Fill style.
    pub fn fill(&self) -> Option<&Fill> {
        self.fill.as_ref()
    }

    /// Stroke style.
    pub fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }

    /// Fill and stroke paint order.
    ///
    /// Since markers will be replaced with regular nodes automatically,
    /// `usvg` doesn't provide the `markers` order type. It's was already done.
    ///
    /// `paint-order` in SVG.
    pub fn paint_order(&self) -> PaintOrder {
        self.paint_order
    }

    /// Rendering mode.
    ///
    /// `shape-rendering` in SVG.
    pub fn rendering_mode(&self) -> ShapeRendering {
        self.rendering_mode
    }

    // TODO: find a better name
    /// Segments list.
    ///
    /// All segments are in absolute coordinates.
    pub fn data(&self) -> &tiny_skia_path::Path {
        self.data.as_ref()
    }

    /// Element's absolute transform.
    ///
    /// Contains all ancestors transforms including elements's transform.
    ///
    /// Note that this is not the relative transform present in SVG.
    /// The SVG one would be set only on groups.
    pub fn abs_transform(&self) -> Transform {
        self.abs_transform
    }

    /// Element's object bounding box.
    ///
    /// `objectBoundingBox` in SVG terms. Meaning it doesn't affected by parent transforms.
    pub fn bounding_box(&self) -> Rect {
        self.bounding_box
    }

    /// Element's bounding box in canvas coordinates.
    ///
    /// `userSpaceOnUse` in SVG terms.
    pub fn abs_bounding_box(&self) -> Rect {
        self.abs_bounding_box
    }

    /// Element's object bounding box including stroke.
    ///
    /// Will have the same value as `bounding_box` when path has no stroke.
    pub fn stroke_bounding_box(&self) -> Rect {
        self.stroke_bounding_box
    }

    /// Element's bounding box including stroke in canvas coordinates.
    ///
    /// Will have the same value as `abs_bounding_box` when path has no stroke.
    pub fn abs_stroke_bounding_box(&self) -> Rect {
        self.abs_stroke_bounding_box
    }

    fn calculate_stroke_bbox(stroke: Option<&Stroke>, path: &tiny_skia_path::Path) -> Option<Rect> {
        let mut stroke = stroke?.to_tiny_skia();
        // According to the spec, dash should not be accounted during bbox calculation.
        stroke.dash = None;

        // TODO: avoid for round and bevel caps

        // Expensive, but there is not much we can do about it.
        if let Some(stroked_path) = path.stroke(&stroke, 1.0) {
            return stroked_path.compute_tight_bounds();
        }

        None
    }

    fn subroots(&self, f: &mut dyn FnMut(&Group)) {
        if let Some(Paint::Pattern(patt)) = self.fill.as_ref().map(|f| &f.paint) {
            f(patt.root());
        }
        if let Some(Paint::Pattern(patt)) = self.stroke.as_ref().map(|f| &f.paint) {
            f(patt.root());
        }
    }
}

/// An embedded image kind.
#[derive(Clone)]
pub enum ImageKind {
    /// A reference to raw JPEG data. Should be decoded by the caller.
    JPEG(Arc<Vec<u8>>),
    /// A reference to raw PNG data. Should be decoded by the caller.
    PNG(Arc<Vec<u8>>),
    /// A reference to raw GIF data. Should be decoded by the caller.
    GIF(Arc<Vec<u8>>),
    /// A reference to raw WebP data. Should be decoded by the caller.
    WEBP(Arc<Vec<u8>>),
    /// A preprocessed SVG tree. Can be rendered as is.
    SVG(Tree),
}

impl ImageKind {
    pub(crate) fn actual_size(&self) -> Option<Size> {
        match self {
            ImageKind::JPEG(data)
            | ImageKind::PNG(data)
            | ImageKind::GIF(data)
            | ImageKind::WEBP(data) => imagesize::blob_size(data)
                .ok()
                .and_then(|size| Size::from_wh(size.width as f32, size.height as f32))
                .log_none(|| log::warn!("Image has an invalid size. Skipped.")),
            ImageKind::SVG(svg) => Some(svg.size),
        }
    }
}

impl std::fmt::Debug for ImageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ImageKind::JPEG(_) => f.write_str("ImageKind::JPEG(..)"),
            ImageKind::PNG(_) => f.write_str("ImageKind::PNG(..)"),
            ImageKind::GIF(_) => f.write_str("ImageKind::GIF(..)"),
            ImageKind::WEBP(_) => f.write_str("ImageKind::WEBP(..)"),
            ImageKind::SVG(_) => f.write_str("ImageKind::SVG(..)"),
        }
    }
}

/// A raster image element.
///
/// `image` element in SVG.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct Image {
    pub id: String,
    pub visible: bool,
    pub size: Size,
    pub rendering_mode: ImageRendering,
    pub kind: ImageKind,
    pub abs_transform: Transform,
    pub(crate) abs_bounding_box: NonZeroRect,
}

impl Image {
    /// Creates a new image node.
    pub fn new(
        id: String,
        visible: bool,
        size: Size,
        rendering_mode: ImageRendering,
        kind: ImageKind,
        abs_transform: Transform,
    ) -> Option<Self> {
        let abs_bounding_box = size.to_non_zero_rect(0.0, 0.0).transform(abs_transform)?;
        Some(Image {
            id,
            visible,
            size,
            rendering_mode,
            kind,
            abs_transform,
            abs_bounding_box,
        })
    }

    /// Element's ID.
    ///
    /// Taken from the SVG itself.
    /// Isn't automatically generated.
    /// Can be empty.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Element visibility.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// The actual image size.
    ///
    /// This is not `width` and `height` attributes,
    /// but rather the actual PNG/JPEG/GIF/SVG image size.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Rendering mode.
    ///
    /// `image-rendering` in SVG.
    pub fn rendering_mode(&self) -> ImageRendering {
        self.rendering_mode
    }

    /// Image data.
    pub fn kind(&self) -> &ImageKind {
        &self.kind
    }

    /// Element's absolute transform.
    ///
    /// Contains all ancestors transforms including elements's transform.
    ///
    /// Note that this is not the relative transform present in SVG.
    /// The SVG one would be set only on groups.
    pub fn abs_transform(&self) -> Transform {
        self.abs_transform
    }

    /// Element's object bounding box.
    ///
    /// `objectBoundingBox` in SVG terms. Meaning it doesn't affected by parent transforms.
    pub fn bounding_box(&self) -> Rect {
        self.size.to_rect(0.0, 0.0).unwrap()
    }

    /// Element's bounding box in canvas coordinates.
    ///
    /// `userSpaceOnUse` in SVG terms.
    pub fn abs_bounding_box(&self) -> Rect {
        self.abs_bounding_box.to_rect()
    }

    fn subroots(&self, f: &mut dyn FnMut(&Group)) {
        if let ImageKind::SVG(ref tree) = self.kind {
            f(&tree.root);
        }
    }
}

/// A nodes tree container.
#[allow(missing_debug_implementations)]
#[derive(Clone, Debug)]
pub struct Tree {
    pub(crate) size: Size,
    pub(crate) root: Group,
    pub(crate) linear_gradients: Vec<Arc<LinearGradient>>,
    pub(crate) radial_gradients: Vec<Arc<RadialGradient>>,
    pub(crate) patterns: Vec<Arc<Pattern>>,
    pub(crate) clip_paths: Vec<Arc<ClipPath>>,
    pub(crate) masks: Vec<Arc<Mask>>,
    pub(crate) filters: Vec<Arc<filter::Filter>>,
    #[cfg(feature = "text")]
    pub(crate) fontdb: Arc<fontdb::Database>,
}

impl Tree {
    /// Creates a new, empty tree with the given size and root group.
    ///
    /// After populating the tree, call [`Tree::finalize`] to compute
    /// bounding boxes and collect paint servers.
    pub fn new(size: Size, root: Group) -> Self {
        Tree {
            size,
            root,
            linear_gradients: Vec::new(),
            radial_gradients: Vec::new(),
            patterns: Vec::new(),
            clip_paths: Vec::new(),
            masks: Vec::new(),
            filters: Vec::new(),
            #[cfg(feature = "text")]
            fontdb: Arc::new(fontdb::Database::new()),
        }
    }

    /// Returns a mutable reference to the root group.
    pub fn root_mut(&mut self) -> &mut Group {
        &mut self.root
    }

    /// Finalizes the tree after programmatic construction.
    ///
    /// This recomputes all bounding boxes (bottom-up) and collects paint
    /// servers, clip paths, masks and filters into the `Tree` storage.
    ///
    /// This is the counterpart of the post-parse processing that the XML
    /// parser performs in `convert_doc`. It must be called once after the
    /// tree has been assembled and before it is rendered.
    pub fn finalize(&mut self) {
        self.linear_gradients.clear();
        self.radial_gradients.clear();
        self.patterns.clear();
        self.clip_paths.clear();
        self.masks.clear();
        self.filters.clear();

        calculate_bounding_boxes_recursive(&mut self.root);
        resolve_object_bounding_box(&mut self.root, &mut 0);
        self.collect_paint_servers();
        self.root.collect_clip_paths(&mut self.clip_paths);
        self.root.collect_masks(&mut self.masks);
        self.root.collect_filters(&mut self.filters);
        self.root.calculate_bounding_boxes();
    }

    /// Image size.
    ///
    /// Size of an image that should be created to fit the SVG.
    ///
    /// `width` and `height` in SVG.
    ///
    /// Note that this does not necessarily represent the bounding box of the
    /// rendered contents. Use
    /// [`self.root().abs_layer_bounding_box()`](Group::abs_layer_bounding_box)
    /// to retrieve it instead.
    pub fn size(&self) -> Size {
        self.size
    }

    /// The root element of the SVG tree.
    pub fn root(&self) -> &Group {
        &self.root
    }

    /// Returns a renderable node by ID.
    ///
    /// If an empty ID is provided, than this method will always return `None`.
    pub fn node_by_id(&self, id: &str) -> Option<&Node> {
        if id.is_empty() {
            return None;
        }

        node_by_id(&self.root, id)
    }

    /// Checks if the current tree has any text nodes.
    pub fn has_text_nodes(&self) -> bool {
        has_text_nodes(&self.root)
    }

    /// Checks if the current tree has any `defs` nodes.
    pub fn has_defs_nodes(&self) -> bool {
        !self.linear_gradients().is_empty()
            || !self.radial_gradients().is_empty()
            || !self.patterns().is_empty()
            || !self.filters().is_empty()
            || !self.clip_paths().is_empty()
            || !self.masks().is_empty()
    }

    /// Returns a list of all unique [`LinearGradient`]s in the tree.
    pub fn linear_gradients(&self) -> &[Arc<LinearGradient>] {
        &self.linear_gradients
    }

    /// Returns a list of all unique [`RadialGradient`]s in the tree.
    pub fn radial_gradients(&self) -> &[Arc<RadialGradient>] {
        &self.radial_gradients
    }

    /// Returns a list of all unique [`Pattern`]s in the tree.
    pub fn patterns(&self) -> &[Arc<Pattern>] {
        &self.patterns
    }

    /// Returns a list of all unique [`ClipPath`]s in the tree.
    pub fn clip_paths(&self) -> &[Arc<ClipPath>] {
        &self.clip_paths
    }

    /// Returns a list of all unique [`Mask`]s in the tree.
    pub fn masks(&self) -> &[Arc<Mask>] {
        &self.masks
    }

    /// Returns a list of all unique [`Filter`](filter::Filter)s in the tree.
    pub fn filters(&self) -> &[Arc<filter::Filter>] {
        &self.filters
    }

    /// Returns the font database that applies to all text nodes in the tree.
    #[cfg(feature = "text")]
    pub fn fontdb(&self) -> &Arc<fontdb::Database> {
        &self.fontdb
    }

    pub(crate) fn collect_paint_servers(&mut self) {
        loop_over_paint_servers(&self.root, &mut |paint| match paint {
            Paint::Color(_) => {}
            Paint::LinearGradient(lg) => {
                if !self
                    .linear_gradients
                    .iter()
                    .any(|other| Arc::ptr_eq(lg, other))
                {
                    self.linear_gradients.push(lg.clone());
                }
            }
            Paint::RadialGradient(rg) => {
                if !self
                    .radial_gradients
                    .iter()
                    .any(|other| Arc::ptr_eq(rg, other))
                {
                    self.radial_gradients.push(rg.clone());
                }
            }
            Paint::Pattern(patt) => {
                if !self.patterns.iter().any(|other| Arc::ptr_eq(patt, other)) {
                    self.patterns.push(patt.clone());
                }
            }
        });
    }
}

fn calculate_bounding_boxes_recursive(group: &mut Group) {
    for child in &mut group.children {
        if let Node::Group(g) = child {
            calculate_bounding_boxes_recursive(g);
        }
    }

    // Mask and clip-path content is stored on the group (not as children) and
    // also needs its bounding boxes computed: otherwise an isolated group inside
    // them (e.g. a shape wrapped for `opacity < 1`) has no `layer_bounding_box`
    // and is silently skipped at render time. The XML parser does the equivalent
    // in `parser::mask::convert` / `parser::clippath::convert`.
    if let Some(mask) = &mut group.mask {
        if let Some(mask) = Arc::get_mut(mask) {
            calculate_bounding_boxes_recursive(&mut mask.root);
            if let Some(sub_mask) = &mut mask.mask {
                if let Some(sub_mask) = Arc::get_mut(sub_mask) {
                    calculate_bounding_boxes_recursive(&mut sub_mask.root);
                }
            }
        }
    }
    if let Some(clip_path) = &mut group.clip_path {
        if let Some(clip_path) = Arc::get_mut(clip_path) {
            calculate_bounding_boxes_recursive(&mut clip_path.root);
            if let Some(sub_clip) = &mut clip_path.clip_path {
                if let Some(sub_clip) = Arc::get_mut(sub_clip) {
                    calculate_bounding_boxes_recursive(&mut sub_clip.root);
                }
            }
        }
    }

    group.calculate_bounding_boxes();
}

// When building a tree programmatically we don't know a shape's bounding box
// until after its path data is set, so gradients/patterns are created preserving
// their `objectBoundingBox` units and then rewritten to `userSpaceOnUse` here —
// the same post-parse step the XML parser performs in
// `parser::paint_server::update_paint_servers`. For rendering, paint servers are
// always `userSpaceOnUse`.
fn resolve_object_bounding_box(group: &mut Group, counter: &mut usize) {
    for child in &mut group.children {
        if let Node::Group(g) = child {
            resolve_object_bounding_box(g, counter);
        } else if let Node::Path(path) = child {
            let bbox = path.bounding_box;
            // A paint server with `objectBoundingBox` units on a zero-sized shape is
            // dropped here, mirroring the parser's `to_user_coordinates` returning
            // `None` and causing `process_fill`/`process_stroke` to clear the paint.
            let keep_fill = match &mut path.fill {
                Some(fill) => resolve_paint_units(&mut fill.paint, bbox, counter),
                None => true,
            };
            if !keep_fill {
                path.fill = None;
            }
            let keep_stroke = match &mut path.stroke {
                Some(stroke) => resolve_paint_units(&mut stroke.paint, bbox, counter),
                None => true,
            };
            if !keep_stroke {
                path.stroke = None;
            }
        }
    }

    // Mask content can reference `objectBoundingBox` gradients/patterns just like
    // regular shapes, so recurse into masks too (the XML parser's
    // `update_paint_servers` does the same). Clip paths carry no paint servers, so
    // they are intentionally left alone.
    if let Some(mask) = &mut group.mask {
        if let Some(mask) = Arc::get_mut(mask) {
            resolve_object_bounding_box(&mut mask.root, counter);
            if let Some(sub_mask) = &mut mask.mask {
                if let Some(sub_mask) = Arc::get_mut(sub_mask) {
                    resolve_object_bounding_box(&mut sub_mask.root, counter);
                }
            }
        }
    }
}

fn resolve_paint_units(paint: &mut Paint, bbox: Rect, counter: &mut usize) -> bool {
    match paint {
        Paint::LinearGradient(lg) => {
            if lg.base.units != Units::ObjectBoundingBox {
                return true;
            }

            // `objectBoundingBox` units on a zero-sized shape are meaningless, so
            // drop the paint (matching the parser's `to_user_coordinates`).
            let Some(non_zero_bbox) = bbox.to_non_zero_rect() else {
                return false;
            };

            // `objectBoundingBox` maps the (0,0)-(1,1) square onto the shape's
            // bbox, so the gradient transform becomes the original transform
            // followed by the bbox scale/translate.
            let transform = lg
                .base
                .transform
                .post_concat(Transform::from_bbox(non_zero_bbox));

            match Arc::get_mut(lg) {
                Some(lg) => {
                    lg.base.transform = transform;
                    lg.base.units = Units::UserSpaceOnUse;
                }
                None => {
                    // The gradient is shared by more than one shape (whose bboxes
                    // differ), so clone it with a fresh id and resolved transform.
                    *counter += 1;
                    let id = NonEmptyString::new(format!("__linear_gradient_{}", counter))
                        .expect("generated gradient id is non-empty");
                    let cloned = LinearGradient {
                        base: BaseGradient {
                            id,
                            units: Units::UserSpaceOnUse,
                            transform,
                            spread_method: lg.base.spread_method,
                            stops: lg.base.stops.clone(),
                        },
                        x1: lg.x1,
                        y1: lg.y1,
                        x2: lg.x2,
                        y2: lg.y2,
                    };
                    *lg = Arc::new(cloned);
                }
            }
            true
        }
        Paint::RadialGradient(rg) => {
            if rg.base.units != Units::ObjectBoundingBox {
                return true;
            }

            let Some(non_zero_bbox) = bbox.to_non_zero_rect() else {
                return false;
            };

            let transform = rg
                .base
                .transform
                .post_concat(Transform::from_bbox(non_zero_bbox));

            match Arc::get_mut(rg) {
                Some(rg) => {
                    rg.base.transform = transform;
                    rg.base.units = Units::UserSpaceOnUse;
                }
                None => {
                    *counter += 1;
                    let id = NonEmptyString::new(format!("__radial_gradient_{}", counter))
                        .expect("generated gradient id is non-empty");
                    let cloned = RadialGradient {
                        base: BaseGradient {
                            id,
                            units: Units::UserSpaceOnUse,
                            transform,
                            spread_method: rg.base.spread_method,
                            stops: rg.base.stops.clone(),
                        },
                        cx: rg.cx,
                        cy: rg.cy,
                        r: rg.r,
                        fx: rg.fx,
                        fy: rg.fy,
                        fr: rg.fr,
                    };
                    *rg = Arc::new(cloned);
                }
            }
            true
        }
        Paint::Pattern(patt) => {
            if patt.units == Units::UserSpaceOnUse
                && patt.content_units == Units::UserSpaceOnUse
                && patt.view_box.is_none()
            {
                return true;
            }

            let Some(non_zero_bbox) = bbox.to_non_zero_rect() else {
                return false;
            };

            // `patternUnits="objectBoundingBox"` maps the pattern tile (its
            // `x`/`y`/`width`/`height`) onto the shape's bbox.
            let rect = if patt.units == Units::ObjectBoundingBox {
                patt.rect.bbox_transform(non_zero_bbox)
            } else {
                patt.rect
            };

            // `patternContentUnits="objectBoundingBox"` (with no viewBox) scales
            // the pattern's content by the bbox dimensions.
            let content_transform =
                if patt.content_units == Units::ObjectBoundingBox && patt.view_box.is_none() {
                    Some(Transform::from_scale(
                        non_zero_bbox.width(),
                        non_zero_bbox.height(),
                    ))
                } else {
                    None
                };

            let view_box_transform = patt
                .view_box
                .map(|view_box| view_box.to_transform(rect.size()));

            match Arc::get_mut(patt) {
                Some(patt) => {
                    patt.rect = rect;
                    patt.units = Units::UserSpaceOnUse;
                    if let Some(transform) = content_transform {
                        push_pattern_transform(&mut patt.root, transform);
                    }
                    if let Some(transform) = view_box_transform {
                        push_pattern_transform(&mut patt.root, transform);
                    }
                    patt.content_units = Units::UserSpaceOnUse;
                }
                None => {
                    // The pattern is shared by shapes with differing bboxes, so
                    // clone it with a fresh id and resolved units/rect/root.
                    let mut root = patt.root.clone();
                    if let Some(transform) = content_transform {
                        push_pattern_transform(&mut root, transform);
                    }
                    if let Some(transform) = view_box_transform {
                        push_pattern_transform(&mut root, transform);
                    }

                    *counter += 1;
                    let id = NonEmptyString::new(format!("__pattern_{}", counter))
                        .expect("generated pattern id is non-empty");
                    *patt = Arc::new(Pattern {
                        id,
                        units: Units::UserSpaceOnUse,
                        content_units: Units::UserSpaceOnUse,
                        transform: patt.transform,
                        rect,
                        view_box: None,
                        root,
                    });
                }
            }
            true
        }
        Paint::Color(_) => true,
    }
}

/// Wraps a pattern's content root in a new group carrying `transform`, so that
/// `patternContentUnits="objectBoundingBox"` and `viewBox` scaling apply to the
/// pattern's children.
///
/// Mirrors the parser's equivalent helper in `parser/paint_server.rs`.
fn push_pattern_transform(root: &mut Group, transform: Transform) {
    // TODO: update `abs_transform` in descendants as well.
    let mut group = std::mem::replace(root, Group::empty());
    group.transform = transform;
    group.abs_transform = transform;

    root.children.push(Node::Group(Box::new(group)));
    let _ = root.calculate_bounding_boxes();
}

fn node_by_id<'a>(parent: &'a Group, id: &str) -> Option<&'a Node> {
    for child in &parent.children {
        if child.id() == id {
            return Some(child);
        }

        if let Node::Group(g) = child {
            if let Some(n) = node_by_id(g, id) {
                return Some(n);
            }
        }
    }

    None
}

fn has_text_nodes(root: &Group) -> bool {
    for node in &root.children {
        if let Node::Text(_) = node {
            return true;
        }

        let mut has_text = false;

        if let Node::Image(image) = node {
            if let ImageKind::SVG(tree) = &image.kind {
                if has_text_nodes(&tree.root) {
                    has_text = true;
                }
            }
        }

        node.subroots(|subroot| has_text |= has_text_nodes(subroot));

        if has_text {
            return true;
        }
    }

    false
}

fn loop_over_paint_servers(parent: &Group, f: &mut dyn FnMut(&Paint)) {
    fn push(paint: Option<&Paint>, f: &mut dyn FnMut(&Paint)) {
        if let Some(paint) = paint {
            f(paint);
        }
    }

    for node in &parent.children {
        match node {
            Node::Group(group) => loop_over_paint_servers(group, f),
            Node::Path(path) => {
                push(path.fill.as_ref().map(|f| &f.paint), f);
                push(path.stroke.as_ref().map(|f| &f.paint), f);
            }
            Node::Image(_) => {}
            // Flattened text would be used instead.
            Node::Text(_) => {}
        }

        node.subroots(|subroot| loop_over_paint_servers(subroot, f));
    }
}

impl Group {
    pub(crate) fn collect_clip_paths(&self, clip_paths: &mut Vec<Arc<ClipPath>>) {
        for node in self.children() {
            if let Node::Group(g) = node {
                if let Some(clip) = &g.clip_path {
                    if !clip_paths.iter().any(|other| Arc::ptr_eq(clip, other)) {
                        clip_paths.push(clip.clone());
                    }

                    if let Some(sub_clip) = &clip.clip_path {
                        if !clip_paths.iter().any(|other| Arc::ptr_eq(sub_clip, other)) {
                            clip_paths.push(sub_clip.clone());
                        }
                    }
                }
            }

            node.subroots(|subroot| subroot.collect_clip_paths(clip_paths));

            if let Node::Group(g) = node {
                g.collect_clip_paths(clip_paths);
            }
        }
    }

    pub(crate) fn collect_masks(&self, masks: &mut Vec<Arc<Mask>>) {
        for node in self.children() {
            if let Node::Group(g) = node {
                if let Some(mask) = &g.mask {
                    if !masks.iter().any(|other| Arc::ptr_eq(mask, other)) {
                        masks.push(mask.clone());
                    }

                    if let Some(sub_mask) = &mask.mask {
                        if !masks.iter().any(|other| Arc::ptr_eq(sub_mask, other)) {
                            masks.push(sub_mask.clone());
                        }
                    }
                }
            }

            node.subroots(|subroot| subroot.collect_masks(masks));

            if let Node::Group(g) = node {
                g.collect_masks(masks);
            }
        }
    }

    pub(crate) fn collect_filters(&self, filters: &mut Vec<Arc<filter::Filter>>) {
        for node in self.children() {
            if let Node::Group(g) = node {
                for filter in g.filters() {
                    if !filters.iter().any(|other| Arc::ptr_eq(filter, other)) {
                        filters.push(filter.clone());
                    }
                }
            }

            node.subroots(|subroot| subroot.collect_filters(filters));

            if let Node::Group(g) = node {
                g.collect_filters(filters);
            }
        }
    }

    pub(crate) fn calculate_object_bbox(&mut self) -> Option<NonZeroRect> {
        let mut bbox = BBox::default();
        for child in &self.children {
            let mut c_bbox = child.bounding_box();
            if let Node::Group(group) = child {
                if let Some(r) = c_bbox.transform(group.transform) {
                    c_bbox = r;
                }
            }

            bbox = bbox.expand(c_bbox);
        }

        bbox.to_non_zero_rect()
    }

    /// Recursively computes bounding boxes for this group and all nested groups,
    /// then returns the object bounding box (the union of the children's bounding
    /// boxes, in this group's local coordinate system).
    ///
    /// This is exposed so that `mask`/`clipPath` regions declared with
    /// `objectBoundingBox` units can be resolved while building a tree
    /// programmatically, before `Tree::finalize` runs.
    pub fn compute_object_bbox(&mut self) -> Option<NonZeroRect> {
        calculate_bounding_boxes_recursive(self);
        self.calculate_object_bbox()
    }

    pub(crate) fn calculate_bounding_boxes(&mut self) -> Option<()> {
        let mut bbox = BBox::default();
        let mut abs_bbox = BBox::default();
        let mut stroke_bbox = BBox::default();
        let mut abs_stroke_bbox = BBox::default();
        let mut layer_bbox = BBox::default();
        for child in &self.children {
            {
                let mut c_bbox = child.bounding_box();
                if let Node::Group(group) = child {
                    if let Some(r) = c_bbox.transform(group.transform) {
                        c_bbox = r;
                    }
                }

                bbox = bbox.expand(c_bbox);
            }

            abs_bbox = abs_bbox.expand(child.abs_bounding_box());

            {
                let mut c_bbox = child.stroke_bounding_box();
                if let Node::Group(group) = child {
                    if let Some(r) = c_bbox.transform(group.transform) {
                        c_bbox = r;
                    }
                }

                stroke_bbox = stroke_bbox.expand(c_bbox);
            }

            abs_stroke_bbox = abs_stroke_bbox.expand(child.abs_stroke_bounding_box());

            if let Node::Group(group) = child {
                let r = group.layer_bounding_box;
                if let Some(r) = r.transform(group.transform) {
                    layer_bbox = layer_bbox.expand(r);
                }
            } else {
                // Not a group - no need to transform.
                layer_bbox = layer_bbox.expand(child.stroke_bounding_box());
            }
        }

        // `bbox` can be None for empty groups, but we still have to
        // calculate `layer_bounding_box after` it.
        if let Some(bbox) = bbox.to_rect() {
            self.bounding_box = bbox;
            self.abs_bounding_box = abs_bbox.to_rect()?;
            self.stroke_bounding_box = stroke_bbox.to_rect()?;
            self.abs_stroke_bounding_box = abs_stroke_bbox.to_rect()?;
        }

        // Filter bbox has a higher priority than layers bbox.
        if let Some(filter_bbox) = self.filters_bounding_box() {
            self.layer_bounding_box = filter_bbox;
        } else {
            self.layer_bounding_box = layer_bbox.to_non_zero_rect()?;
        }

        self.abs_layer_bounding_box = self.layer_bounding_box.transform(self.abs_transform)?;

        Some(())
    }
}
