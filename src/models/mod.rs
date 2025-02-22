pub mod models{
    ///
    /// [ModificationType] represents application's
    /// known modification loaders.
    ///
    enum ModificationType{
        Forge,
        Fabric,
        Quilt,
    }

    ///
    /// [Modification] is Normalized shorten model of
    /// binary properties.
    /// Short loaders type stores in [ModificationType]
    ///
    struct Modification {
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
    struct ForgeManifest {
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
    struct FabricManifest {
        id:           String,
        description:  String,
        version:      String,
        authors:      Vec<String>,
        license:      String,
    }
}