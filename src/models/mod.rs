pub mod models{
    use iced::futures::stream::Next;
    use serde::Deserialize;
    ///
    /// [ModificationType] represents application's
    /// known modification loaders.
    ///
    pub enum ModificationType{
        Forge,
        Fabric,
        Quilt,
    }

    ///
    /// [Modification] is Normalized shorten model of
    /// binary properties.
    /// Short loaders type stores in [ModificationType]
    ///
    pub struct Modification {
        name:              String,
        modification_type: ModificationType,
        path:              String,
        modification_ver:  String,
    }

    ///
    /// [ForgeManifest] represents shorten
    /// list of binary parameters without
    /// dependencies and other
    ///
    #[derive(Debug)]
    pub struct ForgeManifest {
        id:           String,
        description:  String,
        version:      String,
        mod_version:  String,
        mc_version:   String
    }

    ///
    /// [FabricManifest] represents
    /// shorten model of fabric-based JSON
    /// configuration.
    ///
    #[derive(Debug)]
    pub struct FabricManifest {
        id:           String,
        description:  String,
        version:      String,
        authors:      Vec<String>,
        license:      String,
    }
}