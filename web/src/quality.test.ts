// PERF-022 (host part): the start mode of the automatic quality tier.
import { describe, expect, it } from 'vitest';
import { qualityMode } from './quality';

describe('qualityMode', () => {
  it('is automatic by default', () => {
    expect(qualityMode('', false)).toBe('auto');
    expect(qualityMode('?seed=17', false)).toBe('auto');
  });
  it('pins high in automated browsers unless the URL asks', () => {
    expect(qualityMode('?seed=17', true)).toBe('high');
    expect(qualityMode('?seed=17&quality=auto', true)).toBe('auto');
  });
  it('takes the debug override', () => {
    for (const q of ['high', 'low1', 'low']) expect(qualityMode(`?quality=${q}`, false)).toBe(q);
  });
  it('ignores unknown values', () => {
    expect(qualityMode('?quality=ultra', false)).toBe('auto');
  });
});
