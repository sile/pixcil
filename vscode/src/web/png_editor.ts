import * as vscode from "vscode";
import { Disposable, disposeAll } from "./dispose";

interface PngDocumentDelegate {
  getFileData(): Promise<Uint8Array>;
}

class PngDocument extends Disposable implements vscode.CustomDocument {
  private readonly _uri: vscode.Uri;
  private readonly _delegate: PngDocumentDelegate;
  private _documentData: Uint8Array;

  static async create(
    uri: vscode.Uri,
    backupId: string | undefined,
    delegate: PngDocumentDelegate
  ): Promise<PngDocument | PromiseLike<PngDocument>> {
    // If we have a backup, read that. Otherwise read the resource from the workspace
    const dataFile =
      typeof backupId === "string" ? vscode.Uri.parse(backupId) : uri;
    const fileData = await PngDocument.readFile(dataFile);
    return new PngDocument(uri, fileData, delegate);
  }

  private static async readFile(uri: vscode.Uri): Promise<Uint8Array> {
    if (uri.scheme === "untitled") {
      return new Uint8Array();
    }
    return new Uint8Array(await vscode.workspace.fs.readFile(uri));
  }

  private constructor(
    uri: vscode.Uri,
    initialContent: Uint8Array,
    delegate: PngDocumentDelegate
  ) {
    super();
    this._uri = uri;
    this._documentData = initialContent;
    this._delegate = delegate;
  }

  public get uri() {
    return this._uri;
  }

  public get documentData(): Uint8Array {
    return this._documentData;
  }

  private readonly _onDidChangeDocument = this._register(
    new vscode.EventEmitter<{
      readonly content?: Uint8Array;
      // TODO: readonly edits: readonly PngEdit[];
    }>()
  );

  /**
   * Fired to notify webviews that the document has changed.
   */
  public readonly onDidChangeContent = this._onDidChangeDocument.event;

  private readonly _onDidChange = this._register(
    new vscode.EventEmitter<{
      readonly label: string;
      //undo(): void;
      //redo(): void;
    }>()
  );

  /**
   * Called by VS Code when the user calls `revert` on a document.
   */
  async revert(_cancellation: vscode.CancellationToken): Promise<void> {
    const diskContent = await PngDocument.readFile(this.uri);
    this._documentData = diskContent;
    this._onDidChangeDocument.fire({
      content: diskContent,
    });
  }

  /**
   * Fired to tell VS Code that an edit has occurred in the document.
   *
   * This updates the document's dirty indicator.
   */
  public readonly onDidChange = this._onDidChange.event;

  private readonly _onDidDispose = this._register(
    new vscode.EventEmitter<void>()
  );

  /**
   * Fired when the document is disposed of.
   */
  public readonly onDidDispose = this._onDidDispose.event;

  /**
   * Called by VS Code when there are no more references to the document.
   *
   * This happens when all editors for it have been closed.
   */
  dispose(): void {
    this._onDidDispose.fire();
    super.dispose();
  }

  makeDirty() {
    console.log("makeDirty");
    this._onDidChange.fire({
      label: "Dirty",
    });
  }

  /**
   * Called by VS Code when the user saves the document to a new location.
   */
  async saveAs(
    targetResource: vscode.Uri,
    cancellation: vscode.CancellationToken
  ): Promise<void> {
    const fileData = await this._delegate.getFileData();
    if (cancellation.isCancellationRequested) {
      return;
    }
    await vscode.workspace.fs.writeFile(targetResource, fileData);
  }

  /**
   * Called by VS Code to backup the edited document.
   *
   * These backups are used to implement hot exit.
   */
  async backup(
    destination: vscode.Uri,
    cancellation: vscode.CancellationToken
  ): Promise<vscode.CustomDocumentBackup> {
    await this.saveAs(destination, cancellation);

    return {
      id: destination.toString(),
      delete: async () => {
        try {
          await vscode.workspace.fs.delete(destination);
        } catch {
          // noop
        }
      },
    };
  }

  async save(cancellation: vscode.CancellationToken): Promise<void> {
    await this.saveAs(this.uri, cancellation);
  }
}

