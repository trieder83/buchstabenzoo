// Save slot in the browser (GAME-SAVE): the game state is serialised in Rust; the host only
// stores and loads the string. One slot for the whole zoo (levels joined, M5b; profiles:
// Q-011). The M4b/M5a slot of level 1 is read once and migrated by the game (save v1 → v2).
import type { KeyValue } from './ui';

export const SAVE_KEY = 'zoo.save';
/** Slot of the level-1-only saves (M4b/M5a): read when `SAVE_KEY` is empty, then removed. */
export const LEGACY_SAVE_KEY = 'zoo.save.level-1';
/** Hiding places of the last new game (Q-082: the next new game avoids them). */
export const PICKS_KEY = 'zoo.picks';
export const LEGACY_PICKS_KEY = 'zoo.picks.level-1';

/** The subset of the WASM `App` used for saving. */
export interface SaveApp {
  save(): string;
  take_save(): string;
  restore(json: string): boolean;
  new_game(seed: number, avoidJson: string): boolean;
  picks_json(): string;
}

/** Seed of a new game: `?seed=N` in the page URL (tests, debugging), else random. */
export function newGameSeed(search: string, random: () => number = Math.random): number {
  const m = /[?&]seed=(\d+)/.exec(search);
  if (m) return Number(m[1]);
  return Math.floor(random() * 0xffffffff);
}

export interface Removable extends KeyValue {
  removeItem(key: string): void;
}

function read(store: KeyValue | null): string | null {
  try {
    return store?.getItem(SAVE_KEY) ?? store?.getItem(LEGACY_SAVE_KEY) ?? null;
  } catch {
    return null;
  }
}

function write(store: KeyValue | null, json: string): void {
  try {
    store?.setItem(SAVE_KEY, json);
  } catch {
    // storage full or blocked: keep playing without saving
  }
}

export function clearSave(store: Removable | null): void {
  try {
    store?.removeItem(SAVE_KEY);
    store?.removeItem(LEGACY_SAVE_KEY);
  } catch {
    // blocked storage
  }
}

/**
 * Autosave controller: restores at start (a broken save is deleted silently, GAME-SAVE §5),
 * stores due saves every frame and a final save when the page is hidden.
 */
export class SaveSlot {
  private disabled = false;

  constructor(
    private readonly app: SaveApp,
    private readonly store: Removable | null,
  ) {}

  /**
   * Start of the page: restores the stored game, else starts a new one with `seed` that
   * avoids the hiding places of the previous game and remembers its own (GAME-RESCUE §1,
   * Q-082). Returns whether a save was restored.
   */
  start(seed: number): boolean {
    if (this.restore()) return true;
    let avoid = '';
    try {
      avoid = this.store?.getItem(PICKS_KEY) ?? this.store?.getItem(LEGACY_PICKS_KEY) ?? '';
    } catch {
      avoid = '';
    }
    this.app.new_game(seed, avoid);
    try {
      this.store?.setItem(PICKS_KEY, this.app.picks_json());
    } catch {
      // blocked storage: no memory of the picks
    }
    return false;
  }

  /** Restores the stored game before the first frame. Returns whether a save was restored. */
  restore(): boolean {
    const json = read(this.store);
    if (!json) return false;
    if (this.app.restore(json)) {
      // migrated legacy slot: continue in the new slot
      try {
        if (this.store?.getItem(SAVE_KEY) === null) {
          this.store.setItem(SAVE_KEY, this.app.save());
          this.store.removeItem(LEGACY_SAVE_KEY);
        }
      } catch {
        // blocked storage
      }
      return true;
    }
    clearSave(this.store);
    return false;
  }

  /** Per frame: stores a save when the game says one is due. */
  tick(): void {
    if (this.disabled) return;
    const json = this.app.take_save();
    if (json) write(this.store, json);
  }

  /** Page hidden / closed: store the current state. */
  flush(): void {
    if (!this.disabled) write(this.store, this.app.save());
  }

  /** New game: delete the save and stop saving until the page reloads (GAME-SAVE §6). */
  reset(): void {
    this.disabled = true;
    clearSave(this.store);
  }
}
