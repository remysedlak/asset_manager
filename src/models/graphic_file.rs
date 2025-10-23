pub struct GraphicFile {
    pub name: String,
    pub path: String,
}
impl Default for GraphicFile  {
    fn default() -> Self {
        Self {
            name: "Unknown File".to_owned(),
            path: "Unknown".to_owned(),
        }
    }
}