# Model tree

## Separate Model tree and Geometry

Currently, each model node will contain the resulting output geometry after the model node has been rendered.
The `ModelInner` currently looks like this:

```rs
/// The actual model contents
pub struct ModelInner {
    /// Optional id.
    ///
    /// The id is set when the model was created by an assignment: `a = Cube(50mm)`.
    pub id: Option<Identifier>,

    /// Parent object.
    #[debug(skip)]
    pub parent: Option<Model>,
    /// Children of the model.
    pub children: Models,
    /// All models that have been created by an assignment.
    pub assignments: FxHashMap<Identifier, Model>,
    /// Element of the model with [SrcRef].
    pub element: Refer<Element>,
    /// Attributes used for export.
    pub attributes: Attributes,
    /// The output type of the this model.
    pub output: Option<RenderOutput>,
}
```

The output type should be decoupled.
Instead, each model node will have a unique ID (a hash value), which is generated from the `element`, `attributes` and `children`.

Currently, before the a node can be rendered, the method `prerender` has to be called, which computes the hash.

```rs
pub struct Output {
    
}

/// The actual model contents
pub struct ModelInner {
    /// Optional id.
    ///
    /// The id is set when the model was created by an assignment: `a = Cube(50mm)`.
    pub id: Option<Identifier>,

    /// Parent object.
    #[debug(skip)]
    pub parent: Option<Model>,
    /// Children of the model.
    pub children: Hashed<Models>, // Will also contain models by id.
    /// Element of the model with [SrcRef].
    pub element: Hashed<Refer<Element>>,
    /// Attributes used for export.
    pub attributes: Hashed<Attributes>,
    /// The computer hash of this model.
    pub output: Option<Output>,
}
```

The `prerender` function should be replaced with a `finalized` function which computes the hash.
Only finalized nodes can will have `Some(hash)`. For non-finalized nodes, hash will be `None`.

This means we can have a separate data structure that maps each model node id into a geometry.
The geometry type (render output type).

```rs
pub struct HashId(u64); // Or something similar.

pub struct ResolvedRenderData {
    hash: HashId,
    output_type: OutputType, // 2D or 3D
    matrix: Matrix4,
    resolution: RenderResolution,
}

/// The actual model contents
pub struct ModelInner {
    /// Optional id.
    ///
    /// The id is set when the model was created by an assignment: `a = Cube(50mm)`.
    pub id: Option<Identifier>,

    pub parent: Option<Model>,
    /// Children of the model (also contains models created by assignments).
    pub children: Models,

    /// Element of the model with [SrcRef].
    pub element: Refer<Element>,
    /// Attributes used for export.
    pub attributes: Attributes,

    /// These parameter have been created after the model has been finalized.
    pub render: Option<ResolvedRenderData>,
}

impl ModelInner {
    pub fn is_finalized(&self) -> bool {
        self.render.is_some()
    }
}
```

The renderer trait takes a finalized model as input and a geometry type `T`:

```rs
struct RenderCache<T> {
    cache: HashMap<Hash, T>
}

trait Render<T> {
    type Renderer;

    fn render(&self, model: &Model) -> RenderResult<T>;
}

struct Renderer2D {
    cache: RenderCache<Geometry2D>
}

impl Render<T> for Renderer2D {
    fn render(&self, model: &Model) -> RenderResult<T>;
}
```

```rs
// Render a Circle something into a Polygon
type Scalar = f32;

struct Circle {
    radius: Scalar
}

impl Render<Polygon> for Circle { ... }

impl Render<bevy::Mesh> for Circle { ... }

impl Render<Geometry2D> for Circle {
    fn render(&self, model: &Model) -> RenderResult<Geometry2D> {
        Geometry2D::Polygon(microcad_core::Circle::circle_polygon(
            *self.radius,
            model.render_data().resolution,
        ))
    }
}
```

Open question: How to detect which `Render` trait is eventually implemented for which type?
