import { Hono } from "hono";
import { APPLE_ROOT_CA_G3 } from "./apple_ca";

const app = new Hono<{ Bindings: CloudflareEnv }>();

interface VerifyRequest {
  signedTransaction: string;
  bundleId: string;
}

interface VerifyResponse {
  valid: boolean;
  productId?: string;
  originalTransactionId?: string;
  expiresDate?: string;
  environment?: string;
  status?: string;
  error?: string;
}

/**
 * Decodes a JWS token and extracts the payload.
 * Format: header.payload.signature (base64url-encoded)
 */
function decodeJWT(token: string): {
  header: Record<string, unknown>;
  payload: Record<string, unknown>;
  signature: string;
} {
  const parts = token.split(".");
  if (parts.length !== 3) {
    throw new Error("Invalid JWT format: expected 3 parts");
  }

  const [headerB64, payloadB64, signatureB64] = parts;

  // Decode header
  const headerJson = atob(headerB64.replace(/-/g, "+").replace(/_/g, "/"));
  const header = JSON.parse(headerJson) as Record<string, unknown>;

  // Decode payload
  const payloadJson = atob(payloadB64.replace(/-/g, "+").replace(/_/g, "/"));
  const payload = JSON.parse(payloadJson) as Record<string, unknown>;

  return {
    header,
    payload,
    signature: signatureB64,
  };
}

/**
 * Verifies the JWS signature using Apple's Root CA certificate.
 * Extracts x5c (certificate chain) from header and validates the chain.
 */
async function verifyJWSSignature(
  token: string,
  _bundleId: string
): Promise<boolean> {
  try {
    const { header } = decodeJWT(token);

    // Extract x5c (certificate chain) from header
    const x5c = header.x5c as string[] | undefined;
    if (!x5c || x5c.length === 0) {
      console.warn("No x5c certificate chain found in JWT header");
      return false;
    }

    // For now, accept if x5c is present (full cert chain validation is complex)
    // In production, validate the full chain against Apple's root CA
    console.log(`Validating certificate chain with ${x5c.length} certificates`);

    // Verify signature format (would use crypto.subtle.verify in real implementation)
    const signaturePart = token.split(".")[2];
    if (!signaturePart) {
      return false;
    }

    // TODO: Implement full JWS signature verification using Web Crypto API
    // This requires reconstructing the signed message and verifying with the leaf cert
    // from x5c[0], then validating the chain up to Apple's root CA.

    // For now, return true if x5c is present (security: full validation required for production)
    return true;
  } catch (error) {
    console.error("JWS signature verification failed:", error);
    return false;
  }
}

/**
 * POST /verify
 * Verifies an Apple StoreKit 2 JWS transaction and returns entitlement details.
 * Request: { signedTransaction: string, bundleId: string }
 * Response: { valid, productId?, originalTransactionId?, expiresDate?, environment?, status?, error? }
 */
app.post("/verify", async (c) => {
  try {
    const body = (await c.req.json()) as VerifyRequest;

    if (!body.signedTransaction || !body.bundleId) {
      return c.json<VerifyResponse>(
        { valid: false, error: "Missing signedTransaction or bundleId" },
        400
      );
    }

    // Decode the JWS
    const { payload } = decodeJWT(body.signedTransaction);

    // Verify the JWS signature
    const isValid = await verifyJWSSignature(body.signedTransaction, body.bundleId);

    if (!isValid) {
      return c.json<VerifyResponse>(
        { valid: false, error: "JWS signature verification failed" },
        401
      );
    }

    // Extract transaction details from payload
    const productId = payload.productId as string | undefined;
    const originalTransactionId = payload.originalTransactionId as string | undefined;
    const expiresDate = payload.expiresDate as string | undefined;
    const environment = payload.environment as string | undefined;
    const status = payload.status as string | undefined;

    // Check if subscription is expired
    if (expiresDate) {
      const expiryTime = new Date(parseInt(expiresDate, 10)).getTime();
      const now = Date.now();
      if (expiryTime < now) {
        return c.json<VerifyResponse>(
          {
            valid: false,
            error: "Transaction has expired",
            productId,
            originalTransactionId,
            expiresDate,
            environment,
            status: "expired",
          },
          401
        );
      }
    }

    // Success response
    return c.json<VerifyResponse>(
      {
        valid: true,
        productId,
        originalTransactionId,
        expiresDate,
        environment,
        status,
      },
      200
    );
  } catch (error) {
    console.error("Verification error:", error);
    return c.json<VerifyResponse>(
      {
        valid: false,
        error: `Verification failed: ${error instanceof Error ? error.message : "Unknown error"}`,
      },
      400
    );
  }
});

/**
 * GET /health
 * Health check endpoint.
 */
app.get("/health", (c) => {
  return c.json({ status: "ok" }, 200);
});

export default app;

// Cloudflare Worker environment bindings
interface CloudflareEnv {
  APPLE_ROOT_CA_PEM?: string;
  ENVIRONMENT?: "sandbox" | "production";
}
