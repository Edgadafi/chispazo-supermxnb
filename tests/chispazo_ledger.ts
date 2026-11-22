import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("chispazo_ledger", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.ChispazoLedger as Program<any>;
  const authority = provider.wallet as anchor.Wallet;

  const [merchantPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("merchant"), authority.publicKey.toBuffer()],
    program.programId
  );

  const seq1Buf = Buffer.alloc(8);
  seq1Buf.writeBigUInt64LE(1n, 0);
  const [order1Pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("order"), merchantPda.toBuffer(), seq1Buf],
    program.programId
  );

  const toBytes = (txt: string, len: number) => {
    const arr = new Array(len).fill(0);
    const b = Buffer.from(txt, "utf8");
    for (let i = 0; i < Math.min(b.length, len); i++) arr[i] = b[i];
    return arr;
  };

  it("1. Crea MerchantConfig y PaymentOrder (PDAs relacionadas)", async () => {
    const sigInit = await program.methods
      .initializeMerchant(toBytes("CHISPAZO-01", 16), toBytes("CLABE-HASH-01", 32))
      .accounts({
        merchantConfig: merchantPda,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const sigCreate = await program.methods
      .createPaymentOrder(0, new anchor.BN(15000), toBytes("SPEI-REF-001", 32))
      .accounts({
        merchantConfig: merchantPda,
        paymentOrder: order1Pda,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("sigInit:", sigInit);
    console.log("sigCreate:", sigCreate);

    const orderAcc = await program.account.paymentOrder.fetch(order1Pda);
    expect(orderAcc.merchant.toBase58()).to.equal(merchantPda.toBase58());
    expect(orderAcc.status).to.equal(0);
  });

  it("2. Rechaza cierre cuando la orden sigue Pending", async () => {
    let failed = false;
    try {
      await program.methods
        .closePaymentOrder()
        .accounts({
          merchantConfig: merchantPda,
          paymentOrder: order1Pda,
          authority: authority.publicKey,
        })
        .rpc();
    } catch {
      failed = true;
    }
    expect(failed).to.equal(true);
  });

  it("3. Rechaza modificacion desde una autoridad no autorizada", async () => {
    const intruder = Keypair.generate();
    let failed = false;
    try {
      await program.methods
        .updateOrderStatus(1, toBytes("FAKE-SPEI", 32))
        .accounts({
          merchantConfig: merchantPda,
          paymentOrder: order1Pda,
          authority: intruder.publicKey,
        })
        .signers([intruder])
        .rpc();
    } catch {
      failed = true;
    }
    expect(failed).to.equal(true);
  });

  it("4. Modifica estado a Settled y cierra la cuenta (cierre condicionado)", async () => {
    const sigSettle = await program.methods
      .updateOrderStatus(1, toBytes("SPEI-SETTLED-001", 32))
      .accounts({
        merchantConfig: merchantPda,
        paymentOrder: order1Pda,
        authority: authority.publicKey,
      })
      .rpc();

    const settledAcc = await program.account.paymentOrder.fetch(order1Pda);
    expect(settledAcc.status).to.equal(1);

    const sigClose = await program.methods
      .closePaymentOrder()
      .accounts({
        merchantConfig: merchantPda,
        paymentOrder: order1Pda,
        authority: authority.publicKey,
      })
      .rpc();

    console.log("sigSettle:", sigSettle);
    console.log("sigClose:", sigClose);

    const closedAcc = await program.account.paymentOrder.fetchNullable(order1Pda);
    expect(closedAcc).to.equal(null);
  });
});
