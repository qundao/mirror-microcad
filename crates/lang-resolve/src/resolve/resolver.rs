/// The interface for an resolver
pub trait Resolver {
    /// Exposes the underlying filesystem layer used by this session.
    /// Returns a trait object, allowing the backend storage to be swapped cleanly.
    fn file_system(&self) -> &dyn FileSystem;

    /// Return external search paths
    fn external_search_paths(&self) -> Vec<std::path::Path>;

    /// Return the source path of the file to be resolved
    fn source_path(&self) -> std::path::PathBuf;

    fn workspace_path(&self) -> std::path::PathBuf;

    fn lib_file_path(&self) -> Option<std::path::PathBuf>;

    /// Loads the `mu.toml` manifest file as TOML from a path
    fn load_manifest(&mut self, path: std::path::Path) -> Result<Option<Manifest>>;

    /// Resolve the source into an Rst
    fn resolve(&mut self, source: Source) -> CompilationResult<Rst>;

    fn resolve_external_dependency(&mut self, external: External) -> CompilationResult<Rst>;
}
