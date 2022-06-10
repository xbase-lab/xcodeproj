use tap::Pipe;

/// Target Product Type
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum PBXProductType {
    /// Application
    Application,
    /// Framework
    Framework,
    /// StaticFramework
    StaticFramework,
    /// XcFramework
    XcFramework,
    /// DynamicLibrary
    DynamicLibrary,
    /// StaticLibrary
    StaticLibrary,
    /// Bundle
    Bundle,
    /// UnitTestBundle
    UnitTestBundle,
    /// UiTestBundle
    UiTestBundle,
    /// AppExtension
    AppExtension,
    /// CommandLineTool
    CommandLineTool,
    /// WatchApp
    WatchApp,
    /// Watch2App
    Watch2App,
    /// Watch2AppContainer
    Watch2AppContainer,
    /// WatchExtension
    WatchExtension,
    /// Watch2Extension
    Watch2Extension,
    /// TvExtension
    TvExtension,
    /// MessagesApplication
    MessagesApplication,
    /// MessagesExtension
    MessagesExtension,
    /// StickerPack
    StickerPack,
    /// XpcService
    XpcService,
    /// OcUnitTestBundle
    OcUnitTestBundle,
    /// XcodeExtension
    XcodeExtension,
    /// InstrumentsPackage
    InstrumentsPackage,
    /// IntentsServiceExtension
    IntentsServiceExtension,
    /// OnDemandInstallCapableApplication
    OnDemandInstallCapableApplication,
    /// MetalLibrary
    MetalLibrary,
    /// DriverExtension
    DriverExtension,
    /// SystemExtension
    SystemExtension,
    /// None Identified
    None,
}

impl Default for PBXProductType {
    fn default() -> Self {
        Self::None
    }
}

impl PBXProductType {
    /// Return file extension for product type
    pub fn file_extension(&self) -> Option<&str> {
        use PBXProductType::*;
        match self {
            Application
            | WatchApp
            | Watch2App
            | Watch2AppContainer
            | MessagesApplication
            | OnDemandInstallCapableApplication => "app",
            AppExtension
            | TvExtension
            | WatchExtension
            | Watch2Extension
            | MessagesExtension
            | StickerPack
            | XcodeExtension
            | IntentsServiceExtension => "appex",
            Framework | StaticFramework => "framework",
            UnitTestBundle | UiTestBundle => "xctest",
            DynamicLibrary => "dylib",
            StaticLibrary => "a",
            Bundle => "bundle",
            XpcService => "xpc",
            OcUnitTestBundle => "octest",
            InstrumentsPackage => "instrpkg",
            XcFramework => "xcframework",
            MetalLibrary => "metallib",
            SystemExtension => "systemextension",
            DriverExtension => "dext",
            _ => return Option::None,
        }
        .pipe(Some)
    }
}

impl std::fmt::Display for PBXProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PBXProductType::*;
        let value = match self {
            Application => "com.apple.product-type.application",
            Framework => "com.apple.product-type.framework",
            StaticFramework => "com.apple.product-type.framework.static",
            XcFramework => "com.apple.product-type.xcframework",
            DynamicLibrary => "com.apple.product-type.library.dynamic",
            StaticLibrary => "com.apple.product-type.library.static",
            Bundle => "com.apple.product-type.bundle",
            UnitTestBundle => "com.apple.product-type.bundle.unit-test",
            UiTestBundle => "com.apple.product-type.bundle.ui-testing",
            AppExtension => "com.apple.product-type.app-extension",
            CommandLineTool => "com.apple.product-type.tool",
            WatchApp => "com.apple.product-type.application.watchapp",
            Watch2App => "com.apple.product-type.application.watchapp2",
            Watch2AppContainer => "com.apple.product-type.application.watchapp2-container",
            WatchExtension => "com.apple.product-type.watchkit-extension",
            Watch2Extension => "com.apple.product-type.watchkit2-extension",
            TvExtension => "com.apple.product-type.tv-app-extension",
            MessagesApplication => "com.apple.product-type.application.messages",
            MessagesExtension => "com.apple.product-type.app-extension.messages",
            StickerPack => "com.apple.product-type.app-extension.messages-sticker-pack",
            XpcService => "com.apple.product-type.xpc-service",
            OcUnitTestBundle => "com.apple.product-type.bundle.ocunit-test",
            XcodeExtension => "com.apple.product-type.xcode-extension",
            InstrumentsPackage => "com.apple.product-type.instruments-package",
            IntentsServiceExtension => "com.apple.product-type.app-extension.intents-service",
            OnDemandInstallCapableApplication => {
                "com.apple.product-type.application.on-demand-install-capable"
            }
            MetalLibrary => "com.apple.product-type.metal-library",
            DriverExtension => "com.apple.product-type.driver-extension",
            SystemExtension => "com.apple.product-type.system-extension",
            None => return Ok(()),
        };
        write!(f, "{value}")
    }
}

impl From<&str> for PBXProductType {
    fn from(value: &str) -> Self {
        use PBXProductType::*;
        match value {
            "com.apple.product-type.application" => Application,
            "com.apple.product-type.framework" => Framework,
            "com.apple.product-type.framework.static" => StaticFramework,
            "com.apple.product-type.xcframework" => XcFramework,
            "com.apple.product-type.library.dynamic" => DynamicLibrary,
            "com.apple.product-type.library.static" => StaticLibrary,
            "com.apple.product-type.bundle" => Bundle,
            "com.apple.product-type.bundle.unit-test" => UnitTestBundle,
            "com.apple.product-type.bundle.ui-testing" => UiTestBundle,
            "com.apple.product-type.app-extension" => AppExtension,
            "com.apple.product-type.tool" => CommandLineTool,
            "com.apple.product-type.application.watchapp" => WatchApp,
            "com.apple.product-type.application.watchapp2" => Watch2App,
            "com.apple.product-type.application.watchapp2-container" => Watch2AppContainer,
            "com.apple.product-type.watchkit-extension" => WatchExtension,
            "com.apple.product-type.watchkit2-extension" => Watch2Extension,
            "com.apple.product-type.tv-app-extension" => TvExtension,
            "com.apple.product-type.application.messages" => MessagesApplication,
            "com.apple.product-type.app-extension.messages" => MessagesExtension,
            "com.apple.product-type.app-extension.messages-sticker-pack" => StickerPack,
            "com.apple.product-type.xpc-service" => XpcService,
            "com.apple.product-type.bundle.ocunit-test" => OcUnitTestBundle,
            "com.apple.product-type.xcode-extension" => XcodeExtension,
            "com.apple.product-type.instruments-package" => InstrumentsPackage,
            "com.apple.product-type.app-extension.intents-service" => IntentsServiceExtension,
            "com.apple.product-type.application.on-demand-install-capable" => {
                OnDemandInstallCapableApplication
            }
            "com.apple.product-type.metal-library" => MetalLibrary,
            "com.apple.product-type.driver-extension" => DriverExtension,
            "com.apple.product-type.system-extension" => SystemExtension,
            _ => None,
        }
    }
}

impl From<String> for PBXProductType {
    fn from(s: String) -> Self {
        PBXProductType::from(s.as_str())
    }
}
