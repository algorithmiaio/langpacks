# IPA - package/packageset naming standards

## Universal Naming Rules

* lowercase names, with versioning sections delimited by '.'
* new naming system uses a concept called 'feature chaining', this allows us to have a more * descriptive package/packageset name without adding too much non-essential pieces
* every feature of a package/package set name will be defined as
    * 'name-major_version.minor_version.revision_version'
    * examples:
        * python-3.7
        * java-11, etc
* unless you're using a non-standard package manager, don't include as a feature (eg: don't include pip, but include anaconda)
* general feature chain structure is as follows (from left to right), where (..?) defines an optional feature.
    * development_language-version-(auxillary_language-version?)-(nonstandard_buildenv-version?)-primary_dependency_name-version-(auxillary_dependency_name-version?)
    * examples:
        * python-3.7-pytorch-1.0.0
        * r-3.6-python-3.5-tensorflow-1.14.0, etc
* if feature doesn't follow semantic versioning - using the first 4 chars of the git hash or an equivalent version descriptor (eg: 2019.04.2) is valid


## Package name proposal

Packages right now have a flat hierarchy, which made some sense at inception - however in reality there are 3 types of packages that are distinct and should have very distinct naming conventions.

* Base Language Packages
    * Dependencies required to install all required components for a algorithm development language
    * Follows the feature chaining system, but with a base appended added to the end
    * Legacy Examples:
        * python37
        * java11
        * anaconda45
* Examples:
        * python-3.7-base
        * python-2.7-anaconda-4.5-base
        * scala-2.11-sbt-1.3-base
* buildtime/runtime language packages
    * Dependencies required to manage the runtime & buildtime for an algorithm development language, is more version invariant which means the minor version is generally not required.
    * Follows the feature chaining system, but with runtime or buildtime appended to the end
    * Legacy examples:
        * python3-runtime
        * anaconda4-buildtime
        * r36-runtime
    * Examples:
        * python-3-runtime
        * python-3-buildtime
        * python-3-anaconda-4-buildtime
        * r-3-runtime
        * csharp-dotnet_core-2-buildtime
* dependency packages
    * Optional Dependencies required for specific use cases, generally designed for a specific language, but may be general enough for any language.
    * Follows the feature chaining system
    * Packages that are not language specific can be prefixed with 'generic', such as ffmpeg, wand, etc.
    * Examples:
        * python-3-anaconda-4-solaris-1.2.0
        * python-3.7-torch-1.2.0
        * generic-ffmpeg-0.4.6


## Package set name proposal

Package sets have better names and descriptions, but these should be very specific and informative for users, as they won't have any kind of table to reference.

* Follows the feature chaining system
* Examples:
    * python-3.7-pip-19.2
    * python-3.6-pip-15.2-torch_gpu-1.2.0
    * python-3.5-anaconda-4.7-solaris-1.2.0
    * r-3.6-python-3.7-tensorflow-1.14
