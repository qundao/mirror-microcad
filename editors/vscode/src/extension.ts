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

    // DISPOSABLE registrieren
    context.subscriptions.push(client);
}

export async function deactivate() {
    if (client) {
        await client.stop(); // Stoppt den LSP-Server sauber
    }
}