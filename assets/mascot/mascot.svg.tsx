import React, { useMemo } from 'react';
import tokens from './mascot-tokens.json';

/**
 * Coachy — FocalPoint parametric mascot SVG.
 * Props: expression (idle|happy|focused|concerned|sleeping)
 *        intensity (calm|active|intense|post-rule)
 *        breathPhase [0-1] — oscillates continuously
 *        blinkOpen boolean — eye state
 *
 * Emits deterministic parametric SVG via <g> groups.
 * Each body part parameterized by single prop.
 */

interface MascotProps {
  expression?: 'idle' | 'happy' | 'focused' | 'concerned' | 'sleeping';
  intensity?: 'calm' | 'active' | 'intense' | 'post-rule';
  breathPhase?: number; // [0-1]
  blinkOpen?: boolean;
  size?: number;
  animated?: boolean;
}

const Mascot: React.FC<MascotProps> = ({
  expression = 'idle',
  intensity = 'calm',
  breathPhase = 0.5,
  blinkOpen = true,
  size = 256,
  animated = false,
}) => {
  const c = tokens.colors;
  const d = tokens.dimensions;
  const scale = size / 256;

  // Compute breathing offset: sinusoidal sway
  const breathAmount = Math.sin(breathPhase * Math.PI * 2) * (intensity === 'calm' ? 2 : 4);

  // Eye parameters by expression
  const eyeConfig = useMemo(() => {
    const baseEyeY = 32 - breathAmount;
    const baseEyeGap = 18;

    switch (expression) {
      case 'happy':
        return {
          leftEyeY: baseEyeY + 2,
          rightEyeY: baseEyeY + 2,
          eyeGap: baseEyeGap + 2,
          squintAmount: 0.6, // Happy squint
        };
      case 'focused':
        return {
          leftEyeY: baseEyeY - 3,
          rightEyeY: baseEyeY - 3,
          eyeGap: baseEyeGap - 1,
          squintAmount: 0.3, // Intense focus
        };
      case 'concerned':
        return {
          leftEyeY: baseEyeY + 1,
          rightEyeY: baseEyeY - 2,
          eyeGap: baseEyeGap,
          squintAmount: 0, // Worried asymmetry
        };
      case 'sleeping':
        return {
          leftEyeY: baseEyeY + 8,
          rightEyeY: baseEyeY + 8,
          eyeGap: baseEyeGap,
          squintAmount: 1, // Fully closed
        };
      case 'idle':
      default:
        return {
          leftEyeY: baseEyeY,
          rightEyeY: baseEyeY,
          eyeGap: baseEyeGap,
          squintAmount: 0,
        };
    }
  }, [expression, breathAmount]);

  // Mouth parameters by expression + intensity
  const mouthConfig = useMemo(() => {
    const baseY = 60;
    const config: Record<string, any> = {
      idle: { y: baseY, width: 20, height: 8, type: 'neutral' },
      happy: { y: baseY - 2, width: 28, height: 12, type: 'smile' },
      focused: { y: baseY + 2, width: 16, height: 6, type: 'line' },
      concerned: { y: baseY + 1, width: 24, height: 10, type: 'wavy' },
      sleeping: { y: baseY + 6, width: 0, height: 0, type: 'none' },
    };

    let mouth = config[expression] || config.idle;

    // Intensity modulation
    if (intensity === 'active' && expression !== 'sleeping') {
      mouth.y -= 1;
      mouth.height += 1;
    } else if (intensity === 'intense' && expression === 'focused') {
      mouth.width -= 4;
      mouth.height -= 1;
    } else if (intensity === 'post-rule') {
      mouth.type = 'happy';
      mouth.height += 2;
    }

    return mouth;
  }, [expression, intensity]);

  // Body color by intensity
  const bodyColor = intensity === 'post-rule' ? c.ok : intensity === 'intense' ? c.primaryDark : c.primary;

  return (
    <svg
      viewBox={`0 0 256 320`}
      width={size}
      height={(size * 320) / 256}
      xmlns="http://www.w3.org/2000/svg"
      style={{ background: 'transparent' }}
    >
      {/* Cape (behind body) */}
      <g id="cape" opacity={intensity === 'calm' ? 0.8 : 1}>
        <path
          d={`M 96 80 Q 80 120 78 160 L 78 180 Q 78 200 96 210 L 160 210 Q 178 200 178 180 L 178 160 Q 176 120 160 80`}
          fill={bodyColor}
          stroke={c.ink}
          strokeWidth={d.strokeWidth}
        />
      </g>

      {/* Body */}
      <g id="body">
        <ellipse cx="128" cy="140" rx={d.bodyWidth / 2} ry={d.bodyHeight / 2} fill={bodyColor} stroke={c.ink} strokeWidth={d.strokeWidth} />

        {/* Belly highlight */}
        <ellipse cx="128" cy="145" rx={d.bodyWidth / 3} ry={d.bodyHeight / 3} fill={c.primaryLight} opacity={0.4} />
      </g>

      {/* Head */}
      <g id="head" transform={`translate(0, ${breathAmount * 0.5})`}>
        <circle cx="128" cy="60" r={d.headRadius} fill={bodyColor} stroke={c.ink} strokeWidth={d.strokeWidth} />

        {/* Blush (active/happy) */}
        {expression === 'happy' || intensity === 'active' ? (
          <>
            <circle cx="90" cy="65" r="6" fill={c.warn} opacity="0.3" />
            <circle cx="166" cy="65" r="6" fill={c.warn} opacity="0.3" />
          </>
        ) : null}
      </g>

      {/* Eyes */}
      <g id="eyes">
        {/* Left eye */}
        <g>
          <circle cx="104" cy={eyeConfig.leftEyeY} r={d.eyeRadius} fill={c.textPrimary} stroke={c.ink} strokeWidth={d.strokeWidth} />
          {/* Left pupil */}
          {blinkOpen && expression !== 'sleeping' ? (
            <circle cx="104" cy={eyeConfig.leftEyeY} r={d.eyeRadius * (1 - eyeConfig.squintAmount * 0.4)} fill={c.ink} />
          ) : (
            <line
              x1="98"
              y1={eyeConfig.leftEyeY}
              x2="110"
              y2={eyeConfig.leftEyeY}
              stroke={c.ink}
              strokeWidth={d.strokeWidth}
              strokeLinecap="round"
            />
          )}
        </g>

        {/* Right eye */}
        <g>
          <circle cx="152" cy={eyeConfig.rightEyeY} r={d.eyeRadius} fill={c.textPrimary} stroke={c.ink} strokeWidth={d.strokeWidth} />
          {/* Right pupil */}
          {blinkOpen && expression !== 'sleeping' ? (
            <circle cx="152" cy={eyeConfig.rightEyeY} r={d.eyeRadius * (1 - eyeConfig.squintAmount * 0.4)} fill={c.ink} />
          ) : (
            <line
              x1="146"
              y1={eyeConfig.rightEyeY}
              x2="158"
              y2={eyeConfig.rightEyeY}
              stroke={c.ink}
              strokeWidth={d.strokeWidth}
              strokeLinecap="round"
            />
          )}
        </g>

        {/* Brows (expression indicator) */}
        {expression === 'focused' && (
          <>
            <line x1="96" y1="22" x2="112" y2="18" stroke={c.ink} strokeWidth={d.strokeWidth} strokeLinecap="round" />
            <line x1="144" y1="18" x2="160" y2="22" stroke={c.ink} strokeWidth={d.strokeWidth} strokeLinecap="round" />
          </>
        )}
        {expression === 'concerned' && (
          <>
            <line x1="96" y1="20" x2="112" y2="24" stroke={c.block} strokeWidth={d.strokeWidth} strokeLinecap="round" />
            <line x1="144" y1="24" x2="160" y2="20" stroke={c.block} strokeWidth={d.strokeWidth} strokeLinecap="round" />
          </>
        )}
      </g>

      {/* Mouth */}
      <g id="mouth">
        {mouthConfig.type === 'smile' && (
          <path
            d={`M ${128 - mouthConfig.width / 2} ${mouthConfig.y} Q 128 ${mouthConfig.y + mouthConfig.height} ${128 + mouthConfig.width / 2} ${mouthConfig.y}`}
            stroke={c.ink}
            strokeWidth={d.strokeWidth}
            fill="none"
            strokeLinecap="round"
          />
        )}
        {mouthConfig.type === 'neutral' && (
          <line
            x1={128 - mouthConfig.width / 2}
            y1={mouthConfig.y}
            x2={128 + mouthConfig.width / 2}
            y2={mouthConfig.y}
            stroke={c.ink}
            strokeWidth={d.strokeWidth}
            strokeLinecap="round"
          />
        )}
        {mouthConfig.type === 'line' && (
          <line
            x1={128 - mouthConfig.width / 2}
            y1={mouthConfig.y}
            x2={128 + mouthConfig.width / 2}
            y2={mouthConfig.y}
            stroke={c.block}
            strokeWidth={d.strokeWidth}
            strokeLinecap="round"
          />
        )}
        {mouthConfig.type === 'wavy' && (
          <path
            d={`M ${128 - mouthConfig.width / 2} ${mouthConfig.y} Q ${128 - mouthConfig.width / 4} ${mouthConfig.y + mouthConfig.height} 128 ${mouthConfig.y} Q ${128 + mouthConfig.width / 4} ${mouthConfig.y - 2} ${128 + mouthConfig.width / 2} ${mouthConfig.y}`}
            stroke={c.warn}
            strokeWidth={d.strokeWidth}
            fill="none"
            strokeLinecap="round"
          />
        )}
      </g>

      {/* Badge: intensity indicator (top-right) */}
      <g id="badge" opacity={intensity !== 'calm' ? 1 : 0}>
        <circle
          cx="200"
          cy="40"
          r="12"
          fill={intensity === 'post-rule' ? c.ok : intensity === 'intense' ? c.block : c.info}
          stroke={c.ink}
          strokeWidth="1"
        />
        {intensity === 'active' && <circle cx="200" cy="40" r="5" fill={c.textPrimary} />}
        {intensity === 'intense' && <polygon points="200,32 206,48 194,48" fill={c.textPrimary} />}
        {intensity === 'post-rule' && <text x="200" y="46" textAnchor="middle" fontSize="12" fill={c.ink} fontWeight="bold">+</text>}
      </g>

      {/* Animation defs (if animated) */}
      {animated && (
        <defs>
          <style>
            {`
              @keyframes breathe {
                0%, 100% { transform: translateY(0); }
                50% { transform: translateY(2px); }
              }
              #body { animation: breathe ${tokens.durations.breathe}ms infinite ${tokens.easing.breathe}; }
              #head { animation: breathe ${tokens.durations.breathe}ms infinite ${tokens.easing.breathe}; }
            `}
          </style>
        </defs>
      )}
    </svg>
  );
};

export default Mascot;
