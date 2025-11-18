import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';
import * as vscode from 'vscode';
import * as path from 'path';

export async function activate(context: vscode.ExtensionContext) {
    const serverCommand = "microcad-lsp";

    const serverOptions: ServerOptions = {
        run: {
            command: serverCommand,
            args: ["-l", "microcad-lsp.log"]
        },
        debug: {
            command: serverCommand,
            args: ["-l", "microcad-lsp-debug.log"]
        }
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'microcad' }]
    };

    const client = new LanguageClient(
        'MicrocadLanguageServer',
        'Microcad Language Server',
        serverOptions,
        clientOptions
    );

    await client.start();

    // DISPOSABLE registrieren
    context.subscriptions.push(client);
}
