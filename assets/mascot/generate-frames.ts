#!/usr/bin/env node
/**
 * Frame generator: exports all 20 Coachy states as optimized static SVGs.
 * Run: npx ts-node generate-frames.ts
 * Output: frames/coachy-<expr>-<intensity>.svg
 */

import * as fs from 'fs';
import * as path from 'path';
import { renderToString } from 'react-dom/server';
import Mascot from './mascot.svg';

type Expression = 'idle' | 'happy' | 'focused' | 'concerned' | 'sleeping';
type Intensity = 'calm' | 'active' | 'intense' | 'post-rule';

const expressions: Expression[] = ['idle', 'happy', 'focused', 'concerned', 'sleeping'];
const intensities: Intensity[] = ['calm', 'active', 'intense', 'post-rule'];

const framesDir = path.join(__dirname, 'frames');

// Ensure frames directory exists
if (!fs.existsSync(framesDir)) {
  fs.mkdirSync(framesDir, { recursive: true });
}

let totalSize = 0;
let frameCount = 0;

console.log('Generating 20 Coachy frames...\n');

for (const expr of expressions) {
  for (const intensity of intensities) {
    const filename = `coachy-${expr}-${intensity}.svg`;
    const filepath = path.join(framesDir, filename);

    // Render component with frozen breath (middle of cycle for neutral appearance)
    const svg = renderToString(
      Mascot({
        expression: expr,
        intensity: intensity,
        breathPhase: 0.5, // Middle of breathing cycle
        blinkOpen: true,
        size: 256,
        animated: false, // Static frames
      })
    );

    // Write unoptimized frame
    fs.writeFileSync(filepath, svg);
    const sizeBytes = fs.statSync(filepath).size;
    totalSize += sizeBytes;
    frameCount++;

    console.log(`  ${frameCount}. ${filename} (${sizeBytes} bytes)`);
  }
}

console.log(`\n✓ Generated ${frameCount} frames`);
console.log(`✓ Total size: ${totalSize} bytes (~${Math.round(totalSize / 1024)} KB)`);
console.log(`✓ Gzipped estimate: ~${Math.round(totalSize * 0.35 / 1024)} KB`);
console.log(`✓ Output: ${framesDir}/`);
console.log('\nNext step: run `svgo frames/ -o frames-optimized/` for compression');
