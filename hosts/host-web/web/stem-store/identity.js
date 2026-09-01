/** Canonical stem identity spelling from the session schema. */
export const STEM_SHA256_IDENTITY = /^sha256:([0-9a-f]{64})$/

/**
 * Parse a canonical stem identity and return its lowercase digest.
 *
 * @param {string} identity
 * @returns {string}
 */
export function stemDigest(identity) {
  const match = STEM_SHA256_IDENTITY.exec(identity)
  if (match === null) {
    throw new StemResolverError(
      "stem.identity.invalid",
      `Stem identity is not canonical SHA-256: ${String(identity)}`,
      { identity }
    )
  }
  return match[1]
}

/** @param {string} identity */
export function stemFileName(identity) {
  return `sha256-${stemDigest(identity)}`
}

/** A typed resolver refusal. */
export class StemResolverError extends Error {
  /**
   * @param {string} code
   * @param {string} message
   * @param {Record<string, unknown>} [details]
   * @param {unknown} [cause]
   */
  constructor(code, message, details = {}, cause) {
    super(message, cause === undefined ? undefined : { cause })
    this.name = "StemResolverError"
    this.code = code
    this.details = Object.freeze({ ...details })
  }
}
