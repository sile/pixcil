import * as vscode from "vscode";
import { Disposable, disposeAll } from "./dispose";

interface PngDocumentDelegate {
  getFileData(): Promise<Uint8Array>;
}

// TODO: delete
interface PngEdit {
  readonly color: string;
  readonly stroke: ReadonlyArray<[number, number]>;
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
      readonly edits: readonly PngEdit[];
    }>()
  );

  /**
   * Fired to notify webviews that the document has changed.
   */
  public readonly onDidChangeContent = this._onDidChangeDocument.event;

  private readonly _onDidChange = this._register(
    new vscode.EventEmitter<{
      readonly label: string;
      undo(): void;
      redo(): void;
    }>()
  );

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

  // TODO
  // private readonly _onDidDispose = this._register(
  //   new vscode.EventEmitter<void>()
  // );

  // /**
  //  * Called by VS Code when there are no more references to the document.
  //  *
  //  * This happens when all editors for it have been closed.
  //  */
  // dispose(): void {
  //   this._onDidDispose.fire();
  //   super.dispose();
  // }
}

export class PngEditorProvider
  implements vscode.CustomEditorProvider<PngDocument>
{
  private static newPngFileId = 1;
  private static readonly viewType = "pixcil.png";

  public static register(_context: vscode.ExtensionContext): vscode.Disposable {
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
      new PngEditorProvider(),
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
    console.log("hello0");
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

    console.log("hello1");
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
          this.postMessage(webviewPanel, "update", {
            edits: e.edits,
            content: e.content,
          });
        }
      })
    );

    document.onDidDispose(() => disposeAll(listeners));

    console.log("hello2");
    return document;
  }

  private postMessage(
    panel: vscode.WebviewPanel,
    type: string,
    body: any
  ): void {
    panel.webview.postMessage({ type, body });
  }

  private getHtmlForWebview(_webview: vscode.Webview): string {
    // const styleResetUri = webview.asWebviewUri(vscode.Uri.joinPath(
    // 	this._context.extensionUri, 'media', 'reset.css'));

    return `
			<!DOCTYPE html>
			<html lang="en">
			<head>
        <meta charset="UTF-8">
      </head>
      <body>
hello
      </body>
      </html>`;
  }

  async resolveCustomEditor(
    document: PngDocument,
    webviewPanel: vscode.WebviewPanel,
    _token: vscode.CancellationToken
  ): Promise<void> {
    console.log("hello3");
    // Add the webview to our internal set of active webviews
    this.webviews.add(document.uri, webviewPanel);

    // Setup initial content for the webview
    webviewPanel.webview.options = {
      enableScripts: true,
    };
    webviewPanel.webview.html = this.getHtmlForWebview(webviewPanel.webview);

    webviewPanel.webview.onDidReceiveMessage((e) =>
      this.onMessage(document, e)
    );

    // Wait for the webview to be properly ready before we init
    webviewPanel.webview.onDidReceiveMessage((e) => {
      if (e.type === "ready") {
        if (document.uri.scheme === "untitled") {
          this.postMessage(webviewPanel, "init", {
            untitled: true,
            editable: true,
          });
        } else {
          const editable = vscode.workspace.fs.isWritableFileSystem(
            document.uri.scheme
          );

          this.postMessage(webviewPanel, "init", {
            value: document.documentData,
            editable,
          });
        }
      }
    });
  }

  private onMessage(_document: PngDocument, message: any) {
    switch (message.type) {
      case "stroke":
        // TODO
        // document.makeEdit(message as PngEdit);
        return;

      case "response": {
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
    throw new Error("TODO");
  }

  public saveCustomDocumentAs(
    document: PngDocument,
    destination: vscode.Uri,
    cancellation: vscode.CancellationToken
  ): Thenable<void> {
    throw new Error("TODO");
  }

  public revertCustomDocument(
    document: PngDocument,
    cancellation: vscode.CancellationToken
  ): Thenable<void> {
    throw new Error("TODO");
  }

  public backupCustomDocument(
    document: PngDocument,
    context: vscode.CustomDocumentBackupContext,
    cancellation: vscode.CancellationToken
  ): Thenable<vscode.CustomDocumentBackup> {
    throw new Error("TODO");
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
