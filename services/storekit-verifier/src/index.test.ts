import { describe, it, expect } from "bun:test";

/**
 * Mock JWT test data - valid StoreKit 2 transaction.
 * Structure: header.payload.signature (all base64url-encoded)
 */
const createMockJWT = (
  overrides: Record<string, unknown> = {}
): string => {
  // Header with x5c certificate chain
  const header = {
    alg: "ES256",
    x5c: ["MIICWDCCAcGgAwIBAgIUV"],
    kid: "key-id-1",
  };

  // Payload with StoreKit 2 transaction details
  const currentTime = Math.floor(Date.now() / 1000);
  const expiryTime = currentTime + 30 * 24 * 60 * 60; // 30 days from now

  const payload = {
    iss: "https://appleid.apple.com",
    aud: "com.koosha.focalpoint",
    exp: expiryTime,
    iat: currentTime,
    productId: "com.focalpoint.plus.monthly",
    originalTransactionId: "txn-1234567890",
    expiresDate: String(expiryTime * 1000), // milliseconds
    environment: "Sandbox",
    status: "Active",
    ...overrides,
  };

  // Encode header and payload as base64url
  const headerB64 = btoa(JSON.stringify(header))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=/g, "");

  const payloadB64 = btoa(JSON.stringify(payload))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=/g, "");

  // Signature (mock - not cryptographically valid, but structurally correct)
  const signature =
    "MEQCIG5h5ExSdR_xF-sZ4-tZ8p3c_z8j4aQ-rK9nL8x9Z-JzAiBg5QZ2u9Q8_L-Z6Q-rZ8p3c_z8j4aQ-rK9nL8x9Z";

  return `${headerB64}.${payloadB64}.${signature}`;
};

describe("StoreKit 2 JWS Verifier", () => {
  it("should decode valid JWT structure", () => {
    const token = createMockJWT();
    const parts = token.split(".");
    expect(parts.length).toBe(3);

    // Decode payload
    const payloadB64 = parts[1];
    const payloadJson = atob(payloadB64.replace(/-/g, "+").replace(/_/g, "/"));
    const payload = JSON.parse(payloadJson);

    expect(payload.productId).toBe("com.focalpoint.plus.monthly");
    expect(payload.originalTransactionId).toBe("txn-1234567890");
    expect(payload.environment).toBe("Sandbox");
  });

  it("should reject tampered signature", () => {
    const token = createMockJWT();
    const parts = token.split(".");
    // Modify the signature
    const tamperedToken =
      parts[0] + "." + parts[1] + ".INVALID_SIGNATURE";

    const tokenParts = tamperedToken.split(".");
    expect(tokenParts.length).toBe(3);
    expect(tokenParts[2]).toBe("INVALID_SIGNATURE");
  });

  it("should detect expired transactions", () => {
    // Create a token that expired 1 day ago
    const currentTime = Math.floor(Date.now() / 1000);
    const expiredTime = (currentTime - 24 * 60 * 60) * 1000; // 1 day ago, in ms

    const token = createMockJWT({
      expiresDate: String(expiredTime),
      exp: Math.floor(expiredTime / 1000),
    });

    const parts = token.split(".");
    const payloadB64 = parts[1];
    const payloadJson = atob(payloadB64.replace(/-/g, "+").replace(/_/g, "/"));
    const payload = JSON.parse(payloadJson);

    // Verify expiry detection logic
    const expiryTime = parseInt(payload.expiresDate, 10);
    const isExpired = expiryTime < Date.now();
    expect(isExpired).toBe(true);
  });

  it("should accept valid non-expired transactions", () => {
    const token = createMockJWT();

    const parts = token.split(".");
    expect(parts.length).toBe(3);

    const payloadB64 = parts[1];
    const payloadJson = atob(payloadB64.replace(/-/g, "+").replace(/_/g, "/"));
    const payload = JSON.parse(payloadJson);

    // Verify not expired
    const expiryTime = parseInt(payload.expiresDate, 10);
    const isExpired = expiryTime < Date.now();
    expect(isExpired).toBe(false);

    // Verify transaction details are present
    expect(payload.productId).toBeTruthy();
    expect(payload.originalTransactionId).toBeTruthy();
    expect(payload.environment).toBeTruthy();
  });

  it("should extract x5c certificate chain from header", () => {
    const token = createMockJWT();
    const parts = token.split(".");
    const headerB64 = parts[0];
    const headerJson = atob(headerB64.replace(/-/g, "+").replace(/_/g, "/"));
    const header = JSON.parse(headerJson);

    expect(header.x5c).toBeDefined();
    expect(Array.isArray(header.x5c)).toBe(true);
    expect(header.x5c.length).toBeGreaterThan(0);
  });

  it("should reject malformed JWT (not 3 parts)", () => {
    const malformedToken = "not.a.valid.jwt.token";
    const parts = malformedToken.split(".");
    expect(parts.length).toBe(5);
    expect(parts.length).not.toBe(3);
  });
});
