# microcad VSCode plugin

Provides the syntax highlighting and LSP integration for microcad language plugin.

## Manual installation

```sh
sudo npm install -g vsce # Install Visual Studio Code Extension via `npm`
sudo npm install -g typescript --save-dev # Install type script
```

Assuming the current directory is the `microcad` repository root, install this plugin manually with:

```sh
ln -s editors/vscode ~/.vscode/extensions/
```

### Build VSIX package

```sh
sudo npm install -g @vscode/vsce
npm install vscode-languageclient 
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
