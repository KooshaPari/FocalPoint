import * as vscode from 'vscode';
import { execSync, spawn } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';

interface CompileResult {
  success: boolean;
  message: string;
  details?: string;
}

interface IRPreview {
  format: 'json' | 'yaml';
  content: string;
}

let statusBar: vscode.StatusBarItem;
const diagnosticsCollection = vscode.languages.createDiagnosticCollection('fpl');

export function activate(context: vscode.ExtensionContext) {
  console.log('FocalPoint FPL extension activated');

  // Create status bar item
  statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
  statusBar.text = '$(zap) FPL';
  statusBar.tooltip = 'FocalPoint FPL Tools';
  statusBar.show();

  // Register compile command
  const compileCmd = vscode.commands.registerCommand('focalpoint.compile', compileFile);
  context.subscriptions.push(compileCmd);

  // Register run command
  const runCmd = vscode.commands.registerCommand('focalpoint.run', runFile);
  context.subscriptions.push(runCmd);

  // Register preview IR command
  const previewCmd = vscode.commands.registerCommand('focalpoint.previewIr', previewIR);
  context.subscriptions.push(previewCmd);

  // Auto-compile on save
  const saveWatcher = vscode.workspace.onDidSaveTextDocument((doc) => {
    if (doc.languageId === 'fpl') {
      const config = vscode.workspace.getConfiguration('focalpoint');
      if (config.get<boolean>('autoCompileOnSave', true)) {
        compileFile();
      }
    }
  });
  context.subscriptions.push(saveWatcher);

  // Hover provider for rule descriptions
  const hoverProvider = vscode.languages.registerHoverProvider('fpl', {
    provideHover(document, position) {
      const line = document.lineAt(position.line).text;
      const wordRange = document.getWordRangeAtPosition(position);
      const word = wordRange ? document.getText(wordRange) : '';

      // Provide hover hints for FPL constructs
      const hints: { [key: string]: string } = {
        'rule': 'Define a rule with event/schedule trigger, conditions, and actions',
        'on_event': 'Trigger rule on event: on_event("event_type")',
        'on_schedule': 'Trigger rule on cron schedule: on_schedule("0 9 * * *")',
        'reward': 'Reward user action with credits: reward("event", credits=N)',
        'penalize': 'Penalize user action: penalize("event", credits=N)',
        'block': 'Block apps during time window: block(["app1"], "period")',
        'notify': 'Send notification to user: notify("message")',
        'grant_credit': 'Grant wallet credits: grant_credit(N)',
        'confidence_gte': 'Condition: event confidence >= threshold',
        'payload_eq': 'Condition: match event payload: payload_eq(field, value)',
        'task': 'Define a scheduled task with priority and deadline',
        'connector': 'Configure external data source (Canvas, GCal, etc.)',
        'scene': 'Mascot scene with pose, emotion, and haptic feedback',
        'enforcement': 'Define app/domain blocking policy',
      };

      if (hints[word]) {
        return new vscode.Hover(new vscode.MarkdownString(`**FPL:** ${hints[word]}`));
      }

      return null;
    },
  });
  context.subscriptions.push(hoverProvider);

  // Code lens provider for rule compilation hints
  const codeLensProvider = vscode.languages.registerCodeLensProvider('fpl', {
    provideCodeLenses(document) {
      const lenses: vscode.CodeLens[] = [];
      const text = document.getText();
      const ruleRegex = /rule\s*\(/g;
      let match;

      while ((match = ruleRegex.exec(text)) !== null) {
        const startPos = document.positionAt(match.index);
        const range = new vscode.Range(startPos, startPos);

        lenses.push(
          new vscode.CodeLens(range, {
            title: '$(play) Compile to IR',
            command: 'focalpoint.compile',
            tooltip: 'Compile this FPL file to intermediate representation',
          })
        );

        lenses.push(
          new vscode.CodeLens(range, {
            title: '$(preview) Show IR Hash',
            command: 'focalpoint.previewIr',
            tooltip: 'Preview intermediate representation and hash',
          })
        );
      }

      return lenses;
    },
  });
  context.subscriptions.push(codeLensProvider);
}

async function compileFile(): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== 'fpl') {
    vscode.window.showErrorMessage('No active FPL file');
    return;
  }

  const filePath = editor.document.fileName;
  const config = vscode.workspace.getConfiguration('focalpoint');
  const focusBinary = config.get<string>('focusBinary', 'focus');

  try {
    statusBar.text = '$(loading~spin) FPL: Compiling...';

    // Save before compile
    if (editor.document.isDirty) {
      await editor.document.save();
    }

    // Run focus rules import --dry-run
    const output = execSync(`${focusBinary} rules import --dry-run "${filePath}"`, {
      encoding: 'utf-8',
      stdio: 'pipe',
    });

    statusBar.text = '$(check) FPL: Compiled';
    diagnosticsCollection.clear();

    vscode.window.showInformationMessage(`✓ FPL compiled successfully\n${output.substring(0, 200)}`);
  } catch (error: any) {
    statusBar.text = '$(error) FPL: Compile failed';

    const stderr = error.stderr?.toString() || error.message || 'Unknown error';
    const match = stderr.match(/line (\d+):/);
    const lineNum = match ? parseInt(match[1]) - 1 : 0;

    // Set diagnostic
    const range = new vscode.Range(
      new vscode.Position(lineNum, 0),
      new vscode.Position(lineNum, 200)
    );
    const diagnostic = new vscode.Diagnostic(
      range,
      stderr.split('\n')[0],
      vscode.DiagnosticSeverity.Error
    );

    diagnosticsCollection.set(editor.document.uri, [diagnostic]);
    vscode.window.showErrorMessage(`FPL compile failed: ${stderr.substring(0, 300)}`);
  }
}

