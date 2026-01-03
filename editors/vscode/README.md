# microcad VSCode plugin

Provides the syntax highlighting and LSP integration for microcad language plugin.

## Manual installation

```sh
cd editors/vscode
npm install
npm run package
```

Open VS Code Command Palette (F1, Ctrl+Shift+P or Cmd+Shift+P) and select
```
> Developer: Install Extension from Location…
```

Select the this folder `<root>/editors/vscode` to install the extension.
Future builds (e.g. after running `npm run package`) from this folder will be picked up by VS Code on extenstion reload.