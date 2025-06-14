/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export declare class Crypt {
  static base64ToBytes(base64Str: string): Array<number>
  static bytesToBase64(bytes: Uint8Array): string
  static hexToBytes(hex: string): Array<number>
  static bytesToHex(bytes: Uint8Array): string
  static wordsToBytes(words: Array<number>): Array<number>
  static bytesToWords(bytes: Uint8Array): Array<number>
  static randomBytes(len: number): Array<number>
  static rotl(value: number, shift: number): number
  static rotr(value: number, shift: number): number
}
