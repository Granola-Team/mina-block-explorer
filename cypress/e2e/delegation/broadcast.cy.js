import Client from "mina-signer";
import {
  CI_TEST_ACCOUNT_PUBLIC_KEY,
  CI_TEST_ACCOUNT_PRIVATE_KEY,
} from "../constants";

const ONE_MILLION_NANOMINA = 1000000;

suite(["@tier2"], "broadcast page", () => {
  let nonce = null;

  beforeEach(async () => {
    const response = await fetch("https://api.minasearch.com/graphql", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        query: `
                query AccountsQuery(
                    $limit: Int = 1
                    $query: AccountQueryInput
                ) {
                    accounts(limit: $limit, query: $query) {
                        nonce
                    }
                }
            `,
        variables: {
          limit: 1,
          query: {
            publicKey: CI_TEST_ACCOUNT_PUBLIC_KEY,
          },
        },
        operationName: "AccountsQuery",
      }),
    });
    let res = await response.json();
    nonce = (res.data.accounts[0] || { nonce: 0 }).nonce;
  });

  xit("broadcasts offline txn", () => {
    cy.visit("/broadcast/transaction");

    const client = new Client({ network: "mainnet" });

    // Generate keys
    let keypair = client.genKeys();

    let signedPayment = client.signPayment(
      {
        to: keypair.publicKey,
        from: CI_TEST_ACCOUNT_PUBLIC_KEY,
        amount: 1,
        fee: ONE_MILLION_NANOMINA,
        nonce: nonce + 1,
      },
      CI_TEST_ACCOUNT_PRIVATE_KEY,
    );
    if (client.verifyPayment(signedPayment)) {
      cy.log("Payment was verified successfully");
    }

    cy.intercept(
      "POST",
      "https://api.minaexplorer.com/broadcast/transaction",
    ).as("offline-txn");

    cy.get("form textarea").type(JSON.stringify(signedPayment, null, 4), {
      delay: 0,
    });
    cy.get("form textarea").parents("form").submit();

    cy.get("@offline-txn").its("response.statusCode").should("eq", 201);
  });
});
