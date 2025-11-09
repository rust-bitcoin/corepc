# No shebang, this file should not be executed.
# shellcheck disable=SC2148
#
# disable verify unused vars, despite the fact that they are used when sourced
# shellcheck disable=SC2034

# Test all these features with "std" enabled.
FEATURES_WITH_STD=""

# Test all these features without "std" or "alloc" enabled.
FEATURES_WITHOUT_STD=""

# Run these examples.
EXAMPLES=""

# Just check the latest minor version of the last three supported Core versions.
# This is mainly for docs and MSRV - integration tests will catch any other errors.
EXACT_FEATURES=("download,28_2" "download,27_2" "download,26_2")
