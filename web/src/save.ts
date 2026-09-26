// Save slot in the browser (GAME-SAVE): the game state is serialised in Rust; the host only
// stores and loads the string. One slot per level for the PoC (profiles: Q-011).
import type { KeyValue } from './ui';

export const SAVE_KEY = 'zoo.save.level-1';

/** The subset of the WASM `App` used for saving. */
export interface SaveApp {
  save(): string;
  take_save(): string;
  restore(json: string): boolean;
}

export interface Removable extends KeyValue {
  removeItem(key: string): void;
}

function read(store: KeyValue | null): string | null {
  try {
    return store?.getItem(SAVE_KEY) ?? null;
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

  /** Restores the stored game before the first frame. Returns whether a save was restored. */
  restore(): boolean {
    const json = read(this.store);
    if (!json) return false;
    if (this.app.restore(json)) return true;
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
