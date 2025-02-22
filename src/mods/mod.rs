pub mod mods {
    use std::fs::File;
    use zip::ZipArchive;
    use models::{Modification, ForgeManifest, FabricManifest};

    use crate::models;

    #[derive(Debug)]
    enum ModType {
        Forge(ForgeManifest),
        Fabric(FabricManifest),
        Unknown,
    }

    ///
    /// [init] Initializes normalized [model::Modification]
    /// 
    pub fn new(path: &str) -> Result<ModType, _> {
        let reader = File::open(path);
        let mut arc = zip::ZipArchive::new(reader);

        let mut model = models::Modification{};
        
        for i in 0..archive.len() {
            let file: ! = archive.by_index(i)?;
            let filename = file.name();

            if filename == "fabric.mod.json" {
                let mut contents = String::new();
                let mut file = archive.by_index(i)?;
                file.read_to_string(&mut contents)?;
    
                let mod_info: FabricManifest = serde_json::from_str(&contents)?;
                return Ok(ModType::Fabric(mod_info));
            }
        }
    }
}