export class PngEditorProvider
  implements vscode.CustomEditorProvider<PngDocument>
{
  private static newPngFileId = 1;
  private static readonly viewType = "pixcil.png";

  public static register(context: vscode.ExtensionContext): vscode.Disposable {
    vscode.commands.registerCommand("pixcil.png.new", () => {
      const workspaceFolders = vscode.workspace.workspaceFolders;
      if (!workspaceFolders) {
        vscode.window.showErrorMessage(
          "Creating new Pixcil (PNG) files currently requires opening a workspace"
        );
        return;
      }

      const uri = vscode.Uri.joinPath(
        workspaceFolders[0].uri,
        `new-${PngEditorProvider.newPngFileId++}.png`
      ).with({ scheme: "untitled" });

      vscode.commands.executeCommand(
        "vscode.openWith",
        uri,
        PngEditorProvider.viewType
      );
    });

    return vscode.window.registerCustomEditorProvider(
      PngEditorProvider.viewType,
      new PngEditorProvider(context),
      {
        // TODO
        // For this demo extension, we enable `retainContextWhenHidden` which keeps the
        // webview alive even when it is not visible. You should avoid using this setting
        // unless is absolutely required as it does have memory overhead.
        webviewOptions: {
          retainContextWhenHidden: true,
        },
        supportsMultipleEditorsPerDocument: false,
      }
    );
  }

  /**
   * Tracks all known webviews
   */
  private readonly webviews = new WebviewCollection();

  private _requestId = 1;
  private readonly _callbacks = new Map<number, (response: any) => void>();

  private postMessageWithResponse<R = unknown>(
    panel: vscode.WebviewPanel,
    type: string,
    body: any
  ): Promise<R> {
    const requestId = this._requestId++;
    const p = new Promise<R>((resolve) =>
      this._callbacks.set(requestId, resolve)
    );
    panel.webview.postMessage({ type, requestId, body });
    return p;
  }

  async openCustomDocument(
    uri: vscode.Uri,
    openContext: { backupId?: string },
    _token: vscode.CancellationToken
  ): Promise<PngDocument> {
    const document: PngDocument = await PngDocument.create(
      uri,
      openContext.backupId,
      {
        getFileData: async () => {
          const webviewsForDocument = Array.from(
            this.webviews.get(document.uri)
          );
          if (!webviewsForDocument.length) {
            throw new Error("Could not find webview to save for");
          }
          const panel = webviewsForDocument[0];
          const response = await this.postMessageWithResponse<number[]>(
            panel,
            "getFileData",
            {}
          );
          return new Uint8Array(response);
        },
      }
    );

    const listeners: vscode.Disposable[] = [];

    listeners.push(
      document.onDidChange((e) => {
        // Tell VS Code that the document has been edited by the use.
        this._onDidChangeCustomDocument.fire({
          document,
          ...e,
        });
      })
    );

    listeners.push(
      document.onDidChangeContent((e) => {
        // Update all webviews when the document changes
        for (const webviewPanel of this.webviews.get(document.uri)) {
          this.postMessage(webviewPanel, "update", e.content);
        }
      })
    );

    document.onDidDispose(() => disposeAll(listeners));

    return document;
  }

  private postMessage(
    panel: vscode.WebviewPanel,
    type: string,
    body: any
  ): void {
    panel.webview.postMessage({ type, body });
  }

  constructor(private readonly _context: vscode.ExtensionContext) {}

  private getHtmlForWebview(webview: vscode.Webview): string {
    const wasmUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this._context.extensionUri, "assets", "pixcil.wasm")
    );
    const pixcilScriptUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this._context.extensionUri, "assets", "pixcil.js")
    );
    const styleUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this._context.extensionUri, "assets", "style.css")
    );

    // Use a nonce to whitelist which scripts can be run
    const nonce = getNonce();

    const connectSrc = `${wasmUri.scheme}://${wasmUri.authority}`.replace(
      "file+",
      "*"
    );

    return `
			<!DOCTYPE html>
			<html lang="en">
			<head>
        <meta charset="UTF-8">

				<!--
				Use a content security policy to only allow loading images from https or from our extension directory,
				and only allow scripts that have a specific nonce.
-->
        <meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src ${webview.cspSource} blob:; style-src ${webview.cspSource}; script-src 'nonce-${nonce}' 'wasm-unsafe-eval'; connect-src ${connectSrc};">

				<meta name="viewport" content="width=device-width, initial-scale=1.0">
        <link href="${styleUri}" rel="stylesheet" />

        <title>Pixcil</title>
      </head>
      <body>
        <div id="canvas-area">
          <canvas id="canvas"></canvas>
        </div>
				<script nonce="${nonce}" src="${pixcilScriptUri}"></script>
        <script nonce="${nonce}">
          const canvas = document.getElementById("canvas");
          const canvasArea = document.getElementById("canvas-area");
          const wasmPath = "${wasmUri}";
          const vscode = acquireVsCodeApi();
          Pixcil.App.load({wasmPath, canvas, canvasArea, parent: vscode})
                    .then(app => {
                       app.run()
                     })
                    .catch(e => console.warn(e));
        </script>
      </body>
      </html>`;
  }

  async resolveCustomEditor(
    document: PngDocument,
    webviewPanel: vscode.WebviewPanel,
    _token: vscode.CancellationToken
  ): Promise<void> {
    // Add the webview to our internal set of active webviews
    this.webviews.add(document.uri, webviewPanel);

    // Setup initial content for the webview
    webviewPanel.webview.options = {
      enableScripts: true,
    };
    webviewPanel.webview.html = this.getHtmlForWebview(webviewPanel.webview);

    webviewPanel.webview.onDidReceiveMessage((e) =>
      this.onMessage(webviewPanel, document, e)
    );

    webviewPanel.webview.onDidReceiveMessage((e) => {
      if (e.type === "ready") {
        this.postMessage(webviewPanel, "loadWorkspace", document.documentData);
      }
    });
  }

  // TODO: s/any/Message/
  private onMessage(
    webviewPanel: vscode.WebviewPanel,
    document: PngDocument,
    message: any
  ) {
    switch (message.type) {
      case "makeDirty":
        document.makeDirty();
        break;
      case "inputNumber":
        vscode.window
          .showInputBox({
            title: "foo",
            prompt: "Please input a number",
            validateInput: (param) => {
              var regex = /\d+/;
              return regex.test(param) ? null : "Not a number";
            },
          })
          .then((value) => {
            if (value) {
              this.postMessage(webviewPanel, "number", {
                id: message.inputId,
                number: value,
              });
            }
          });
        break;
      case "response": {
        // TODO: error check
        const callback = this._callbacks.get(message.requestId);
        callback?.(message.body);
        return;
      }
    }
  }

  private readonly _onDidChangeCustomDocument = new vscode.EventEmitter<
    vscode.CustomDocumentEditEvent<PngDocument>
  >();
  public readonly onDidChangeCustomDocument =
    this._onDidChangeCustomDocument.event;

  public saveCustomDocument(
    document: PngDocument,
    cancellation: vscode.CancellationToken
  ): Thenable<void> {
    return document.save(cancellation);
  }

  public saveCustomDocumentAs(
    document: PngDocument,
    destination: vscode.Uri,
    cancellation: vscode.CancellationToken
  ): Thenable<void> {
    return document.saveAs(destination, cancellation);
  }

  public revertCustomDocument(
    document: PngDocument,
    cancellation: vscode.CancellationToken
  ): Thenable<void> {
    return document.revert(cancellation);
  }

  public backupCustomDocument(
    document: PngDocument,
    context: vscode.CustomDocumentBackupContext,
    cancellation: vscode.CancellationToken
  ): Thenable<vscode.CustomDocumentBackup> {
    console.log("# Backup");
    return document.backup(context.destination, cancellation);
  }
}

/**
 * Tracks all webviews.
 */
class WebviewCollection {
  private readonly _webviews = new Set<{
    readonly resource: string;
    readonly webviewPanel: vscode.WebviewPanel;
  }>();

  /**
   * Get all known webviews for a given uri.
   */
  public *get(uri: vscode.Uri): Iterable<vscode.WebviewPanel> {
    const key = uri.toString();
    for (const entry of this._webviews) {
      if (entry.resource === key) {
        yield entry.webviewPanel;
      }
    }
  }

  /**
   * Add a new webview to the collection.
   */
  public add(uri: vscode.Uri, webviewPanel: vscode.WebviewPanel) {
    const entry = { resource: uri.toString(), webviewPanel };
    this._webviews.add(entry);

    webviewPanel.onDidDispose(() => {
      this._webviews.delete(entry);
    });
  }
}

export function getNonce() {
  let text = "";
  const possible =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}
