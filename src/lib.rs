use zed_extension_api as zed;

struct DotnetBuildExtension;

impl zed::Extension for DotnetBuildExtension {
	// No special logic is needed here for providing tasks.
	// The tasks are defined in the `tasks.json` file.
}

zed::register_extension!(DotnetBuildExtension);
