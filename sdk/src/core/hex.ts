const HEX_DIGITS = "0123456789abcdef";

export function hexLower(bytes: Uint8Array): string {
  let output = "";
  for (const byte of bytes) {
    output += HEX_DIGITS[byte >>> 4] + HEX_DIGITS[byte & 0x0f];
  }
  return output;
}
