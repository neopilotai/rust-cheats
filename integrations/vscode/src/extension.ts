import * as vscode from "vscode";
import { execFile } from "node:child_process";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

export function activate(context: vscode.ExtensionContext): void {
  const command = vscode.commands.registerCommand("rustCheats.search", async () => {
    const query = await vscode.window.showInputBox({
      prompt: "Search local programming cheat sheets",
      placeHolder: "e.g. Rust error handling",
    });
    if (!query?.trim()) return;

    try {
      const { stdout } = await execFileAsync("rust-cheats", ["search", query], {
        timeout: 5000,
        maxBuffer: 1024 * 1024,
      });
      const document = await vscode.workspace.openTextDocument({
        language: "plaintext",
        content: stdout || "No results.",
      });
      await vscode.window.showTextDocument(document, { preview: true });
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      void vscode.window.showErrorMessage(`rust-cheats failed: ${message}`);
    }
  });
  context.subscriptions.push(command);
}

export function deactivate(): void {}
