import Client from "mina-signer";
import {
  CI_TEST_ACCOUNT_PUBLIC_KEY,
  CI_TEST_ACCOUNT_PRIVATE_KEY,
} from "../constants";

const TWO_MINA = 2100000000;

const FOUR_MINUTES = 240000;

suite(["@tier2"], "broadcast page", () => {
  it("broadcasts offline txn", () => {
    cy.visit("/broadcast/transaction");

    const client = new Client({ network: "mainnet" });

    // Generate keys
    let keypair = client.genKeys();

    let signedPayment = client.signPayment(
      {
        to: keypair.publicKey,
        from: CI_TEST_ACCOUNT_PUBLIC_KEY,
        amount: 1,
        fee: TWO_MINA,
        nonce: 0,
      },
      CI_TEST_ACCOUNT_PRIVATE_KEY,
    );
    if (client.verifyPayment(signedPayment)) {
      cy.log("Payment was verified successfully");
    }

    cy.get("form textarea").type(JSON.stringify(signedPayment, null, 4), {
      delay: 0,
    });
    cy.get("form textarea").parents("form").submit();

    // blocks are produced every 3 minutes so we are waiting 4 minutes
    cy.wait(FOUR_MINUTES);

    cy.visit(`/commands/user?q-from=${CI_TEST_ACCOUNT_PUBLIC_KEY}`);
    cy.tableColumnValuesEqual(
      "User Commands",
      "From",
      CI_TEST_ACCOUNT_PUBLIC_KEY,
    );
    cy.tableColumnValuesEqual("User Commands", "Status", "Failed");
    cy.tableColumnValuesEqual("User Commands", "Type", "PAYMENT");
  });
});