async function runFile(): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== 'fpl') {
    vscode.window.showErrorMessage('No active FPL file');
    return;
  }

  const filePath = editor.document.fileName;
  const config = vscode.workspace.getConfiguration('focalpoint');
  const focusBinary = config.get<string>('focusBinary', 'focus');
  const dbPath = expandPath(config.get<string>('database', '~/Library/Application Support/focalpoint/core.db'));

  try {
    statusBar.text = '$(loading~spin) FPL: Importing...';

    if (editor.document.isDirty) {
      await editor.document.save();
    }

    // Run focus rules import (actual import to DB)
    const output = execSync(
      `${focusBinary} rules import --db="${dbPath}" "${filePath}"`,
      { encoding: 'utf-8', stdio: 'pipe' }
    );

    statusBar.text = '$(check) FPL: Imported';
    diagnosticsCollection.clear();

    vscode.window.showInformationMessage(
      `✓ Rule imported to local DB\n${output.substring(0, 200)}`
    );
  } catch (error: any) {
    statusBar.text = '$(error) FPL: Import failed';
    const stderr = error.stderr?.toString() || error.message;
    vscode.window.showErrorMessage(`FPL import failed: ${stderr.substring(0, 300)}`);
  }
}

async function previewIR(): Promise<void> {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== 'fpl') {
    vscode.window.showErrorMessage('No active FPL file');
    return;
  }

  const filePath = editor.document.fileName;
  const config = vscode.workspace.getConfiguration('focalpoint');
  const focusBinary = config.get<string>('focusBinary', 'focus');
  const format = config.get<string>('irPreviewFormat', 'json');

  try {
    statusBar.text = '$(loading~spin) FPL: Generating IR...';

    if (editor.document.isDirty) {
      await editor.document.save();
    }

    // Use focus eval to get IR
    const output = execSync(
      `${focusBinary} eval --format=${format} "${filePath}"`,
      { encoding: 'utf-8', stdio: 'pipe' }
    );

    statusBar.text = '$(check) FPL: IR Ready';

    // Open IR in split pane
    const irUri = vscode.Uri.from({
      scheme: 'untitled',
      path: path.join(os.tmpdir(), `fpl-ir-${Date.now()}.${format}`),
    });

    const irDoc = await vscode.workspace.openTextDocument(irUri);
    const irEditor = await vscode.window.showTextDocument(irDoc, vscode.ViewColumn.Two);

    // Parse format for syntax highlighting
    const language = format === 'yaml' ? 'yaml' : 'json';
    await vscode.languages.setTextDocumentLanguage(irDoc, language);

    // Insert IR content
    await irEditor.edit((editBuilder) => {
      editBuilder.insert(new vscode.Position(0, 0), output);
    });
  } catch (error: any) {
    statusBar.text = '$(error) FPL: IR failed';
    const stderr = error.stderr?.toString() || error.message;
    vscode.window.showErrorMessage(`FPL IR generation failed: ${stderr.substring(0, 300)}`);
  }
}

function expandPath(inputPath: string): string {
  if (inputPath.startsWith('~')) {
    return path.join(os.homedir(), inputPath.slice(1));
  }
  return inputPath;
}

export function deactivate() {
  diagnosticsCollection.dispose();
  statusBar.dispose();
}
