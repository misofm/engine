const HEX_DIGITS = "0123456789abcdef";

function hexDigit(index: number): string {
  const digit = HEX_DIGITS[index];
  if (digit === undefined) throw new RangeError("hex nibble out of range");
  return digit;
}

export function hexLower(bytes: Uint8Array): string {
  let output = "";
  for (const byte of bytes) {
    output += hexDigit(byte >>> 4) + hexDigit(byte & 0x0f);
  }
  return output;
}
