import * as vscode from "vscode";
import { Disposable, disposeAll } from "./dispose";

interface PngDrawDocumentDelegate {
  getFileData(): Promise<Uint8Array>;
}

class PngDrawDocument extends Disposable implements vscode.CustomDocument {
  private readonly _uri: vscode.Uri;
  private readonly _delegate: PngDrawDocumentDelegate;
  private _documentData: Uint8Array;

  static async create(
    uri: vscode.Uri,
    backupId: string | undefined,
    delegate: PngDrawDocumentDelegate
  ): Promise<PngDrawDocument | PromiseLike<PngDrawDocument>> {
    // If we have a backup, read that. Otherwise read the resource from the workspace
    const dataFile =
      typeof backupId === "string" ? vscode.Uri.parse(backupId) : uri;
    const fileData = await PngDrawDocument.readFile(dataFile);
    return new PngDrawDocument(uri, fileData, delegate);
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
    delegate: PngDrawDocumentDelegate
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

export class PngDrawEditorProvider
  implements vscode.CustomEditorProvider<PngDrawDocument>
{
  public static register(context: vscode.ExtensionContext): vscode.Disposable {
    throw new Error("TODO");
  }

  async openCustomDocument(
    uri: vscode.Uri,
    openContext: { backupId?: string },
    _token: vscode.CancellationToken
  ): Promise<PngDrawDocument> {
    throw new Error("TODO");
  }

  async resolveCustomEditor(
    document: PngDrawDocument,
    webviewPanel: vscode.WebviewPanel,
    _token: vscode.CancellationToken
  ): Promise<void> {
    throw new Error("TODO");
  }

  private readonly _onDidChangeCustomDocument = new vscode.EventEmitter<
    vscode.CustomDocumentEditEvent<PngDrawDocument>
  >();
  public readonly onDidChangeCustomDocument =
    this._onDidChangeCustomDocument.event;

  public saveCustomDocument(
    document: PngDrawDocument,
    cancellation: vscode.CancellationToken
  ): Thenable<void> {
    throw new Error("TODO");
  }

  public saveCustomDocumentAs(
    document: PngDrawDocument,
    destination: vscode.Uri,
    cancellation: vscode.CancellationToken
  ): Thenable<void> {
    throw new Error("TODO");
  }

  public revertCustomDocument(
    document: PngDrawDocument,
    cancellation: vscode.CancellationToken
  ): Thenable<void> {
    throw new Error("TODO");
  }

  public backupCustomDocument(
    document: PngDrawDocument,
    context: vscode.CustomDocumentBackupContext,
    cancellation: vscode.CancellationToken
  ): Thenable<vscode.CustomDocumentBackup> {
    throw new Error("TODO");
  }
}
