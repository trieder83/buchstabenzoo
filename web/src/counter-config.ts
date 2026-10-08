// Anonymous counters (counter.ts): the Firebase project and its PUBLIC web API key. The key only identifies the
// project; what can be written is decided by `firestore.rules` (create/increment of /c/{id}, nothing else, no reads).
export const COUNTER_PROJECT = 'letterzoo';
export const COUNTER_API_KEY = 'AIzaSyBPqqYHOXf1x_VRoUPDhO6DvR_MaSe2NPk';

/** Build define (vite.config.ts): short git hash of the build, else the package version. */
declare const __APP_VERSION__: string;
export const APP_VERSION: string = typeof __APP_VERSION__ !== 'undefined' ? __APP_VERSION__ : 'dev';
