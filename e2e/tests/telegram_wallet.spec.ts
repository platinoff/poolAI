import { test, expect } from "@playwright/test";

const standRoot = process.env.POOLAI_E2E_STAND_ROOT;

const VALID_PUBKEY = "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU";

test.describe("Telegram wallet bind E2E smoke (PH-S139)", () => {
  test.skip(
    !standRoot,
    "requires bin/e2e-playwright.sh --start (POOLAI_E2E_STAND_ROOT)",
  );

  test("POST /api/v1/virtual-nodes/telegram/wallet binds verified Solana payout", async ({
    request,
  }) => {
    const telegramUserId = `e2e-wallet-ok-${Date.now()}`;
    const res = await request.post("/api/v1/virtual-nodes/telegram/wallet", {
      data: {
        telegram_user_id: telegramUserId,
        chat_id: "-1001234567890",
        payout_pubkey: VALID_PUBKEY,
        chain: "solana",
      },
    });
    expect(res.status()).toBe(200);
    const body = await res.json();
    expect(body.wallet.telegram_user_id).toBe(telegramUserId);
    expect(body.wallet.payout_pubkey).toBe(VALID_PUBKEY);
    expect(body.wallet.chain).toBe("solana");
    expect(body.wallet.verified).toBe(true);
  });

  test("POST /api/v1/virtual-nodes/telegram/wallet rejects invalid pubkey with 400", async ({
    request,
  }) => {
    const telegramUserId = `e2e-wallet-bad-${Date.now()}`;
    const res = await request.post("/api/v1/virtual-nodes/telegram/wallet", {
      data: {
        telegram_user_id: telegramUserId,
        chat_id: "-10099",
        payout_pubkey: "not-valid!",
        chain: "solana",
      },
    });
    expect(res.status()).toBe(400);
  });
});
