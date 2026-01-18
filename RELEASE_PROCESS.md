# Release process

## Versioning
- Tag releases as vMAJOR.MINOR.PATCH.

## Publish
- Python: build wheels/sdist and upload to PyPI.
- Rust: publish crate to crates.io.
- Go: tags are consumed via module proxy.

## Note on immutability
- PyPI does not allow re-uploading the same file name/version.
- Always bump the version for a new publish.
