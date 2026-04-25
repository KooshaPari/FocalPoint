#!/usr/bin/env node
/**
 * Direct SVG frame generator for Coachy states.
 * Outputs 20 optimized SVGs without React dependency.
 * Run: node gen-frames.js
 */

const fs = require('fs');
const path = require('path');

const tokens = require('./mascot-tokens.json');

const expressions = ['idle', 'happy', 'focused', 'concerned', 'sleeping'];
const intensities = ['calm', 'active', 'intense', 'post-rule'];

const c = tokens.colors;
const d = tokens.dimensions;

const framesDir = path.join(__dirname, 'frames');
if (!fs.existsSync(framesDir)) {
  fs.mkdirSync(framesDir, { recursive: true });
}

function generateMascot(expression, intensity) {
  const breathAmount = 0; // Middle of cycle
  const blinkOpen = true;

  // Eye config
  let eyeConfig = {
    leftEyeY: 32,
    rightEyeY: 32,
    eyeGap: 18,
    squintAmount: 0,
  };

  if (expression === 'happy') {
    eyeConfig = { leftEyeY: 34, rightEyeY: 34, eyeGap: 20, squintAmount: 0.6 };
  } else if (expression === 'focused') {
    eyeConfig = { leftEyeY: 29, rightEyeY: 29, eyeGap: 17, squintAmount: 0.3 };
  } else if (expression === 'concerned') {
    eyeConfig = { leftEyeY: 33, rightEyeY: 30, eyeGap: 18, squintAmount: 0 };
  } else if (expression === 'sleeping') {
    eyeConfig = { leftEyeY: 40, rightEyeY: 40, eyeGap: 18, squintAmount: 1 };
  }

  // Mouth config
  let mouthConfig = { y: 60, width: 20, height: 8, type: 'neutral' };
  if (expression === 'happy') mouthConfig = { y: 58, width: 28, height: 12, type: 'smile' };
  else if (expression === 'focused') mouthConfig = { y: 62, width: 16, height: 6, type: 'line' };
  else if (expression === 'concerned') mouthConfig = { y: 61, width: 24, height: 10, type: 'wavy' };
  else if (expression === 'sleeping') mouthConfig = { y: 66, width: 0, height: 0, type: 'none' };

  if (intensity === 'active') {
    mouthConfig.y -= 1;
    mouthConfig.height += 1;
  } else if (intensity === 'intense' && expression === 'focused') {
    mouthConfig.width -= 4;
    mouthConfig.height -= 1;
  } else if (intensity === 'post-rule') {
    mouthConfig.type = 'happy';
    mouthConfig.height += 2;
  }

  const bodyColor = intensity === 'post-rule' ? c.ok : intensity === 'intense' ? c.primaryDark : c.primary;

  let svg = `<svg viewBox="0 0 256 320" xmlns="http://www.w3.org/2000/svg" width="256" height="320">
  <!-- Cape -->
  <path d="M 96 80 Q 80 120 78 160 L 78 180 Q 78 200 96 210 L 160 210 Q 178 200 178 180 L 178 160 Q 176 120 160 80"
    fill="${bodyColor}" stroke="${c.ink}" stroke-width="2" opacity="${intensity === 'calm' ? '0.8' : '1'}"/>

  <!-- Body -->
  <ellipse cx="128" cy="140" rx="32" ry="36" fill="${bodyColor}" stroke="${c.ink}" stroke-width="2"/>
  <ellipse cx="128" cy="145" rx="21" ry="24" fill="${c.primaryLight}" opacity="0.4"/>

  <!-- Head -->
  <circle cx="128" cy="60" r="48" fill="${bodyColor}" stroke="${c.ink}" stroke-width="2"/>`;

  // Blush for happy/active
  if (expression === 'happy' || intensity === 'active') {
    svg += `
  <circle cx="90" cy="65" r="6" fill="${c.warn}" opacity="0.3"/>
  <circle cx="166" cy="65" r="6" fill="${c.warn}" opacity="0.3"/>`;
  }

  // Eyes
  svg += `
  <!-- Left eye -->
  <circle cx="104" cy="${eyeConfig.leftEyeY}" r="8" fill="${c.textPrimary}" stroke="${c.ink}" stroke-width="2"/>`;

  if (blinkOpen && expression !== 'sleeping') {
    const pupilRadius = 8 * (1 - eyeConfig.squintAmount * 0.4);
    svg += `
  <circle cx="104" cy="${eyeConfig.leftEyeY}" r="${pupilRadius}" fill="${c.ink}"/>`;
  } else {
    svg += `
  <line x1="98" y1="${eyeConfig.leftEyeY}" x2="110" y2="${eyeConfig.leftEyeY}" stroke="${c.ink}" stroke-width="2" stroke-linecap="round"/>`;
  }

  svg += `

  <!-- Right eye -->
  <circle cx="152" cy="${eyeConfig.rightEyeY}" r="8" fill="${c.textPrimary}" stroke="${c.ink}" stroke-width="2"/>`;

  if (blinkOpen && expression !== 'sleeping') {
    const pupilRadius = 8 * (1 - eyeConfig.squintAmount * 0.4);
    svg += `
  <circle cx="152" cy="${eyeConfig.rightEyeY}" r="${pupilRadius}" fill="${c.ink}"/>`;
  } else {
    svg += `
  <line x1="146" y1="${eyeConfig.rightEyeY}" x2="158" y2="${eyeConfig.rightEyeY}" stroke="${c.ink}" stroke-width="2" stroke-linecap="round"/>`;
  }

  // Brows
  if (expression === 'focused') {
    svg += `
  <!-- Focused brows -->
  <line x1="96" y1="22" x2="112" y2="18" stroke="${c.ink}" stroke-width="2" stroke-linecap="round"/>
  <line x1="144" y1="18" x2="160" y2="22" stroke="${c.ink}" stroke-width="2" stroke-linecap="round"/>`;
  } else if (expression === 'concerned') {
    svg += `
  <!-- Concerned brows -->
  <line x1="96" y1="20" x2="112" y2="24" stroke="${c.block}" stroke-width="2" stroke-linecap="round"/>
  <line x1="144" y1="24" x2="160" y2="20" stroke="${c.block}" stroke-width="2" stroke-linecap="round"/>`;
  }

  // Mouth
  const mx = mouthConfig.width / 2;
  if (mouthConfig.type === 'smile') {
    svg += `
  <path d="M ${128 - mx} ${mouthConfig.y} Q 128 ${mouthConfig.y + mouthConfig.height} ${128 + mx} ${mouthConfig.y}"
    stroke="${c.ink}" stroke-width="2" fill="none" stroke-linecap="round"/>`;
  } else if (mouthConfig.type === 'neutral') {
    svg += `
  <line x1="${128 - mx}" y1="${mouthConfig.y}" x2="${128 + mx}" y2="${mouthConfig.y}"
    stroke="${c.ink}" stroke-width="2" stroke-linecap="round"/>`;
  } else if (mouthConfig.type === 'line') {
    svg += `
  <line x1="${128 - mx}" y1="${mouthConfig.y}" x2="${128 + mx}" y2="${mouthConfig.y}"
    stroke="${c.block}" stroke-width="2" stroke-linecap="round"/>`;
  } else if (mouthConfig.type === 'wavy') {
    svg += `
  <path d="M ${128 - mx} ${mouthConfig.y} Q ${128 - mx / 2} ${mouthConfig.y + mouthConfig.height} 128 ${mouthConfig.y} Q ${128 + mx / 2} ${mouthConfig.y - 2} ${128 + mx} ${mouthConfig.y}"
    stroke="${c.warn}" stroke-width="2" fill="none" stroke-linecap="round"/>`;
  }

  // Badge
  if (intensity !== 'calm') {
    let badgeColor = c.info;
    let badgeContent = '';
    if (intensity === 'post-rule') {
      badgeColor = c.ok;
      badgeContent = '<text x="200" y="46" text-anchor="middle" font-size="14" fill="#0e0f13" font-weight="bold">+</text>';
    } else if (intensity === 'intense') {
      badgeColor = c.block;
      badgeContent = '<polygon points="200,32 206,48 194,48" fill="#f2f3f5"/>';
    } else if (intensity === 'active') {
      badgeContent = '<circle cx="200" cy="40" r="5" fill="#f2f3f5"/>';
    }

    svg += `
  <!-- Badge -->
  <circle cx="200" cy="40" r="12" fill="${badgeColor}" stroke="${c.ink}" stroke-width="1"/>
  ${badgeContent}`;
  }

  svg += `
</svg>`;

  return svg;
}

let totalSize = 0;
let frameCount = 0;

console.log('Generating 20 Coachy frames...\n');

for (const expr of expressions) {
  for (const intensity of intensities) {
    const filename = `coachy-${expr}-${intensity}.svg`;
    const filepath = path.join(framesDir, filename);
    const svg = generateMascot(expr, intensity);

    fs.writeFileSync(filepath, svg);
    const sizeBytes = svg.length;
    totalSize += sizeBytes;
    frameCount++;

    console.log(`  ${frameCount}. ${filename} (${sizeBytes} bytes)`);
  }
}

console.log(`\n✓ Generated ${frameCount} frames`);
console.log(`✓ Total size: ${totalSize} bytes (~${Math.round(totalSize / 1024)} KB)`);
console.log(`✓ Gzipped estimate: ~${Math.round(totalSize * 0.35 / 1024)} KB`);
console.log(`✓ Output: ${framesDir}/`);
