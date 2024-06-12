const WHISPERIT_TXN_HASH =
  "CkpZhvEYxb9YFZULWEw61khrHAqGZhTktZSiGQ1CKfe6n2TcnxdgD";
suite(["@tier1"], "user command memo field", () => {
  it("is present and decoded where appropriate", () => {
    cy.visit(`/commands/user?q-txn-hash=${WHISPERIT_TXN_HASH}`);
    cy.aliasTableRows("User Commands", "table-rows");
    cy.get("@table-rows").should("have.length", 1);
    cy.contains("whisperit#8145").should("exist");
    cy.visit(
      `/addresses/accounts/B62qoaMj7u1JzuqXaBByQBL5jzqLguK8e7LHVPdY9LcvvLXK7HPsusD?q-txn-hash=${WHISPERIT_TXN_HASH}`,
    );
    cy.aliasTableRows("User Commands", "table-rows");
    cy.get("@table-rows").should("have.length", 1);
    cy.contains("whisperit#8145").should("exist");
    cy.visit(`/commands/${WHISPERIT_TXN_HASH}`);
    cy.contains("whisperit#8145").should("exist");
  });
});
