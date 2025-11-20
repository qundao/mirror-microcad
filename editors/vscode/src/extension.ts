import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';
import * as vscode from 'vscode';
import * as path from 'path';

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext) {
    const serverCommand = "microcad-lsp";

    const serverOptions: ServerOptions = {
        run: {
            command: serverCommand,
            args: ["-l", "microcad-lsp.log"],
            transport: TransportKind.stdio
        },
        debug: {
            command: serverCommand,
            args: ["-l", "microcad-lsp-debug.log"],
            transport: TransportKind.socket
        }
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'microcad' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.µcad')
        }
    };

    client = new LanguageClient(
        'microcadLSP',
        'Microcad Language Server',
        serverOptions,
        clientOptions
    );

    await client.start();

    const showPreviewCmd = vscode.commands.registerCommand(
        "microcad.showPreview",
        async () => {
            if (!client) { return; }

            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showErrorMessage("No active editor");
                return;
            }
            const uri = editor.document.uri.toString();
            try {
                const result = await client.sendRequest("workspace/executeCommand", {
                    command: "microcad.showPreview",
                    arguments: [{ uri }]
                });

                vscode.window.showInformationMessage("Preview requested.");
            } catch (err) {
                vscode.window.showErrorMessage("Show Preview failed: " + err);
            }
        }
    );

    const hidePreviewCmd = vscode.commands.registerCommand(
        "microcad.hidePreview",
        async () => {
            if (!client) { return; }
            try {
                const result = await client.sendRequest("workspace/executeCommand", {
                    command: "microcad.hidePreview",
                    arguments: []
                });

                vscode.window.showInformationMessage("Hide Preview requested.");
            } catch (err) {
                vscode.window.showErrorMessage("Hide Preview failed: " + err);
            }
        }
    );

    context.subscriptions.push(showPreviewCmd);
    context.subscriptions.push(client);
}

export async function deactivate() {
    if (client) {
        await client.stop(); // Stoppt den LSP-Server sauber
    }
}