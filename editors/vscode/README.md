# microcad language syntax

Provides the syntax of microcad language.

## Manual installation

Assuming the current directory is the `microcad` repository root, install this plugin manually with:

```sh
ln -s editors/vscode ~/.vscode/extensions/
```

### Build VSIX package

```sh
npm install -g @vscode/vsce
vsce package
```

This will produce a package `microcad-0.0.1.vsix`.

### Install VSIX in VSCode

```sh
code --install-extension microcad-0.0.1.vsix 
```

This will install Extension `microcad-0.0.1.vsix` was successfully installed.

## Release Notes

### 0.0.1

Initial release of microcad-syntax.
