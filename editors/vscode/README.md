# microcad VSCode plugin

Provides the syntax highlighting and LSP integration for microcad language plugin.

## Installation

### Manual installation

```sh
cd editors/vscode
npm install
npm run package
```

Open VS Code Command Palette (F1, Ctrl+Shift+P or Cmd+Shift+P) and select

```vscode
> Developer: Install Extension from Location…
```

Select the this folder `<root>/editors/vscode` to install the extension.
Future builds (e.g. after running `npm run package`) from this folder will be picked up by VS Code on extenstion reload.

### Build VSIX package

```sh
npm install
npm run vscode:package
```

This will produce a package `microcad-0.0.2.vsix`.

#### Install VSIX in VSCode

```sh
code --install-extension microcad-0.0.2.vsix
```

This will install the extension and you should see output similar to:

```
Extension `microcad-0.0.2.vsix` was successfully installed.
```

## ❤️ Support the project

This extension is part of the [microcad project](https://microcad.xyz).

If you like this project, you can help us spend more time on it by donating:

<a href="https://opencollective.com/microcad/donate" target="_blank">
<img src="https://opencollective.com/microcad/donate/button@2x.png?color=blue" width=300 />
</a>
