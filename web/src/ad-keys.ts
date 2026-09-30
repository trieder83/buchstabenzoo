// Public key(s) that sign the ad campaigns (GAME-ADS "External content", Q-240).
//
// EMPTY until the owner generated a key pair with `tools/ads/keygen.py` and pasted the printed
// public key here (base64, 32 bytes). While the list is empty the game makes NO request for ads
// and every board shows its local placeholder. Two entries allow a key rotation: sign with the
// new key only after a game release that contains it. The PRIVATE key never enters the repo.
export const AD_PUBLIC_KEYS: readonly string[] = [];
